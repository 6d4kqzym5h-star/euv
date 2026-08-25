use super::*;

/// The four phases of a transition lifecycle.
///
/// Lifecycle: `Exited` → `Entering` → `Entered` →
/// `Exiting` → `Exited` (looping back). The state
/// machine is symmetric on the way in and out — the
/// `Entering` and `Exiting` phases both have a `progress`
/// value in `(0.0, 1.0)`, while `Entered` and `Exited`
/// are terminal resting states (`progress` pinned to
/// `1.0` and `0.0` respectively).
///
/// Phases are public and `Clone + Copy + PartialEq` so
/// consumers can write `match phase { ... }` and dispatch
/// on them without going through the signal indirection
/// every time.
///
/// Note: not deriving `Data` / `New` from lombok because
/// both `Data` and `New` derive macros are only
/// supported for structs (see `UNSUPPORTED_DATA_DERIVE`
/// in lombok-macros 2.0.36). The standard library
/// derives below are sufficient — we don't need
/// getters/setters on the variants.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TransitionPhase {
    /// The element is not yet on-screen (initial state).
    /// `progress` is `0.0`.
    Exited,
    /// The element is animating in. `progress` rises from
    /// `0.0` toward `1.0` as `tick(elapsed_ms)` is called.
    Entering,
    /// The element is fully on-screen. `progress` is
    /// `1.0`.
    Entered,
    /// The element is animating out. `progress` falls from
    /// `1.0` toward `0.0` as `tick(elapsed_ms)` is called.
    Exiting,
}
