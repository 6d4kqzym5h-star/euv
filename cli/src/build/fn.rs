use crate::*;

/// Checks whether `wasm_pack_args` already contains a build mode flag.
///
/// Returns `true` if any of `--dev`, `--release`, or `--profiling`
/// is present in the arguments list.
///
/// # Arguments
///
/// - `&[String]` - The wasm-pack arguments to search.
///
/// # Returns
///
/// - `bool` - Whether a build mode flag is already present.
pub(crate) fn has_build_mode_flag(wasm_pack_args: &[String]) -> bool {
    wasm_pack_args
        .iter()
        .any(|arg: &String| arg == DEV_FLAG || arg == RELEASE_FLAG || arg == PROFILING_FLAG)
}

/// Filters out euv-specific arguments from the wasm-pack arguments.
///
/// First locates the genuine passthrough arguments by taking everything
/// after the last `--` separator (or the full list if no `--` is present).
/// Then removes all known euv-specific flags and their values so that
/// only wasm-pack-compatible arguments remain.
///
/// # Arguments
///
/// - `&[String]` - The raw wasm-pack arguments to filter.
///
/// # Returns
///
/// - `Vec<String>` - The filtered arguments safe for wasm-pack.
pub(crate) fn filter_euv_args(wasm_pack_args: &[String]) -> Vec<String> {
    let raw_args: &[String] = if let Some(position) = wasm_pack_args
        .iter()
        .rposition(|arg: &String| arg == DOUBLE_DASH)
    {
        &wasm_pack_args[position + 1..]
    } else {
        wasm_pack_args
    };
    let mut filtered: Vec<String> = Vec::new();
    let mut skip_next: bool = false;
    for arg in raw_args {
        if skip_next {
            skip_next = false;
            continue;
        }
        if EUV_ARGS.contains(&arg.as_str()) {
            if arg.contains('=') {
                continue;
            }
            skip_next = true;
            continue;
        }
        filtered.push(arg.clone());
    }
    filtered
}

/// Reconciles euv-specific arguments that may have been collected into
/// `wasm_pack_args` (e.g. when placed after `--`) back into the
/// corresponding `ModeArgs` fields.
///
/// Because clap's `trailing_var_arg` collects all unrecognized arguments
/// into `wasm_pack_args`, any euv flag placed after `--` is not parsed
/// by clap into its dedicated field. This function scans `wasm_pack_args`
/// for known euv flags and overwrites the `ModeArgs` fields so that the
/// rest of the codebase can rely on the typed accessors regardless of
/// argument order.
///
/// # Arguments
///
/// - `&mut ModeArgs` - The CLI arguments to reconcile in-place.
pub(crate) fn reconcile_args(args: &mut ModeArgs) {
    let wasm_pack_args: Vec<String> = args.get_wasm_pack_args().clone();
    let mut crate_path: Option<PathBuf> = None;
    let mut port: Option<u16> = None;
    let mut www_dir: Option<String> = None;
    let mut index_html: Option<Option<PathBuf>> = None;
    let mut no_gitignore: Option<bool> = None;
    let mut dev: Option<bool> = None;
    let mut release: Option<bool> = None;
    let mut profiling: Option<bool> = None;
    let mut iter: std::slice::Iter<String> = wasm_pack_args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            CRATE_PATH_ARG | CRATE_PATH_ARG_SHORT => {
                if let Some(value) = iter.next() {
                    crate_path = Some(PathBuf::from(value));
                }
            }
            PORT_ARG | PORT_ARG_SHORT => {
                if let Some(value) = iter.next()
                    && let Ok(parsed_port) = value.parse::<u16>()
                {
                    port = Some(parsed_port);
                }
            }
            WWW_DIR_ARG => {
                if let Some(value) = iter.next() {
                    www_dir = Some(value.clone());
                }
            }
            INDEX_HTML_ARG => {
                if let Some(value) = iter.next() {
                    index_html = Some(Some(PathBuf::from(value)));
                }
            }
            NO_GITIGNORE_ARG => {
                no_gitignore = Some(true);
            }
            DEV_FLAG => {
                dev = Some(true);
            }
            RELEASE_FLAG => {
                release = Some(true);
            }
            PROFILING_FLAG => {
                profiling = Some(true);
            }
            other => {
                if let Some(value) = other.strip_prefix(&format!("{CRATE_PATH_ARG}=")) {
                    crate_path = Some(PathBuf::from(value));
                } else if let Some(value) = other.strip_prefix(&format!("{PORT_ARG}=")) {
                    if let Ok(parsed_port) = value.parse::<u16>() {
                        port = Some(parsed_port);
                    }
                } else if let Some(value) = other.strip_prefix(&format!("{WWW_DIR_ARG}=")) {
                    www_dir = Some(value.to_string());
                } else if let Some(value) = other.strip_prefix(&format!("{INDEX_HTML_ARG}=")) {
                    index_html = Some(Some(PathBuf::from(value)));
                }
            }
        }
    }
    if let Some(value) = crate_path {
        args.set_crate_path(value);
    }
    if let Some(value) = port {
        args.set_port(value);
    }
    if let Some(value) = www_dir {
        args.set_www_dir(value);
    }
    if let Some(value) = index_html {
        args.set_index_html(value);
    }
    if let Some(value) = no_gitignore {
        args.set_no_gitignore(value);
    }
    if let Some(value) = dev {
        args.set_dev(value);
    }
    if let Some(value) = release {
        args.set_release(value);
    }
    if let Some(value) = profiling {
        args.set_profiling(value);
    }
}

