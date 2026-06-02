/// The name of the browser API used to schedule a microtask.
///
/// This is the standard `queueMicrotask` function available on the `window`
/// object, used to defer the dispatch callback until the next microtask
/// checkpoint when `setTimeout` is unavailable.
pub(crate) const QUEUE_MICROTASK: &str = "queueMicrotask";
