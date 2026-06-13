use crate::*;

/// Creates a click event handler that logs a badge click message at the appropriate level.
///
/// The log level is determined by the `LogLevel` enum parameter:
/// - `LogLevel::Log` → `Console::log`
/// - `LogLevel::Warn` → `Console::warn`
/// - `LogLevel::Error` → `Console::error`
///
/// # Arguments
///
/// - `&str` - The badge name for the log message.
/// - `LogLevel` - The log level determining which console method to use.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that logs the badge click.
pub(crate) fn badge_on_click(badge_name: &str, level: LogLevel) -> Option<Rc<dyn Fn(Event)>> {
    let name: String = badge_name.to_string();
    Some(Rc::new(move |_: Event| {
        let message: String = format!("{} badge clicked!", name);
        match level {
            LogLevel::Log => Console::log(&message),
            LogLevel::Warn => Console::warn(&message),
            LogLevel::Error => Console::error(&message),
        }
    }))
}