/// Resolves the build mode from CLI arguments.
///
/// First checks the explicit `--dev`, `--release`, and `--profiling` flags on `ModeArgs`.
/// If none of those are set, inspects `wasm_pack_args` for any build mode flag
/// that may have been forwarded by the user.
/// Defaults to `BuildMode::Dev` if no build mode flag is found anywhere.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments containing the build mode flags and wasm_pack_args.
///
/// # Returns
///
/// - `BuildMode` - The resolved build mode.
pub(crate) fn resolve_build_mode(args: &ModeArgs) -> BuildMode {
    if args.get_profiling() {
        BuildMode::Profiling
    } else if args.get_release() {
        BuildMode::Release
    } else if args.get_dev() {
        BuildMode::Dev
    } else if args
        .get_wasm_pack_args()
        .iter()
        .any(|arg: &String| arg == PROFILING_FLAG)
    {
        BuildMode::Profiling
    } else if args
        .get_wasm_pack_args()
        .iter()
        .any(|arg: &String| arg == RELEASE_FLAG)
    {
        BuildMode::Release
    } else {
        BuildMode::Dev
    }
}

/// Converts a `BuildMode` to the corresponding wasm-pack flag string.
///
/// # Arguments
///
/// - `BuildMode` - The build mode to convert.
///
/// # Returns
///
/// - `&'static str` - The wasm-pack command-line flag.
pub(crate) fn build_mode_to_flag(build_mode: BuildMode) -> &'static str {
    match build_mode {
        BuildMode::Dev => DEV_FLAG,
        BuildMode::Release => RELEASE_FLAG,
        BuildMode::Profiling => PROFILING_FLAG,
    }
}

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
        log::warn!("Failed to load .gitignore: {error}");
    }
    match builder.build() {
        Ok(gitignore) => {
            if gitignore_exists {
                log::info!("Loaded .gitignore to filter file change events");
            }
            gitignore
        }
        Err(error) => {
            log::warn!("Failed to build gitignore matcher: {error}");
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
    let name: String = if let Some(out_name) = extract_out_name(args.get_wasm_pack_args()) {
        out_name
    } else {
        let cargo_toml_path: PathBuf = args.get_crate_path().join(CARGO_TOML_FILE_NAME);
        read_crate_name_from_toml(&cargo_toml_path).unwrap_or_else(|| {
            args.get_crate_path()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        })
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
    let www_absolute: PathBuf = args.get_crate_path().join(args.get_www_dir());
    let out_dir_absolute: PathBuf = resolve_out_dir(args);
    let relative: PathBuf = match out_dir_absolute.strip_prefix(&www_absolute) {
        Ok(rel) => rel.to_path_buf(),
        Err(_) => out_dir_absolute,
    };
    let mut components: Vec<String> = relative
        .components()
        .filter_map(|component: Component| match component {
            Component::Normal(os_str) => os_str.to_str().map(|s: &str| s.to_string()),
            _ => None,
        })
        .collect();
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
    let out_dir_path: PathBuf = PathBuf::from(
        extract_out_dir(args.get_wasm_pack_args())
            .unwrap_or_else(|| format!("{}/{PKG_DIR_NAME}", args.get_www_dir())),
    );
    if out_dir_path.is_absolute() {
        out_dir_path
    } else {
        args.get_crate_path().join(&out_dir_path)
    }
}

/// Executes a build-only pipeline: formats euv macros, cleans output directory,
/// builds WASM, and generates HTML.
///
/// Unlike `run_build_pipeline`, this cleans the output directory before building
/// and skips reload notifications — only the essential WASM build artifacts are kept.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments.
///
/// # Returns
///
/// - `Result<()>` - Indicates success or failure of the build.
pub(crate) async fn run_build_only_pipeline(args: &ModeArgs) -> Result<()> {
    let src_path: PathBuf = args.get_crate_path().join(SRC_DIR_NAME);
    if let Err(error) = format_dir(&src_path, FmtMode::Write).await {
        log::warn!("euv fmt error: {error}");
    }
    let out_dir: PathBuf = resolve_out_dir(args);
    clean_out_dir(&out_dir).await;
    build_wasm(args).await?;
    log::info!("WASM build completed successfully");
    let www_dir: PathBuf = resolve_www_dir_from_args(args).await;
    let import_path: String = resolve_import_path(args);
    let is_release: bool = resolve_build_mode(args) == BuildMode::Release;
    let custom_html: &Option<PathBuf> = args.try_get_index_html();
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
            if let Err(error) = remove_dir_all(&path).await {
                log::warn!("Failed to remove directory '{}': {error}", path.display());
            }
        } else if let Err(error) = remove_file(&path).await {
            log::warn!("Failed to remove file '{}': {error}", path.display());
        }
    }
}

