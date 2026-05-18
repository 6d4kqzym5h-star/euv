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

/// Writes a clean `index.html` from `DEFAULT_INDEX_HTML`, then reads it back,
/// injects the live-reload script, and writes the final result back to disk.
///
/// Always overwrites `index.html` to ensure a consistent base before
/// injecting the reload script.
///
/// # Arguments
///
/// - `&Path` - The path to the www directory where `index.html` will be written.
///
/// # Returns
///
/// - `Result<String>` - The modified HTML with the reload script injected.
pub(crate) async fn generate_dev_html(www_dir: &Path) -> Result<String> {
    let index_path: PathBuf = www_dir.join("index.html");
    create_dir_all(www_dir)
        .await
        .map_err(|error| anyhow!("Failed to create www directory: {}", error))?;
    write(&index_path, DEFAULT_INDEX_HTML)
        .await
        .map_err(|error| anyhow!("Failed to write index.html: {}", error))?;
    let original: String = read_to_string(&index_path)
        .await
        .map_err(|error| anyhow!("Failed to read index.html: {}", error))?;
    let mut html: String = if original.contains("</html>") {
        original.replace("</html>", &format!("{}\n</html>", RELOAD_SCRIPT))
    } else {
        format!("{}\n{}", original, RELOAD_SCRIPT)
    };
    html = html.replace("./euv_example.js", "./pkg/euv_example.js");
    write(&index_path, &html)
        .await
        .map_err(|error| anyhow!("Failed to write dev index.html: {}", error))?;
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
