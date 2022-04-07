use nix::errno::Errno;
use nix::sys::signal;
use nix::sys::signal::Signal;
use nix::unistd;
use once_cell::sync::OnceCell;
use signal_hook::low_level::channel::Channel as SignalSafeChannel;
use std::error::Error;
use std::iter::Iterator;
use std::os::unix::io::RawFd;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::Duration;
use test_utils::set;
use test_utils::ShadowTest;
use test_utils::TestEnvironment as TestEnv;

const SS_AUTODISARM: libc::c_int = 1 << 31;

fn sigaltstack(new: Option<&libc::stack_t>, old: Option<&mut libc::stack_t>) -> Result<(), Errno> {
    let new = match new {
        Some(r) => r as *const libc::stack_t,
        None => std::ptr::null(),
    };
    let old = match old {
        Some(r) => r as *mut libc::stack_t,
        None => std::ptr::null_mut(),
    };
    Errno::result(unsafe { libc::sigaltstack(new, old) })?;
    Ok(())
}

// Record of having received a signal.
#[derive(Debug, Eq, PartialEq)]
struct Record {
    signal: i32,
    pid: unistd::Pid,
    tid: unistd::Pid,
    info: Option<libc::siginfo_t>,
}

// Global channel to be written from the signal handler.  We use
// signal_hook::low_level::channel::Channel here, which is explicitly designed
// to be async-signal-safe.
fn signal_channel() -> &'static SignalSafeChannel<Record> {
    static INSTANCE: OnceCell<SignalSafeChannel<Record>> = OnceCell::new();
    INSTANCE.get_or_init(|| SignalSafeChannel::new())
}

// Signal handler used throughout. Tests read from `signal_channel` to validate
// properties of the received signal.
extern "C" fn signal_action(signal: i32, info: *mut libc::siginfo_t, _ctx: *mut std::ffi::c_void) {
    // Try to use only async-signal-safe functions. See signal-safety(7).
    //
    // Following this strictly is *very* restrictive, since most apis don't
    // bother to document/guarantee signal safety. Definitely avoid anything
    // known to be non-reentrant, though, including heap allocation.

    let record = Record {
        signal,
        pid: unistd::getpid(),
        tid: unistd::gettid(),
        info: unsafe { info.as_ref().cloned() },
    };
    signal_channel().send(record);
}

// Legacy/simple style handler.
extern "C" fn signal_handler(sig: i32) {
    signal_action(sig, std::ptr::null_mut(), std::ptr::null_mut());
}

fn catchable_signals() -> Box<dyn Iterator<Item = Signal>> {
    Box::new(Signal::iterator().filter(|s| match s {
        // Can't be caught, as per `sigaction(2)`.
        Signal::SIGKILL | Signal::SIGSTOP => false,
        _ => true,
    }))
}

fn tkill(tid: unistd::Pid, signal: Signal) -> Result<(), Errno> {
    Errno::result(unsafe { libc::syscall(libc::SYS_tkill, tid, signal) })?;
    Ok(())
}

fn tgkill(pid: unistd::Pid, tid: unistd::Pid, signal: Signal) -> Result<(), Errno> {
    Errno::result(unsafe { libc::syscall(libc::SYS_tgkill, pid, tid, signal) })?;
    Ok(())
}

// Not exposed in Rust's libc crate.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[allow(non_camel_case_types)]
enum SignalCode {
    SI_TKILL = -6,
    SI_USER = 0,
}

