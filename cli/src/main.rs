//! euv CLI
//!
//! The official CLI tool for the euv UI framework, providing
//! run/build modes with hot reload and wasm-pack integration.

mod build;
mod logger;
mod server;

use {build::*, logger::*, server::*};

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
/// mode (run/build).
///
/// - `run` — build + file watcher + dev server
/// - `build` — build only
#[tokio::main]
async fn main() -> Result<()> {
    Logger::init(log::LevelFilter::Info);
    let cli: Cli = Cli::parse();
    let (action, mut args): (Action, ModeArgs) = match &cli.command {
        Mode::Run(mode_args) => (Action::Run, mode_args.clone()),
        Mode::Build(mode_args) => (Action::Build, mode_args.clone()),
    };
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
    print_banner(action, &server_url);
    let www_absolute: PathBuf = args.crate_path.join(&args.www_dir);
    let www_absolute: PathBuf = resolve_www_dir(&www_absolute).await;
    if action == Action::Build {
        run_build_only_pipeline(&args).await?;
        log::info!("Build completed. Exiting (build-only mode).");
        return Ok(());
    }
    let initial_html: String = match run_build_pipeline(&args, None).await {
        Ok(html) => html,
        Err(error) => {
            log::error!("Initial build pipeline failed: {}", error);
            let import_path: String = resolve_import_path(&args);
            generate_html(&www_absolute, &import_path).await?
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
    server_config.set_address(Server::format_bind_address("127.0.0.1", args.port));
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
