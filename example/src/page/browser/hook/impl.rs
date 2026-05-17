use crate::*;

/// Provides a default empty browser API state with placeholder signals.
impl Default for UseBrowserApi {
    fn default() -> Self {
        UseBrowserApi {
            local_key: Signal::new("".to_string()),
            local_value: Signal::new("".to_string()),
            local_result: Signal::new("".to_string()),
            session_key: Signal::new("".to_string()),
            session_value: Signal::new("".to_string()),
            session_result: Signal::new("".to_string()),
            clipboard_text: Signal::new("".to_string()),
            clipboard_result: Signal::new("".to_string()),
            window_size: Signal::new("".to_string()),
            user_agent: Signal::new("".to_string()),
            language: Signal::new("".to_string()),
            location_url: Signal::new("".to_string()),
            location_origin_val: Signal::new("".to_string()),
            location_pathname_val: Signal::new("".to_string()),
            console_input: Signal::new("".to_string()),
        }
    }
}
