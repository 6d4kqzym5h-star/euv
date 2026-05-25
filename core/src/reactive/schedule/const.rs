/// The property name used to store the dispatch callback on the `window` object.
///
/// This key is used by `ensure_dispatch_callback` to register a `Closure`
/// on the global `window` so it can be invoked via `queueMicrotask`.
pub(crate) const EUV_DISPATCH: &str = "__euv_dispatch";

/// The name of the browser API used to schedule a microtask.
///
/// This is the standard `queueMicrotask` function available on the `window`
/// object, used to defer the dispatch callback until the next microtask checkpoint.
pub(crate) const QUEUE_MICROTASK: &str = "queueMicrotask";
