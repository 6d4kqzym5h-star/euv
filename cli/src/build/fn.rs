use super::*;

/// Builds a `Gitignore` matcher from the `.gitignore` file at the given root path.
///
/// # Arguments
///
/// - `&PathBuf`: The root directory where `.gitignore` is located.
///
/// # Returns
///
/// - `Gitignore`: The compiled gitignore matcher.
fn build_gitignore(root: &PathBuf) -> Gitignore {
    let gitignore_path: PathBuf = root.join(".gitignore");
    let mut builder: GitignoreBuilder = GitignoreBuilder::new(root);
    if gitignore_path.exists()
        && let Some(error) = builder.add(&gitignore_path)
    {
        log::warn!("Failed to load .gitignore: {}", error);
    }
    match builder.build() {
        Ok(gitignore) => {
            if gitignore_path.exists() {
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

/// Watches source files and triggers WASM builds.
///
/// # Arguments
///
/// - `Arc<AppState>`: The shared application state.
///
/// # Returns
///
/// - `Result<()>`: Indicates success or failure of the file watcher.
pub(crate) async fn watch_and_build(state: Arc<AppState>) -> Result<()> {
    let crate_path: PathBuf = state.args.crate_path.clone();
    let src_path: PathBuf = crate_path.join("src");
    let gitignore: Gitignore = build_gitignore(&crate_path);
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
    let mut debounce: tokio::time::Interval = tokio::time::interval(Duration::from_millis(500));
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
        log::info!("File change detected: {}", filtered_paths.join(", "));
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
            let src_path: PathBuf = state_for_build.args.crate_path.join("src");
            if let Err(error) = tokio::task::spawn_blocking(move || format_dir(&src_path)).await {
                log::warn!("Formatter error: {}", error);
            }
            if let Err(error) = run_hyperlane_fmt().await {
                log::warn!("hyperlane-cli fmt error: {}", error);
            }
            match build_wasm(&state_for_build.args).await {
                Ok(()) => {
                    log::info!("WASM build completed successfully");
                    if let Err(error) = crate::server::update_html(&state_for_build).await {
                        log::error!("Failed to update HTML: {}", error);
                    }
                    let _ = state_for_build.reload_tx.send(ReloadEvent::Reload);
                }
                Err(error) => {
                    log::error!("WASM build failed: {}", error);
                    let _ = state_for_build
                        .reload_tx
                        .send(ReloadEvent::Error(error.to_string()));
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
/// - `&Cli`: The parsed CLI arguments containing build configuration.
///
/// # Returns
///
/// - `Result<()>`: Indicates success or failure of the wasm-pack build.
pub(crate) async fn build_wasm(args: &Cli) -> Result<()> {
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
    log::info!(
        "Running: wasm-pack build --target web --out-dir {} ...",
        args.out_dir.display()
    );
    let output: Output = command
        .output()
        .await
        .context("Failed to execute wasm-pack")?;
    if !output.status.success() {
        let stderr: String = String::from_utf8_lossy(&output.stderr).to_string();
        anyhow::bail!("wasm-pack build failed:\n{}", stderr);
    }
    Ok(())
}

/// Runs `hyperlane-cli fmt` to format the Rust source files.
/// If `hyperlane-cli` is not installed, automatically installs it via `cargo install`.
///
/// # Returns
///
/// - `Result<()>`: Indicates success or failure of the formatting operation.
pub(crate) async fn run_hyperlane_fmt() -> Result<()> {
    let which_output: Output = Command::new("hyperlane-cli")
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .context("Failed to check hyperlane-cli availability")?;
    if !which_output.status.success() {
        log::info!("hyperlane-cli not found, installing via cargo install...");
        let install_output: Output = Command::new("cargo")
            .args(["install", "hyperlane-cli"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("Failed to execute cargo install hyperlane-cli")?;
        if !install_output.status.success() {
            let stderr: String = String::from_utf8_lossy(&install_output.stderr).to_string();
            anyhow::bail!("cargo install hyperlane-cli failed:\n{}", stderr);
        }
        log::info!("hyperlane-cli installed successfully");
    }
    let fmt_output: Output = Command::new("hyperlane-cli")
        .arg("fmt")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .context("Failed to execute hyperlane-cli fmt")?;
    if !fmt_output.status.success() {
        let stderr: String = String::from_utf8_lossy(&fmt_output.stderr).to_string();
        anyhow::bail!("hyperlane-cli fmt failed:\n{}", stderr);
    }
    Ok(())
}
