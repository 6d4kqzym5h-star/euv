use crate::*;

/// Global static pointer to the Console log signal.
///
/// Initialized lazily via `init_console` and never freed.
/// Safe in single-threaded WASM contexts.
pub(crate) static mut CONSOLE_LOG_SIGNAL: *mut SignalInner<Vec<ConsoleEntry>> =
    std::ptr::null_mut();
