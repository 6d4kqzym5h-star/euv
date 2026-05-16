use crate::*;

/// Creates a click event handler that toggles a boolean signal.
///
/// Produces a `NativeEventHandler` that flips the value of the given
/// boolean signal on each click. Useful for toggle buttons, visibility
/// switches, and drawer open/close patterns.
///
/// # Arguments
///
/// - `Signal<bool>`: The boolean signal to toggle.
///
/// # Returns
///
/// - `NativeEventHandler`: A click event handler that toggles the signal.
pub fn use_toggle(signal: Signal<bool>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let current: bool = signal.get();
        signal.set(!current);
    })
}
