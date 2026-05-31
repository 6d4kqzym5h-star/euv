use crate::*;

/// Executes the build-only pipeline.
///
/// # Arguments
///
/// - `ModeArgs` - The CLI arguments.
///
/// # Returns
///
/// - `Result<()>` - Indicates success or failure.
pub(crate) async fn build_mode(mut args: ModeArgs) -> Result<()> {
    args.crate_path =
        std::fs::canonicalize(&args.crate_path).map_err(|error: io::Error| EuvError::IoPath {
            message: String::from("Invalid crate-path"),
            path: args.crate_path.clone(),
            error,
        })?;
    let crate_path_str: String = args.crate_path.to_string_lossy().to_string();
    if crate_path_str.starts_with(WINDOWS_UNC_PREFIX) {
        args.crate_path = PathBuf::from(
            crate_path_str
                .strip_prefix(WINDOWS_UNC_PREFIX)
                .unwrap_or(&crate_path_str),
        );
    }
    print_banner(Action::Build);
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
pub(crate) async fn fmt_mode(args: FmtArgs) -> Result<()> {
    let fmt_path: PathBuf = if args.path.is_absolute() {
        args.path.clone()
    } else {
        std::env::current_dir()
            .map_err(|error: io::Error| EuvError::Io {
                message: String::from("Failed to get current directory"),
                error,
            })?
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
pub(crate) async fn run_mode(mut args: ModeArgs) -> Result<()> {
    args.crate_path =
        std::fs::canonicalize(&args.crate_path).map_err(|error: io::Error| EuvError::IoPath {
            message: String::from("Invalid crate-path"),
            path: args.crate_path.clone(),
            error,
        })?;
    let crate_path_str: String = args.crate_path.to_string_lossy().to_string();
    if crate_path_str.starts_with(WINDOWS_UNC_PREFIX) {
        args.crate_path = PathBuf::from(
            crate_path_str
                .strip_prefix(WINDOWS_UNC_PREFIX)
                .unwrap_or(&crate_path_str),
        );
    }
    let www_route_prefix: String = args.www_dir.replace(CHAR_SLASH_BACK, STR_SLASH_FORWARD);
    let www_absolute: PathBuf = args.crate_path.join(&args.www_dir);
    let www_absolute: PathBuf = resolve_www_dir(&www_absolute).await;
    let initial_html: String = match run_build_pipeline(&args, None).await {
        Ok(html) => html,
        Err(error) => {
            log::error!("Initial build pipeline failed: {error}");
            let import_path: String = resolve_import_path(&args);
            let is_release: bool = resolve_build_mode(&args) == BuildMode::Release;
            let custom_html: Option<&Path> = args.index_html.as_deref();
            generate_html(&www_absolute, &import_path, is_release, custom_html).await?
        }
    };
    print_banner(Action::Run);
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
            log::error!("Watch error: {error}");
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
    server.route::<IndexRoute>(format!("{www_route_prefix}/{{path:.*}}"));
    server.route::<ReloadRoute>(RELOAD_ROUTE);
    if let Err(error) = set_global_state(Arc::clone(&state)) {
        log::error!("Failed to set global state: {error}");
    }
    print_server_urls(args.port, &www_route_prefix, INDEX_HTML_FILE_NAME);
    let server_control_hook: ServerControlHook = server
        .run()
        .await
        .map_err(|error: ServerError| EuvError::Server(error.to_string()))?;
    server_control_hook.wait().await;
    Ok(())
}
