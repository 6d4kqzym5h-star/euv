use super::*;

/// A single WebSocket message with its display text and timestamp.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct WsMessage {
    /// The display text extracted from the JSON `data` field.
    pub(crate) data: String,
    /// The formatted time string extracted from the JSON `time` field.
    pub(crate) time: String,
}

/// The client-sent WebSocket text message format for serialization.
///
/// Serialized as: `{"type":"Text","data":"<user input>"}`
#[derive(Clone, Debug, Serialize)]
pub(crate) struct WsClientTextMessage {
    /// The message type, always `"Text"`.
    pub(crate) r#type: &'static str,
    /// The user input text payload.
    pub(crate) data: String,
}

/// The server-sent WebSocket message format for deserialization.
///
/// Example JSON: `{"type":"Text","name":"uuid","data":"hello","time":1780741281040}`
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct WsServerMessage {
    /// The message payload.
    pub(crate) data: String,
    /// The millisecond timestamp when the message was sent.
    pub(crate) time: f64,
}

/// Reactive state for the WebSocket demo page.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseWebSocket {
    /// The WebSocket endpoint URL input.
    #[get(type(copy))]
    pub(crate) url: Signal<String>,
    /// Whether the WebSocket connection is currently active.
    #[get(type(copy))]
    pub(crate) connected: Signal<bool>,
    /// Whether the WebSocket connection is currently being established.
    #[get(type(copy))]
    pub(crate) connecting: Signal<bool>,
    /// The WebSocket message input.
    #[get(type(copy))]
    pub(crate) message_input: Signal<String>,
    /// The received WebSocket messages.
    #[get(type(copy))]
    pub(crate) messages: Signal<Vec<WsMessage>>,
    /// The error message, empty if no error.
    #[get(type(copy))]
    pub(crate) error: Signal<String>,
    /// The interval handle for the Ping keep-alive timer.
    #[get(type(copy))]
    pub(crate) ping_handle: Signal<Option<IntervalHandle>>,
}
