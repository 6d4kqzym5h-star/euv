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
            show_basic: App::use_signal(|| false),
            show_confirm: App::use_signal(|| false),
            show_form: App::use_signal(|| false),
            show_nested_1: App::use_signal(|| false),
            show_nested_2: App::use_signal(|| false),
            show_nested_3: App::use_signal(|| false),
            confirm_result: App::use_signal(String::new),
            modal_name: App::use_signal(String::new),
            modal_email: App::use_signal(String::new),
            modal_submitted: App::use_signal(String::new),
            modal_error: App::use_signal(String::new),
            name_error: App::use_signal(String::new),
            email_error: App::use_signal(String::new),
        }
    }
}
