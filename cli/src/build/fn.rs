use super::*;

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
    while let Some(_event) = rx.recv().await {
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
