use crate::*;

/// Reactive state for the native bridge system info feature.
///
/// Aggregates all signals needed for the bridge permissions display.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseEuvNativeBridge {
    /// Whether the native bridge is available on this platform.
    #[get(type(copy))]
    pub(crate) available: Signal<bool>,
    /// Whether data is currently being loaded from the native bridge.
    #[get(type(copy))]
    pub(crate) loading: Signal<bool>,
    /// The resolved bridge group permissions.
    #[get(type(copy))]
    pub(crate) permissions: Signal<String>,
}

/// Reactive state for the cache update feature.
///
/// Tracks the documentation build status, the latest version string,
/// and whether a cache update is currently in progress.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseCacheUpdate {
    /// Whether the docs.rs build status indicates documentation is available.
    #[get(type(copy))]
    pub(crate) doc_status: Signal<bool>,
    /// The latest version string returned by docs.rs.
    #[get(type(copy))]
    pub(crate) version: Signal<String>,
    /// Whether a cache update operation is currently in progress.
    #[get(type(copy))]
    pub(crate) updating: Signal<bool>,
}

/// The result returned by a cache update closure.
///
/// The UI component uses this value to update its internal state signals
/// without knowing how the update check was performed.
#[derive(Clone, Data, New)]
pub(crate) struct UpdateResult {
    /// Whether the documentation build succeeded.
    #[get(type(copy))]
    pub(crate) doc_status: bool,
    /// The latest version string of the crate.
    pub(crate) version: String,
    /// Whether a cache update operation was triggered.
    #[get(type(copy))]
    pub(crate) updating: bool,
}

/// Configuration for bridge initialization.
///
/// Allows customization of bridge behavior.
#[derive(Clone, Data, Debug, New)]
pub(crate) struct BridgeConfig {
    /// The global bridge object key on window.
    #[get(type(copy))]
    pub(crate) global_key: &'static str,
    /// The core module key.
    #[get(type(copy))]
    pub(crate) core_key: &'static str,
    /// The invoke function key.
    #[get(type(copy))]
    pub(crate) invoke_key: &'static str,
}