// Tests basic signal delivery to self.
fn test_raise(
    raise_fn: &dyn Fn(Signal),
    expected_code: Option<SignalCode>,
) -> Result<(), Box<dyn Error>> {
    for handler in &[
        signal::SigHandler::SigIgn,
        signal::SigHandler::SigAction(signal_action),
        signal::SigHandler::Handler(signal_handler),
    ] {
        for signal in catchable_signals() {
            println!("Signal {}", signal);
            unsafe {
                signal::sigaction(
                    signal,
                    &signal::SigAction::new(
                        *handler,
                        signal::SaFlags::empty(),
                        signal::SigSet::empty(),
                    ),
                )
                .unwrap()
            };

            raise_fn(signal);

            match handler {
                signal::SigHandler::SigIgn => {
                    // Should be ignored.
                    assert_eq!(signal_channel().recv(), None);
                    continue;
                }
                _ => (),
            };

            // Exactly one signal should have been delivered, synchronously.
            let record = signal_channel().recv().unwrap();
            assert_eq!(signal_channel().recv(), None);

            assert_eq!(record.pid, unistd::getpid());
            assert_eq!(record.tid, unistd::gettid());
            assert_eq!(Signal::try_from(record.signal).unwrap(), signal);

            match handler {
                signal::SigHandler::SigAction(_) => {
                    let info = record.info.unwrap();
                    assert_eq!(Signal::try_from(info.si_signo).unwrap(), signal);
                    if let Some(expected_code) = expected_code {
                        assert_eq!(info.si_code, expected_code as i32);
                    }
                    assert_eq!(
                        unistd::Pid::from_raw(unsafe { info.si_pid() }),
                        unistd::getpid()
                    );
                }
                signal::SigHandler::Handler(_) => {
                    assert_eq!(record.info, None);
                }
                _ => unreachable!(),
            };

            unsafe {
                signal::sigaction(
                    signal,
                    &signal::SigAction::new(
                        signal::SigHandler::SigDfl,
                        signal::SaFlags::empty(),
                        signal::SigSet::empty(),
                    ),
                )
                .unwrap()
            };
        }
    }
    Ok(())
}

// Test process-directed signals with multiple threads.
fn test_process_pending_multithreaded() -> Result<(), Box<dyn Error>> {
    let signal = Signal::SIGUSR1;

    unsafe {
        signal::sigaction(
            signal,
            &signal::SigAction::new(
                signal::SigHandler::Handler(signal_handler),
                signal::SaFlags::empty(),
                signal::SigSet::empty(),
            ),
        )
        .unwrap()
    };

    let run = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let mut threads = Vec::new();
    for _ in 0..10 {
        let run = run.clone();
        threads.push(std::thread::spawn(move || {
            while run.load(std::sync::atomic::Ordering::Relaxed) {
                std::thread::sleep(Duration::from_millis(1));
            }
        }));
    }

    signal::kill(unistd::getpid(), signal).unwrap();

    // Signal should be delivered exactly once, to one of the threads.  When
    // there are other threads with the signal unblocked, POSIX.1 doesn't
    // guarantee that the signal will be delivered to the current thread nor
    // that it will delivered before `kill` returns.
    let mut record = signal_channel().recv();
    while record.is_none() {
        std::thread::sleep(Duration::from_millis(1));
        record = signal_channel().recv();
    }
    let record = record.unwrap();
    assert_eq!(record.pid, unistd::getpid());
    // TODO: check that tid is one of our threads?
    // assert_ne!(record.tid, unistd::gettid());
    assert_eq!(Signal::try_from(record.signal).unwrap(), signal);
    assert_eq!(signal_channel().recv(), None);

    // Shut down
    run.store(false, std::sync::atomic::Ordering::Relaxed);
    for thread in threads {
        thread.join().unwrap();
    }

    unsafe {
        signal::sigaction(
            signal,
            &signal::SigAction::new(
                signal::SigHandler::SigDfl,
                signal::SaFlags::empty(),
                signal::SigSet::empty(),
            ),
        )
        .unwrap()
    };
    Ok(())
}

fn test_sigprocmask() -> Result<(), Box<dyn Error>> {
    for signal in catchable_signals() {
        println!("{}", signal);
        unsafe {
            signal::sigaction(
                signal,
                &signal::SigAction::new(
                    signal::SigHandler::Handler(signal_handler),
                    signal::SaFlags::empty(),
                    signal::SigSet::empty(),
                ),
            )
            .unwrap()
        };

        // Block the signal
        let mut sigset_to_block = signal::SigSet::empty();
        sigset_to_block.add(signal);
        signal::sigprocmask(signal::SigmaskHow::SIG_BLOCK, Some(&sigset_to_block), None).unwrap();

        // Validate that the signal is in the returned mask.
        let mut current_sigset = signal::SigSet::empty();
        signal::sigprocmask(
            signal::SigmaskHow::SIG_BLOCK,
            None,
            Some(&mut current_sigset),
        )
        .unwrap();
        assert!(current_sigset.contains(signal));

        // Raise multiple times; won't be delivered yet because it's masked,
        // and subsequent signals aren't queued.
        signal::raise(signal).unwrap();
        signal::raise(signal).unwrap();
        signal::raise(signal).unwrap();

        // Should be nothing delivered yet, since the signal is unblocked.
        assert_eq!(signal_channel().recv(), None);

        // Unblock. The pending signal should be delivered synchronously.
        signal::sigprocmask(
            signal::SigmaskHow::SIG_UNBLOCK,
            Some(&sigset_to_block),
            None,
        )
        .unwrap();

        // Exactly one signal should have been delivered, synchronously.
        let record = signal_channel().recv().unwrap();
        assert_eq!(record.pid, unistd::getpid());
        assert_eq!(record.tid, unistd::gettid());
        assert_eq!(Signal::try_from(record.signal).unwrap(), signal);
        assert_eq!(signal_channel().recv(), None);

        unsafe {
            signal::sigaction(
                signal,
                &signal::SigAction::new(
                    signal::SigHandler::SigDfl,
                    signal::SaFlags::empty(),
                    signal::SigSet::empty(),
                ),
            )
            .unwrap()
        };
    }
    Ok(())
}

