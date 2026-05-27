use crate::*;

/// Sets the global application state.
///
/// # Arguments
///
/// - `Arc<AppState>` - The shared application state to store globally.
///
/// # Returns
///
/// - `Result<()>` - Indicates success or failure of the initialization.
pub(crate) fn set_global_state(state: Arc<AppState>) -> Result<()> {
    APP_STATE
        .set(state)
        .map_err(|_: Arc<AppState>| anyhow!("Global state already initialized"))
}

/// Retrieves the global application state.
///
/// # Returns
///
/// - `Option<Arc<AppState>>` - The global state if initialized.
pub(crate) fn get_global_state() -> Option<Arc<AppState>> {
    APP_STATE.get().cloned()
}

/// Generates `index.html` based on the build profile.
///
/// Uses `INDEX_HTML_RELEASE` when `is_release` is `true` (no live-reload script),
/// otherwise uses `INDEX_HTML_DEV` (includes live-reload instrumentation).
///
/// Then writes the template with the import path placeholder replaced to disk.
///
/// # Arguments
///
/// - `&Path` - The path to the www directory where `index.html` will be written.
/// - `&str` - The JS import path relative to the www directory (e.g. `./pkg/euv` or `./euv`).
/// - `bool` - Whether to use the release template (no live-reload).
///
/// # Returns
///
/// - `Result<String>` - The generated HTML content written to disk.
pub(crate) async fn generate_html(
    www_dir: &Path,
    import_path: &str,
    is_release: bool,
) -> Result<String> {
    let template: &str = if is_release {
        INDEX_HTML_RELEASE
    } else {
        INDEX_HTML_DEV
    };
    let html: String = template
        .replace(IMPORT_PATH_PLACEHOLDER, import_path)
        .replace(RELOAD_ROUTE_PLACEHOLDER, RELOAD_ROUTE);
    let index_path: PathBuf = www_dir.join(INDEX_HTML_FILE_NAME);
    create_dir_all(www_dir)
        .await
        .map_err(|error: io::Error| anyhow!("Failed to create static directory: {}", error))?;
    write(&index_path, &html)
        .await
        .map_err(|error: io::Error| anyhow!("Failed to write index.html: {}", error))?;
    Ok(html)
}

/// Resolves the effective www directory, handling wasm-pack nested output.
///
/// # Arguments
///
/// - `&Path` - The candidate www directory path.
///
/// # Returns
///
/// - `PathBuf` - The resolved www directory containing `index.html`.
pub(crate) async fn resolve_www_dir(www_dir: &Path) -> PathBuf {
    if metadata(www_dir.join(INDEX_HTML_FILE_NAME)).await.is_ok() {
        return www_dir.to_path_buf();
    }
    let parent_name: Option<&str> = www_dir
        .file_name()
        .and_then(|file_name_os_str: &ffi::OsStr| file_name_os_str.to_str());
    if let Some(name) = parent_name {
        let nested: PathBuf = www_dir.join(name);
        if metadata(nested.join(INDEX_HTML_FILE_NAME)).await.is_ok() {
            return nested;
        }
    }
    www_dir.to_path_buf()
}

/// Resolves the pkg directory for serving WASM artifacts.
///
/// Delegates to `resolve_out_dir` which respects `--out-dir`
/// from wasm-pack args or defaults to `{www_dir}/pkg`.
///
/// # Arguments
///
/// - `&ModeArgs` - The CLI arguments for resolving out_dir.
///
/// # Returns
///
/// - `PathBuf` - The resolved pkg directory containing WASM build artifacts.
pub(crate) fn resolve_pkg_dir(args: &ModeArgs) -> PathBuf {
    resolve_out_dir(args)
}
