use crate::*;

/// Reactive state for the native bridge system info feature.
///
/// Aggregates all signals needed for the bridge permissions display.
#[derive(Clone, Copy, Data, New)]
pub struct UseEuvNativeBridge {
    /// Whether the native bridge is available on this platform.
    #[get(type(copy))]
    pub available: Signal<bool>,
    /// Whether data is currently being loaded from the native bridge.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The resolved bridge group permissions.
    #[get(type(copy))]
    pub permissions: Signal<String>,
}

/// Reactive state for the cache update feature.
///
/// Tracks the documentation build status, the latest version string,
/// and whether a cache update is currently in progress.
#[derive(Clone, Copy, Data, New)]
pub struct UseCacheUpdate {
    /// Whether the docs.rs build status indicates documentation is available.
    #[get(type(copy))]
    pub doc_status: Signal<bool>,
    /// The latest version string returned by docs.rs.
    #[get(type(copy))]
    pub version: Signal<String>,
    /// Whether a cache update operation is currently in progress.
    #[get(type(copy))]
    pub updating: Signal<bool>,
}

/// The result returned by a cache update closure.
///
/// The UI component uses this value to update its internal state signals
/// without knowing how the update check was performed.
#[derive(Clone, Data, New)]
pub struct UpdateResult {
    /// Whether the documentation build succeeded.
    #[get(type(copy))]
    pub doc_status: bool,
    /// The latest version string of the crate.
    pub version: String,
    /// Whether a cache update operation was triggered.
    #[get(type(copy))]
    pub updating: bool,
}

/// Configuration for bridge initialization.
///
/// Allows customization of bridge behavior.
#[derive(Clone, Data, Debug, Default, New)]
pub struct BridgeConfig {
    /// The global bridge object key on window.
    #[get(type(copy))]
    pub global_key: &'static str,
    /// The core module key.
    #[get(type(copy))]
    pub core_key: &'static str,
    /// The invoke function key.
    #[get(type(copy))]
    pub invoke_key: &'static str,
}
