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

/// Creates an input event handler that updates a string signal.
///
/// # Arguments
///
/// - `Signal<String>`: The signal to update with the input value.
///
/// # Returns
///
/// - `NativeEventHandler`: An input handler.
pub fn on_input_value(signal: Signal<String>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Input, move |event: NativeEvent| {
        if let NativeEvent::Input(input_event) = event {
            signal.set(input_event.get_value().clone());
        }
    })
}

/// Creates a change event handler that updates a string signal.
///
/// # Arguments
///
/// - `Signal<String>`: The signal to update with the change value.
///
/// # Returns
///
/// - `NativeEventHandler`: A change handler.
pub fn on_change_value(signal: Signal<String>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Change, move |event: NativeEvent| {
        if let NativeEvent::Change(change_event) = event {
            signal.set(change_event.get_value().clone());
        }
    })
}

/// Creates a change event handler that updates a boolean signal from checkbox.
///
/// # Arguments
///
/// - `Signal<bool>`: The signal to update with the checked state.
///
/// # Returns
///
/// - `NativeEventHandler`: A change handler.
pub fn on_change_checked(signal: Signal<bool>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Change, move |event: NativeEvent| {
        if let NativeEvent::Change(change_event) = event {
            signal.set(*change_event.get_checked());
        }
    })
}
