use crate::*;

/// Reactive state for the modal demo feature.
#[derive(Clone, Copy, Data, New)]
pub struct UseModal {
    /// Whether the basic modal is visible.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub show_basic: Signal<bool>,
    /// Whether the confirm modal is visible.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub show_confirm: Signal<bool>,
    /// Whether the form modal is visible.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub show_form: Signal<bool>,
    /// The confirm action result message.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub confirm_result: Signal<String>,
    /// The modal form name input.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub modal_name: Signal<String>,
    /// The modal form email input.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub modal_email: Signal<String>,
    /// The modal form submission result.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub modal_submitted: Signal<String>,
}

/// Creates modal demo state signals wrapped in a `UseModal` struct.
///
/// # Returns
///
/// - `UseModal` - The modal state.
pub fn use_modal() -> UseModal {
    UseModal::new(
        use_signal(|| false),
        use_signal(|| false),
        use_signal(|| false),
        use_signal(String::new),
        use_signal(String::new),
        use_signal(String::new),
        use_signal(String::new),
    )
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
        state.show_basic.set(true);
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
        state.show_confirm.set(true);
        state.confirm_result.set("".to_string());
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
        state.show_form.set(true);
        state.modal_name.set("".to_string());
        state.modal_email.set("".to_string());
        state.modal_submitted.set("".to_string());
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
        state.confirm_result.set("Action confirmed!".to_string());
        state.show_confirm.set(false);
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
        let name: String = state.modal_name.get();
        let email: String = state.modal_email.get();
        if !name.trim().is_empty() && !email.trim().is_empty() {
            state
                .modal_submitted
                .set(format!("Signed up: {} ({})", name, email));
            state.show_form.set(false);
        }
    })
}
