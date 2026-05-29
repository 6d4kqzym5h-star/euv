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
    let gitignore_path: PathBuf = root.join(GITIGNORE_FILE_NAME);
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
                .unwrap_or_else(|_error: ignore::Error| Gitignore::empty())
        }
    }
}

/// Extracts the value of `--out-name` from the wasm-pack arguments.
///
/// Returns `None` if `--out-name` is not specified.
///
/// # Arguments
///
/// - `&[String]` - The wasm-pack arguments to search.
///
/// # Returns
///
/// - `Option<String>` - The value of `--out-name` if found.
fn extract_out_name(wasm_pack_args: &[String]) -> Option<String> {
    let mut iter = wasm_pack_args.iter();
    while let Some(arg) = iter.next() {
        if arg == OUT_NAME_ARG {
            return iter.next().cloned();
        }
        if let Some(value) = arg.strip_prefix(&format!("{OUT_NAME_ARG}=")) {
            return Some(value.to_string());
        }
    }
    None
}

/// Extracts the value of `--out-dir` from the wasm-pack arguments.
///
/// Returns `None` if `--out-dir` is not specified.
///
/// # Arguments
///
/// - `&[String]` - The wasm-pack arguments to search.
///
/// # Returns
///
/// - `Option<String>` - The value of `--out-dir` if found.
fn extract_out_dir(wasm_pack_args: &[String]) -> Option<String> {
    let mut iter = wasm_pack_args.iter();
    while let Some(arg) = iter.next() {
        if arg == OUT_DIR_ARG {
            return iter.next().cloned();
        }
        if let Some(value) = arg.strip_prefix(&format!("{OUT_DIR_ARG}=")) {
            return Some(value.to_string());
        }
    }
    None
}

/// Resolves the output JS filename for HTML generation.
///
/// Uses `--out-name` from wasm-pack args if specified,
/// otherwise reads the crate name from `Cargo.toml` `[package] name` field
/// and replaces hyphens with underscores (matching wasm-pack behavior).
/// Appends `.js` extension to form the complete JS filename.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments containing crate_path and wasm_pack_args.
///
/// # Returns
///
/// - `String` - The resolved JS filename with `.js` extension (e.g. `euv_example.js`).
pub(crate) fn resolve_out_name(args: &ModeArgs) -> String {
    let name: String = if let Some(out_name) = extract_out_name(&args.wasm_pack_args) {
        out_name
    } else {
        let cargo_toml_path: PathBuf = args.crate_path.join(CARGO_TOML_FILE_NAME);
        let crate_name: String = read_crate_name_from_toml(&cargo_toml_path).unwrap_or_else(|| {
            args.crate_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        });
        crate_name.replace('-', "_")
    };
    format!("{name}{JS_EXTENSION}")
}

/// Reads the `name` field from a Cargo.toml file.
///
/// Parses the file line-by-line looking for `name = "..."` within the `[package]` section.
///
/// # Arguments
///
/// - `&Path` - The path to the Cargo.toml file.
///
/// # Returns
///
/// - `Option<String>` - The crate name if found.
fn read_crate_name_from_toml(path: &Path) -> Option<String> {
    let content: String = std::fs::read_to_string(path).ok()?;
    let mut in_package: bool = false;
    for line in content.lines() {
        let trimmed: &str = line.trim();
        if trimmed.starts_with('[') {
            in_package = trimmed == "[package]";
            continue;
        }
        if in_package
            && trimmed.starts_with("name")
            && let Some(value) = trimmed.strip_prefix("name")
        {
            let value: &str = value.trim().strip_prefix('=')?.trim();
            let value: &str = value.strip_prefix('"')?.strip_suffix('"')?;
            return Some(value.to_string());
        }
    }
    None
}

