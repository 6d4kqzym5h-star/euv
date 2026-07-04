use crate::*;

/// Reactive state for the browser API features.
///
/// Aggregates all signals needed for the localStorage, sessionStorage,
/// clipboard, window, navigator, location, and console sections.
#[derive(Clone, Copy, Data, Debug, Eq, PartialEq)]
pub struct UseEuvBrowser {
    /// The localStorage key input.
    #[get(type(copy))]
    pub local_key: Signal<String>,
    /// The localStorage value input.
    #[get(type(copy))]
    pub local_value: Signal<String>,
    /// The localStorage operation result.
    #[get(type(copy))]
    pub local_result: Signal<String>,
    /// The sessionStorage key input.
    #[get(type(copy))]
    pub session_key: Signal<String>,
    /// The sessionStorage value input.
    #[get(type(copy))]
    pub session_value: Signal<String>,
    /// The sessionStorage operation result.
    #[get(type(copy))]
    pub session_result: Signal<String>,
    /// The clipboard text input.
    #[get(type(copy))]
    pub clipboard_text: Signal<String>,
    /// The clipboard operation result.
    #[get(type(copy))]
    pub clipboard_result: Signal<String>,
    /// The window size display.
    #[get(type(copy))]
    pub window_size: Signal<String>,
    /// The user agent string.
    #[get(type(copy))]
    pub user_agent: Signal<String>,
    /// The navigator language.
    #[get(type(copy))]
    pub language: Signal<String>,
    /// The location href.
    #[get(type(copy))]
    pub location_url: Signal<String>,
    /// The location origin.
    #[get(type(copy))]
    pub location_origin_val: Signal<String>,
    /// The location pathname.
    #[get(type(copy))]
    pub location_pathname_val: Signal<String>,
    /// The console message input.
    #[get(type(copy))]
    pub console_input: Signal<String>,
}