fn test_send_to_thread_and_process() -> Result<(), Box<dyn Error>> {
    let signal = Signal::SIGUSR1;
    unsafe {
        signal::sigaction(
            signal,
            &signal::SigAction::new(
                signal::SigHandler::Handler(signal_handler),
                signal::SaFlags::empty(),
                signal::SigSet::empty(),
            ),
        )
        .unwrap()
    };

    // Block the signal
    let mut sigset_to_block = signal::SigSet::empty();
    sigset_to_block.add(signal);
    signal::sigprocmask(signal::SigmaskHow::SIG_BLOCK, Some(&sigset_to_block), None).unwrap();

    // Raising should set it pending at the process level.
    signal::kill(unistd::getpid(), signal).unwrap();

    // tkill should *also* set it pending at the thread level.
    tkill(unistd::gettid(), signal).unwrap();

    // Should be nothing delivered yet, since the signal is unblocked.
    assert_eq!(signal_channel().recv(), None);

    // Unblock. The pending signal should be delivered synchronously.
    signal::sigprocmask(
        signal::SigmaskHow::SIG_UNBLOCK,
        Some(&sigset_to_block),
        None,
    )
    .unwrap();

    // Should be delivered exactly twice.
    signal_channel().recv().unwrap();
    signal_channel().recv().unwrap();
    assert_eq!(signal_channel().recv(), None);

    unsafe {
        signal::sigaction(
            signal,
            &signal::SigAction::new(
                signal::SigHandler::SigDfl,
                signal::SaFlags::empty(),
                signal::SigSet::empty(),
            ),
        )
        .unwrap()
    };
    Ok(())
}

struct BlockedThread {
    handle: std::thread::JoinHandle<Result<usize, Errno>>,
    write_fd: RawFd,
    tid: unistd::Pid,
}

extern "C" fn nop_signal_handler(_sig: i32) {}

impl BlockedThread {
    pub fn new() -> Self {
        let (read_fd, write_fd) = unistd::pipe().unwrap();
        let (tid_sender, tid_receiver) = channel();
        let handle = std::thread::spawn(move || {
            tid_sender.send(unistd::gettid()).unwrap();
            let mut buf = [0; 100];
            unistd::read(read_fd, &mut buf)
        });
        let tid = tid_receiver.recv().unwrap();
        // Wait until the thread is blocked in `read` (hopefully).  Ideally we'd
        // use some syscall that atomically blocks the current thread and
        // changes some observable state in other threads, so that we could wait
        // until we knew the thread was blocked. I don't know of any such
        // syscall that other threads can check without side effects.
        std::thread::sleep(Duration::from_millis(10));
        Self {
            handle,
            write_fd,
            tid,
        }
    }
}

fn test_handled_tkill_interrupts_syscall() -> Result<(), Box<dyn Error>> {
    let signal = Signal::SIGUSR1;
    unsafe {
        signal::sigaction(
            signal,
            &signal::SigAction::new(
                signal::SigHandler::Handler(nop_signal_handler),
                signal::SaFlags::empty(),
                signal::SigSet::empty(),
            ),
        )
        .unwrap()
    };

    let blocked_thread = BlockedThread::new();
    tkill(blocked_thread.tid, signal).unwrap();
    assert_eq!(blocked_thread.handle.join().unwrap(), Err(Errno::EINTR));

    Ok(())
}

