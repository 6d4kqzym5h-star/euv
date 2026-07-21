use super::*;

/// Reactive state for the modal demo feature.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseModal {
    /// Whether the basic modal is visible.
    #[get(type(copy))]
    pub(crate) show_basic: Signal<bool>,
    /// Whether the confirm modal is visible.
    #[get(type(copy))]
    pub(crate) show_confirm: Signal<bool>,
    /// Whether the form modal is visible.
    #[get(type(copy))]
    pub(crate) show_form: Signal<bool>,
    /// Whether the first (outermost) nested modal layer is visible.
    #[get(type(copy))]
    pub(crate) show_nested_1: Signal<bool>,
    /// Whether the second nested modal layer is visible.
    #[get(type(copy))]
    pub(crate) show_nested_2: Signal<bool>,
    /// Whether the third (innermost) nested modal layer is visible.
    #[get(type(copy))]
    pub(crate) show_nested_3: Signal<bool>,
    /// The confirm action result message.
    #[get(type(copy))]
    pub(crate) confirm_result: Signal<String>,
    /// The modal form name input.
    #[get(type(copy))]
    pub(crate) modal_name: Signal<String>,
    /// The modal form email input.
    #[get(type(copy))]
    pub(crate) modal_email: Signal<String>,
    /// The modal form submission result.
    #[get(type(copy))]
    pub(crate) modal_submitted: Signal<String>,
    /// The modal form validation error message.
    #[get(type(copy))]
    pub(crate) modal_error: Signal<String>,
    /// The modal form name validation error message.
    #[get(type(copy))]
    pub(crate) name_error: Signal<String>,
    /// The modal form email validation error message.
    #[get(type(copy))]
    pub(crate) email_error: Signal<String>,
}
