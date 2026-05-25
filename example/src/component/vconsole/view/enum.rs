/// Log level for console entries, mirroring the web console API.
///
/// Each variant corresponds to a standard browser console method
/// and is rendered with a distinct color in the vConsole panel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum LogLevel {
    /// Informational message (console.log), rendered in white/green.
    Log,
    /// Warning message (console.warn), rendered in yellow/amber.
    Warn,
    /// Error message (console.error), rendered in red.
    Error,
}
