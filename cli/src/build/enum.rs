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

/// The action to perform.
///
/// - `run` — build and start the development server
/// - `build` — build only, do not start the server
#[derive(Clone, Debug, Parser)]
pub enum Mode {
    /// Build and start the server (with file watcher)
    Run(ModeArgs),
    /// Build only, do not start the server
    Build(ModeArgs),
}
