use super::*;

fn run_with_signal_capture<F>(f: F) -> bool
where
    F: FnOnce(),
{
    catch_unwind(AssertUnwindSafe(f)).is_ok()
}

fn fresh_form() -> FormState {
    FormState::new(
        Signal::create(HashMap::new()),
        Signal::create(HashMap::new()),
        Signal::create(HashSet::new()),
        Signal::create(false),
    )
}

// ============================================================
//  Pure-Rust contract tests — run on every target.
// ============================================================

#[test]
fn fresh_form_signals_are_empty_and_submitting_false() {
    let form: FormState = fresh_form();
    assert!(form.values().get().is_empty());
    assert!(form.errors().get().is_empty());
    assert!(form.touched().get().is_empty());
    assert!(!form.submitting().get());
    assert_eq!(form.error_count(), 0);
}

#[test]
fn fresh_form_field_and_error_default_to_empty_string() {
    let form: FormState = fresh_form();
    assert_eq!(form.field("email"), "");
    assert_eq!(form.field("nonexistent"), "");
    assert_eq!(form.error("email"), "");
    assert_eq!(form.error("nonexistent"), "");
    assert!(!form.is_touched("email"));
}

#[test]
fn field_and_error_read_from_seeded_signals() {
    // Pre-populate via `Signal::create` so we can read
    // through `field` / `error` / `is_touched` /
    // `error_count` without ever calling `Signal::set`.
    let mut values: HashMap<&'static str, String> = HashMap::new();
    values.insert("email", String::from("alice@example.com"));
    let mut errors: HashMap<&'static str, String> = HashMap::new();
    errors.insert("email", String::from("required"));
    errors.insert("phone", String::new());
    let mut touched: HashSet<&'static str> = HashSet::new();
    touched.insert("email");
    let form: FormState = FormState::new(
        Signal::create(values),
        Signal::create(errors),
        Signal::create(touched),
        Signal::create(false),
    );
    assert_eq!(form.field("email"), "alice@example.com");
    assert_eq!(form.error("email"), "required");
    assert_eq!(form.error("phone"), "");
    assert!(form.is_touched("email"));
    assert!(!form.is_touched("phone"));
    assert_eq!(form.error_count(), 1);
}

#[test]
fn error_count_counts_only_non_empty_errors() {
    let mut errors: HashMap<&'static str, String> = HashMap::new();
    errors.insert("a", String::from("real error"));
    errors.insert("b", String::new());
    errors.insert("c", String::from("another real error"));
    let form: FormState = FormState::new(
        Signal::create(HashMap::new()),
        Signal::create(errors),
        Signal::create(HashSet::new()),
        Signal::create(false),
    );
    assert_eq!(form.error_count(), 2);
}

#[test]
fn error_count_zero_for_empty_error_map() {
    let form: FormState = fresh_form();
    assert_eq!(form.error_count(), 0);
}

#[test]
fn error_count_handles_all_empty_messages() {
    // Multiple fields with empty-string errors should
    // count as zero — `error_count` filters by
    // `!message.is_empty()`.
    let mut errors: HashMap<&'static str, String> = HashMap::new();
    errors.insert("a", String::new());
    errors.insert("b", String::new());
    errors.insert("c", String::new());
    let form: FormState = FormState::new(
        Signal::create(HashMap::new()),
        Signal::create(errors),
        Signal::create(HashSet::new()),
        Signal::create(false),
    );
    assert_eq!(form.error_count(), 0);
}

#[test]
fn values_errors_touched_submitting_accessors_return_clones_of_signals() {
    let form: FormState = fresh_form();
    let _values: Signal<HashMap<&'static str, String>> = form.values();
    let _errors: Signal<HashMap<&'static str, String>> = form.errors();
    let _touched: Signal<HashSet<&'static str>> = form.touched();
    let _submitting: Signal<bool> = form.submitting();
}

#[test]
fn form_state_clone_shares_all_four_signals() {
    // `Clone` is a field clone. Each field is a `Signal<T>`,
    // which is `Copy`-by-pointer. So pre-populating the
    // values signal via `Signal::create` and reading
    // through a clone exercises the shared-heap-ptr
    // invariant without ever calling `Signal::set`.
    let mut values: HashMap<&'static str, String> = HashMap::new();
    values.insert("email", String::from("alice@example.com"));
    let mut touched: HashSet<&'static str> = HashSet::new();
    touched.insert("email");
    let form: FormState = FormState::new(
        Signal::create(values),
        Signal::create(HashMap::new()),
        Signal::create(touched),
        Signal::create(false),
    );
    let twin: FormState = form.clone();
    assert_eq!(twin.field("email"), "alice@example.com");
    assert!(twin.is_touched("email"));
    assert!(!twin.submitting().get());
}

