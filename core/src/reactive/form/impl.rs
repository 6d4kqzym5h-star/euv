//! `App::use_form` and the matching `HookContext::form`
//! factory. Identical pattern to the profiler factory —
//! see `reactive/profiler/handle.rs` for prior art and the
//! reasoning behind the downcast-ref + index-increment
//! implementation.

use super::*;
use crate::{HookContext, Signal};
use std::collections::{HashMap, HashSet};

/// Extension trait that exposes the hook-context factory
/// used by `App::use_form`.
///
/// Splitting the factory into a trait (rather than a bare
/// `impl HookContext`) keeps the public surface compact
/// and lets future hooks slot into the same trait.
pub trait HookContextFormExt {
    /// Returns the `FormState` for the current hook
    /// context slot.
    fn form() -> FormState;
}

impl HookContextFormExt for HookContext {
    fn form() -> FormState {
        let hook_context: HookContext = Self::current();
        let Ok(mut inner) = hook_context.get_inner().try_borrow_mut() else {
            return FormState::new(
                Signal::create(HashMap::new()),
                Signal::create(HashMap::new()),
                Signal::create(HashSet::new()),
                Signal::create(false),
            );
        };
        let index: usize = inner.get_hook_index();
        inner.set_hook_index(index + 1);
        if index < inner.get_hooks().len()
            && let Some(existing) = inner.get_hooks()[index].downcast_ref::<FormState>()
        {
            return existing.clone();
        }
        let state: FormState = FormState::new(
            Signal::create(HashMap::new()),
            Signal::create(HashMap::new()),
            Signal::create(HashSet::new()),
            Signal::create(false),
        );
        if index < inner.get_hooks().len() {
            inner.get_mut_hooks()[index] = Box::new(state.clone());
        } else {
            inner.get_mut_hooks().push(Box::new(state.clone()));
        }
        state
    }
}
