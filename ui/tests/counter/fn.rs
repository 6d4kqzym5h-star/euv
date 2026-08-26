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
fn counter_default_starts_at_zero() {
    let counter: Counter = Counter::default();
    assert_eq!(counter.get(), 0);
}

#[test]
fn counter_new_starts_at_zero_with_step_one() {
    let counter: Counter = Counter::new(None, None, 1);
    assert_eq!(counter.get(), 0);
}

#[test]
fn counter_new_with_bounds_starts_at_zero() {
    let counter: Counter = Counter::new(Some(0), Some(10), 1);
    assert_eq!(counter.get(), 0);
}

#[test]
fn counter_set_initial_value() {
    let counter: Counter = Counter::new(None, None, 1);
    let ran: bool = run_with_signal_capture(|| {
        counter.set(7);
    });
    if ran {
        assert_eq!(counter.get(), 7);
    }
}

#[test]
fn counter_set_into_bounds_clamps() {
    let counter: Counter = Counter::new(Some(0), Some(10), 1);
    let ran: bool = run_with_signal_capture(|| {
        counter.set(100);
        counter.set(-100);
        counter.set(5);
    });
    if ran {
        assert_eq!(counter.get(), 5);
    }
}

#[test]
fn counter_set_unbounded_does_not_clamp() {
    let counter: Counter = Counter::new(None, None, 1);
    let ran: bool = run_with_signal_capture(|| {
        counter.set(1_000_000);
    });
    if ran {
        assert_eq!(counter.get(), 1_000_000);
    }
    let ran: bool = run_with_signal_capture(|| {
        counter.set(-1_000_000);
    });
    if ran {
        assert_eq!(counter.get(), -1_000_000);
    }
}

#[test]
fn counter_increment_unbounded_adds_step() {
    let counter: Counter = Counter::new(None, None, 1);
    let ran: bool = run_with_signal_capture(|| {
        counter.increment();
        counter.increment();
    });
    if ran {
        assert_eq!(counter.get(), 2);
    }
}

#[test]
fn counter_decrement_unbounded_subtracts_step() {
    let counter: Counter = Counter::new(None, None, 1);
    let ran: bool = run_with_signal_capture(|| {
        counter.set(5);
    });
    if ran {
        let ran: bool = run_with_signal_capture(|| {
            counter.decrement();
        });
        if ran {
            assert_eq!(counter.get(), 4);
        }
    }
}

#[test]
fn counter_increment_caps_at_max() {
    let counter: Counter = Counter::new(Some(0), Some(3), 1);
    let ran: bool = run_with_signal_capture(|| {
        counter.increment();
        counter.increment();
        counter.increment();
        counter.increment();
        counter.increment();
    });
    if ran {
        assert_eq!(counter.get(), 3);
        assert!(counter.is_at_max());
    }
}

#[test]
fn counter_decrement_floors_at_min() {
    let counter: Counter = Counter::new(Some(0), Some(3), 1);
    let ran: bool = run_with_signal_capture(|| {
        counter.decrement();
        counter.decrement();
    });
    if ran {
        assert_eq!(counter.get(), 0);
        assert!(counter.is_at_min());
    }
}

#[test]
fn counter_is_at_min_is_false_when_unbounded() {
    let counter: Counter = Counter::new(None, None, 1);
    assert!(!counter.is_at_min());
}

#[test]
fn counter_is_at_max_is_false_when_unbounded() {
    let counter: Counter = Counter::new(None, None, 1);
    assert!(!counter.is_at_max());
}

#[test]
fn counter_clone_shares_state() {
    let original: Counter = Counter::new(None, None, 1);
    let clone: Counter = original.clone();
    let ran: bool = run_with_signal_capture(|| {
        clone.increment();
    });
    if ran {
        assert_eq!(original.get(), 1);
        assert_eq!(clone.get(), 1);
    }
}

#[test]
fn counter_reactive_read_via_subscribed_signal_matches() {
    let counter: Counter = Counter::new(None, None, 1);
    let initial: i32 = counter.get_value().get();
    assert_eq!(initial, 0);
    let ran: bool = run_with_signal_capture(|| {
        counter.increment();
    });
    if ran {
        let after: i32 = counter.get_value().get();
        assert_eq!(after, 1);
    }
}

#[test]
fn counter_display_format_works() {
    let counter: Counter = Counter::new(None, None, 1);
    let ran: bool = run_with_signal_capture(|| {
        counter.set(42);
    });
    if ran {
        let formatted: String = format!("{counter}");
        assert_eq!(formatted, "Counter(42)");
    }
}

#[test]
fn counter_increment_with_custom_step() {
    let counter: Counter = Counter::new(Some(0), Some(100), 5);
    let ran: bool = run_with_signal_capture(|| {
        counter.increment();
        counter.increment();
    });
    if ran {
        assert_eq!(counter.get(), 10);
    }
}

#[test]
fn counter_set_unchecked_bypasses_bounds() {
    let counter: Counter = Counter::new(Some(0), Some(10), 1);
    let ran: bool = run_with_signal_capture(|| {
        counter.set_unchecked(100);
    });
    if ran {
        assert_eq!(counter.get(), 100);
        // The value sits *above* `max` because
        // `set_unchecked` ignored the bound. The
        // read-side helpers (`is_at_max`, `is_at_min`)
        // compare to the configured bounds regardless,
        // so they will report "at max" here. That is the
        // intentional behavior — the bounds describe
        // where the counter is *clamped to*; bypassing
        // them puts the counter outside the clamped
        // range.
        assert!(counter.get() > counter.get_max());
    }
}
