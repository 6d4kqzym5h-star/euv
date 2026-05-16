use crate::*;

/// Reactive state for the select and textarea demo feature.
///
/// # Fields
///
/// - `Signal<String>`: The selected fruit value.
/// - `Signal<String>`: The selected country value.
/// - `Signal<String>`: The selected city value.
/// - `Signal<String>`: The feedback result message.
/// - `Signal<String>`: The textarea content.
#[derive(Clone, Copy)]
pub struct UseSelect {
    pub selected_fruit: Signal<String>,
    pub selected_country: Signal<String>,
    pub selected_city: Signal<String>,
    pub feedback: Signal<String>,
    pub textarea_content: Signal<String>,
}

/// Creates select demo state signals wrapped in a `UseSelect` struct.
///
/// # Returns
///
/// - `UseSelect`: The select state.
pub fn use_select() -> UseSelect {
    UseSelect {
        selected_fruit: use_signal(|| "apple".to_string()),
        selected_country: use_signal(|| "".to_string()),
        selected_city: use_signal(|| "".to_string()),
        feedback: use_signal(|| "".to_string()),
        textarea_content: use_signal(|| "".to_string()),
    }
}

/// Creates a change event handler that updates the country and resets the city.
///
/// # Arguments
///
/// - `UseSelect`: The select state.
///
/// # Returns
///
/// - `NativeEventHandler`: A change handler for the country select.
pub fn select_on_country_change(state: UseSelect) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Change, move |event: NativeEvent| {
        if let NativeEvent::Change(change_event) = event {
            state.selected_country.set(change_event.get_value().clone());
            state.selected_city.set("".to_string());
        }
    })
}

/// Creates a click event handler that submits the textarea feedback.
///
/// # Arguments
///
/// - `UseSelect`: The select state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to submit feedback.
pub fn select_on_submit_feedback(state: UseSelect) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let content: String = state.textarea_content.get();
        if content.trim().is_empty() {
            state
                .feedback
                .set("Please enter some feedback.".to_string());
        } else if content.len() > 200 {
            state
                .feedback
                .set("Feedback is too long (max 200 chars).".to_string());
        } else {
            state
                .feedback
                .set(format!("Thank you for your feedback: \"{}\"", content));
            state.textarea_content.set("".to_string());
        }
    })
}
