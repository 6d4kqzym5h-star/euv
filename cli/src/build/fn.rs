use crate::*;

/// Builds a `Gitignore` matcher from the `.gitignore` file at the given root path.
///
/// # Arguments
///
/// - `&PathBuf` - The root directory where `.gitignore` is located.
///
/// # Returns
///
/// - `Gitignore` - The compiled gitignore matcher.
async fn build_gitignore(root: &PathBuf) -> Gitignore {
    let gitignore_path: PathBuf = root.join(".gitignore");
    let mut builder: GitignoreBuilder = GitignoreBuilder::new(root);
    let gitignore_exists: bool = metadata(&gitignore_path).await.is_ok();
    if gitignore_exists && let Some(error) = builder.add(&gitignore_path) {
        log::warn!("Failed to load .gitignore: {}", error);
    }
    match builder.build() {
        Ok(gitignore) => {
            if gitignore_exists {
                log::info!("Loaded .gitignore to filter file change events");
            }
            gitignore
        }
        Err(error) => {
            log::warn!("Failed to build gitignore matcher: {}", error);
            GitignoreBuilder::new(root)
                .build()
                .unwrap_or_else(|_| Gitignore::empty())
        }
    }
}

/// Executes a full build pipeline: format source files, run hyperlane-cli fmt,
/// build WASM, notify reload channel, and generate updated HTML.
///
/// # Arguments
///
/// - `&ModeArgs` - The parsed CLI arguments containing build configuration.
/// - `Profile` - The build profile (dev or release).
/// - `Option<&broadcast::Sender<ReloadEvent>>` - Optional reload channel for notifying clients.
///
/// # Returns
///
/// - `Result<String>` - The generated HTML with reload script injected on success.
pub(crate) async fn run_build_pipeline(
    args: &ModeArgs,
    profile: Profile,
    reload_tx: Option<&broadcast::Sender<ReloadEvent>>,
) -> Result<String> {
    let src_path: PathBuf = args.crate_path.join("src");
    if let Err(error) = format_dir(&src_path).await {
        log::warn!("Formatter error: {}", error);
    }
    if let Err(error) = run_hyperlane_fmt().await {
        log::warn!("hyperlane-cli fmt error: {}", error);
    }
    match build_wasm(args, profile).await {
        Ok(()) => {
            log::info!("WASM build completed successfully");
            if let Some(sender) = reload_tx {
                let _ = sender.send(ReloadEvent::Reload);
            }
        }
        Err(error) => {
            log::error!("WASM build failed: {}", error);
            if let Some(sender) = reload_tx {
                let _ = sender.send(ReloadEvent::Error(error.to_string()));
            }
        }
    }
    let www_absolute: PathBuf = if args.www_dir.is_absolute() {
        args.www_dir.clone()
    } else {
        args.crate_path.join(&args.www_dir)
    };
    let www_absolute: PathBuf = resolve_www_dir(&www_absolute).await;
    let html: String = generate_dev_html(&www_absolute).await?;
    Ok(html)
}

/// Watches source files and triggers WASM builds.
///
/// # Arguments
///
/// - `Arc<AppState>` - The shared application state.
///
/// # Returns
///
/// - `Result<()>` - Indicates success or failure of the file watcher.
pub(crate) async fn watch_and_build(state: Arc<AppState>) -> Result<()> {
    let crate_path: PathBuf = state.args.crate_path.clone();
    let src_path: PathBuf = crate_path.join("src");
    let gitignore: Gitignore = build_gitignore(&crate_path).await;
    let (tx, mut rx): (Sender<Event>, Receiver<Event>) = channel(32);
    let mut watcher: RecommendedWatcher = RecommendedWatcher::new(
        move |result: Result<Event, notify::Error>| {
            if let Ok(event) = result {
                let _ = tx.blocking_send(event);
            }
        },
        Config::default(),
    )?;
    watcher.watch(&src_path, RecursiveMode::Recursive)?;
    log::info!("Watching {} for changes...", src_path.display());
    let mut debounce: Interval = interval(Duration::from_millis(500));
    debounce.tick().await;
    while let Some(event) = rx.recv().await {
        let filtered_paths: Vec<String> = event
            .paths
            .iter()
            .filter(|path: &&PathBuf| !gitignore.matched(*path, path.is_dir()).is_ignore())
            .map(|path: &PathBuf| path.display().to_string())
            .collect();
        if filtered_paths.is_empty() {
            continue;
        }
        log::warn!("File change detected: {}", filtered_paths.join(", "));
        debounce.reset();
        sleep(Duration::from_millis(300)).await;
        let mut building: MutexGuard<bool> = state.is_building.lock().await;
        if *building {
            continue;
        }
        *building = true;
        drop(building);
        let state_for_build: Arc<AppState> = Arc::clone(&state);
        tokio::spawn(async move {
            let args: ModeArgs = state_for_build.args.clone();
            let profile: Profile = state_for_build.profile;
            let reload_tx: broadcast::Sender<ReloadEvent> = state_for_build.reload_tx.clone();
            match run_build_pipeline(&args, profile, Some(&reload_tx)).await {
                Ok(html) => {
                    let mut content: RwLockWriteGuard<String> =
                        state_for_build.html_content.write().await;
                    *content = html;
                }
                Err(error) => {
                    log::error!("Build pipeline error: {}", error);
                }
            }
            let mut building: MutexGuard<bool> = state_for_build.is_building.lock().await;
            *building = false;
        });
    }
    Ok(())
}

