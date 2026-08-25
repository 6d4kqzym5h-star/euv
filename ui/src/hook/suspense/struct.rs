use super::*;

/// A handle that drives a `SuspenseState`.
///
/// Created via `SuspenseHandle::new()`. The handle owns
/// the underlying `Signal<SuspensePhase<T>>` and exposes
/// `state()` to read it. The handle itself is the only
/// thing that can mutate the phase, via
/// `resolve_sync` (every target) or `resolve_async`
/// (wasm-only).
#[derive(Clone, Data, Debug)]
pub struct SuspenseHandle<T: Clone + PartialEq + 'static> {
    /// The phase signal.
    pub(crate) phase: Signal<SuspensePhase<T>>,
}
