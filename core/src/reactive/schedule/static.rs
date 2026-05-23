use crate::*;

/// Scheduling flag to batch signal updates within a single tick.
pub(crate) static SCHEDULED: AtomicBool = AtomicBool::new(false);

/// Suppress flag to prevent `schedule_signal_update()` from dispatching
/// during internal operations such as `watch!` initialisation.
pub(crate) static SUPPRESS_SCHEDULE: AtomicBool = AtomicBool::new(false);

/// The currently active `HookContext`.
pub(crate) static mut CURRENT_HOOK_CONTEXT: CurrentHookContextCell =
    CurrentHookContextCell(UnsafeCell::new(None));
