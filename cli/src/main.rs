//! euv CLI
//!
//! The official CLI tool for the euv UI framework, providing a development
//! server with hot reload and wasm-pack integration.

mod r#const;
mod r#fn;
mod r#impl;
mod r#static;
mod r#struct;

use {r#const::*, r#fn::*, r#static::*, r#struct::*};

use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    process::Stdio,
    sync::{Arc, OnceLock},
};

use {
    anyhow::{Context as AnyhowContext, Result},
    clap::Parser,
    hyperlane::*,
    notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher},
    tokio::{
        fs,
        process::Command,
        sync::{Mutex, RwLock, broadcast},
        time::{Duration, sleep},
    },
};

/// Entry point for the euv CLI development server.
///
/// Parses command-line arguments, starts the file watcher, and serves
/// the application with live reload support.
#[tokio::main]
async fn main() -> Result<()> {
    let args: Cli = Cli::parse();
    let www_absolute: PathBuf = if args.www_dir.is_absolute() {
        args.www_dir.clone()
    } else {
        args.crate_path.join(&args.www_dir)
    };
    let www_absolute: PathBuf = resolve_www_dir(&www_absolute);
    let initial_html: String = generate_dev_html(&www_absolute).await?;
    let (reload_tx, _): (broadcast::Sender<()>, broadcast::Receiver<()>) = broadcast::channel(16);
    let state: Arc<AppState> = Arc::new(AppState {
        html_content: RwLock::new(initial_html),
        reload_tx: reload_tx.clone(),
        is_building: Mutex::new(false),
        args: args.clone(),
    });
    let state_for_watch: Arc<AppState> = Arc::clone(&state);
    tokio::spawn(async move {
        if let Err(error) = watch_and_build(state_for_watch).await {
            eprintln!("Watch error: {}", error);
        }
    });
    let pkg_dir: PathBuf = resolve_pkg_dir(&www_absolute);
    println!("Serving pkg from: {}", pkg_dir.display());
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
            .filter(|c: &std::path::Component| !matches!(c, std::path::Component::CurDir))
            .collect();
        normalized.to_string_lossy().replace('\\', "/")
    };
    server.route::<IndexRoute>(format!("{}/{{path:.*}}", www_route_prefix));
    server.route::<ReloadRoute>("/__euv_reload");
    if let Err(error) = set_global_state(Arc::clone(&state)) {
        eprintln!("Failed to set global state: {}", error);
    }
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], args.port));
    println!(
        "euv dev server running at http://{}/{}/index.html",
        addr, www_route_prefix,
    );
    let server_control_hook: ServerControlHook = server
        .run()
        .await
        .map_err(|e: ServerError| anyhow::Error::msg(e.to_string()))?;
    server_control_hook.wait().await;
    Ok(())
}