/// Runs wasm-pack build for the target crate.
///
/// # Arguments
///
/// - `&ModeArgs` - The parsed CLI arguments containing build configuration.
/// - `Profile` - The build profile (dev or release).
///
/// # Returns
///
/// - `Result<()>` - Indicates success or failure of the wasm-pack build.
pub(crate) async fn build_wasm(args: &ModeArgs, profile: Profile) -> Result<()> {
    let mut command: Command = Command::new("wasm-pack");
    command
        .arg("build")
        .arg("--target")
        .arg("web")
        .arg("--out-dir")
        .arg(&args.out_dir)
        .current_dir(&args.crate_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if profile == Profile::Release {
        command.arg("--release");
    }
    log::info!(
        "Running: wasm-pack build --target web --out-dir {}{} ...",
        args.out_dir.display(),
        if profile == Profile::Release {
            " --release"
        } else {
            ""
        }
    );
    let output: Output = command
        .output()
        .await
        .map_err(|e| anyhow!("Failed to execute wasm-pack: {}", e))?;
    if !output.status.success() {
        let stderr: String = String::from_utf8_lossy(&output.stderr).to_string();
        bail!("wasm-pack build failed:\n{}", stderr);
    }
    Ok(())
}

/// Prints the startup banner and command information.
///
/// # Arguments
///
/// - `Profile` - The build profile (dev or release).
/// - `Action` - The action to perform (run or build).
/// - `&str` - The server URL (only meaningful in run mode).
pub(crate) fn print_banner(profile: Profile, action: Action, server_url: &str) {
    log::info!("euv-cli v{}", env!("CARGO_PKG_VERSION"));
    let profile_name: &str = match profile {
        Profile::Dev => "dev",
        Profile::Release => "release",
    };
    let action_name: &str = match action {
        Action::Run => "run",
        Action::Build => "build",
    };
    log::info!("Profile: {} | Mode: {}", profile_name, action_name);
    if action == Action::Run {
        log::info!("Server: {}", server_url);
    }
    log::info!(".gitignore can exclude unwanted file change events from triggering rebuilds");
}

/// Runs `hyperlane-cli fmt` to format the Rust source files.
/// If `hyperlane-cli` is not installed, automatically installs it via `cargo install`.
///
/// # Returns
///
/// - `Result<()>` - Indicates success or failure of the formatting operation.
pub(crate) async fn run_hyperlane_fmt() -> Result<()> {
    let which_output: Output = Command::new("hyperlane-cli")
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| anyhow!("Failed to check hyperlane-cli availability: {}", e))?;
    if !which_output.status.success() {
        log::info!("hyperlane-cli not found, installing via cargo install...");
        let install_output: Output = Command::new("cargo")
            .args(["install", "hyperlane-cli"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| anyhow!("Failed to execute cargo install hyperlane-cli: {}", e))?;
        if !install_output.status.success() {
            let stderr: String = String::from_utf8_lossy(&install_output.stderr).to_string();
            bail!("cargo install hyperlane-cli failed:\n{}", stderr);
        }
        log::info!("hyperlane-cli installed successfully");
    }
    let fmt_output: Output = Command::new("hyperlane-cli")
        .arg("fmt")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| anyhow!("Failed to execute hyperlane-cli fmt: {}", e))?;
    if !fmt_output.status.success() {
        let stderr: String = String::from_utf8_lossy(&fmt_output.stderr).to_string();
        bail!("hyperlane-cli fmt failed:\n{}", stderr);
    }
    Ok(())
}
