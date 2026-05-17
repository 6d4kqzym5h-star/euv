use crate::*;

/// Provides a default empty modal state with all signals initialized to their zero values.
impl Default for UseModal {
    fn default() -> Self {
        UseModal {
            show_basic: Signal::new(false),
            show_confirm: Signal::new(false),
            show_form: Signal::new(false),
            confirm_result: Signal::new("".to_string()),
            modal_name: Signal::new("".to_string()),
            modal_email: Signal::new("".to_string()),
            modal_submitted: Signal::new("".to_string()),
            modal_error: Signal::new("".to_string()),
        }
    }
}
