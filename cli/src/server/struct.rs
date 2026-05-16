use super::*;

/// Shared application state.
///
/// Holds the generated HTML, reload channel, build lock, and CLI arguments
/// for coordination between the HTTP server and file watcher.
pub struct AppState {
    /// The generated HTML with injected reload script.
    pub html_content: tokio::sync::RwLock<String>,
    /// Broadcast channel for reload events.
    pub reload_tx: tokio::sync::broadcast::Sender<ReloadEvent>,
    /// Whether a build is currently in progress.
    pub is_building: tokio::sync::Mutex<bool>,
    /// CLI arguments.
    pub args: crate::build::Cli,
}

/// Request middleware.
pub struct RequestMiddleware;

/// Response middleware.
pub struct ResponseMiddleware;

/// Route handler for the root path serving the injected development HTML.
pub struct IndexRoute;

/// Route handler for the reload endpoint.
pub struct ReloadRoute;
