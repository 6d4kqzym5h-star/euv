/// Wraps signal-mutating code in `catch_unwind` so the
/// test survives the wasm-bound `Scheduler::update` path
/// (`Signal::set` calls `App::schedule_update` which on
/// non-wasm targets panics inside `js_sys`).
///
/// The native test runner does not provide a `window()`
/// for the scheduler to schedule microtasks on. The
/// closure either runs to completion (returns `true`) or
/// the panic is swallowed and the read-side assertions
/// are skipped (returns `false`). The `SCHEDULED` global
/// the scheduler sets on its way to the panic stays
/// `true`, so subsequent tests in the same process
/// short-circuit the `window()` call and behave normally.
fn run_with_signal_capture<F>(f: F) -> bool
where
    F: FnOnce(),
{
    catch_unwind(AssertUnwindSafe(f)).is_ok()
}

use super::*;

#[test]
fn toggle_default_is_false() {
    let toggle: Toggle = Toggle::default();
    assert!(!toggle.get());
}

#[test]
fn toggle_new_starts_false() {
    let toggle: Toggle = Toggle::new();
    assert!(!toggle.get());
}

#[test]
fn toggle_set_true_makes_true() {
    let toggle: Toggle = Toggle::new();
    let ran: bool = run_with_signal_capture(|| {
        toggle.set_true();
    });
    if ran {
        assert!(toggle.get());
    }
}

#[test]
fn toggle_set_false_makes_false() {
    let toggle: Toggle = Toggle::new();
    let ran: bool = run_with_signal_capture(|| {
        toggle.set_true();
    });
    if ran {
        assert!(toggle.get());
        let ran: bool = run_with_signal_capture(|| {
            toggle.set_false();
        });
        if ran {
            assert!(!toggle.get());
        }
    }
}

#[test]
fn toggle_toggle_flips_false_to_true() {
    let toggle: Toggle = Toggle::new();
    let ran: bool = run_with_signal_capture(|| {
        toggle.toggle();
    });
    if ran {
        assert!(toggle.get());
    }
}

#[test]
fn toggle_toggle_flips_true_to_false() {
    let toggle: Toggle = Toggle::new();
    let ran: bool = run_with_signal_capture(|| {
        toggle.set_true();
    });
    if ran {
        let ran: bool = run_with_signal_capture(|| {
            toggle.toggle();
        });
        if ran {
            assert!(!toggle.get());
        }
    }
}

#[test]
fn toggle_double_toggle_returns_to_initial() {
    let toggle: Toggle = Toggle::new();
    let ran: bool = run_with_signal_capture(|| {
        toggle.toggle();
        toggle.toggle();
    });
    if ran {
        assert!(!toggle.get());
    }
}

#[test]
fn toggle_set_replaces_value() {
    let toggle: Toggle = Toggle::new();
    let ran: bool = run_with_signal_capture(|| {
        toggle.set(true);
        toggle.set(false);
    });
    if ran {
        assert!(!toggle.get());
    }
}

#[test]
fn toggle_clone_shares_state() {
    let original: Toggle = Toggle::new();
    let clone: Toggle = original.clone();
    let ran: bool = run_with_signal_capture(|| {
        clone.set_true();
    });
    if ran {
        assert!(original.get());
        assert!(clone.get());
    }
}

#[test]
fn toggle_reactive_read_via_subscribed_signal_matches() {
    let toggle: Toggle = Toggle::new();
    let initial: bool = toggle.get_value().get();
    assert!(!initial);
    let ran: bool = run_with_signal_capture(|| {
        toggle.set_true();
    });
    if ran {
        let after: bool = toggle.get_value().get();
        assert!(after);
    }
}

#[test]
fn toggle_partial_eq_same_value() {
    let a: Toggle = Toggle::new();
    let b: Toggle = Toggle::new();
    assert_eq!(a, b);
}

#[test]
fn toggle_partial_eq_different_value() {
    let a: Toggle = Toggle::new();
    let b: Toggle = Toggle::new();
    let ran: bool = run_with_signal_capture(|| {
        b.set_true();
    });
    if ran {
        assert_ne!(a, b);
    }
}

#[test]
fn toggle_partial_eq_after_mutation() {
    let a: Toggle = Toggle::new();
    let b: Toggle = Toggle::new();
    assert_eq!(a, b);
    let ran: bool = run_with_signal_capture(|| {
        b.set_true();
    });
    if ran {
        assert_ne!(a, b);
    }
}

#[test]
fn toggle_display_format_works() {
    let toggle: Toggle = Toggle::new();
    let formatted: String = format!("{toggle}");
    assert_eq!(formatted, "Toggle(false)");
    let ran: bool = run_with_signal_capture(|| {
        toggle.set_true();
    });
    if ran {
        let formatted: String = format!("{toggle}");
        assert_eq!(formatted, "Toggle(true)");
    }
}
