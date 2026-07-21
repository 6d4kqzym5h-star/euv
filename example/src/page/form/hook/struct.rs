use super::*;

/// Reactive state for a registration form feature.
#[derive(Clone, Copy, Data, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseForm {
    /// The username input.
    #[get(type(copy))]
    pub(crate) username: Signal<String>,
    /// The email input.
    #[get(type(copy))]
    pub(crate) email: Signal<String>,
    /// The password input.
    #[get(type(copy))]
    pub(crate) password: Signal<String>,
    /// The agree checkbox state.
    #[get(type(copy))]
    pub(crate) agree: Signal<bool>,
    /// The submission result message.
    #[get(type(copy))]
    pub(crate) submitted: Signal<String>,
    /// The validation error message.
    #[get(type(copy))]
    pub(crate) errors: Signal<String>,
    /// The username validation error message.
    #[get(type(copy))]
    pub(crate) username_error: Signal<String>,
    /// The email validation error message.
    #[get(type(copy))]
    pub(crate) email_error: Signal<String>,
    /// The password validation error message.
    #[get(type(copy))]
    pub(crate) password_error: Signal<String>,
    /// The agree validation error message.
    #[get(type(copy))]
    pub(crate) agree_error: Signal<String>,
}
