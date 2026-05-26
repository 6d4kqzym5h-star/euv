use crate::*;

/// Provides a default empty form state with all signals initialized.
impl Default for UseForm {
    /// Creates a default `UseForm` with empty signals.
    ///
    /// # Returns
    ///
    /// - `Self`- A new `UseForm` with all signals initialized to defaults.
    fn default() -> Self {
        Self {
            username: use_signal(String::new),
            email: use_signal(String::new),
            password: use_signal(String::new),
            agree: use_signal(|| true),
            submitted: use_signal(String::new),
            errors: use_signal(String::new),
            username_error: use_signal(String::new),
            email_error: use_signal(String::new),
            password_error: use_signal(String::new),
            agree_error: use_signal(String::new),
        }
    }
}
