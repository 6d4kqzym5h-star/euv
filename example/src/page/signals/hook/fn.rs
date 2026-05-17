use crate::*;

/// Creates a click event handler that increments a counter signal.
///
/// # Arguments
///
/// - `Signal<i32>` - The counter signal to increment.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler that increments the counter.
pub fn counter_on_increment(counter: Signal<i32>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let current: i32 = counter.get();
        counter.set(current + 1);
    })
}

/// Creates a click event handler that logs a badge click message.
///
/// # Arguments
///
/// - `&str` - The badge name for the log message.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler that logs the badge click.
pub fn badge_on_click(badge_name: &str) -> NativeEventHandler {
    let name: String = badge_name.to_string();
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        Console::log(&format!("{} badge clicked!", name));
    })
}
