//! `App::use_transition` and the matching
//! `HookContext::transition` factory. Same pattern as the
//! profiler / form / i18n factories.

use super::*;

/// Extension trait that exposes the hook-context factory
/// used by `App::use_transition`.
pub trait HookContextTransitionExt {
    /// Returns the `TransitionState` for the current hook
    /// context slot, configured with `config`.
    fn transition(config: TransitionConfig) -> TransitionState;
}

impl HookContextTransitionExt for HookContext {
    fn transition(config: TransitionConfig) -> TransitionState {
        let hook_context: HookContext = Self::current();
        let Ok(mut inner) = hook_context.get_inner().try_borrow_mut() else {
            return TransitionState::new(
                Signal::create(TransitionPhase::Exited),
                Signal::create(0.0_f64),
                Signal::create(config),
            );
        };
        let index: usize = inner.get_hook_index();
        inner.set_hook_index(index + 1);
        if index < inner.get_hooks().len()
            && let Some(existing) = inner.get_hooks()[index].downcast_ref::<TransitionState>()
        {
            // Slot already has a state — refresh its
            // config but leave the existing phase /
            // progress intact. (Callers that want to
            // reset explicitly should call
            // `TransitionState::reset`.)
            existing.change_config(config);
            return existing.clone();
        }
        let state: TransitionState = TransitionState::new(
            Signal::create(TransitionPhase::Exited),
            Signal::create(0.0_f64),
            Signal::create(config),
        );
        if index < inner.get_hooks().len() {
            inner.get_mut_hooks()[index] = Box::new(state.clone());
        } else {
            inner.get_mut_hooks().push(Box::new(state.clone()));
        }
        state
    }
}