fn test_ignored_tkill_doesnt_interrupt_syscall() -> Result<(), Box<dyn Error>> {
    let signal = Signal::SIGUSR1;
    unsafe {
        signal::sigaction(
            signal,
            &signal::SigAction::new(
                signal::SigHandler::SigIgn,
                signal::SaFlags::empty(),
                signal::SigSet::empty(),
            ),
        )
        .unwrap()
    };

    let blocked_thread = BlockedThread::new();
    tkill(blocked_thread.tid, signal).unwrap();
    // Give some time for the thread to (incorrectly) interrupt.
    std::thread::sleep(Duration::from_millis(10));
    // Write so that the thread can finish.
    unistd::write(blocked_thread.write_fd, &[0]).unwrap();
    assert_eq!(blocked_thread.handle.join().unwrap(), Ok(1));

    Ok(())
}

fn test_default_ignored_tkill_doesnt_interrupt_syscall() -> Result<(), Box<dyn Error>> {
    // Default action of this signal is to ignore.
    let signal = Signal::SIGURG;

    unsafe {
        signal::sigaction(
            signal,
            &signal::SigAction::new(
                // Explicity set to default action.
                signal::SigHandler::SigDfl,
                signal::SaFlags::empty(),
                signal::SigSet::empty(),
            ),
        )
        .unwrap()
    };

    let blocked_thread = BlockedThread::new();
    tkill(blocked_thread.tid, signal).unwrap();
    // Give some time for the thread to (incorrectly) interrupt.
    std::thread::sleep(Duration::from_millis(10));
    // Write so that the thread can finish.
    unistd::write(blocked_thread.write_fd, &[0]).unwrap();
    assert_eq!(blocked_thread.handle.join().unwrap(), Ok(1));

    Ok(())
}
fn test_handled_kill_interrupts_syscall() -> Result<(), Box<dyn Error>> {
    let signal = Signal::SIGUSR1;
    unsafe {
        signal::sigaction(
            signal,
            &signal::SigAction::new(
                signal::SigHandler::Handler(nop_signal_handler),
                signal::SaFlags::empty(),
                signal::SigSet::empty(),
            ),
        )
        .unwrap()
    };

    let blocked_thread = BlockedThread::new();

    // Block the signal in the current thread, so that it'll interrupt the other.
    let mut sigset_to_block = signal::SigSet::empty();
    sigset_to_block.add(signal);
    signal::sigprocmask(signal::SigmaskHow::SIG_BLOCK, Some(&sigset_to_block), None).unwrap();

    // Send a *process* directed signal.
    signal::kill(unistd::getpid(), signal).unwrap();
    assert_eq!(blocked_thread.handle.join().unwrap(), Err(Errno::EINTR));

    signal::sigprocmask(
        signal::SigmaskHow::SIG_UNBLOCK,
        Some(&sigset_to_block),
        None,
    )
    .unwrap();

    Ok(())
}

// Record of having received a signal.
#[derive(Debug)]
struct SigaltstackRecord {
    rsp: usize,
    altstack: libc::stack_t,
}

// Normally the void* inside libc::stack_t prevents SigaltstackRecord from being
// `Send`. We never dereference the pointer, though.
unsafe impl Send for SigaltstackRecord {}

// Global channel to be written from the signal handler.  We use
// signal_hook::low_level::channel::Channel here, which is explicitly designed
// to be async-signal-safe.
fn sigaltstack_channel() -> &'static SignalSafeChannel<SigaltstackRecord> {
    static INSTANCE: OnceCell<SignalSafeChannel<SigaltstackRecord>> = OnceCell::new();
    INSTANCE.get_or_init(|| SignalSafeChannel::new())
}

extern "C" fn sigaltstack_action(
    _signal: i32,
    _info: *mut libc::siginfo_t,
    _ctx: *mut std::ffi::c_void,
) {
    // Try to use only async-signal-safe functions. See signal-safety(7).
    //
    // Following this strictly is *very* restrictive, since most apis don't
    // bother to document/guarantee signal safety. Definitely avoid anything
    // known to be non-reentrant, though, including heap allocation.

    // Get address of a stack-allocated value, which we use to detect which
    // stack the handler is running on. Using assembly to get `rsp` directly
    // might be a little nicer, but is currently unstable in Rust.
    let stack_var = 0u64;
    let rsp = &stack_var as *const u64 as usize;

    let mut altstack = libc::stack_t {
        ss_sp: std::ptr::null_mut(),
        ss_flags: 0,
        ss_size: 0,
    };
    sigaltstack(None, Some(&mut altstack)).unwrap();

    let record = SigaltstackRecord { rsp, altstack };
    sigaltstack_channel().send(record);
}

