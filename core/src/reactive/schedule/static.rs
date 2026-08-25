use super::*;

/// Scheduling flag to batch signal updates within a single tick.
///
/// Set to `true` when `schedule_update()` queues a microtask,
/// and reset to `false` when the dispatch callback fires.
pub static SCHEDULED: AtomicBool = AtomicBool::new(false);

/// Suppress flag to prevent `schedule_update()` from dispatching
/// during internal operations such as `batch`.
pub static SUPPRESS_SCHEDULE: AtomicBool = AtomicBool::new(false);

/// The currently active `HookContext`.
///
/// SAFETY: Must only be accessed from the main thread (WASM single-threaded context).
pub(crate) static mut CURRENT_HOOK_CONTEXT: CurrentHookContextCell =
    CurrentHookContextCell(UnsafeCell::new(None));

/// The dynamic node ID currently being rendered/set up.
///
/// When a DynamicNode is being initialized or re-rendered, this is set to
/// the dynamic_id so that any signal `get()` calls during that render
/// can register the dependency. `usize::MAX` means "no tracking active".
///
/// SAFETY: Must only be accessed from the main thread (WASM single-threaded context).
pub static CURRENT_TRACKING_DYNAMIC_ID: AtomicUsize = AtomicUsize::new(usize::MAX);

thread_local! {
    /// The persistent dispatch `Closure`, kept alive for the lifetime of the
    /// program so it can be handed to `setTimeout` repeatedly.
    ///
    /// The closure resets the `SCHEDULED` flag and then runs the queued signal
    /// update callbacks. Resetting `SCHEDULED` here is what allows the next
    /// `schedule_update` call to schedule a fresh dispatch; if
    /// this never ran, the flag would stay `true` forever and every reactive
    /// update would be silently dropped.
    pub static DISPATCH_CLOSURE: Closure<dyn FnMut()> =
        Closure::wrap(Box::new(|| {
            SCHEDULED.store(false, Ordering::Relaxed);
            Scheduler::dispatch_updates();
        }));

    /// OPT 7: thread-local cache for the `Function` reference to
    /// `window.queueMicrotask`, resolved lazily on first use. The
    /// `Function::call1` in `Scheduler::update` skips the
    /// `Reflect::get` / `dyn_into` lookup on every signal update.
    pub static MICROTASK_CACHE: MicrotaskCacheCell =
        MicrotaskCacheCell(UnsafeCell::new(MicrotaskCache {
            queue_microtask: None,
        }));
}