/// Executes a full build pipeline: euv fmt, build wasm, generate HTML.
/// After the serial pipeline completes, hyperlane-cli fmt is spawned in the
/// background so it does not block the caller.
/// Notifies the reload channel on build success or failure.
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
    let src_path: PathBuf = args.get_crate_path().join(SRC_DIR_NAME);
    if let Err(error) = format_dir(&src_path, FmtMode::Write).await {
        log::warn!("euv fmt error: {error}");
    }
    match build_wasm(args).await {
        Ok(()) => {
            log::info!("WASM build completed successfully");
            if let Some(sender) = reload_tx {
                let _ = sender.send(ReloadEvent::Reload);
            }
        }
        Err(error) => {
            log::error!("WASM build failed: {error}");
            if let Some(sender) = reload_tx {
                let _ = sender.send(ReloadEvent::Error(error.to_string()));
            }
        }
    }
    let www_dir: PathBuf = resolve_www_dir_from_args(args).await;
    let import_path: String = resolve_import_path(args);
    let is_release: bool = resolve_build_mode(args) == BuildMode::Release;
    let custom_html: &Option<PathBuf> = args.try_get_index_html();
    let html: String = generate_html(&www_dir, &import_path, is_release, custom_html).await?;
    spawn(async move {
        if let Err(error) = run_hyperlane_fmt().await {
            log::warn!("hyperlane-cli fmt error: {error}");
        }
    });
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
    let www_absolute: PathBuf = args.get_crate_path().join(args.get_www_dir());
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
    let crate_path: PathBuf = state.get_args().get_crate_path().clone();
    let src_path: PathBuf = crate_path.join(SRC_DIR_NAME);
    let gitignore: Gitignore = build_gitignore(&crate_path).await;
    let (tx, mut rx): (Sender<Event>, Receiver<Event>) = channel(32);
    let mut watcher: RecommendedWatcher = RecommendedWatcher::new(
        move |result: std::result::Result<Event, notify::Error>| {
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
                    log::error!("Build pipeline error: {error}");
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
    let build_mode: BuildMode = resolve_build_mode(args);
    let build_mode_flag: &str = build_mode_to_flag(build_mode);
    let filtered_args: Vec<String> = filter_euv_args(args.get_wasm_pack_args());
    let has_existing_build_mode: bool = has_build_mode_flag(&filtered_args);
    let default_out_dir: String = format!("{}/{PKG_DIR_NAME}", args.get_www_dir());
    let mut command: Command = Command::new(WASM_PACK_COMMAND);
    command.arg(WASM_PACK_BUILD_SUBCOMMAND);
    if !has_existing_build_mode {
        command.arg(build_mode_flag);
    }
    command
        .args(&filtered_args)
        .env(RUST_MIN_STACK_ENV, RUST_MIN_STACK_VALUE);
    let has_out_dir: bool = extract_out_dir(&filtered_args).is_some();
    if !has_out_dir {
        command.arg(OUT_DIR_ARG).arg(&default_out_dir);
    }
    let has_target: bool = filtered_args
        .iter()
        .any(|arg: &String| arg == TARGET_ARG || arg.starts_with(&format!("{TARGET_ARG}=")));
    if !has_target {
        command.arg(TARGET_ARG).arg(TARGET_WEB);
    }
    command.current_dir(args.get_crate_path());
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let display_args: Vec<String> = (if has_existing_build_mode {
        filtered_args.to_vec()
    } else {
        std::iter::once(build_mode_flag.to_string())
            .chain(filtered_args.iter().cloned())
            .collect::<Vec<String>>()
    })
    .into_iter()
    .chain(if has_out_dir {
        Vec::new()
    } else {
        vec![OUT_DIR_ARG.to_string(), default_out_dir.clone()]
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
        .map_err(|error: std::io::Error| EuvError::IoPath {
            message: String::from("Failed to create output directory"),
            path: out_dir_absolute.clone(),
            error,
        })?;
    log::info!(
        "Running: {WASM_PACK_COMMAND} {WASM_PACK_BUILD_SUBCOMMAND} {} ...",
        display_args.join(" ")
    );
    let output: Output = command
        .output()
        .await
        .map_err(|error: std::io::Error| EuvError::Io {
            message: String::from("Failed to execute wasm-pack"),
            error,
        })?;
    let stdout: String = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr: String = String::from_utf8_lossy(&output.stderr).to_string();
    if args.get_no_gitignore() {
        let gitignore_path: PathBuf = out_dir_absolute.join(GITIGNORE_FILE_NAME);
        if gitignore_path.exists()
            && let Err(error) = remove_file(&gitignore_path).await
        {
            log::warn!("Failed to remove '{}': {error}", gitignore_path.display());
        }
    }
    for line in stdout.lines().filter(|line: &&str| !line.is_empty()) {
        log::info!("{line}");
    }
    if output.status.success() {
        for line in stderr.lines().filter(|line: &&str| !line.is_empty()) {
            log::info!("{line}");
        }
    } else {
        for line in stderr.lines().filter(|line: &&str| !line.is_empty()) {
            log::error!("{line}");
        }
        return Err(EuvError::Message(String::from("wasm-pack build failed")));
    }
    Ok(())
}

/// Prints the startup banner and command information.
///
/// # Arguments
///
/// - `Action` - The action to perform (run or build).
pub(crate) fn print_banner(action: Action) {
    let version: &str = env!("CARGO_PKG_VERSION");
    if version.is_empty() {
        log::warn!("Failed to parse version from root Cargo.toml");
    } else {
        log::info!("euv v{version}");
    }
    let action_name: &str = match action {
        Action::Run => ACTION_RUN,
        Action::Build => ACTION_BUILD,
    };
    log::info!("Mode: {action_name}");
    log::info!(
        "Use .gitignore to filter file change events; pass --no-gitignore to remove .gitignore from output"
    );
}

/// Enumerates all network interface IP addresses and prints each server URL
/// along with its corresponding QR code to the console.
///
/// Includes both loopback (127.0.0.1) and all private/public IPv4 addresses
/// bound to the host's network interfaces. Each address produces one URL line
/// followed by a Unicode QR code rendered with half-block characters,
/// where every line carries the standard log prefix (timestamp + level).
///
/// # Arguments
///
/// - `u16` - The port number the server is listening on.
/// - `&str` - The www route prefix (e.g. "www").
/// - `&str` - The index HTML file name (e.g. "index.html").
pub(crate) fn print_server_urls(port: u16, www_route_prefix: &str, index_html_file_name: &str) {
    let mut addresses: Vec<std::net::IpAddr> = Vec::new();
    match if_addrs::get_if_addrs() {
        Ok(interfaces) => {
            for interface in interfaces {
                let ip: std::net::IpAddr = interface.addr.ip();
                if !addresses.contains(&ip) {
                    addresses.push(ip);
                }
            }
        }
        Err(error) => {
            log::warn!("Failed to enumerate network interfaces: {error}");
        }
    }
    if addresses.is_empty() {
        addresses.push(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST));
    }
    for ip in addresses {
        let host: String = match ip {
            std::net::IpAddr::V6(_) => format!("[{ip}]"),
            std::net::IpAddr::V4(_) => format!("{ip}"),
        };
        let url: String =
            format!("{HTTP_SCHEME}://{host}:{port}/{www_route_prefix}/{index_html_file_name}");
        log::info!("Server: {url}");
        match qrcode::QrCode::new(url.as_str()) {
            Ok(code) => {
                let string: String = code
                    .render::<qrcode::render::unicode::Dense1x2>()
                    .quiet_zone(false)
                    .build();
                for line in string.lines() {
                    log::info!("{line}");
                }
            }
            Err(error) => {
                log::warn!("Failed to generate QR code: {error}");
            }
        }
    }
}

/// Executes `hyperlane-cli fmt` via the library API to format Rust source files.
///
/// # Returns
///
/// - `Result<()>` - Indicates success or failure of the formatting operation.
pub(crate) async fn run_hyperlane_fmt() -> Result<()> {
    let args: hyperlane_cli::Args = hyperlane_cli::Args {
        command: hyperlane_cli::CommandType::Fmt,
        check: false,
        manifest_path: None,
        bump_type: None,
        max_retries: 0,
        project_name: None,
        template_type: None,
        model_sub_type: None,
        component_name: None,
    };
    hyperlane_cli::execute_fmt(&args)
        .await
        .map_err(|error: std::io::Error| EuvError::Io {
            message: String::from("hyperlane-cli fmt error"),
            error,
        })
}
