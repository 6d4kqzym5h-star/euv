//! euv CLI
//!
//! The official CLI tool for the euv UI framework, providing dev/release
//! profiles with run/build modes, hot reload, and wasm-pack integration.

mod build;
mod formatter;
mod logger;
mod server;

use {build::*, formatter::*, logger::*, server::*};

use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
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
        fs::{
            ReadDir, canonicalize, create_dir_all, metadata, read, read_dir, read_to_string, write,
        },
        process::Command,
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
/// profile (dev/release) and mode (run/build).
///
/// - `dev run` — debug build + file watcher + dev server
/// - `dev build` — debug build only
/// - `release run` — optimized build + dev server
/// - `release build` — optimized build only
#[tokio::main]
async fn main() -> Result<()> {
    Logger::init(log::LevelFilter::Info);
    let cli: Cli = Cli::parse();
    let (profile, action, args): (Profile, Action, ModeArgs) = match &cli.command {
        CliCommand::Dev { mode } => {
            let (action, mode_args): (Action, ModeArgs) = match mode {
                Mode::Run(a) => (Action::Run, a.clone()),
                Mode::Build(a) => (Action::Build, a.clone()),
            };
            (Profile::Dev, action, mode_args)
        }
        CliCommand::Release { mode } => {
            let (action, mode_args): (Action, ModeArgs) = match mode {
                Mode::Run(a) => (Action::Run, a.clone()),
                Mode::Build(a) => (Action::Build, a.clone()),
            };
            (Profile::Release, action, mode_args)
        }
    };
    let www_route_prefix: String = {
        let combined: PathBuf = if args.www_dir.is_absolute() {
            args.www_dir.clone()
        } else {
            args.crate_path.join(&args.www_dir)
        };
        let normalized: PathBuf = combined
            .components()
            .filter(|component: &std::path::Component| {
                !matches!(component, std::path::Component::CurDir)
            })
            .collect();
        normalized.to_string_lossy().replace('\\', "/")
    };
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], args.port));
    let server_url: String = format!("http://{}/{}/index.html", addr, www_route_prefix);
    print_banner(profile, action, &server_url);
    let www_absolute: PathBuf = if args.www_dir.is_absolute() {
        args.www_dir.clone()
    } else {
        args.crate_path.join(&args.www_dir)
    };
    let www_absolute: PathBuf = resolve_www_dir(&www_absolute).await;
    let initial_html: String = match run_build_pipeline(&args, profile, None).await {
        Ok(html) => html,
        Err(error) => {
            log::error!("Initial build pipeline failed: {}", error);
            generate_dev_html(&www_absolute).await?
        }
    };
    if action == Action::Build {
        log::info!("Build completed. Exiting (build-only mode).");
        return Ok(());
    }
    let (reload_tx, _): (
        broadcast::Sender<ReloadEvent>,
        broadcast::Receiver<ReloadEvent>,
    ) = broadcast::channel(16);
    let state: Arc<AppState> = Arc::new(AppState {
        html_content: RwLock::new(initial_html),
        reload_tx: reload_tx.clone(),
        is_building: Mutex::new(false),
        args: args.clone(),
        profile,
    });
    let state_for_watch: Arc<AppState> = Arc::clone(&state);
    tokio::spawn(async move {
        if let Err(error) = watch_and_build(state_for_watch).await {
            log::error!("Watch error: {}", error);
        }
    });
    let pkg_dir: PathBuf = resolve_pkg_dir(&www_absolute).await;
    log::info!("Serving pkg from: {}", pkg_dir.display());
    let mut server: Server = Server::default();
    let mut server_config: ServerConfig = ServerConfig::default();
    server_config.set_address(Server::format_bind_address("127.0.0.1", args.port));
    server.server_config(server_config);
    server.request_middleware::<RequestMiddleware>();
    server.response_middleware::<ResponseMiddleware>();
    server.route::<IndexRoute>(format!("{}/{{path:.*}}", www_route_prefix));
    server.route::<ReloadRoute>("/__euv_reload");
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