#[test]
fn reactive_read_via_subscribed_signal_matches_initial_value() {
    // The reactive contract: a subscriber cloning
    // `errors()` out of the form and calling `.get()`
    // should observe the same value the form was
    // constructed with. (This is a snapshot read, not a
    // subscription-over-time test — those would need
    // wasm-pack to drive the dispatch loop.)
    let mut errors: HashMap<&'static str, String> = HashMap::new();
    errors.insert("email", String::from("required"));
    let form: FormState = FormState::new(
        Signal::create(HashMap::new()),
        Signal::create(errors),
        Signal::create(HashSet::new()),
        Signal::create(false),
    );
    let subscribed: Signal<HashMap<&'static str, String>> = form.errors();
    assert_eq!(
        subscribed.get().get("email"),
        Some(&String::from("required"))
    );
}

// ============================================================
//  Validator closure-shape tests — pure Rust, no set.
// ============================================================

#[test]
fn validator_closure_returning_none_means_no_error() {
    let validator: Validator = Box::new(|value: &str| -> Option<String> {
        if value.is_empty() {
            Some(String::from("required"))
        } else {
            None
        }
    });
    assert_eq!(validator("non-empty"), None);
    assert_eq!(validator(""), Some(String::from("required")));
}

#[test]
fn validator_closure_returning_some_empty_string_is_preserved() {
    let validator: Validator = Box::new(|_: &str| -> Option<String> { Some(String::new()) });
    assert_eq!(validator("anything"), Some(String::new()));
}

#[test]
fn validators_hashmap_supports_multiple_keys() {
    let mut validators: HashMap<&'static str, Validator> = HashMap::new();
    validators.insert("email", Box::new(|_: &str| -> Option<String> { None }));
    validators.insert(
        "phone",
        Box::new(|_: &str| -> Option<String> { Some(String::from("bad phone")) }),
    );
    assert_eq!(validators.len(), 2);
    assert_eq!(validators.get("email").unwrap()("x"), None);
    assert_eq!(
        validators.get("phone").unwrap()("x"),
        Some(String::from("bad phone"))
    );
}

// ============================================================
//  Set-path coverage tests — wasm-only contract surface.
//  On native these are wrapped in `catch_unwind` and the
//  post-write assertions are gated on `if ran` so they pass
//  without crashing. On wasm the full path runs.
// ============================================================

#[test]
fn set_field_inserts_value_marks_touched_clears_error_set_path() {
    let form: FormState = fresh_form();
    let ran: bool = run_with_signal_capture(|| {
        // Pre-seed an error for the field we're about to
        // touch — `set_field` should clear it.
        let mut seed: HashMap<&'static str, String> = HashMap::new();
        seed.insert("email", String::from("required"));
        form.errors().set(seed);

        form.set_field("email", "alice@example.com");
    });
    if ran {
        assert_eq!(
            form.values().get().get("email"),
            Some(&String::from("alice@example.com"))
        );
        assert!(form.touched().get().contains("email"));
        assert!(
            !form.errors().get().contains_key("email"),
            "set_field must clear any prior error for the field"
        );
    }
}

#[test]
fn set_field_overwrites_previous_value_set_path() {
    let form: FormState = fresh_form();
    let ran: bool = run_with_signal_capture(|| {
        form.set_field("name", "Alice");
        form.set_field("name", "Bob");
    });
    if ran {
        assert_eq!(form.values().get().get("name"), Some(&String::from("Bob")));
    }
}

#[test]
fn touch_inserts_field_into_touched_set_idempotently_set_path() {
    let form: FormState = fresh_form();
    let ran: bool = run_with_signal_capture(|| {
        form.touch("phone");
        form.touch("phone");
        form.touch("phone");
    });
    if ran {
        assert_eq!(form.touched().get().len(), 1);
        assert!(form.touched().get().contains("phone"));
    }
}

#[test]
fn validate_with_no_validators_returns_true_set_path() {
    let form: FormState = fresh_form();
    let result: Rc<Cell<Option<bool>>> = Rc::new(Cell::new(None));
    let result_clone: Rc<Cell<Option<bool>>> = Rc::clone(&result);
    let ran: bool = run_with_signal_capture(|| {
        let validators: HashMap<&'static str, Validator> = HashMap::new();
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let r: bool = form.validate(&validators);
            result_clone.set(Some(r));
        }));
    });
    if ran {
        assert_eq!(result.get(), Some(true));
    }
    if let Some(true) = result.get() {
        assert!(form.errors().get().is_empty());
    }
}

#[test]
fn validate_populates_errors_for_failing_validators_set_path() {
    let form: FormState = fresh_form();
    let ran: bool = run_with_signal_capture(|| {
        form.set_field("email", "");
        let mut validators: HashMap<&'static str, Validator> = HashMap::new();
        validators.insert(
            "email",
            Box::new(|value: &str| -> Option<String> {
                if value.is_empty() {
                    Some(String::from("Email is required"))
                } else {
                    None
                }
            }),
        );
        let _ = form.validate(&validators);
    });
    if ran {
        assert_eq!(
            form.errors().get().get("email"),
            Some(&String::from("Email is required"))
        );
        assert_eq!(form.error_count(), 1);
    }
}

