use crate::*;

/// Implements the Console struct providing web console API methods.
///
/// Each method outputs to both the browser developer console and the
/// vConsole panel signal, with appropriate log level classification.
/// Methods are associated functions that internally access the global
/// Console instance, so callers never need to hold a reference.
impl Console {
    /// Logs an informational message (equivalent to console.log).
    ///
    /// # Arguments
    ///
    /// - `&str` - The message to log.
    ///
    /// # Panics
    ///
    /// Panics if `init_console` has not been called.
    pub fn log(message: &str) {
        web_sys::console::log_1(&message.into());
        Self::append_entry(ConsoleEntry {
            level: LogLevel::Log,
            message: message.to_string(),
        });
    }

    /// Logs a warning message (equivalent to console.warn).
    ///
    /// # Arguments
    ///
    /// - `&str` - The warning message to log.
    ///
    /// # Panics
    ///
    /// Panics if `init_console` has not been called.
    pub fn warn(message: &str) {
        web_sys::console::warn_1(&message.into());
        Self::append_entry(ConsoleEntry {
            level: LogLevel::Warn,
            message: message.to_string(),
        });
    }

    /// Logs an error message (equivalent to console.error).
    ///
    /// # Arguments
    ///
    /// - `&str` - The error message to log.
    ///
    /// # Panics
    ///
    /// Panics if `init_console` has not been called.
    pub fn error(message: &str) {
        web_sys::console::error_1(&message.into());
        Self::append_entry(ConsoleEntry {
            level: LogLevel::Error,
            message: message.to_string(),
        });
    }

    /// Clears all log entries from the vConsole panel signal.
    ///
    /// # Panics
    ///
    /// Panics if `init_console` has not been called.
    pub fn clear() {
        let log: Signal<Vec<ConsoleEntry>> = Self::get_signal();
        log.set(Vec::new());
    }

    /// Returns the global vConsole log signal.
    ///
    /// # Returns
    ///
    /// - `Signal<Vec<ConsoleEntry>>` - The console log signal.
    ///
    /// # Panics
    ///
    /// Panics if `init_console` has not been called.
    fn get_signal() -> Signal<Vec<ConsoleEntry>> {
        CONSOLE_LOG_SIGNAL.get()
    }

    /// Appends an entry to the vConsole log signal, trimming if over capacity.
    ///
    /// # Arguments
    ///
    /// - `ConsoleEntry` - The console entry to append.
    ///
    /// # Panics
    ///
    /// Panics if `init_console` has not been called.
    fn append_entry(entry: ConsoleEntry) {
        let log: Signal<Vec<ConsoleEntry>> = Self::get_signal();
        let mut current: Vec<ConsoleEntry> = log.get();
        current.push(entry);
        if current.len() > MAX_CONSOLE_LOG_ENTRIES {
            let excess: usize = current.len() - MAX_CONSOLE_LOG_ENTRIES;
            current.drain(0..excess);
        }
        log.set(current);
    }
}
