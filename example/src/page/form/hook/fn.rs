use crate::*;

/// Reactive state for a registration form feature.
#[derive(Clone, Copy, Data, New)]
pub struct UseForm {
    /// The username input.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub username: Signal<String>,
    /// The email input.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub email: Signal<String>,
    /// The password input.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub password: Signal<String>,
    /// The agree checkbox state.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub agree: Signal<bool>,
    /// The submission result message.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub submitted: Signal<String>,
    /// The validation error message.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub errors: Signal<String>,
}

/// Creates form state signals wrapped in a `UseForm` struct.
///
/// # Returns
///
/// - `UseForm` - The form state.
pub fn use_form() -> UseForm {
    UseForm::new(
        use_signal(String::new),
        use_signal(String::new),
        use_signal(String::new),
        use_signal(|| true),
        use_signal(String::new),
        use_signal(String::new),
    )
}

/// Creates a click event handler that validates and submits the form.
///
/// # Arguments
///
/// - `UseForm` - The form state.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler for form submission.
pub fn form_on_submit(state: UseForm) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let mut validation_errors: Vec<String> = Vec::new();
        if state.get_username().get().trim().is_empty() {
            validation_errors.push("Username is required".to_string());
        }
        if state.get_email().get().trim().is_empty() {
            validation_errors.push("Email is required".to_string());
        }
        if state.get_password().get().len() < 6 {
            validation_errors.push("Password must be at least 6 characters".to_string());
        }
        if !state.get_agree().get() {
            validation_errors.push("You must agree to the terms".to_string());
        }
        if validation_errors.is_empty() {
            state.get_errors().set("".to_string());
            state.get_submitted().set(format!(
                "Submitted: username={}, email={}",
                state.get_username().get(),
                state.get_email().get()
            ));
        } else {
            state.get_errors().set(validation_errors.join("; "));
        }
    })
}
