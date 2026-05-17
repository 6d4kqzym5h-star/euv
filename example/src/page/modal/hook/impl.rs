use crate::*;

/// Provides a default empty modal state with all signals initialized to their zero values.
impl Default for UseModal {
    fn default() -> Self {
        UseModal {
            show_basic: use_signal(|| false),
            show_confirm: use_signal(|| false),
            show_form: use_signal(|| false),
            confirm_result: use_signal(String::new),
            modal_name: use_signal(String::new),
            modal_email: use_signal(String::new),
            modal_submitted: use_signal(String::new),
            modal_error: use_signal(String::new),
        }
    }
}
