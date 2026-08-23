//! `App::use_i18n` and the matching `HookContext::i18n`
//! factory. Same pattern as the profiler / form factories —
//! see `reactive/profiler/handle.rs` for prior art.

use super::*;
use crate::{HookContext, Signal};
use std::collections::HashMap;

/// Extension trait that exposes the hook-context factory
/// used by `App::use_i18n`.
pub trait HookContextI18nExt {
    /// Returns the `I18n` for the current hook context
    /// slot.
    fn i18n() -> I18n;
}

impl HookContextI18nExt for HookContext {
    fn i18n() -> I18n {
        let hook_context: HookContext = Self::current();
        let Ok(mut inner) = hook_context.get_inner().try_borrow_mut() else {
            // No active hook context — return an
            // orphaned handle. Same pattern as
            // `HookContext::profiler` / `HookContext::form`.
            return I18n::new(
                Signal::create(String::from("en")),
                Signal::create(String::from("en")),
                Signal::create(HashMap::new()),
            );
        };
        let index: usize = inner.get_hook_index();
        inner.set_hook_index(index + 1);
        if index < inner.get_hooks().len()
            && let Some(existing) = inner.get_hooks()[index].downcast_ref::<I18n>()
        {
            return existing.clone();
        }
        let state: I18n = I18n::new(
            Signal::create(String::from("en")),
            Signal::create(String::from("en")),
            Signal::create(HashMap::new()),
        );
        if index < inner.get_hooks().len() {
            inner.get_mut_hooks()[index] = Box::new(state.clone());
        } else {
            inner.get_mut_hooks().push(Box::new(state.clone()));
        }
        state
    }
}