/// Resolves the JS import path for HTML generation.
///
/// Computes the relative path from the www directory to the output directory,
/// then appends the JS filename (from `resolve_out_name`, which includes `.js`)
/// to form the full import path (e.g. `./pkg/euv.js` or `./euv.js`).
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments containing crate_path, www_dir, wasm_pack_args.
///
/// # Returns
///
/// - `String` - The resolved JS import path relative to the www directory.
pub(crate) fn resolve_import_path(args: &ModeArgs) -> String {
    let out_name: String = resolve_out_name(args);
    let www_absolute: PathBuf = args.crate_path.join(&args.www_dir);
    let out_dir_absolute: PathBuf = resolve_out_dir(args);
    let relative: PathBuf = match out_dir_absolute.strip_prefix(&www_absolute) {
        Ok(rel) => rel.to_path_buf(),
        Err(_) => out_dir_absolute.clone(),
    };
    let mut components: Vec<String> = Vec::new();
    for component in relative.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(os_str) => {
                if let Some(s) = os_str.to_str() {
                    components.push(s.to_string());
                }
            }
            _ => {}
        }
    }
    components.push(out_name);
    format!("{RELATIVE_PATH_PREFIX}{}", components.join(PATH_SEPARATOR))
}

/// Resolves the output directory for wasm-pack artifacts.
///
/// Uses `--out-dir` from wasm-pack args if specified,
/// otherwise defaults to `{www_dir}/pkg` so that build artifacts
/// are placed directly inside the www directory served
/// by the development server.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments containing crate_path, www_dir, and wasm_pack_args.
///
/// # Returns
///
/// - `PathBuf` - The resolved output directory (absolute if crate_path is joined).
pub(crate) fn resolve_out_dir(args: &ModeArgs) -> PathBuf {
    let default_out_dir: String = format!("{}/{PKG_DIR_NAME}", args.www_dir);
    let out_dir: String = extract_out_dir(&args.wasm_pack_args).unwrap_or(default_out_dir);
    let out_dir_path: PathBuf = PathBuf::from(out_dir);
    if out_dir_path.is_absolute() {
        out_dir_path
    } else {
        args.crate_path.join(&out_dir_path)
    }
}

/// Executes a build-only pipeline: formats euv macros, builds WASM and removes unnecessary files.
///
/// Unlike `run_build_pipeline`, this skips reload notifications
/// — only the essential WASM build artifacts are kept.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments.
///
/// # Returns
///
/// - `Result<()>` - Indicates success or failure of the build.
pub(crate) async fn run_build_only_pipeline(args: &ModeArgs) -> Result<()> {
    let src_path: PathBuf = args.crate_path.join(SRC_DIR_NAME);
    if let Err(error) = format_dir(&src_path, FmtMode::Write).await {
        log::warn!("euv fmt error: {}", error);
    }
    let out_dir: PathBuf = resolve_out_dir(args);
    clean_out_dir(&out_dir).await;
    build_wasm(args).await?;
    log::info!("WASM build completed successfully");
    let pkg_dir: PathBuf = resolve_out_dir(args);
    clean_pkg_dir(&pkg_dir).await;
    let www_dir: PathBuf = resolve_www_dir_from_args(args).await;
    let import_path: String = resolve_import_path(args);
    let is_release: bool = args.wasm_pack_args.contains(&RELEASE_FLAG.to_string());
    let custom_html: Option<&Path> = args.index_html.as_deref();
    generate_html(&www_dir, &import_path, is_release, custom_html).await?;
    Ok(())
}

/// Cleans the output directory before a fresh build.
///
/// Removes all files and subdirectories within the output directory
/// so that stale artifacts from previous builds do not remain.
/// The directory itself is preserved (recreated if missing).
///
/// # Arguments
///
/// - `&Path` - The output directory to clean.
pub(crate) async fn clean_out_dir(out_dir: &Path) {
    let mut entries: ReadDir = match read_dir(out_dir).await {
        Ok(dir) => dir,
        Err(_) => return,
    };
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            if let Err(error) = tokio::fs::remove_dir_all(&path).await {
                log::warn!("Failed to remove directory '{}': {}", path.display(), error);
            }
        } else if let Err(error) = remove_file(&path).await {
            log::warn!("Failed to remove file '{}': {}", path.display(), error);
        }
    }
}

