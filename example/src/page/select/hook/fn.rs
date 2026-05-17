use crate::*;

/// Creates select demo state signals wrapped in a `UseSelect` struct.
///
/// # Returns
///
/// - `UseSelect` - The select state.
pub fn use_select() -> UseSelect {
    UseSelect::new(
        use_signal(|| "apple".to_string()),
        use_signal(String::new),
        use_signal(String::new),
        use_signal(String::new),
        use_signal(String::new),
        use_signal(String::new),
    )
}

/// Validates the textarea content and updates the error signal.
///
/// # Arguments
///
/// - `UseSelect` - The select state.
pub fn validate_select_textarea(state: UseSelect) {
    let textarea_value: String = state.get_textarea_content().get();
    if textarea_value.trim().is_empty() {
        state
            .get_textarea_error()
            .set("Please enter some feedback.".to_string());
    } else if textarea_value.len() > 200 {
        state
            .get_textarea_error()
            .set("Feedback is too long (max 200 chars).".to_string());
    } else {
        state.get_textarea_error().set(String::new());
    }
}

/// Creates a change event handler that updates the country and resets the city.
///
/// # Arguments
///
/// - `UseSelect` - The select state.
///
/// # Returns
///
/// - `NativeEventHandler` - A change handler for the country select.
pub fn select_on_country_change(state: UseSelect) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Change, move |event: NativeEvent| {
        if let NativeEvent::Change(change_event) = event {
            state
                .get_selected_country()
                .set(change_event.get_value().clone());
            state.get_selected_city().set(String::new());
        }
    })
}

/// Creates an input event handler that updates the textarea content and validates it.
///
/// # Arguments
///
/// - `UseSelect` - The select state.
///
/// # Returns
///
/// - `NativeEventHandler` - An input handler.
pub fn select_on_input_textarea(state: UseSelect) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Input, move |event: NativeEvent| {
        if let NativeEvent::Input(input_event) = event {
            state
                .get_textarea_content()
                .set(input_event.get_value().clone());
        }
        validate_select_textarea(state);
    })
}

/// Creates a click event handler that submits the textarea feedback.
///
/// # Arguments
///
/// - `UseSelect` - The select state.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler to submit feedback.
pub fn select_on_submit_feedback(state: UseSelect) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        validate_select_textarea(state);
        let textarea_error_value: String = state.get_textarea_error().get();
        if textarea_error_value.is_empty() {
            let content: String = state.get_textarea_content().get();
            state
                .get_feedback()
                .set(format!("Thank you for your feedback: \"{}\"", content));
            state.get_textarea_content().set(String::new());
        }
    })
}
