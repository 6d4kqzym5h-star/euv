use crate::*;

/// Provides a default empty browser API state with placeholder signals.
impl Default for UseBrowserApi {
    /// Creates a default `UseBrowserApi` with empty string signals.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `UseBrowserApi` with all signals initialized to empty strings.
    fn default() -> Self {
        Self {
            local_key: Signal::create(String::new()),
            local_value: Signal::create(String::new()),
            local_result: Signal::create(String::new()),
            session_key: Signal::create(String::new()),
            session_value: Signal::create(String::new()),
            session_result: Signal::create(String::new()),
            clipboard_text: Signal::create(String::new()),
            clipboard_result: Signal::create(String::new()),
            window_size: Signal::create(String::new()),
            user_agent: Signal::create(String::new()),
            language: Signal::create(String::new()),
            location_url: Signal::create(String::new()),
            location_origin_val: Signal::create(String::new()),
            location_pathname_val: Signal::create(String::new()),
            console_input: Signal::create(String::new()),
        }
    }
}
