use crate::*;

/// Reactive state for the browser API demo page.
///
/// Aggregates all signals needed for the localStorage, sessionStorage,
/// clipboard, window, navigator, location, and console sections.
#[derive(Clone, Copy, Data)]
pub(crate) struct UseBrowserApi {
    /// The localStorage key input.
    #[get(type(copy))]
    pub(crate) local_key: Signal<String>,
    /// The localStorage value input.
    #[get(type(copy))]
    pub(crate) local_value: Signal<String>,
    /// The localStorage operation result.
    #[get(type(copy))]
    pub(crate) local_result: Signal<String>,
    /// The sessionStorage key input.
    #[get(type(copy))]
    pub(crate) session_key: Signal<String>,
    /// The sessionStorage value input.
    #[get(type(copy))]
    pub(crate) session_value: Signal<String>,
    /// The sessionStorage operation result.
    #[get(type(copy))]
    pub(crate) session_result: Signal<String>,
    /// The clipboard text input.
    #[get(type(copy))]
    pub(crate) clipboard_text: Signal<String>,
    /// The clipboard operation result.
    #[get(type(copy))]
    pub(crate) clipboard_result: Signal<String>,
    /// The window size display.
    #[get(type(copy))]
    pub(crate) window_size: Signal<String>,
    /// The user agent string.
    #[get(type(copy))]
    pub(crate) user_agent: Signal<String>,
    /// The navigator language.
    #[get(type(copy))]
    pub(crate) language: Signal<String>,
    /// The location href.
    #[get(type(copy))]
    pub(crate) location_url: Signal<String>,
    /// The location origin.
    #[get(type(copy))]
    pub(crate) location_origin_val: Signal<String>,
    /// The location pathname.
    #[get(type(copy))]
    pub(crate) location_pathname_val: Signal<String>,
    /// The console message input.
    #[get(type(copy))]
    pub(crate) console_input: Signal<String>,
}
