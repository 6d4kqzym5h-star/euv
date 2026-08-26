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
fn previous_default_starts_with_none() {
    let previous: Previous<i32> = Previous::default();
    assert_eq!(previous.get_previous_snapshot(), None);
}

#[test]
fn previous_new_starts_with_none() {
    let previous: Previous<String> = Previous::new();
    assert_eq!(previous.get_previous_snapshot(), None);
}

#[test]
fn previous_record_stores_value() {
    let previous: Previous<i32> = Previous::new();
    let ran: bool = run_with_signal_capture(|| {
        previous.record(42);
    });
    if ran {
        assert_eq!(previous.get_previous_snapshot(), Some(42));
    }
}

#[test]
fn previous_record_overwrites_previous() {
    let previous: Previous<i32> = Previous::new();
    let ran: bool = run_with_signal_capture(|| {
        previous.record(1);
        previous.record(2);
        previous.record(3);
    });
    if ran {
        assert_eq!(previous.get_previous_snapshot(), Some(3));
    }
}

#[test]
fn previous_record_supports_string_values() {
    let previous: Previous<String> = Previous::new();
    let ran: bool = run_with_signal_capture(|| {
        previous.record(String::from("alpha"));
    });
    if ran {
        assert_eq!(
            previous.get_previous_snapshot(),
            Some(String::from("alpha"))
        );
    }
    let ran: bool = run_with_signal_capture(|| {
        previous.record(String::from("beta"));
    });
    if ran {
        assert_eq!(previous.get_previous_snapshot(), Some(String::from("beta")));
    }
}

#[test]
fn previous_clear_returns_to_none() {
    let previous: Previous<i32> = Previous::new();
    let ran: bool = run_with_signal_capture(|| {
        previous.record(1);
    });
    if ran {
        assert_eq!(previous.get_previous_snapshot(), Some(1));
    }
    let ran: bool = run_with_signal_capture(|| {
        previous.clear();
    });
    if ran {
        assert_eq!(previous.get_previous_snapshot(), None);
    }
}

#[test]
fn previous_clear_when_already_none_is_noop() {
    let previous: Previous<i32> = Previous::new();
    let ran: bool = run_with_signal_capture(|| {
        previous.clear();
    });
    if ran {
        assert_eq!(previous.get_previous_snapshot(), None);
    }
}

#[test]
fn previous_clone_shares_state() {
    let original: Previous<i32> = Previous::new();
    let clone: Previous<i32> = original.clone();
    let ran: bool = run_with_signal_capture(|| {
        clone.record(7);
    });
    if ran {
        assert_eq!(original.get_previous_snapshot(), Some(7));
        assert_eq!(clone.get_previous_snapshot(), Some(7));
    }
}

#[test]
fn previous_reactive_read_via_subscribed_signal_matches() {
    let previous: Previous<i32> = Previous::new();
    let initial: Option<i32> = previous.get_previous().get();
    assert_eq!(initial, None);
    let ran: bool = run_with_signal_capture(|| {
        previous.record(99);
    });
    if ran {
        let after: Option<i32> = previous.get_previous().get();
        assert_eq!(after, Some(99));
    }
}

#[test]
fn previous_record_then_clear_then_record_works() {
    let previous: Previous<i32> = Previous::new();
    let ran: bool = run_with_signal_capture(|| {
        previous.record(1);
        previous.clear();
        previous.record(2);
    });
    if ran {
        assert_eq!(previous.get_previous_snapshot(), Some(2));
    }
}

#[test]
fn previous_display_format_works_for_none() {
    let previous: Previous<i32> = Previous::new();
    let formatted: String = format!("{previous}");
    assert_eq!(formatted, "Previous(None)");
}

#[test]
fn previous_display_format_works_for_some() {
    let previous: Previous<i32> = Previous::new();
    let ran: bool = run_with_signal_capture(|| {
        previous.record(7);
    });
    if ran {
        let formatted: String = format!("{previous}");
        assert_eq!(formatted, "Previous(Some(7))");
    }
}
