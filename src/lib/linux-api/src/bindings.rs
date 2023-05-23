/* automatically generated by rust-bindgen 0.65.1 */

/* Build script: ./gen-kernel-bindings.sh */
/* Kernel tag: v6.2 */

pub const SIGHUP: u32 = 1;
pub const SIGINT: u32 = 2;
pub const SIGQUIT: u32 = 3;
pub const SIGILL: u32 = 4;
pub const SIGTRAP: u32 = 5;
pub const SIGABRT: u32 = 6;
pub const SIGIOT: u32 = 6;
pub const SIGBUS: u32 = 7;
pub const SIGFPE: u32 = 8;
pub const SIGKILL: u32 = 9;
pub const SIGUSR1: u32 = 10;
pub const SIGSEGV: u32 = 11;
pub const SIGUSR2: u32 = 12;
pub const SIGPIPE: u32 = 13;
pub const SIGALRM: u32 = 14;
pub const SIGTERM: u32 = 15;
pub const SIGSTKFLT: u32 = 16;
pub const SIGCHLD: u32 = 17;
pub const SIGCONT: u32 = 18;
pub const SIGSTOP: u32 = 19;
pub const SIGTSTP: u32 = 20;
pub const SIGTTIN: u32 = 21;
pub const SIGTTOU: u32 = 22;
pub const SIGURG: u32 = 23;
pub const SIGXCPU: u32 = 24;
pub const SIGXFSZ: u32 = 25;
pub const SIGVTALRM: u32 = 26;
pub const SIGPROF: u32 = 27;
pub const SIGWINCH: u32 = 28;
pub const SIGIO: u32 = 29;
pub const SIGPOLL: u32 = 29;
pub const SIGPWR: u32 = 30;
pub const SIGSYS: u32 = 31;
pub const SIGUNUSED: u32 = 31;
pub const SIGRTMIN: u32 = 32;
pub const SA_RESTORER: u32 = 67108864;
pub const SIGSTKSZ: u32 = 8192;
pub const SA_NOCLDSTOP: u32 = 1;
pub const SA_NOCLDWAIT: u32 = 2;
pub const SA_SIGINFO: u32 = 4;
pub const SA_UNSUPPORTED: u32 = 1024;
pub const SA_EXPOSE_TAGBITS: u32 = 2048;
pub const SA_ONSTACK: u32 = 134217728;
pub const SA_RESTART: u32 = 268435456;
pub const SA_NODEFER: u32 = 1073741824;
pub const SA_RESETHAND: u32 = 2147483648;
pub const SA_NOMASK: u32 = 1073741824;
pub const SA_ONESHOT: u32 = 2147483648;
pub const SIG_BLOCK: u32 = 0;
pub const SIG_UNBLOCK: u32 = 1;
pub const SIG_SETMASK: u32 = 2;
pub const SIGEV_SIGNAL: u32 = 0;
pub const SIGEV_NONE: u32 = 1;
pub const SIGEV_THREAD: u32 = 2;
pub const SIGEV_THREAD_ID: u32 = 4;
pub const SIGEV_MAX_SIZE: u32 = 64;
pub type __u32 = ::core::ffi::c_uint;
pub type __kernel_long_t = ::core::ffi::c_long;
pub type __kernel_pid_t = ::core::ffi::c_int;
pub type __kernel_uid32_t = ::core::ffi::c_uint;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::core::ffi::c_int;
pub type sigset_t = ::core::ffi::c_ulong;
pub type __signalfn_t = ::core::option::Option<unsafe extern "C" fn(arg1: ::core::ffi::c_int)>;
pub type __sighandler_t = __signalfn_t;
pub type __restorefn_t = ::core::option::Option<unsafe extern "C" fn()>;
pub type __sigrestore_t = __restorefn_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_flags: ::core::ffi::c_ulong,
    pub sa_restorer: __sigrestore_t,
    pub sa_mask: sigset_t,
}
#[test]
fn bindgen_test_layout_sigaction() {
    const UNINIT: ::core::mem::MaybeUninit<sigaction> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<sigaction>(),
        32usize,
        concat!("Size of: ", stringify!(sigaction))
    );
    assert_eq!(
        ::core::mem::align_of::<sigaction>(),
        8usize,
        concat!("Alignment of ", stringify!(sigaction))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).sa_handler) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigaction),
            "::",
            stringify!(sa_handler)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).sa_flags) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sigaction),
            "::",
            stringify!(sa_flags)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).sa_restorer) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(sigaction),
            "::",
            stringify!(sa_restorer)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).sa_mask) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(sigaction),
            "::",
            stringify!(sa_mask)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
