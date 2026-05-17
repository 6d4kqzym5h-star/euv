use crate::*;

/// Scheduling flag to batch signal updates within a single tick.
///
/// When `true`, a `__euv_signal_update__` dispatch is already pending
/// and subsequent updates are coalesced into the same microtask.
pub(crate) static SCHEDULED: AtomicBool = AtomicBool::new(false);

/// Suppress flag to prevent `schedule_signal_update()` from dispatching
/// during internal operations such as `watch!` initialisation.
///
/// When `true`, any call to `schedule_signal_update()` is silently ignored.
/// The caller is responsible for calling `schedule_signal_update()` once
/// after the suppressed block ends, if any signal was actually mutated.
pub(crate) static SUPPRESS_SCHEDULE: AtomicBool = AtomicBool::new(false);

/// Default empty `HookContextInner` for the global fallback.
///
/// Used as the initial value for `CURRENT_HOOK_CONTEXT` when no render
/// cycle is active. Because WASM is single-threaded, concurrent writes
/// are impossible and no memory corruption can occur.
pub static DEFAULT_HOOK_CONTEXT: HookContextCell =
    HookContextCell(UnsafeCell::new(HookContextInner::new()));

/// Global pointer to the currently active `HookContextInner`.
///
/// Set by the renderer before invoking a `DynamicNode`'s render function,
/// and restored to the default context afterwards. Stores a raw pointer
/// to the inner state; callers use `get_current_hook_context` to obtain
/// a `HookContext` handle.
pub static mut CURRENT_HOOK_CONTEXT: *mut HookContextInner = DEFAULT_HOOK_CONTEXT.0.get();
