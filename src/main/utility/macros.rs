/// Log a warning, and if a debug build then panic.
macro_rules! debug_panic {
    ($($x:tt)+) => {
        log::warn!($($x)+);
        #[cfg(debug_assertions)]
        panic!($($x)+);
    };
}

/// Log a message once at level `lvl_once`, and any later log messages from this line at level
/// `lvl_remaining`. A log target is not supported. The string "(LOG_ONCE)" will be prepended to the
/// message to indicate that future messages won't be logged at `lvl_once`.
///
/// ```
/// # use log::Level;
/// # use shadow_rs::log_once_at_level;
/// log_once_at_level!(Level::Warn, Level::Debug, "Unexpected flag {}", 10);
/// ```
#[allow(unused_macros)]
#[macro_export]
macro_rules! log_once_at_level {
    ($lvl_once:expr, $lvl_remaining:expr, $str:literal $($x:tt)*) => {
        // don't do atomic operations if this log statement isn't enabled
        if log::log_enabled!($lvl_once) || log::log_enabled!($lvl_remaining) {
            static HAS_LOGGED: std::sync::atomic::AtomicBool =
                std::sync::atomic::AtomicBool::new(false);

            // TODO: doing just a `load()` might be faster in the typical case, but would need to
            // have performance metrics to back that up
            match HAS_LOGGED.compare_exchange(
                false,
                true,
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
            ) {
                Ok(_) => log::log!($lvl_once, "(LOG_ONCE) {}", format_args!($str $($x)*)),
                Err(_) => log::log!($lvl_remaining, "(LOG_ONCE) {}", format_args!($str $($x)*)),
            }
        }
    };
}

/// Log a message once at level `lvl_once` for each distinct value, and any
/// later log messages from this line with an already-logged value at level
/// `lvl_remaining`. A log target is not supported. The string "(LOG_ONCE)" will
/// be prepended to the message to indicate that future messages won't be logged
/// at `lvl_once`.
///
/// The fast-path (where the given value has already been logged) aquires a
/// read-lock and looks up the value in a hash table.
///
/// ```
/// # use log::Level;
/// # use shadow_rs::log_once_per_value_at_level;
/// # let unknown_flag: i32 = 0;
/// log_once_per_value_at_level!(unknown_flag, i32, Level::Warn, Level::Debug, "Unknown flag value {unknown_flag}");
/// ```
#[allow(unused_macros)]
#[macro_export]
macro_rules! log_once_per_value_at_level {
    ($value:expr, $t:ty, $lvl_once:expr, $lvl_remaining:expr, $str:literal $($x:tt)*) => {
        // don't do atomic operations if this log statement isn't enabled
        if log::log_enabled!($lvl_once) || log::log_enabled!($lvl_remaining) {
            use $crate::utility::once_set::OnceSet;
            static LOGGED_SET : OnceSet<$t> = OnceSet::new();

            let level = if LOGGED_SET.insert($value) {
                $lvl_once
            } else {
                $lvl_remaining
            };
            log::log!(level, "(LOG_ONCE) {}", format_args!($str $($x)*))
        }
    };
}

/// Log a message once at warn level, and any later log messages from this line at debug level. A
/// log target is not supported. The string "(LOG_ONCE)" will be prepended to the message to
/// indicate that future messages won't be logged at warn level.
///
/// ```ignore
/// warn_once_then_debug!("Unexpected flag {}", 10);
/// ```
#[allow(unused_macros)]
macro_rules! warn_once_then_debug {
    ($($x:tt)+) => {
        log_once_at_level!(log::Level::Warn, log::Level::Debug, $($x)+);
    };
}

/// Log a message once at warn level, and any later log messages from this line at trace level. A
/// log target is not supported. The string "(LOG_ONCE)" will be prepended to the message to
/// indicate that future messages won't be logged at warn level.
///
/// ```ignore
/// warn_once_then_trace!("Unexpected flag {}", 10);
/// ```
#[allow(unused_macros)]
macro_rules! warn_once_then_trace {
    ($($x:tt)+) => {
        log_once_at_level!(log::Level::Warn, log::Level::Trace, $($x)+);
    };
}

/// Implements logging functions that were generated by the `log_syscall` macro.
pub struct SyscallLogger;

/// Creates a logging function. This is written so that the macro can be called from within an
/// `impl` block, ideally directly before the syscall function is defined. See the macro definition
/// for the exact argument types that must be provided to the generated function. The macro itself
/// takes the syscall name, the return type, and the argument types.
///
/// The macro:
///
/// ```ignore
/// log_syscall!(close, /* rv */ c_int, /* fd */ c_int);
/// ```
///
/// expands to something like (excluding some extra boilerplate):
///
/// ```ignore
/// impl SyscallLogger {
///     pub fn close(...) -> std::io::Result<()> { ... }
/// }
/// ```
///
/// This generated function can later be called using:
///
/// ```ignore
/// SyscallLogger::close(...)?;
/// ```
macro_rules! log_syscall {
    ($name:ident, $rv:ty $(,)?) => {
        log_syscall!($name, $rv,,);
    };
    ($name:ident, $rv:ty, $($args:ty),* $(,)?) => {
        paste::paste! { log_syscall!([< _syscall_logger_ $name >]; $name, $rv, $($args),*); }
    };
    ($const_name:ident; $name:ident, $rv:ty, $($args:ty),*) => {
        // we use a constant as a hack so that we can do "impl SyscallLogger { ... }" while
        // already inside a "impl SyscallHandler { ... }" block
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        const $const_name : () = {
            impl crate::utility::macros::SyscallLogger {
                pub fn $name(
                    writer: impl std::io::Write,
                    args: [shadow_shim_helper_rs::syscall_types::SyscallReg; 6],
                    rv: &crate::host::syscall::types::SyscallResult,
                    fmt: crate::host::syscall::formatter::FmtOptions,
                    tid: crate::host::thread::ThreadId,
                    mem: &crate::host::memory_manager::MemoryManager,
                ) -> std::io::Result<()>
                {
                    let syscall_args = <crate::host::syscall::formatter::SyscallArgsFmt::<$($args),*>>::new(args, fmt, mem);
                    let syscall_rv = crate::host::syscall::formatter::SyscallResultFmt::<$rv>::new(&rv, args, fmt, mem);

                    crate::host::syscall::formatter::write_syscall(
                        writer,
                        &crate::host::syscall::handler::Worker::current_time().unwrap(),
                        tid,
                        std::stringify!($name),
                        syscall_args,
                        syscall_rv,
                    )
                }
            }
        };
    };
}

#[cfg(test)]
mod tests {
    // will panic in debug mode
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn debug_panic_macro() {
        debug_panic!("Hello {}", "World");
    }

    // will *not* panic in release mode
    #[test]
    #[cfg(not(debug_assertions))]
    fn debug_panic_macro() {
        debug_panic!("Hello {}", "World");
    }

    #[test]
    fn log_once_at_level() {
        // we don't have a logger set up so we can't actually inspect the log output (well we
        // probably could with a custom logger), so instead we just make sure it compiles
        for x in 0..10 {
            log_once_at_level!(log::Level::Warn, log::Level::Debug, "{x}");
        }

        log_once_at_level!(log::Level::Warn, log::Level::Debug, "A");
        log_once_at_level!(log::Level::Warn, log::Level::Debug, "A");

        // expected log output is:
        // Warn: 0
        // Debug: 1
        // Debug: 2
        // ...
        // Warn: A
        // Warn: A
    }

    #[test]
    fn warn_once() {
        warn_once_then_trace!("A");
        warn_once_then_debug!("A");
    }
}
