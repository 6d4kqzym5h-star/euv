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
    )
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
        let content: String = state.get_textarea_content().get();
        if content.trim().is_empty() {
            state
                .get_feedback()
                .set("Please enter some feedback.".to_string());
        } else if content.len() > 200 {
            state
                .get_feedback()
                .set("Feedback is too long (max 200 chars).".to_string());
        } else {
            state
                .get_feedback()
                .set(format!("Thank you for your feedback: \"{}\"", content));
            state.get_textarea_content().set(String::new());
        }
    })
}
