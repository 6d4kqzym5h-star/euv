//! euv CLI
//!
//! The official CLI tool for the euv UI framework,
//! providing run/build/fmt modes with hot reload and
//! wasm-pack integration.

mod build;
mod fmt;
mod logger;
mod server;

use {build::*, fmt::*, logger::*, server::*};

use std::{
    net::SocketAddr,
    path::{Component, Path, PathBuf},
    process::{Output, Stdio},
    sync::{Arc, OnceLock},
    time::Duration,
};

use {
    anyhow::{Result, anyhow, bail},
    clap::Parser,
    hyperlane::*,
    ignore::gitignore::{Gitignore, GitignoreBuilder},
    notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher},
    serde::Serialize,
    tokio::{
        fs::{ReadDir, canonicalize, create_dir_all, metadata, read, read_dir, remove_file, write},
        process::Command,
        spawn,
        sync::{
            Mutex, MutexGuard, RwLock, RwLockWriteGuard, broadcast,
            mpsc::{Receiver, Sender, channel},
        },
        time::{Interval, interval, sleep},
    },
};

/// Entry point for the euv CLI.
///
/// Parses command-line arguments and dispatches to the appropriate
/// mode (run/build/fmt).
///
/// - `run` — build + file watcher + dev server
/// - `build` — build only
/// - `fmt` — format euv macro invocations
#[tokio::main]
async fn main() -> Result<()> {
    Logger::init(log::LevelFilter::Info);
    let cli: Cli = Cli::parse();
    match &cli.command {
        Mode::Run(mode_args) => {
            run_mode(mode_args.clone()).await?;
        }
        Mode::Build(mode_args) => {
            build_mode(mode_args.clone()).await?;
        }
        Mode::Fmt(fmt_args) => {
            fmt_mode(fmt_args.clone()).await?;
        }
    }
    Ok(())
}

/// Executes the build-only pipeline.
///
/// # Arguments
///
/// - `ModeArgs` - The CLI arguments.
///
/// # Returns
///
/// - `Result<()>` - Indicates success or failure.
async fn build_mode(mut args: ModeArgs) -> Result<()> {
    args.crate_path = std::fs::canonicalize(&args.crate_path).map_err(|error| {
        anyhow!(
            "Invalid crate-path '{}': {}",
            args.crate_path.display(),
            error
        )
    })?;
    let crate_path_str: String = args.crate_path.to_string_lossy().to_string();
    if crate_path_str.starts_with(r"\\?\") {
        args.crate_path = PathBuf::from(
            crate_path_str
                .strip_prefix(r"\\?\")
                .unwrap_or(&crate_path_str),
        );
    }
    print_banner(Action::Build, "");
    run_build_only_pipeline(&args).await?;
    log::info!("Build completed. Exiting (build-only mode).");
    Ok(())
}

/// Executes the format command.
///
/// # Arguments
///
/// - `FmtArgs` - The CLI arguments for the fmt command.
///
/// # Returns
///
/// - `Result<()>` - Indicates success or failure.
async fn fmt_mode(args: FmtArgs) -> Result<()> {
    let fmt_path: PathBuf = if args.path.is_absolute() {
        args.path.clone()
    } else {
        std::env::current_dir()
            .map_err(|error| anyhow!("Failed to get current directory: {}", error))?
            .join(&args.path)
    };
    let mode: FmtMode = if args.check {
        FmtMode::Check
    } else {
        FmtMode::Write
    };
    format_dir(&fmt_path, mode).await
}

/// Executes the run mode (build + dev server + hot reload).
///
/// # Arguments
///
/// - `ModeArgs` - The CLI arguments.
///
/// # Returns
///
/// - `Result<()>` - Indicates success or failure.
async fn run_mode(mut args: ModeArgs) -> Result<()> {
    args.crate_path = std::fs::canonicalize(&args.crate_path).map_err(|error| {
        anyhow!(
            "Invalid crate-path '{}': {}",
            args.crate_path.display(),
            error
        )
    })?;
    let crate_path_str: String = args.crate_path.to_string_lossy().to_string();
    if crate_path_str.starts_with(r"\\?\") {
        args.crate_path = PathBuf::from(
            crate_path_str
                .strip_prefix(r"\\?\")
                .unwrap_or(&crate_path_str),
        );
    }
    let www_route_prefix: String = args.www_dir.replace('\\', "/");
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], args.port));
    let server_url: String = format!("http://{}/{}/index.html", addr, www_route_prefix);
    print_banner(Action::Run, &server_url);
    let www_absolute: PathBuf = args.crate_path.join(&args.www_dir);
    let www_absolute: PathBuf = resolve_www_dir(&www_absolute).await;
    let initial_html: String = match run_build_pipeline(&args, None).await {
        Ok(html) => html,
        Err(error) => {
            log::error!("Initial build pipeline failed: {}", error);
            let import_path: String = resolve_import_path(&args);
            let is_release: bool = args.wasm_pack_args.contains(&RELEASE_FLAG.to_string());
            generate_html(&www_absolute, &import_path, is_release).await?
        }
    };
    let (reload_tx, _): (
        broadcast::Sender<ReloadEvent>,
        broadcast::Receiver<ReloadEvent>,
    ) = broadcast::channel(16);
    let state: Arc<AppState> = Arc::new(AppState {
        html_content: RwLock::new(initial_html),
        reload_tx: reload_tx.clone(),
        is_building: Mutex::new(false),
        args: args.clone(),
    });
    let state_for_watch: Arc<AppState> = Arc::clone(&state);
    tokio::spawn(async move {
        if let Err(error) = watch_and_build(state_for_watch).await {
            log::error!("Watch error: {}", error);
        }
    });
    let pkg_dir: PathBuf = resolve_pkg_dir(&args);
    log::info!("Serving pkg from: {}", pkg_dir.display());
    let mut server: Server = Server::default();
    let mut server_config: ServerConfig = ServerConfig::default();
    server_config.set_address(Server::format_bind_address(DEFAULT_HOST, args.port));
    server.server_config(server_config);
    server.request_middleware::<RequestMiddleware>();
    server.response_middleware::<ResponseMiddleware>();
    server.route::<IndexRoute>(format!("{}/{{path:.*}}", www_route_prefix));
    server.route::<ReloadRoute>(RELOAD_ROUTE);
    if let Err(error) = set_global_state(Arc::clone(&state)) {
        log::error!("Failed to set global state: {}", error);
    }
    let server_control_hook: ServerControlHook = server
        .run()
        .await
        .map_err(|error: ServerError| anyhow::Error::msg(error.to_string()))?;
    server_control_hook.wait().await;
    Ok(())
}
