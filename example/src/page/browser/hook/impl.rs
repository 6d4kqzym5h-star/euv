use crate::*;

/// Provides a default empty browser API state with placeholder signals.
impl Default for UseBrowserApi {
    fn default() -> Self {
        UseBrowserApi {
            local_key: Signal::new(String::new()),
            local_value: Signal::new(String::new()),
            local_result: Signal::new(String::new()),
            session_key: Signal::new(String::new()),
            session_value: Signal::new(String::new()),
            session_result: Signal::new(String::new()),
            clipboard_text: Signal::new(String::new()),
            clipboard_result: Signal::new(String::new()),
            window_size: Signal::new(String::new()),
            user_agent: Signal::new(String::new()),
            language: Signal::new(String::new()),
            location_url: Signal::new(String::new()),
            location_origin_val: Signal::new(String::new()),
            location_pathname_val: Signal::new(String::new()),
            console_input: Signal::new(String::new()),
        }
    }
}