/// Removes unnecessary files from the wasm-pack output directory.
///
/// Keeps only the files required for runtime: `.js` and `.wasm`.
/// Removes TypeScript declarations (`.d.ts`), package metadata
/// (`package.json`, `README.md`, `LICENSE`), and `.gitignore`.
///
/// # Arguments
///
/// - `&Path` - The pkg output directory to clean.
pub(crate) async fn clean_pkg_dir(pkg_dir: &Path) {
    let unnecessary_extensions: &[&str] = &[D_TS_EXTENSION];
    let unnecessary_names: &[&str] = &[
        PACKAGE_JSON_FILE_NAME,
        README_FILE_NAME,
        LICENSE_FILE_NAME,
        GITIGNORE_FILE_NAME,
    ];
    let mut read_dir: ReadDir = match read_dir(pkg_dir).await {
        Ok(dir) => dir,
        Err(_) => {
            log::warn!("pkg directory not found: {}", pkg_dir.display());
            return;
        }
    };
    while let Ok(Some(entry)) = read_dir.next_entry().await {
        let path: PathBuf = entry.path();
        let file_name: String = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let extension: String = path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let full_extension: String = if path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .ends_with(D_TS_EXTENSION)
        {
            D_TS_EXTENSION.to_string()
        } else {
            extension
        };
        let should_remove: bool = unnecessary_names.contains(&file_name.as_str())
            || unnecessary_extensions.contains(&full_extension.as_str());
        if should_remove {
            match remove_file(&path).await {
                Ok(()) => {
                    log::info!("Removed unnecessary file: {}", file_name);
                }
                Err(error) => {
                    log::warn!("Failed to remove {}: {}", file_name, error);
                }
            }
        }
    }
}