fn test_sigaltstack_unconfigured() -> Result<(), Box<dyn Error>> {
    let signal = Signal::SIGUSR1;
    unsafe {
        signal::sigaction(
            signal,
            &signal::SigAction::new(
                signal::SigHandler::SigAction(sigaltstack_action),
                signal::SaFlags::empty(),
                signal::SigSet::empty(),
            ),
        )
        .unwrap()
    };

    let altstack = libc::stack_t {
        ss_sp: std::ptr::null_mut(),
        ss_flags: libc::SS_DISABLE,
        ss_size: 0,
    };
    sigaltstack(Some(&altstack), None)?;

    // No altstack configured.
    signal::raise(signal).unwrap();
    let record = sigaltstack_channel().recv().unwrap();
    assert_eq!(record.altstack, altstack);

    Ok(())
}

fn test_sigaltstack_configured_but_unused() -> Result<(), Box<dyn Error>> {
    let signal = Signal::SIGUSR1;
    unsafe {
        signal::sigaction(
            signal,
            &signal::SigAction::new(
                signal::SigHandler::SigAction(sigaltstack_action),
                signal::SaFlags::empty(),
                signal::SigSet::empty(),
            ),
        )
        .unwrap()
    };

    // Configure an altstack.
    const STACK_SZ: usize = 1 << 20;
    let mut stack_space = Box::new([0u8; STACK_SZ]);
    let stack_range =
        (&stack_space[0] as *const u8 as usize)..(&stack_space[0] as *const u8 as usize + STACK_SZ);
    let altstack = libc::stack_t {
        ss_sp: &mut stack_space[0] as *mut u8 as *mut libc::c_void,
        ss_flags: 0,
        ss_size: STACK_SZ,
    };
    sigaltstack(Some(&altstack), None)?;

    // Should see the configured altstack in the handler.
    signal::raise(signal).unwrap();
    let record = sigaltstack_channel().recv().unwrap();
    assert_eq!(record.altstack, altstack);

    // Handler *shouldn't* be running on the altstack, since it was registered
    // without SA_ONSTACK.
    assert!(!stack_range.contains(&record.rsp));

    Ok(())
}

fn test_sigaltstack_used() -> Result<(), Box<dyn Error>> {
    let signal = Signal::SIGUSR1;
    unsafe {
        signal::sigaction(
            signal,
            &signal::SigAction::new(
                signal::SigHandler::SigAction(sigaltstack_action),
                signal::SaFlags::SA_ONSTACK,
                signal::SigSet::empty(),
            ),
        )
        .unwrap()
    };

    // Configure an altstack.
    const STACK_SZ: usize = 1 << 20;
    let mut stack_space = Box::new([0u8; STACK_SZ]);
    let stack_range =
        (&stack_space[0] as *const u8 as usize)..(&stack_space[0] as *const u8 as usize + STACK_SZ);
    let altstack = libc::stack_t {
        ss_sp: &mut stack_space[0] as *mut u8 as *mut libc::c_void,
        ss_flags: 0,
        ss_size: STACK_SZ,
    };
    sigaltstack(Some(&altstack), None)?;

    // Should see the configured altstack in the handler.
    signal::raise(signal).unwrap();
    let record = sigaltstack_channel().recv().unwrap();

    // Handler *should* be running on the altstack
    let mut expected_stack = altstack;
    expected_stack.ss_flags |= libc::SS_ONSTACK;
    assert_eq!(record.altstack, expected_stack);
    assert!(stack_range.contains(&record.rsp));

    Ok(())
}

