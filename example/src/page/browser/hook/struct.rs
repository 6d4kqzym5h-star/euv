use crate::*;

/// Reactive state for the browser API demo page.
///
/// Aggregates all signals needed for the localStorage, sessionStorage,
/// clipboard, window, navigator, location, and console sections.
#[derive(Clone, Copy, Data)]
pub struct UseBrowserApi {
    /// The localStorage key input.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub local_key: Signal<String>,
    /// The localStorage value input.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub local_value: Signal<String>,
    /// The localStorage operation result.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub local_result: Signal<String>,
    /// The sessionStorage key input.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub session_key: Signal<String>,
    /// The sessionStorage value input.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub session_value: Signal<String>,
    /// The sessionStorage operation result.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub session_result: Signal<String>,
    /// The clipboard text input.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub clipboard_text: Signal<String>,
    /// The clipboard operation result.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub clipboard_result: Signal<String>,
    /// The window size display.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub window_size: Signal<String>,
    /// The user agent string.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub user_agent: Signal<String>,
    /// The navigator language.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub language: Signal<String>,
    /// The location href.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub location_url: Signal<String>,
    /// The location origin.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub location_origin_val: Signal<String>,
    /// The location pathname.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub location_pathname_val: Signal<String>,
    /// The console message input.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub console_input: Signal<String>,
}