#[test]
fn bindgen_test_layout_sigval() {
    const UNINIT: ::core::mem::MaybeUninit<sigval> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<sigval>(),
        8usize,
        concat!("Size of: ", stringify!(sigval))
    );
    assert_eq!(
        ::core::mem::align_of::<sigval>(),
        8usize,
        concat!("Alignment of ", stringify!(sigval))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).sival_int) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigval),
            "::",
            stringify!(sival_int)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).sival_ptr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigval),
            "::",
            stringify!(sival_ptr)
        )
    );
}
pub type sigval_t = sigval;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __sifields {
    pub _kill: __sifields__bindgen_ty_1,
    pub _timer: __sifields__bindgen_ty_2,
    pub _rt: __sifields__bindgen_ty_3,
    pub _sigchld: __sifields__bindgen_ty_4,
    pub _sigfault: __sifields__bindgen_ty_5,
    pub _sigpoll: __sifields__bindgen_ty_6,
    pub _sigsys: __sifields__bindgen_ty_7,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sifields__bindgen_ty_1 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
}
#[test]
fn bindgen_test_layout___sifields__bindgen_ty_1() {
    const UNINIT: ::core::mem::MaybeUninit<__sifields__bindgen_ty_1> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<__sifields__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(__sifields__bindgen_ty_1))
    );
    assert_eq!(
        ::core::mem::align_of::<__sifields__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(__sifields__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._pid) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_1),
            "::",
            stringify!(_pid)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._uid) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_1),
            "::",
            stringify!(_uid)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sifields__bindgen_ty_2 {
    pub _tid: __kernel_timer_t,
    pub _overrun: ::core::ffi::c_int,
    pub _sigval: sigval_t,
    pub _sys_private: ::core::ffi::c_int,
}
#[test]
fn bindgen_test_layout___sifields__bindgen_ty_2() {
    const UNINIT: ::core::mem::MaybeUninit<__sifields__bindgen_ty_2> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<__sifields__bindgen_ty_2>(),
        24usize,
        concat!("Size of: ", stringify!(__sifields__bindgen_ty_2))
    );
    assert_eq!(
        ::core::mem::align_of::<__sifields__bindgen_ty_2>(),
        8usize,
        concat!("Alignment of ", stringify!(__sifields__bindgen_ty_2))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._tid) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_2),
            "::",
            stringify!(_tid)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._overrun) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_2),
            "::",
            stringify!(_overrun)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._sigval) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_2),
            "::",
            stringify!(_sigval)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._sys_private) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_2),
            "::",
            stringify!(_sys_private)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sifields__bindgen_ty_3 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _sigval: sigval_t,
}
#[test]
fn bindgen_test_layout___sifields__bindgen_ty_3() {
    const UNINIT: ::core::mem::MaybeUninit<__sifields__bindgen_ty_3> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<__sifields__bindgen_ty_3>(),
        16usize,
        concat!("Size of: ", stringify!(__sifields__bindgen_ty_3))
    );
    assert_eq!(
        ::core::mem::align_of::<__sifields__bindgen_ty_3>(),
        8usize,
        concat!("Alignment of ", stringify!(__sifields__bindgen_ty_3))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._pid) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_3),
            "::",
            stringify!(_pid)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._uid) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_3),
            "::",
            stringify!(_uid)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._sigval) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_3),
            "::",
            stringify!(_sigval)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sifields__bindgen_ty_4 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _status: ::core::ffi::c_int,
    pub _utime: __kernel_clock_t,
    pub _stime: __kernel_clock_t,
}
#[test]
fn bindgen_test_layout___sifields__bindgen_ty_4() {
    const UNINIT: ::core::mem::MaybeUninit<__sifields__bindgen_ty_4> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<__sifields__bindgen_ty_4>(),
        32usize,
        concat!("Size of: ", stringify!(__sifields__bindgen_ty_4))
    );
    assert_eq!(
        ::core::mem::align_of::<__sifields__bindgen_ty_4>(),
        8usize,
        concat!("Alignment of ", stringify!(__sifields__bindgen_ty_4))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._pid) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_4),
            "::",
            stringify!(_pid)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._uid) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_4),
            "::",
            stringify!(_uid)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._status) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_4),
            "::",
            stringify!(_status)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._utime) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_4),
            "::",
            stringify!(_utime)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._stime) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_4),
            "::",
            stringify!(_stime)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sifields__bindgen_ty_5 {
    pub _addr: *mut ::core::ffi::c_void,
    pub __bindgen_anon_1: __sifields__bindgen_ty_5__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __sifields__bindgen_ty_5__bindgen_ty_1 {
    pub _trapno: ::core::ffi::c_int,
    pub _addr_lsb: ::core::ffi::c_short,
    pub _addr_bnd: __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1,
    pub _addr_pkey: __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2,
    pub _perf: __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 {
    pub _dummy_bnd: [::core::ffi::c_char; 8usize],
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[test]
fn bindgen_test_layout___sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1() {
    const UNINIT: ::core::mem::MaybeUninit<__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1>(),
        24usize,
        concat!(
            "Size of: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::core::mem::align_of::<__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._dummy_bnd) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(_dummy_bnd)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._lower) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(_lower)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._upper) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(_upper)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2 {
    pub _dummy_pkey: [::core::ffi::c_char; 8usize],
    pub _pkey: __u32,
}
#[test]
fn bindgen_test_layout___sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2() {
    const UNINIT: ::core::mem::MaybeUninit<__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2>(),
        12usize,
        concat!(
            "Size of: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::core::mem::align_of::<__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._dummy_pkey) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(_dummy_pkey)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._pkey) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(_pkey)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3 {
    pub _data: ::core::ffi::c_ulong,
    pub _type: __u32,
    pub _flags: __u32,
}
#[test]
fn bindgen_test_layout___sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3() {
    const UNINIT: ::core::mem::MaybeUninit<__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3>(),
        16usize,
        concat!(
            "Size of: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::core::mem::align_of::<__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(_data)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._type) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(_type)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._flags) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(_flags)
        )
    );
}
#[test]
fn bindgen_test_layout___sifields__bindgen_ty_5__bindgen_ty_1() {
    const UNINIT: ::core::mem::MaybeUninit<__sifields__bindgen_ty_5__bindgen_ty_1> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<__sifields__bindgen_ty_5__bindgen_ty_1>(),
        24usize,
        concat!(
            "Size of: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::core::mem::align_of::<__sifields__bindgen_ty_5__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._trapno) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1),
            "::",
            stringify!(_trapno)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._addr_lsb) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1),
            "::",
            stringify!(_addr_lsb)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._addr_bnd) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1),
            "::",
            stringify!(_addr_bnd)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._addr_pkey) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1),
            "::",
            stringify!(_addr_pkey)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._perf) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_5__bindgen_ty_1),
            "::",
            stringify!(_perf)
        )
    );
}
#[test]
fn bindgen_test_layout___sifields__bindgen_ty_5() {
    const UNINIT: ::core::mem::MaybeUninit<__sifields__bindgen_ty_5> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<__sifields__bindgen_ty_5>(),
        32usize,
        concat!("Size of: ", stringify!(__sifields__bindgen_ty_5))
    );
    assert_eq!(
        ::core::mem::align_of::<__sifields__bindgen_ty_5>(),
        8usize,
        concat!("Alignment of ", stringify!(__sifields__bindgen_ty_5))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._addr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_5),
            "::",
            stringify!(_addr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sifields__bindgen_ty_6 {
    pub _band: ::core::ffi::c_long,
    pub _fd: ::core::ffi::c_int,
}
#[test]
fn bindgen_test_layout___sifields__bindgen_ty_6() {
    const UNINIT: ::core::mem::MaybeUninit<__sifields__bindgen_ty_6> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<__sifields__bindgen_ty_6>(),
        16usize,
        concat!("Size of: ", stringify!(__sifields__bindgen_ty_6))
    );
    assert_eq!(
        ::core::mem::align_of::<__sifields__bindgen_ty_6>(),
        8usize,
        concat!("Alignment of ", stringify!(__sifields__bindgen_ty_6))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._band) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_6),
            "::",
            stringify!(_band)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._fd) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_6),
            "::",
            stringify!(_fd)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sifields__bindgen_ty_7 {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}
