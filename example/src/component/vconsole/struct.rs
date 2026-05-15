use crate::*;

/// A console abstraction providing web console API methods.
///
/// Methods are associated functions (not instance methods) that internally
/// access the global Console signal, so callers use `Console::log(msg)`
/// without needing to obtain or hold a Console instance.
pub(crate) struct Console;

/// A single console entry containing a message and its log level.
///
/// Used to store entries in the global vConsole signal for both
/// browser console output and vConsole panel rendering.
#[derive(Clone, Debug, PartialEq)]
pub struct ConsoleEntry {
    /// The log level determining the entry's category and display color.
    pub level: LogLevel,
    /// The text content of the log message.
    pub message: String,
}