/// Executes a full build pipeline: euv fmt,
/// build WASM, notify reload channel, and generate updated HTML.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments.
/// - `Option<&broadcast::Sender<ReloadEvent>>` - Optional reload channel for notifying clients.
///
/// # Returns
///
/// - `Result<String>` - The generated HTML with reload script injected on success.
pub(crate) async fn run_build_pipeline(
    args: &ModeArgs,
    reload_tx: Option<&broadcast::Sender<ReloadEvent>>,
) -> Result<String> {
    let src_path: PathBuf = args.crate_path.join(SRC_DIR_NAME);
    if let Err(error) = format_dir(&src_path, FmtMode::Write).await {
        log::warn!("euv fmt error: {}", error);
    }
    if let Err(error) = run_hyperlane_fmt().await {
        log::warn!("hyperlane-cli fmt error: {}", error);
    }
    let out_dir: PathBuf = resolve_out_dir(args);
    clean_out_dir(&out_dir).await;
    match build_wasm(args).await {
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
    let www_dir: PathBuf = resolve_www_dir_from_args(args).await;
    let import_path: String = resolve_import_path(args);
    let is_release: bool = args.wasm_pack_args.contains(&RELEASE_FLAG.to_string());
    let custom_html: Option<&Path> = args.index_html.as_deref();
    let html: String = generate_html(&www_dir, &import_path, is_release, custom_html).await?;
    Ok(html)
}

/// Resolves the www directory from CLI arguments.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments.
///
/// # Returns
///
/// - `PathBuf` - The resolved www directory.
async fn resolve_www_dir_from_args(args: &ModeArgs) -> PathBuf {
    let www_absolute: PathBuf = args.crate_path.join(&args.www_dir);
    resolve_www_dir(&www_absolute).await
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
    let src_path: PathBuf = crate_path.join(SRC_DIR_NAME);
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
        spawn(async move {
            let args: ModeArgs = state_for_build.args.clone();
            let reload_tx: broadcast::Sender<ReloadEvent> = state_for_build.reload_tx.clone();
            match run_build_pipeline(&args, Some(&reload_tx)).await {
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
/// All arguments in `args.wasm_pack_args` are transparently forwarded
/// to `wasm-pack build`. If `--out-dir` is not specified by the user,
/// `--out-dir {www_dir}/pkg` is automatically injected so that build artifacts
/// are placed inside the www directory served by the development server.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments containing crate_path, www_dir, and wasm_pack_args.
///
/// # Returns
///
/// - `Result<()>` - Indicates success or failure of the wasm-pack build.
pub(crate) async fn build_wasm(args: &ModeArgs) -> Result<()> {
    let default_out_dir: String = format!("{}/{PKG_DIR_NAME}", args.www_dir);
    let mut command: Command = Command::new(WASM_PACK_COMMAND);
    command
        .arg(WASM_PACK_BUILD_SUBCOMMAND)
        .args(&args.wasm_pack_args);
    let has_out_dir: bool = extract_out_dir(&args.wasm_pack_args).is_some();
    if !has_out_dir {
        command.arg(OUT_DIR_ARG).arg(&default_out_dir);
    }
    let has_target: bool = args
        .wasm_pack_args
        .iter()
        .any(|arg: &String| arg == TARGET_ARG || arg.starts_with(&format!("{TARGET_ARG}=")));
    if !has_target {
        command.arg(TARGET_ARG).arg(TARGET_WEB);
    }
    command.current_dir(&args.crate_path);
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let display_args: Vec<String> = args
        .wasm_pack_args
        .iter()
        .cloned()
        .chain(if has_out_dir {
            Vec::new()
        } else {
            vec![OUT_DIR_ARG.to_string(), default_out_dir]
        })
        .chain(if has_target {
            Vec::new()
        } else {
            vec![TARGET_ARG.to_string(), TARGET_WEB.to_string()]
        })
        .collect();
    let out_dir_absolute: PathBuf = resolve_out_dir(args);
    create_dir_all(&out_dir_absolute)
        .await
        .map_err(|error: std::io::Error| {
            anyhow!(
                "Failed to create output directory '{}': {}",
                out_dir_absolute.display(),
                error
            )
        })?;
    log::info!(
        "Running: {} {} {} ...",
        WASM_PACK_COMMAND,
        WASM_PACK_BUILD_SUBCOMMAND,
        display_args.join(" ")
    );
    let output: Output = command
        .output()
        .await
        .map_err(|error: std::io::Error| anyhow!("Failed to execute wasm-pack: {}", error))?;
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
/// - `Action` - The action to perform (run or build).
pub(crate) fn print_banner(action: Action) {
    log::info!("euv v{}", env!("CARGO_PKG_VERSION"));
    let action_name: &str = match action {
        Action::Run => ACTION_RUN,
        Action::Build => ACTION_BUILD,
    };
    log::info!("Mode: {}", action_name);
    log::info!(".gitignore can exclude unwanted file change events from triggering rebuilds");
}

/// Runs `hyperlane-cli fmt` to format the Rust source files.
/// If `hyperlane-cli` is not installed, automatically installs it via `cargo install`.
///
/// # Returns
///
/// - `Result<()>` - Indicates success or failure of the formatting operation.
pub(crate) async fn run_hyperlane_fmt() -> Result<()> {
    let which_output: Output = Command::new(HYPERLANE_CLI_COMMAND)
        .arg(VERSION_ARG)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|error: std::io::Error| {
            anyhow!("Failed to check hyperlane-cli availability: {}", error)
        })?;
    if !which_output.status.success() {
        log::info!("hyperlane-cli not found, installing via cargo install...");
        let install_output: Output = Command::new(CARGO_COMMAND)
            .args([CARGO_INSTALL_SUBCOMMAND, HYPERLANE_CLI_COMMAND])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|error: std::io::Error| {
                anyhow!("Failed to execute cargo install hyperlane-cli: {}", error)
            })?;
        if !install_output.status.success() {
            let stderr: String = String::from_utf8_lossy(&install_output.stderr).to_string();
            bail!("cargo install hyperlane-cli failed:\n{}", stderr);
        }
        log::info!("hyperlane-cli installed successfully");
    }
    let fmt_output: Output = Command::new(HYPERLANE_CLI_COMMAND)
        .arg(FMT_SUBCOMMAND)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|error: std::io::Error| {
            anyhow!("Failed to execute hyperlane-cli fmt: {}", error)
        })?;
    if !fmt_output.status.success() {
        let stderr: String = String::from_utf8_lossy(&fmt_output.stderr).to_string();
        bail!("hyperlane-cli fmt failed:\n{}", stderr);
    }
    Ok(())
}
