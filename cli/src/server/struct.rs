use crate::*;

/// Shared application state.
///
/// Holds the generated HTML, reload channel, build lock, and CLI arguments
/// for coordination between the HTTP server and file watcher.
#[derive(Data, New)]
pub struct AppState {
    /// The generated HTML with injected reload script.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) html_content: RwLock<String>,
    /// Broadcast channel for reload events.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) reload_tx: broadcast::Sender<ReloadEvent>,
    /// Whether a build is currently in progress.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_building: Mutex<bool>,
    /// CLI arguments.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) args: ModeArgs,
}

/// Request middleware that injects cache-control headers.
///
/// Sets `Cache-Control: no-cache, no-store, must-revalidate`, `Pragma: no-cache`,
/// and `Expires: 0` on every response to prevent stale WASM assets during development.
#[derive(Data, New)]
pub struct RequestMiddleware;

/// Response middleware that writes the serialized response to the stream.
///
/// Builds the HTTP response bytes and sends them through the connection stream,
/// closing the stream if the send fails.
#[derive(Data, New)]
pub struct ResponseMiddleware;

/// Route handler for the root path serving the injected development HTML.
///
/// When the request targets `index.html`, returns the in-memory HTML
/// that has the live-reload script injected. For all other files,
/// reads the content from disk with path-traversal protection.
#[derive(Data, New)]
pub struct IndexRoute;

/// Route handler for the reload endpoint using long-polling.
///
/// Holds the connection open until a reload event is broadcast, then returns
/// a single JSON response so the client can distinguish between a successful
/// rebuild and an error.
#[derive(Data, New)]
pub struct ReloadRoute;
