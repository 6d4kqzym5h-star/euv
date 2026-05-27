use crate::*;

/// euv CLI for real-time WASM compilation and hot-reload development server.
///
/// Defines the command-line interface with three sub-commands
/// (`run` / `build` / `fmt`). All arguments after `--` are transparently
/// forwarded to `wasm-pack build` without interpretation.
#[derive(Clone, Data, Debug, New, Parser)]
#[command(name = "euv")]
#[command(about = "euv development server with live WASM compilation")]
pub struct Cli {
    /// The mode to run in: run (build + server), build (build only), or fmt (format)
    #[command(subcommand)]
    pub command: Mode,
}

/// euv-specific arguments combined with wasm-pack passthrough.
///
/// Only `--crate-path`, `--port`, and `--www-dir` belong to euv; everything
/// in `wasm_pack_args` is forwarded to `wasm-pack build` as-is.
#[derive(Clone, Data, Debug, New, Parser)]
pub struct ModeArgs {
    /// Path to the Rust crate containing the WASM application
    #[arg(short, long, default_value = ".")]
    pub crate_path: PathBuf,
    /// Port for the development server
    #[arg(short, long, default_value_t = 80)]
    #[get(type(copy))]
    pub port: u16,
    /// Directory name for static assets and generated HTML (relative to crate-path)
    #[arg(long, default_value = "www")]
    pub www_dir: String,
    /// Arguments transparently forwarded to `wasm-pack build`
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub wasm_pack_args: Vec<String>,
}

/// Arguments for the `fmt` subcommand.
///
/// Formats euv macro invocations (`html!`, `class!`, `css_vars!`, `watch!`)
/// in Rust source files.
#[derive(Clone, Data, Debug, New, Parser)]
pub struct FmtArgs {
    /// Path to the directory or file to format
    #[arg(short, long, default_value = ".")]
    pub path: PathBuf,
    /// Check if formatting is needed without modifying files
    #[arg(long, default_value_t = false)]
    #[get(type(copy))]
    pub check: bool,
}
