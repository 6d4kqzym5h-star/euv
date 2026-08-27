use super::*;

/// A cached `Function` wrapper around `window.queueMicrotask`.
///
/// OPT 7: resolving the `queueMicrotask` JS function used to take a
/// `Reflect::get(&window, &JsValue::from_str("queueMicrotask"))` call
/// (one JS round-trip + a string-to-JsValue conversion) and then a
/// `dyn_into::<Function>` call (another conversion) on every signal
/// update. The resolved handle is now looked up once on first use and
/// cached for the page's lifetime. Subsequent scheduling just
/// `Function::call1(window, dispatch_function)` — one JS round-trip
/// per dispatch instead of three.
pub(crate) struct MicrotaskCache {
    /// The `window.queueMicrotask` function, resolved once on first
    /// call and reused across the page's lifetime. `None` if the
    /// browser does not expose `queueMicrotask` (the dispatch path
    /// then falls through to `setTimeout` / `requestAnimationFrame`).
    pub(crate) queue_microtask: Option<Function>,
}

/// `Sync` wrapper around `MicrotaskCache` for `thread_local!` storage.
/// SAFETY: only used on the WASM single-threaded runtime.
#[derive(CustomDebug, Data)]
pub(crate) struct MicrotaskCacheCell(
    /// `UnsafeCell` interior-mutability so `update` can lazily populate
    /// `queue_microtask` once and reuse it on subsequent calls.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    pub UnsafeCell<MicrotaskCache>,
);

/// A `Sync` wrapper for single-threaded global `Option<HookContextRc>` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct CurrentHookContextCell(
    /// Interior-mutable storage for the current hook context.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub UnsafeCell<Option<HookContextRc>>,
);

/// A zero-sized struct providing static methods for scheduling
/// signal update dispatches and batching.
///
/// All methods are crate-internal associated functions that manage
/// the global scheduling flags and dispatch closure.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct Scheduler;
