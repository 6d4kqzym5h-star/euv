use crate::*;

/// euv CLI for real-time WASM compilation and hot-reload development server.
///
/// Defines the command-line interface with two profile commands
/// (`dev` / `release`) each supporting `run` and `build` sub-modes.
#[derive(Clone, Debug, Parser)]
#[command(name = "euv-cli")]
#[command(about = "euv development server with live WASM compilation")]
pub struct Cli {
    /// The mode to run in: development or release
    #[command(subcommand)]
    pub command: CliCommand,
}

/// Shared arguments for all commands and modes.
///
/// Common configuration options that apply regardless of
/// whether the user selects dev/release or run/build.
#[derive(Clone, Debug, Parser)]
pub struct ModeArgs {
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