#[test]
fn bindgen_test_layout___sifields__bindgen_ty_7() {
    const UNINIT: ::core::mem::MaybeUninit<__sifields__bindgen_ty_7> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<__sifields__bindgen_ty_7>(),
        16usize,
        concat!("Size of: ", stringify!(__sifields__bindgen_ty_7))
    );
    assert_eq!(
        ::core::mem::align_of::<__sifields__bindgen_ty_7>(),
        8usize,
        concat!("Alignment of ", stringify!(__sifields__bindgen_ty_7))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._call_addr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_7),
            "::",
            stringify!(_call_addr)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._syscall) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_7),
            "::",
            stringify!(_syscall)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._arch) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields__bindgen_ty_7),
            "::",
            stringify!(_arch)
        )
    );
}
#[test]
fn bindgen_test_layout___sifields() {
    const UNINIT: ::core::mem::MaybeUninit<__sifields> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<__sifields>(),
        32usize,
        concat!("Size of: ", stringify!(__sifields))
    );
    assert_eq!(
        ::core::mem::align_of::<__sifields>(),
        8usize,
        concat!("Alignment of ", stringify!(__sifields))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._kill) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields),
            "::",
            stringify!(_kill)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._timer) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields),
            "::",
            stringify!(_timer)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._rt) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields),
            "::",
            stringify!(_rt)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._sigchld) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields),
            "::",
            stringify!(_sigchld)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._sigfault) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields),
            "::",
            stringify!(_sigfault)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._sigpoll) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields),
            "::",
            stringify!(_sigpoll)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._sigsys) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sifields),
            "::",
            stringify!(_sigsys)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siginfo {
    pub __bindgen_anon_1: siginfo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union siginfo__bindgen_ty_1 {
    pub __bindgen_anon_1: siginfo__bindgen_ty_1__bindgen_ty_1,
    pub _si_pad: [::core::ffi::c_int; 32usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siginfo__bindgen_ty_1__bindgen_ty_1 {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub _sifields: __sifields,
}
#[test]
fn bindgen_test_layout_siginfo__bindgen_ty_1__bindgen_ty_1() {
    const UNINIT: ::core::mem::MaybeUninit<siginfo__bindgen_ty_1__bindgen_ty_1> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<siginfo__bindgen_ty_1__bindgen_ty_1>(),
        48usize,
        concat!("Size of: ", stringify!(siginfo__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::core::mem::align_of::<siginfo__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(siginfo__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).si_signo) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(siginfo__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(si_signo)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).si_errno) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(siginfo__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(si_errno)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).si_code) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(siginfo__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(si_code)
        )
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._sifields) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(siginfo__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(_sifields)
        )
    );
}
#[test]
fn bindgen_test_layout_siginfo__bindgen_ty_1() {
    const UNINIT: ::core::mem::MaybeUninit<siginfo__bindgen_ty_1> =
        ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::core::mem::size_of::<siginfo__bindgen_ty_1>(),
        128usize,
        concat!("Size of: ", stringify!(siginfo__bindgen_ty_1))
    );
    assert_eq!(
        ::core::mem::align_of::<siginfo__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(siginfo__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr)._si_pad) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(siginfo__bindgen_ty_1),
            "::",
            stringify!(_si_pad)
        )
    );
}
#[test]
fn bindgen_test_layout_siginfo() {
    assert_eq!(
        ::core::mem::size_of::<siginfo>(),
        128usize,
        concat!("Size of: ", stringify!(siginfo))
    );
    assert_eq!(
        ::core::mem::align_of::<siginfo>(),
        8usize,
        concat!("Alignment of ", stringify!(siginfo))
    );
}
pub type siginfo_t = siginfo;