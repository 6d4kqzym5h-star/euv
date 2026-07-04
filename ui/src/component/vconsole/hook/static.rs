use crate::*;

/// Global storage for the Console log signal.
///
/// Initialized via `init_console` and accessed through `get_console_signal`.
/// Uses `SignalCell` for safe single-threaded WASM contexts without raw pointers.
pub(crate) static CONSOLE_LOG_SIGNAL: SignalCell<Vec<ConsoleEntry>> = SignalCell::none();
