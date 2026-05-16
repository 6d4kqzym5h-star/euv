use super::*;

/// Represents the type of reload event sent to connected clients.
///
/// Used as the message type in the broadcast channel so that the
/// frontend can distinguish between a successful rebuild and an error.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum ReloadEvent {
    /// A successful WASM rebuild; the client should reload the page.
    Reload,
    /// A rebuild error occurred; the message field contains details.
    Error(String),
}
