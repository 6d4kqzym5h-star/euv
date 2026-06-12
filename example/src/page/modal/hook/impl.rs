use crate::*;

/// Provides a default empty modal state with all signals initialized.
impl Default for UseModal {
    /// Creates a default `UseModal` with all modal signals initialized.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `UseModal` with visibility set to `false` and empty string signals.
    fn default() -> Self {
        Self {
            show_basic: use_signal(|| false),
            show_confirm: use_signal(|| false),
            show_form: use_signal(|| false),
            show_nested_1: use_signal(|| false),
            show_nested_2: use_signal(|| false),
            show_nested_3: use_signal(|| false),
            confirm_result: use_signal(String::new),
            modal_name: use_signal(String::new),
            modal_email: use_signal(String::new),
            modal_submitted: use_signal(String::new),
            modal_error: use_signal(String::new),
            name_error: use_signal(String::new),
            email_error: use_signal(String::new),
        }
    }
}
