use crate::*;

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
