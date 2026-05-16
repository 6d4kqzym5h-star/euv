use crate::*;

/// Sets the global application state.
///
/// # Arguments
///
/// - `Arc<AppState>`: The shared application state to store globally.
///
/// # Returns
///
/// - `Result<()>`: Indicates success or failure of the initialization.
pub(crate) fn set_global_state(state: Arc<AppState>) -> Result<()> {
    APP_STATE
        .set(state)
        .map_err(|_| anyhow::anyhow!("Global state already initialized"))
}

/// Retrieves the global application state.
///
/// # Returns
///
/// - `Option<Arc<AppState>>`: The global state if initialized.
pub(crate) fn get_global_state() -> Option<Arc<AppState>> {
    APP_STATE.get().cloned()
}

/// Reads the original index.html and injects the live-reload script.
/// If index.html does not exist, a default one is created automatically.
///
/// # Arguments
///
/// - `&Path`: The path to the www directory containing `index.html`.
///
/// # Returns
///
/// - `Result<String>`: The modified HTML with the reload script injected.
pub(crate) async fn generate_dev_html(www_dir: &Path) -> Result<String> {
    let index_path: PathBuf = www_dir.join("index.html");
    if !index_path.exists() {
        fs::create_dir_all(www_dir)
            .await
            .context("Failed to create www directory")?;
        fs::write(&index_path, DEFAULT_INDEX_HTML)
            .await
            .context("Failed to write default index.html")?;
    }
    let original: String = fs::read_to_string(&index_path)
        .await
        .context("Failed to read index.html")?;
    let mut html: String = if original.contains("</body>") {
        original.replace("</body>", &format!("{}\n</body>", RELOAD_SCRIPT))
    } else {
        format!("{}\n{}", original, RELOAD_SCRIPT)
    };
    html = html.replace("./euv_example.js", "./pkg/euv_example.js");
    Ok(html)
}

/// Updates the served HTML after a successful build.
///
/// # Arguments
///
/// - `&AppState`: The shared application state containing the current HTML.
///
/// # Returns
///
/// - `Result<()>`: Indicates success or failure of the HTML update.
pub(crate) async fn update_html(state: &AppState) -> Result<()> {
    let www_absolute: PathBuf = if state.args.www_dir.is_absolute() {
        state.args.www_dir.clone()
    } else {
        state.args.crate_path.join(&state.args.www_dir)
    };
    let www_absolute: PathBuf = resolve_www_dir(&www_absolute);
    let new_html: String = generate_dev_html(&www_absolute).await?;
    let mut html: RwLockWriteGuard<String> = state.html_content.write().await;
    *html = new_html;
    Ok(())
}

/// Resolves the effective www directory, handling wasm-pack nested output.
///
/// # Arguments
///
/// - `&Path`: The candidate www directory path.
///
/// # Returns
///
/// - `PathBuf`: The resolved www directory containing `index.html`.
pub(crate) fn resolve_www_dir(www_dir: &Path) -> PathBuf {
    if www_dir.join("index.html").exists() {
        return www_dir.to_path_buf();
    }
    let parent_name: Option<&str> = www_dir.file_name().and_then(|n| n.to_str());
    if let Some(name) = parent_name {
        let nested = www_dir.join(name);
        if nested.join("index.html").exists() {
            return nested;
        }
    }
    www_dir.to_path_buf()
}

/// Resolves the pkg directory for serving WASM artifacts.
///
/// # Arguments
///
/// - `&Path`: The www directory path to search within.
///
/// # Returns
///
/// - `PathBuf`: The resolved pkg directory containing WASM build artifacts.
pub(crate) fn resolve_pkg_dir(www_dir: &Path) -> PathBuf {
    let direct_pkg: PathBuf = www_dir.join("pkg");
    if direct_pkg.join("euv_example.js").exists() || direct_pkg.join(".gitignore").exists() {
        return direct_pkg;
    }
    let parent_name: Option<&str> = www_dir.file_name().and_then(|n| n.to_str());
    if let Some(name) = parent_name {
        let nested_pkg: PathBuf = www_dir.join(name).join("pkg");
        if nested_pkg.join("euv_example.js").exists() || nested_pkg.join(".gitignore").exists() {
            return nested_pkg;
        }
    }
    let grandparent: Option<&Path> = www_dir.parent();
    if let Some(parent) = grandparent {
        let sibling_pkg = parent.join("pkg");
        if sibling_pkg.join("euv_example.js").exists() || sibling_pkg.join(".gitignore").exists() {
            return sibling_pkg;
        }
    }
    direct_pkg
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
    println!("Watching {} for changes...", src_path.display());
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
                    println!("WASM build completed successfully");
                    if let Err(error) = update_html(&state_for_build).await {
                        eprintln!("Failed to update HTML: {}", error);
                    }
                    let _ = state_for_build.reload_tx.send(ReloadEvent::Reload);
                }
                Err(error) => {
                    eprintln!("WASM build failed: {}", error);
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
    println!(
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
