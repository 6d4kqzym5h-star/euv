use crate::*;

/// The action to perform.
///
/// - `Run` — build and start the development server with hot-reload
/// - `Build` — build only, do not start the server
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    /// Build and start the server (with file watcher).
    Run,
    /// Build only, do not start the server.
    Build,
}

/// The build mode for wasm-pack compilation.
///
/// - `Dev` — development build with debug info and no optimizations
/// - `Release` — release build with optimizations and no debug info
/// - `Profiling` — profiling build with optimizations and debug info
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BuildMode {
    /// Development build (default).
    #[default]
    Dev,
    /// Release build with optimizations.
    Release,
    /// Profiling build with optimizations and debug info.
    Profiling,
}

/// The action to perform.
///
/// - `run` — build and start the development server
/// - `build` — build only, do not start the server
/// - `fmt` — format euv macro invocations
#[derive(Clone, Debug, Parser)]
pub enum Mode {
    /// Build and start the server (with file watcher)
    Run(ModeArgs),
    /// Build only, do not start the server
    Build(ModeArgs),
    /// Format euv macro invocations in source files
    Fmt(FmtArgs),
}

/// Represents the type of reload event sent to connected clients.
///
/// Used as the message type in the broadcast channel so that the
/// frontend can distinguish between a successful rebuild and an error.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum ReloadEvent {
    /// A successful WASM rebuild; the client should reload the page.
    Reload,
    /// A rebuild error occurred; the message field contains details.
    Error(String),
}
