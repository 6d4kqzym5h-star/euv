use crate::*;

/// The build profile determining optimization level and debug assertions.
///
/// - `Dev` — development mode with debug assertions and watch support
/// - `Release` — optimized production build
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Profile {
    /// Development mode with debug assertions and hot-reload support.
    Dev,
    /// Optimized production build.
    Release,
}

/// The action to perform within a given profile.
///
/// - `Run` — build and start the development server
/// - `Build` — build only, do not start the server
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    /// Build and start the server (with file watcher in dev mode).
    Run,
    /// Build only, do not start the server.
    Build,
}

/// Top-level command selecting the build profile.
///
/// - `dev` — development mode with debug assertions and watch support
/// - `release` — optimized production build
#[derive(Clone, Debug, Parser)]
pub enum CliCommand {
    /// Development mode with hot-reload and debug build
    Dev {
        /// The action to perform: run or build
        #[command(subcommand)]
        mode: Mode,
    },
    /// Release mode with optimized production build
    Release {
        /// The action to perform: run or build
        #[command(subcommand)]
        mode: Mode,
    },
}

/// The action to perform within a given profile.
///
/// - `run` — build and start the development server
/// - `build` — build only, do not start the server
#[derive(Clone, Debug, Parser)]
pub enum Mode {
    /// Build and start the server (with file watcher in dev mode)
    Run(ModeArgs),
    /// Build only, do not start the server
    Build(ModeArgs),
}
