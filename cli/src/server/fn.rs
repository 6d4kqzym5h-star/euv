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

/// Reads the original index.html and injects the live-reload script.
/// If index.html does not exist, a default one is created automatically.
///
/// # Arguments
///
/// - `&Path` - The path to the www directory containing `index.html`.
///
/// # Returns
///
/// - `Result<String>` - The modified HTML with the reload script injected.
pub(crate) async fn generate_dev_html(www_dir: &Path) -> Result<String> {
    let index_path: PathBuf = www_dir.join("index.html");
    if !index_path.exists() {
        create_dir_all(www_dir)
            .await
            .map_err(|e| anyhow!("Failed to create www directory: {}", e))?;
        write(&index_path, DEFAULT_INDEX_HTML)
            .await
            .map_err(|e| anyhow!("Failed to write default index.html: {}", e))?;
    }
    let original: String = read_to_string(&index_path)
        .await
        .map_err(|e| anyhow!("Failed to read index.html: {}", e))?;
    let mut html: String = if original.contains("</body>") {
        original.replace("</body>", &format!("{}\n</body>", RELOAD_SCRIPT))
    } else {
        format!("{}\n{}", original, RELOAD_SCRIPT)
    };
    html = html.replace("./euv_example.js", "./pkg/euv_example.js");
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
/// # Arguments
///
/// - `&Path` - The www directory path to search within.
///
/// # Returns
///
/// - `PathBuf` - The resolved pkg directory containing WASM build artifacts.
pub(crate) async fn resolve_pkg_dir(www_dir: &Path) -> PathBuf {
    let direct_pkg: PathBuf = www_dir.join("pkg");
    if metadata(direct_pkg.join("euv_example.js")).await.is_ok()
        || metadata(direct_pkg.join(".gitignore")).await.is_ok()
    {
        return direct_pkg;
    }
    let parent_name: Option<&str> = www_dir.file_name().and_then(|n| n.to_str());
    if let Some(name) = parent_name {
        let nested_pkg: PathBuf = www_dir.join(name).join("pkg");
        if metadata(nested_pkg.join("euv_example.js")).await.is_ok()
            || metadata(nested_pkg.join(".gitignore")).await.is_ok()
        {
            return nested_pkg;
        }
    }
    let grandparent: Option<&Path> = www_dir.parent();
    if let Some(parent) = grandparent {
        let sibling_pkg: PathBuf = parent.join("pkg");
        if metadata(sibling_pkg.join("euv_example.js")).await.is_ok()
            || metadata(sibling_pkg.join(".gitignore")).await.is_ok()
        {
            return sibling_pkg;
        }
    }
    direct_pkg
}
