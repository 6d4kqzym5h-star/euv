use crate::*;

/// Provides a default empty form state with all signals initialized.
impl Default for UseForm {
    /// Creates a default `UseForm` with empty signals.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `UseForm` with all signals initialized to defaults.
    fn default() -> Self {
        Self {
            username: App::use_signal(String::new),
            email: App::use_signal(String::new),
            password: App::use_signal(String::new),
            agree: App::use_signal(|| true),
            submitted: App::use_signal(String::new),
            errors: App::use_signal(String::new),
            username_error: App::use_signal(String::new),
            email_error: App::use_signal(String::new),
            password_error: App::use_signal(String::new),
            agree_error: App::use_signal(String::new),
        }
    }
}
