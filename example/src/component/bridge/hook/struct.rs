use crate::*;

/// Reactive state for the native bridge system info feature.
///
/// Aggregates all signals needed for the bridge permissions display.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseNativeBridge {
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

/// The JSON response from docs.rs crate status endpoint.
///
/// Example JSON: `{"doc_status": true, "version": "0.6.23"}`
#[derive(Clone, Data, Default, Deserialize, New)]
pub(crate) struct DocsStatus {
    /// Whether the documentation build succeeded.
    #[get(type(copy))]
    pub(crate) doc_status: bool,
    /// The latest version string of the crate.
    pub(crate) version: String,
}
