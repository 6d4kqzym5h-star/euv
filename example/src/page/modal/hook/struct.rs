use crate::*;

/// Reactive state for the modal demo feature.
#[derive(Clone, Copy, Data)]
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
    /// The modal form validation error message.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub modal_error: Signal<String>,
    /// The modal form name validation error message.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub name_error: Signal<String>,
    /// The modal form email validation error message.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub email_error: Signal<String>,
}
