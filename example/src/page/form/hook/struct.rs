use crate::*;

/// Reactive state for a registration form feature.
#[derive(Clone, Copy, Data)]
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
    /// The username validation error message.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub username_error: Signal<String>,
    /// The email validation error message.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub email_error: Signal<String>,
    /// The password validation error message.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub password_error: Signal<String>,
    /// The agree validation error message.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub agree_error: Signal<String>,
}
