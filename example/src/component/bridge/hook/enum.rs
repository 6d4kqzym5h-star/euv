/// `CacheUpdateStatus` on the native side.
///
/// Wire-only enum: lives outside `UpdateResult` because no UI signal reads
/// it — `try_notify_native_once` materializes it from the native payload
/// via `serde_wasm_bindgen::from_value` and feeds it to the retry loop,
/// which then surfaces the result through `UpdateResult::updating`. Keeping
/// the enum separate from the UI DTO avoids spurious dead-field noise and
/// keeps each type's responsibility narrow.
///
/// Deserialization is hand-written in `impl.rs` so the wire-tag mapping is
/// expressed as Rust control flow against the canonical constants
/// `UPDATE_RESULT_SUCCESS` / `UPDATE_RESULT_FAILED` from `const.rs` — the
/// constants are the single source of truth, and the parse function is the
/// only place that consults them. `Display` reads through those same
/// constants so `format_payload` can embed the wire tag into log lines
/// without re-hardcoding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UpdateStatus {
    /// The cache snapshot was successfully staged. Wire tag constant: `UPDATE_RESULT_SUCCESS`.
    Success,
    /// The cache update did not produce a usable snapshot. Wire tag constant: `UPDATE_RESULT_FAILED`.
    Failed,
}
