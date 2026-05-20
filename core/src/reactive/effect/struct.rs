use crate::*;

/// Inner state of a RenderEffect, holding the effect closure and dependency tracking.
///
/// Each RenderEffect tracks which signals it depends on, and when any of those
/// signals change, the effect is re-executed automatically. Before each
/// re-execution, old dependencies are cleaned up to prevent listener accumulation.
#[derive(CustomDebug, Data)]
pub(crate) struct RenderEffectInner {
    /// The effect closure to re-execute when dependencies change.
    #[debug(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) effect_fn: Box<dyn FnMut()>,
    /// Addresses of signals this effect currently depends on.
    ///
    /// Populated during `run_once` by `track_signal`. Before each
    /// `run_once`, these dependencies are cleaned up (the effect's
    /// subscriber entry is removed from each signal's registry entry).
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dependencies: Vec<usize>,
    /// Re-entrancy guard. Set to `true` while the effect closure is
    /// executing. If `run_once` is called recursively while this is
    /// `true`, the call is skipped to prevent infinite loops caused
    /// by signals being set inside the effect closure.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) running: bool,
    /// Whether this effect has been disposed and should no longer execute.
    ///
    /// When a `RenderEffect` is disposed, `run_once` becomes a no-op and
    /// `cleanup_effect_dependencies` is called to remove all subscriber
    /// entries from the global registry. This prevents stale effects from
    /// being scheduled or executed after their associated DOM nodes have
    /// been removed (e.g., during match arm switches in routing).
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) disposed: bool,
}

/// A reactive effect that automatically tracks signal dependencies and
/// re-executes when any dependency changes.
///
/// Unlike the global `__euv_signal_update__` mechanism which broadcasts to
/// all DynamicNodes, a RenderEffect only re-runs when signals it actually
/// reads have changed, enabling fine-grained reactive updates.
///
/// SAFETY: The inner pointer is allocated via `Box::leak` and lives for the
/// entire program. This is safe in single-threaded WASM contexts.
#[derive(Clone, Copy, Data, Debug, Default, Eq, PartialEq)]
pub struct RenderEffect {
    /// Address of the heap-allocated inner state.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) inner: usize,
}
