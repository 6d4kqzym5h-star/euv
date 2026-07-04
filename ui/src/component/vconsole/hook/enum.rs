/// Log level for console entries, mirroring the web console API.
///
/// Each variant corresponds to a standard browser console method
/// and is rendered with a distinct color in the vConsole panel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogLevel {
    /// Informational message (console.log), rendered in default text color.
    Log,
    /// Warning message (console.warn), rendered in yellow/amber.
    Warn,
    /// Error message (console.error), rendered in red.
    Error,
}

/// Filter level for the vConsole panel log entry display.
///
/// Each variant corresponds to a filter button in the vConsole drawer,
/// controlling which log levels are visible in the panel body.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogFilter {
    /// Show all log levels.
    All,
    /// Show only informational messages (LogLevel::Log).
    Log,
    /// Show only warning messages (LogLevel::Warn).
    Warn,
    /// Show only error messages (LogLevel::Error).
    Error,
}
