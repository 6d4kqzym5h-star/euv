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
/// whether a cache update is currently in progress, and the most recent
/// data / message reported by the native `update_cache` bridge. The last
/// two let the UI surface the new snapshot name on success and the native
/// error string on failure without reaching into the retry loop.
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
    /// Most recent `data` payload from the native `update_cache` bridge.
    ///
    /// On `Success` this is the freshly minted snapshot name (e.g.
    /// `v_1731321600123`); on `Failed` it is an empty string.
    #[get(type(copy))]
    pub(crate) data: Signal<String>,
    /// Most recent `message` payload from the native `update_cache` bridge.
    ///
    /// On `Success` this is a fixed "cache updated successfully" string;
    /// on `Failed` it carries the underlying error from the cache layer.
    /// Surfacing it here lets the UI show the native error verbatim
    /// instead of inventing its own message.
    #[get(type(copy))]
    pub(crate) message: Signal<String>,
}

/// The result returned by a cache update closure.
///
/// The UI component uses this value to update its internal state signals
/// via `UseCacheUpdate::load`. Every field is consumed — the docs.rs-side
/// fields come from the webview's own fetch, the `data` / `message`
/// fields come from the native bridge payload (see `UpdateStatus` /
/// `UpdateResultPayload` for the wire details).
#[derive(Clone, Data, New)]
pub(crate) struct UpdateResult {
    /// Whether the documentation build succeeded.
    #[get(type(copy))]
    pub(crate) doc_status: bool,
    /// The latest version string of the crate.
    pub(crate) version: String,
    /// Whether a cache update operation was currently in progress when
    /// this result was produced.
    #[get(type(copy))]
    pub(crate) updating: bool,
    /// Snapshot version name reported by the native bridge (empty on failure).
    pub(crate) data: String,
    /// Human-readable status / error message reported by the native bridge.
    pub(crate) message: String,
}

/// Wire-level mirror of the native `CacheUpdateResult` payload.
///
/// `result` is deserialized as an enum so an unknown tag fails the whole
/// payload rather than silently mis-classifying. `data` carries the new
/// snapshot name on success; `message` carries the native error
/// description on failure or a success sentinel on success. Both fields
/// are propagated into `UpdateResult` for the UI to surface.
///
/// Only `try_notify_native_once` reads from this type (in `view/fn.rs`);
/// it lives here so every payload-facing type lives next to `UpdateResult`
/// / `UpdateStatus` and the struct-vs-fn responsibility split stays clean.
#[derive(Data, Deserialize, New)]
pub(crate) struct UpdateResultPayload {
    /// Outcome tag deserialized as an enum, so an unknown tag fails the
    /// whole payload rather than silently mis-classifying.
    pub(crate) result: UpdateStatus,
    /// Snapshot version name reported by the native side; empty on failure.
    /// Propagated into `UpdateResult::data` for the UI to surface.
    pub(crate) data: String,
    /// Human-readable status / error message from the native side. Propagated
    /// into `UpdateResult::message` so the UI shows the native error verbatim.
    pub(crate) message: String,
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
