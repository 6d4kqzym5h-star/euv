use crate::*;

/// Scheduling flag to batch signal updates within a single tick.
///
/// Set to `true` when `schedule_signal_update()` queues a microtask,
/// and reset to `false` when the dispatch callback fires.
pub(crate) static SCHEDULED: AtomicBool = AtomicBool::new(false);

/// Suppress flag to prevent `schedule_signal_update()` from dispatching
/// during internal operations such as `watch!` initialisation.
pub(crate) static SUPPRESS_SCHEDULE: AtomicBool = AtomicBool::new(false);

/// The currently active `HookContext`.
///
/// SAFETY: Must only be accessed from the main thread (WASM single-threaded context).
pub(crate) static mut CURRENT_HOOK_CONTEXT: CurrentHookContextCell =
    CurrentHookContextCell(UnsafeCell::new(None));
