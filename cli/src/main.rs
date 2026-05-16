//! euv CLI
//!
//! The official CLI tool for the euv UI framework, providing a development
//! server with hot reload and wasm-pack integration.

mod build;
mod logger;
mod server;

use {build::*, logger::*, server::*};

use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use {
    anyhow::Result,
    clap::Parser,
    hyperlane::*,
    tokio::sync::{Mutex, RwLock, broadcast},
};

/// Entry point for the euv CLI development server.
///
/// Parses command-line arguments, starts the file watcher, and serves
/// the application with live reload support.
#[tokio::main]
async fn main() -> Result<()> {
    Logger::init(log::LevelFilter::Info);
    let args: Cli = Cli::parse();
    let www_absolute: PathBuf = if args.www_dir.is_absolute() {
        args.www_dir.clone()
    } else {
        args.crate_path.join(&args.www_dir)
    };
    let www_absolute: PathBuf = resolve_www_dir(&www_absolute);
    let initial_html: String = generate_dev_html(&www_absolute).await?;
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
    let src_path: PathBuf = args.crate_path.join("src");
    if let Err(error) = tokio::task::spawn_blocking(move || format_dir(&src_path)).await {
        log::warn!("Initial formatter error: {}", error);
    }
    if let Err(error) = run_hyperlane_fmt().await {
        log::warn!("hyperlane-cli fmt error: {}", error);
    }
    match build_wasm(&args).await {
        Ok(()) => {
            log::info!("Initial WASM build completed successfully");
        }
        Err(error) => {
            log::error!("Initial WASM build failed: {}", error);
        }
    }
    log::info!(".gitignore can exclude unwanted file change events from triggering rebuilds");
    let state_for_watch: Arc<AppState> = Arc::clone(&state);
    tokio::spawn(async move {
        if let Err(error) = watch_and_build(state_for_watch).await {
            log::error!("Watch error: {}", error);
        }
    });
    let pkg_dir: PathBuf = resolve_pkg_dir(&www_absolute);
    log::info!("Serving pkg from: {}", pkg_dir.display());
    let mut server: Server = Server::default();
    let mut server_config: ServerConfig = ServerConfig::default();
    server_config.set_address(Server::format_bind_address("127.0.0.1", args.port));
    server.server_config(server_config);
    server.request_middleware::<RequestMiddleware>();
    server.response_middleware::<ResponseMiddleware>();
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
    server.route::<IndexRoute>(format!("{}/{{path:.*}}", www_route_prefix));
    server.route::<ReloadRoute>("/__euv_reload");
    if let Err(error) = set_global_state(Arc::clone(&state)) {
        log::error!("Failed to set global state: {}", error);
    }
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], args.port));
    log::info!(
        "euv dev server running at http://{}/{}/index.html",
        addr,
        www_route_prefix,
    );
    let server_control_hook: ServerControlHook = server
        .run()
        .await
        .map_err(|error: ServerError| anyhow::Error::msg(error.to_string()))?;
    server_control_hook.wait().await;
    Ok(())
}