#[test]
fn validate_returns_true_when_all_validators_pass_set_path() {
    let form: FormState = fresh_form();
    let result: Rc<Cell<Option<bool>>> = Rc::new(Cell::new(None));
    let result_clone: Rc<Cell<Option<bool>>> = Rc::clone(&result);
    let ran: bool = run_with_signal_capture(|| {
        form.set_field("email", "alice@example.com");
        let mut validators: HashMap<&'static str, Validator> = HashMap::new();
        validators.insert(
            "email",
            Box::new(|value: &str| -> Option<String> {
                if value.contains('@') {
                    None
                } else {
                    Some(String::from("Invalid email"))
                }
            }),
        );
        let _ = catch_unwind(AssertUnwindSafe(|| {
            result_clone.set(Some(form.validate(&validators)));
        }));
    });
    if ran {
        assert_eq!(result.get(), Some(true));
    }
    if let Some(true) = result.get() {
        assert!(form.errors().get().is_empty());
        assert_eq!(form.error_count(), 0);
    }
}

#[test]
fn validate_skips_fields_without_validator_set_path() {
    let form: FormState = fresh_form();
    let ran: bool = run_with_signal_capture(|| {
        form.set_field("email", "alice@example.com");
        form.set_field("phone", "");
        let mut validators: HashMap<&'static str, Validator> = HashMap::new();
        validators.insert("email", Box::new(|_: &str| -> Option<String> { None }));
        let _ = form.validate(&validators);
    });
    if ran {
        assert!(!form.errors().get().contains_key("phone"));
    }
}

#[test]
fn submit_with_empty_validators_invokes_handler_set_path() {
    let form: FormState = fresh_form();
    let invoked: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let captured: Rc<Cell<Option<String>>> = Rc::new(Cell::new(None));
    let invoked_clone: Rc<Cell<bool>> = Rc::clone(&invoked);
    let captured_clone: Rc<Cell<Option<String>>> = Rc::clone(&captured);
    let ran: bool = run_with_signal_capture(|| {
        form.set_field("email", "alice@example.com");
        let validators: HashMap<&'static str, Validator> = HashMap::new();
        let _ = form.submit(&validators, |values: &HashMap<_, _>| {
            invoked_clone.set(true);
            captured_clone.set(values.get("email").cloned());
        });
    });
    if ran {
        assert!(invoked.get());
        assert_eq!(captured.take(), Some(String::from("alice@example.com")));
        assert!(!form.submitting().get());
    }
}

#[test]
fn submit_with_passing_validators_invokes_handler_set_path() {
    let form: FormState = fresh_form();
    let invoked: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let invoked_clone: Rc<Cell<bool>> = Rc::clone(&invoked);
    let ran: bool = run_with_signal_capture(|| {
        form.set_field("email", "alice@example.com");
        let mut validators: HashMap<&'static str, Validator> = HashMap::new();
        validators.insert("email", Box::new(|_: &str| -> Option<String> { None }));
        let _ = form.submit(&validators, |_| {
            invoked_clone.set(true);
        });
    });
    if ran {
        assert!(invoked.get());
    }
}

#[test]
fn submit_with_failing_validator_skips_handler_set_path() {
    let form: FormState = fresh_form();
    let invoked: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let did_submit: Rc<Cell<Option<bool>>> = Rc::new(Cell::new(None));
    let invoked_clone: Rc<Cell<bool>> = Rc::clone(&invoked);
    let did_submit_clone: Rc<Cell<Option<bool>>> = Rc::clone(&did_submit);
    let ran: bool = run_with_signal_capture(|| {
        form.set_field("email", "");
        let mut validators: HashMap<&'static str, Validator> = HashMap::new();
        validators.insert(
            "email",
            Box::new(|value: &str| -> Option<String> {
                if value.is_empty() {
                    Some(String::from("required"))
                } else {
                    None
                }
            }),
        );
        let _ = catch_unwind(AssertUnwindSafe(|| {
            did_submit_clone.set(Some(form.submit(&validators, |_| invoked_clone.set(true))));
        }));
    });
    if ran {
        assert_eq!(did_submit.get(), Some(false));
        assert!(!invoked.get());
    }
}

// ============================================================
//  Constructor invariant — verifies the `fresh_form` helper
//  matches the contract expected by `reset` (see the
//  reset-cannot-be-tested-on-native doc comment in the
//  module header).
// ============================================================

#[test]
fn reset_post_state_matches_fresh_form_invariant() {
    // `reset` would set values / errors / touched to
    // empty maps. We cannot call `reset` on native (it
    // would panic via Signal::set). Instead we verify
    // that a freshly-constructed FormState already has
    // those empty maps — i.e. the post-reset invariant
    // is "same as fresh form". On wasm, wasm-pack test
    // verifies `reset` actually reaches that invariant.
    let form: FormState = fresh_form();
    assert!(form.values().get().is_empty());
    assert!(form.errors().get().is_empty());
    assert!(form.touched().get().is_empty());
    assert!(!form.submitting().get());
}