fn test_sigaltstack_autodisarm() -> Result<(), Box<dyn Error>> {
    let signal = Signal::SIGUSR1;
    unsafe {
        signal::sigaction(
            signal,
            &signal::SigAction::new(
                signal::SigHandler::SigAction(sigaltstack_action),
                signal::SaFlags::SA_ONSTACK,
                signal::SigSet::empty(),
            ),
        )
        .unwrap()
    };

    // Configure an altstack.
    const STACK_SZ: usize = 1 << 20;
    let mut stack_space = Box::new([0u8; STACK_SZ]);
    let stack_range =
        (&stack_space[0] as *const u8 as usize)..(&stack_space[0] as *const u8 as usize + STACK_SZ);
    let altstack = libc::stack_t {
        ss_sp: &mut stack_space[0] as *mut u8 as *mut libc::c_void,
        ss_flags: SS_AUTODISARM,
        ss_size: STACK_SZ,
    };
    sigaltstack(Some(&altstack), None)?;

    // Should see the configured altstack in the handler.
    signal::raise(signal).unwrap();
    let record = sigaltstack_channel().recv().unwrap();

    // Handler should be running on the altstack.
    assert!(stack_range.contains(&record.rsp));

    // The altstack config should be disarmed while the handler was running.
    assert_eq!(
        record.altstack,
        libc::stack_t {
            ss_sp: std::ptr::null_mut(),
            ss_flags: libc::SS_DISABLE,
            ss_size: 0
        }
    );

    // The altstack config should be restored now that the handler has returned.
    let mut final_altstack = libc::stack_t {
        ss_sp: std::ptr::null_mut(),
        ss_flags: 0,
        ss_size: 0,
    };
    sigaltstack(None, Some(&mut final_altstack)).unwrap();
    assert_eq!(final_altstack, altstack);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // should we restrict the tests we run?
    let filter_shadow_passing = std::env::args().any(|x| x == "--shadow-passing");
    let filter_libc_passing = std::env::args().any(|x| x == "--libc-passing");
    // should we summarize the results rather than exit on a failed test
    let summarize = std::env::args().any(|x| x == "--summarize");

    let all_envs = set![TestEnv::Libc, TestEnv::Shadow];
    let mut tests: Vec<test_utils::ShadowTest<(), Box<dyn Error>>> = vec![
        ShadowTest::new(
            "raise",
            &|| {
                test_raise(
                    &|s| signal::raise(s).unwrap(),
                    // The documentation for `raise` is ambiguous whether the signal is
                    // thread-directed or process-directed in a single-threaded program,
                    // and different versions of libc have different behavior. Therefore
                    // we don't check the SignalCode.
                    None,
                )
            },
            all_envs.clone(),
        ),
        ShadowTest::new(
            "raise via kill",
            &|| {
                test_raise(
                    &|s| signal::kill(unistd::getpid(), s).unwrap(),
                    Some(SignalCode::SI_USER),
                )
            },
            all_envs.clone(),
        ),
        ShadowTest::new(
            "raise via tkill",
            &|| {
                test_raise(
                    &|s| tkill(unistd::gettid(), s).unwrap(),
                    Some(SignalCode::SI_TKILL),
                )
            },
            all_envs.clone(),
        ),
        ShadowTest::new(
            "raise via tgkill",
            &|| {
                test_raise(
                    &|s| tgkill(unistd::getpid(), unistd::gettid(), s).unwrap(),
                    Some(SignalCode::SI_TKILL),
                )
            },
            all_envs.clone(),
        ),
        ShadowTest::new("sigprocmask", test_sigprocmask, all_envs.clone()),
        ShadowTest::new(
            "send to thread and process",
            test_send_to_thread_and_process,
            all_envs.clone(),
        ),
        ShadowTest::new(
            "raise process targeted signal multithreaded",
            test_process_pending_multithreaded,
            all_envs.clone(),
        ),
        ShadowTest::new(
            "handled tkill interrupts syscall",
            test_handled_tkill_interrupts_syscall,
            all_envs.clone(),
        ),
        ShadowTest::new(
            "ignored tkill doesn't interrupt syscall",
            test_ignored_tkill_doesnt_interrupt_syscall,
            all_envs.clone(),
        ),
        ShadowTest::new(
            "default-ignored signal doesn't interrupt syscall",
            test_default_ignored_tkill_doesnt_interrupt_syscall,
            all_envs.clone(),
        ),
        ShadowTest::new(
            "handled kill interrupts syscall",
            test_handled_kill_interrupts_syscall,
            all_envs.clone(),
        ),
        ShadowTest::new(
            "sigaltstack unconfigured",
            test_sigaltstack_unconfigured,
            all_envs.clone(),
        ),
        ShadowTest::new(
            "sigaltstack unused",
            test_sigaltstack_configured_but_unused,
            all_envs.clone(),
        ),
        ShadowTest::new("sigaltstack used", test_sigaltstack_used, all_envs.clone()),
        ShadowTest::new(
            "sigaltstack autodisarm",
            test_sigaltstack_autodisarm,
            all_envs.clone(),
        ),
    ];

    if filter_shadow_passing {
        tests = tests
            .into_iter()
            .filter(|x| x.passing(TestEnv::Shadow))
            .collect()
    }
    if filter_libc_passing {
        tests = tests
            .into_iter()
            .filter(|x| x.passing(TestEnv::Libc))
            .collect()
    }

    test_utils::run_tests(&tests, summarize)?;

    println!("Success.");
    Ok(())
}