use crate::*;

/// Reactive state for the Server-Sent Events (SSE) demo page.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseSse {
    /// The SSE endpoint URL input.
    #[get(type(copy))]
    pub(crate) url: Signal<String>,
    /// Whether the SSE connection is currently active.
    #[get(type(copy))]
    pub(crate) connected: Signal<bool>,
    /// Whether the SSE connection is currently being established.
    #[get(type(copy))]
    pub(crate) connecting: Signal<bool>,
    /// The collected SSE event messages.
    #[get(type(copy))]
    pub(crate) messages: Signal<Vec<String>>,
    /// The error message, empty if no error.
    #[get(type(copy))]
    pub(crate) error: Signal<String>,
}
