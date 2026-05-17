use crate::*;

/// Creates modal state signals wrapped in a `UseModal` struct.
///
/// # Returns
///
/// - `UseModal` - The modal state.
pub fn use_modal() -> UseModal {
    UseModal::default()
}

/// Validates the modal name field and updates the error signal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
pub fn validate_modal_name(state: UseModal) {
    let name_value: String = state.get_modal_name().get();
    if name_value.trim().is_empty() {
        state.get_name_error().set("Name is required".to_string());
    } else {
        state.get_name_error().set(String::new());
    }
}

/// Validates the modal email field and updates the error signal.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
pub fn validate_modal_email(state: UseModal) {
    let email_value: String = state.get_modal_email().get();
    if email_value.trim().is_empty() {
        state.get_email_error().set("Email is required".to_string());
    } else if !email_value.contains('@') || !email_value.contains('.') {
        state
            .get_email_error()
            .set("Please enter a valid email".to_string());
    } else {
        state.get_email_error().set(String::new());
    }
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
        state.get_confirm_result().set(String::new());
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
        state.get_modal_name().set(String::new());
        state.get_modal_email().set(String::new());
        state.get_modal_submitted().set(String::new());
        state.get_modal_error().set(String::new());
        state.get_name_error().set(String::new());
        state.get_email_error().set(String::new());
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

/// Creates an input event handler that updates the modal name and validates it.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `NativeEventHandler` - An input handler.
pub fn modal_on_input_name(state: UseModal) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Input, move |event: NativeEvent| {
        if let NativeEvent::Input(input_event) = event {
            state.get_modal_name().set(input_event.get_value().clone());
        }
        validate_modal_name(state);
    })
}

/// Creates an input event handler that updates the modal email and validates it.
///
/// # Arguments
///
/// - `UseModal` - The modal state.
///
/// # Returns
///
/// - `NativeEventHandler` - An input handler.
pub fn modal_on_input_email(state: UseModal) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Input, move |event: NativeEvent| {
        if let NativeEvent::Input(input_event) = event {
            state.get_modal_email().set(input_event.get_value().clone());
        }
        validate_modal_email(state);
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
        validate_modal_name(state);
        validate_modal_email(state);
        let name_error_value: String = state.get_name_error().get();
        let email_error_value: String = state.get_email_error().get();
        let mut validation_errors: Vec<String> = Vec::new();
        if !name_error_value.is_empty() {
            validation_errors.push(name_error_value);
        }
        if !email_error_value.is_empty() {
            validation_errors.push(email_error_value);
        }
        if validation_errors.is_empty() {
            state.get_modal_error().set(String::new());
            state.get_modal_submitted().set(format!(
                "Signed up: {} ({})",
                state.get_modal_name().get(),
                state.get_modal_email().get()
            ));
            state.get_show_form().set(false);
        } else {
            state.get_modal_error().set(validation_errors.join("; "));
        }
    })
}
