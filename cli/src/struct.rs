use crate::*;

/// euv CLI for real-time WASM compilation and hot-reload development server.
///
/// Defines the command-line interface and configuration options
/// for the euv development server.
#[derive(Clone, Debug, Parser)]
#[command(name = "euv-cli")]
#[command(about = "euv development server with live WASM compilation")]
pub struct Cli {
    /// Path to the Rust crate containing the WASM application
    #[arg(short, long, default_value = ".")]
    pub crate_path: PathBuf,

    /// Directory to serve static files from
    #[arg(short, long, default_value = "www")]
    pub www_dir: PathBuf,

    /// Port for the development server
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,

    /// Output directory for wasm-pack (relative to crate_path)
    #[arg(short, long, default_value = "www/pkg")]
    pub out_dir: PathBuf,
}

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
    pub args: Cli,
}

/// Request middleware.
pub struct RequestMiddleware;

/// Response middleware.
pub struct ResponseMiddleware;

/// Route handler for the root path serving the injected development HTML.
pub struct IndexRoute;

/// Route handler for the reload endpoint.
pub struct ReloadRoute;
