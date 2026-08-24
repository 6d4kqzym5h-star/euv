use super::*;

use lombok_macros::{Data, New};
use std::collections::{HashMap, HashSet};

/// The aggregate form state.
///
/// Constructed once per `<form>` via `App::use_form()`,
/// threaded through the form's inputs and the submit
/// handler. Cheap to `Clone` (the four internal signals are
/// each `Copy`-by-pointer).
///
/// # Field reactivity
///
/// `values`, `errors`, `touched`, and `submitting` are all
/// `Signal`s, so any `html!` body that calls
/// `state.values().get()` (or any of the other three) inside
/// its render closure re-renders when the corresponding
/// signal changes. There is no manual subscribe API; the
/// reactive read IS the subscription.
#[derive(Clone, Data, New)]
pub struct FormState {
    /// Per-field current value, keyed by field name.
    pub(crate) values: Signal<HashMap<&'static str, String>>,
    /// Per-field validation error, keyed by field name.
    /// Empty string means "no error".
    pub(crate) errors: Signal<HashMap<&'static str, String>>,
    /// Per-field "user has interacted with this field" flag.
    pub(crate) touched: Signal<HashSet<&'static str>>,
    /// True while a `submit` handler is running. Render
    /// `<button disabled: state.submitting().get()>` to
    /// prevent double-submit.
    pub(crate) submitting: Signal<bool>,
}
