use super::*;

/// Extension trait that exposes the hook-context factory
/// used by `App::use_transition`.
pub trait HookContextTransitionExt {
    /// Returns the `TransitionState` for the current hook
    /// context slot, configured with `config`.
    fn transition(config: TransitionConfig) -> TransitionState;
}
