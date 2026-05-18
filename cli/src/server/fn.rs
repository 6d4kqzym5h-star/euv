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
        .map_err(|_| anyhow!("Global state already initialized"))
}

/// Retrieves the global application state.
///
/// # Returns
///
/// - `Option<Arc<AppState>>` - The global state if initialized.
pub(crate) fn get_global_state() -> Option<Arc<AppState>> {
    APP_STATE.get().cloned()
}

/// Generates `index.html` based on the compile-time profile.
///
/// Uses the `INDEX_HTML` constant selected by `#[cfg]` at compile time:
/// - `debug_assertions` enabled → dev template with live-reload script
/// - `debug_assertions` disabled → release template (minimal production HTML)
///
/// Then writes the template with the import path placeholder replaced to disk.
///
/// # Arguments
///
/// - `&Path` - The path to the www directory where `index.html` will be written.
/// - `&str` - The JS import path relative to the www directory (e.g. `./pkg/euv` or `./euv`).
///
/// # Returns
///
/// - `Result<String>` - The generated HTML content written to disk.
pub(crate) async fn generate_html(www_dir: &Path, import_path: &str) -> Result<String> {
    let html: String = INDEX_HTML
        .replace(IMPORT_PATH_PLACEHOLDER, import_path)
        .replace(RELOAD_ROUTE_PLACEHOLDER, RELOAD_ROUTE);
    let index_path: PathBuf = www_dir.join("index.html");
    create_dir_all(www_dir)
        .await
        .map_err(|error| anyhow!("Failed to create static directory: {}", error))?;
    write(&index_path, &html)
        .await
        .map_err(|error| anyhow!("Failed to write index.html: {}", error))?;
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
    if metadata(www_dir.join("index.html")).await.is_ok() {
        return www_dir.to_path_buf();
    }
    let parent_name: Option<&str> = www_dir.file_name().and_then(|n| n.to_str());
    if let Some(name) = parent_name {
        let nested: PathBuf = www_dir.join(name);
        if metadata(nested.join("index.html")).await.is_ok() {
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
