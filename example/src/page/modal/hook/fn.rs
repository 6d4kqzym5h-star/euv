use crate::*;

/// Creates modal demo state signals wrapped in a `UseModal` struct.
///
/// # Returns
///
/// - `UseModal` - The modal state.
pub fn use_modal() -> UseModal {
    UseModal::default()
}

/// Creates a click event handler that opens the basic modal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler to open the basic modal.
pub fn modal_on_open_basic(state: UseModal) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        state.get_show_basic().set(true);
    })
}

/// Creates a click event handler that opens the confirm modal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler to open the confirm modal.
pub fn modal_on_open_confirm(state: UseModal) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        state.get_show_confirm().set(true);
        state.get_confirm_result().set("".to_string());
    })
}

/// Creates a click event handler that opens the form modal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler to open the form modal.
pub fn modal_on_open_form(state: UseModal) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        state.get_show_form().set(true);
        state.get_modal_name().set("".to_string());
        state.get_modal_email().set("".to_string());
        state.get_modal_submitted().set("".to_string());
        state.get_modal_error().set("".to_string());
    })
}

/// Creates a click event handler that confirms the action and closes the confirm modal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler to confirm the action.
pub fn modal_on_confirm(state: UseModal) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        state
            .get_confirm_result()
            .set("Action confirmed!".to_string());
        state.get_show_confirm().set(false);
    })
}

/// Creates a click event handler that submits the form modal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler to submit the form modal.
pub fn modal_on_form_submit(state: UseModal) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let name: String = state.get_modal_name().get();
        let email: String = state.get_modal_email().get();
        let mut validation_errors: Vec<String> = Vec::new();
        if name.trim().is_empty() {
            validation_errors.push("Name is required".to_string());
        }
        if email.trim().is_empty() {
            validation_errors.push("Email is required".to_string());
        } else if !email.contains('@') || !email.contains('.') {
            validation_errors.push("Please enter a valid email".to_string());
        }
        if validation_errors.is_empty() {
            state.get_modal_error().set("".to_string());
            state
                .get_modal_submitted()
                .set(format!("Signed up: {} ({})", name, email));
            state.get_show_form().set(false);
        } else {
            state.get_modal_error().set(validation_errors.join("; "));
        }
    })
}
