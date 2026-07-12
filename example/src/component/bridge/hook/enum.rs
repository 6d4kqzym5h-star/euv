use crate::*;

/// `CacheUpdateStatus` on the native side.
///
/// Wire-only enum: lives outside `UpdateResult` because no UI signal reads
/// it — `try_notify_native_once` materializes it from the native payload
/// via `serde_wasm_bindgen::from_value` and feeds it to the retry loop,
/// which then surfaces the result through `UpdateResult::updating`. Keeping
/// the enum separate from the UI DTO avoids spurious dead-field noise and
/// keeps each type's responsibility narrow.
///
/// The `Deserialize` impl maps the lowercase wire tags
/// (`"success"` / `"failed"`) emitted by `CacheUpdateStatus::as_tag()` on
/// the native side — `serde`'s default derive would expect Rust-style
/// variant names (`"Success"` / `"Failed"`) instead.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub(crate) enum UpdateStatus {
    /// The cache snapshot was successfully staged.
    #[serde(rename = "success")]
    Success,
    /// The cache update did not produce a usable snapshot.
    #[serde(rename = "failed")]
    Failed,
}
