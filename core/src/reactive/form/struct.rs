use crate::Signal;
use lombok_macros::{Data, New};
use std::collections::{HashMap, HashSet};
/// A single validator closure, used by
/// [`FormState::validate`].
///
/// The closure receives the current value of the field and
/// returns `Some(error_message)` if the value is invalid, or
/// `None` if it is valid. Returning `Some("")` and returning
/// `None` are both treated as "no error" downstream — but
/// using `Some("")` is discouraged because it round-trips
/// through `is_empty()` checks in the UI layer.
///
/// `Validator` is intentionally type-erased (`Box<dyn Fn>`)
/// rather than a generic, because the validators map is
/// typically built up once at component mount and stored in
/// a `HashMap<&'static str, Validator>` — generics over a
/// map would either force a single validator type or balloon
/// the API.
pub type Validator = Box<dyn Fn(&str) -> Option<String>>;

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
    values: Signal<HashMap<&'static str, String>>,
    /// Per-field validation error, keyed by field name.
    /// Empty string means "no error".
    errors: Signal<HashMap<&'static str, String>>,
    /// Per-field "user has interacted with this field" flag.
    touched: Signal<HashSet<&'static str>>,
    /// True while a `submit` handler is running. Render
    /// `<button disabled: state.submitting().get()>` to
    /// prevent double-submit.
    submitting: Signal<bool>,
}

impl FormState {
    /// Returns a `Signal` clone of the `values` map.
    ///
    /// Read with `.get()` inside a render closure to
    /// subscribe to value changes. Callers needing a
    /// single field's value should use
    /// `FormState::field(name)` instead, which is a
    /// convenience that avoids re-reading the whole map.
    pub fn values(&self) -> Signal<HashMap<&'static str, String>> {
        self.values
    }

    /// Returns a `Signal` clone of the `errors` map.
    pub fn errors(&self) -> Signal<HashMap<&'static str, String>> {
        self.errors
    }

    /// Returns a `Signal` clone of the `touched` set.
    pub fn touched(&self) -> Signal<HashSet<&'static str>> {
        self.touched
    }

    /// Returns a `Signal<bool>` clone of the `submitting`
    /// flag.
    pub fn submitting(&self) -> Signal<bool> {
        self.submitting
    }

    /// Returns the current value of the named field, or
    /// `""` if the field has never been set.
    ///
    /// This is a snapshot read, not a subscription —
    /// callers inside a render closure that want to
    /// re-render on value changes should use
    /// `state.values().get().get(name).cloned().unwrap_or_default()`
    /// instead, so the closure actually subscribes.
    pub fn field(&self, name: &'static str) -> String {
        self.values.get().get(name).cloned().unwrap_or_default()
    }

    /// Returns the current error for the named field, or
    /// `""` if the field has no error.
    ///
    /// Snapshot read — see `field` for the subscription
    /// caveat.
    pub fn error(&self, name: &'static str) -> String {
        self.errors.get().get(name).cloned().unwrap_or_default()
    }

    /// Returns `true` if the user has interacted with the
    /// named field.
    pub fn is_touched(&self, name: &'static str) -> bool {
        self.touched.get().contains(name)
    }

    /// Sets the value of the named field.
    ///
    /// Marks the field as touched (mirroring the
    /// `oninput` event that triggered the call) and
    /// clears any prior error for the field. The error
    /// clear is a UX choice — the next `validate` call
    /// will repopulate it if the new value is still
    /// invalid.
    pub fn set_field(&self, name: &'static str, value: &str) {
        let mut current: HashMap<&'static str, String> = self.values.get();
        current.insert(name, value.to_string());
        self.values.set(current);

        let mut touched: HashSet<&'static str> = self.touched.get();
        touched.insert(name);
        self.touched.set(touched);

        let mut errors: HashMap<&'static str, String> = self.errors.get();
        errors.remove(name);
        self.errors.set(errors);
    }

    /// Marks the named field as touched without changing
    /// its value. Used by `onblur` handlers — "the user
    /// left this field, so it counts as interacted".
    pub fn touch(&self, name: &'static str) {
        let mut touched: HashSet<&'static str> = self.touched.get();
        touched.insert(name);
        self.touched.set(touched);
    }

    /// Runs every validator in `validators` and updates the
    /// `errors` signal.
    ///
    /// Returns `true` if every field validated
    /// successfully (i.e. every validator returned
    /// `None`), `false` otherwise. The errors signal is
    /// always updated, regardless of return value —
    /// callers should call `validate` and then branch on
    /// the boolean.
    ///
    /// Fields with no validator are silently skipped —
    /// they cannot produce an error.
    ///
    /// # Arguments
    ///
    /// - `&HashMap<&'static str, Validator>` -
    ///   Per-field validator map. Each validator is a
    ///   closure that takes the current value and
    ///   returns `Some(error_message)` or `None`.
    pub fn validate(&self, validators: &HashMap<&'static str, Validator>) -> bool {
        let values: HashMap<&'static str, String> = self.values.get();
        let mut next_errors: HashMap<&'static str, String> = HashMap::new();
        let mut all_valid: bool = true;
        for (name, validator) in validators.iter() {
            let current_value: &str = values.get(name).map(String::as_str).unwrap_or("");
            if let Some(error_message) = validator(current_value) {
                if !error_message.is_empty() {
                    all_valid = false;
                }
                next_errors.insert(name, error_message);
            }
        }
        self.errors.set(next_errors);
        all_valid
    }

    /// Runs the user-supplied submit handler if all
    /// validators pass.
    ///
    /// Sets `submitting` to `true` for the duration of the
    /// call (so a `disabled={state.submitting().get()}`
    /// button stays disabled until the handler returns),
    /// then resets it to `false`. If validators were
    /// supplied AND at least one field failed validation,
    /// the submit handler is NOT invoked and `submitting`
    /// is left `false`.
    ///
    /// Returns `true` if the handler was invoked,
    /// `false` if validation failed and the handler was
    /// skipped.
    ///
    /// # Arguments
    ///
    /// - `&HashMap<&'static str, Validator>` -
    ///   Validators to run before invoking the handler.
    ///   Pass an empty map to skip validation entirely
    ///   (the handler always runs).
    /// - `impl FnOnce(&HashMap<&'static str, String>)` -
    ///   The submit handler. Receives the current values
    ///   map by reference — clone what you need to keep
    ///   past the call.
    pub fn submit<F>(&self, validators: &HashMap<&'static str, Validator>, on_submit: F) -> bool
    where
        F: FnOnce(&HashMap<&'static str, String>),
    {
        let all_valid: bool = if validators.is_empty() {
            true
        } else {
            self.validate(validators)
        };
        if !all_valid {
            return false;
        }
        self.submitting.set(true);
        let snapshot: HashMap<&'static str, String> = self.values.get();
        on_submit(&snapshot);
        self.submitting.set(false);
        true
    }

    /// Clears values, errors, and touched state. Leaves
    /// `submitting` untouched (it should already be
    /// `false`).
    ///
    /// Useful for "form submitted successfully, reset for
    /// the next entry" UX flows.
    pub fn reset(&self) {
        self.values.set(HashMap::new());
        self.errors.set(HashMap::new());
        self.touched.set(HashSet::new());
    }

    /// Returns the number of fields that currently have
    /// a non-empty error. Useful for "submit button stays
    /// disabled until form is valid" without re-running
    /// validation.
    pub fn error_count(&self) -> usize {
        self.errors
            .get()
            .values()
            .filter(|message: &&String| !message.is_empty())
            .count()
    }
}
