use crate::*;

/// Creates a click event handler that increments a counter signal.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal to increment.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that increments the counter.
pub(crate) fn counter_on_increment(counter: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: i32 = counter.get();
        counter.set(current + 1);
    }))
}

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
