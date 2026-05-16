use crate::*;

/// Reactive state for the select and textarea demo feature.
#[derive(Clone, Copy, Data, New)]
pub struct UseSelect {
    /// The selected fruit value.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub selected_fruit: Signal<String>,
    /// The selected country value.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub selected_country: Signal<String>,
    /// The selected city value.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub selected_city: Signal<String>,
    /// The feedback result message.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub feedback: Signal<String>,
    /// The textarea content.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub textarea_content: Signal<String>,
}

/// Creates select demo state signals wrapped in a `UseSelect` struct.
///
/// # Returns
///
/// - `UseSelect`: The select state.
pub fn use_select() -> UseSelect {
    UseSelect::new(
        use_signal(|| "apple".to_string()),
        use_signal(|| "".to_string()),
        use_signal(|| "".to_string()),
        use_signal(|| "".to_string()),
        use_signal(|| "".to_string()),
    )
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
