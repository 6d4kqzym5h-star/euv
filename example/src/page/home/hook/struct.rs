use crate::*;

/// Reactive state for the native bridge system info feature.
///
/// Aggregates all signals needed for the cached resource and
/// bridge permissions display.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseNativeBridge {
    /// Whether the native bridge is available on this platform.
    #[get(type(copy))]
    pub(crate) available: Signal<bool>,
    /// Whether data is currently being loaded from the native bridge.
    #[get(type(copy))]
    pub(crate) loading: Signal<bool>,
    /// Whether the cached resource was loaded from cache.
    #[get(type(copy))]
    pub(crate) from_cache: Signal<bool>,
    /// The remote URL of the cached resource.
    #[get(type(copy))]
    pub(crate) remote_url: Signal<String>,
    /// The source status of the cached resource ("active" or "none").
    #[get(type(copy))]
    pub(crate) source: Signal<String>,
    /// The local cache path of the cached resource.
    #[get(type(copy))]
    pub(crate) cache_path: Signal<String>,
    /// The resolved bridge group permissions.
    #[get(type(copy))]
    pub(crate) permissions: Signal<String>,
}
