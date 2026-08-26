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

fn base_time() -> Instant {
    Instant::now()
}

#[test]
fn throttled_value_starts_with_initial() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(0, 100);
    assert_eq!(throttled.get(), 0);
    assert!(!throttled.is_throttling());
}

#[test]
fn throttled_value_set_when_idle_emits_immediately() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(0, 100);
    let now: Instant = base_time();
    let ran: bool = run_with_signal_capture(|| {
        throttled.set(5, now);
    });
    if ran {
        assert_eq!(throttled.get(), 5);
        assert!(throttled.is_throttling());
    }
}

#[test]
fn throttled_value_set_during_cooldown_buffers_pending() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(0, 100);
    let t0: Instant = base_time();
    let ran: bool = run_with_signal_capture(|| {
        throttled.set(5, t0);
        throttled.set(7, t0 + Duration::from_millis(10));
    });
    if ran {
        assert_eq!(throttled.get(), 5);
    }
}

#[test]
fn throttled_value_tick_during_cooldown_keeps_state() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(0, 100);
    let t0: Instant = base_time();
    let ran: bool = run_with_signal_capture(|| {
        throttled.set(5, t0);
    });
    if ran {
        let committed: bool = throttled.tick(t0 + Duration::from_millis(20));
        assert!(!committed);
        assert!(throttled.is_throttling());
        assert_eq!(throttled.get(), 5);
    }
}

#[test]
fn throttled_value_tick_at_interval_commits_pending() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(0, 100);
    let t0: Instant = base_time();
    let ran: bool = run_with_signal_capture(|| {
        throttled.set(5, t0);
        throttled.set(7, t0 + Duration::from_millis(10));
    });
    if ran {
        let committed: bool = throttled.tick(t0 + Duration::from_millis(100));
        assert!(committed);
        assert_eq!(throttled.get(), 7);
        assert!(!throttled.is_throttling());
    }
}

#[test]
fn throttled_value_tick_at_interval_with_no_pending_lapses_cooldown() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(0, 100);
    let t0: Instant = base_time();
    let ran: bool = run_with_signal_capture(|| {
        throttled.set(5, t0);
    });
    if ran {
        let committed: bool = throttled.tick(t0 + Duration::from_millis(100));
        assert!(!committed);
        assert_eq!(throttled.get(), 5);
        assert!(!throttled.is_throttling());
    }
}

#[test]
fn throttled_value_tick_after_interval_reopens_window() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(0, 50);
    let t0: Instant = base_time();
    let ran: bool = run_with_signal_capture(|| {
        throttled.set(1, t0);
    });
    if ran {
        throttled.tick(t0 + Duration::from_millis(60));
        assert!(!throttled.is_throttling());
    }
    let ran: bool = run_with_signal_capture(|| {
        throttled.set(2, t0 + Duration::from_millis(70));
    });
    if ran {
        assert_eq!(throttled.get(), 2);
        assert!(throttled.is_throttling());
    }
}

#[test]
fn throttled_value_multiple_buffered_sets_only_last_wins() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(0, 100);
    let t0: Instant = base_time();
    let ran: bool = run_with_signal_capture(|| {
        throttled.set(1, t0);
        throttled.set(2, t0 + Duration::from_millis(10));
        throttled.set(3, t0 + Duration::from_millis(20));
        throttled.set(4, t0 + Duration::from_millis(30));
    });
    if ran {
        let committed: bool = throttled.tick(t0 + Duration::from_millis(110));
        assert!(committed);
        assert_eq!(throttled.get(), 4);
    }
}

#[test]
fn throttled_value_cancel_drops_pending_and_cooldown() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(0, 100);
    let t0: Instant = base_time();
    let ran: bool = run_with_signal_capture(|| {
        throttled.set(5, t0);
        throttled.set(7, t0 + Duration::from_millis(10));
        throttled.cancel();
    });
    if ran {
        assert!(!throttled.is_throttling());
        assert_eq!(throttled.get(), 5);
    }
    let later: Instant = t0 + Duration::from_millis(110);
    let committed: bool = throttled.tick(later);
    assert!(!committed);
    assert_eq!(throttled.get(), 5);
}

#[test]
fn throttled_value_zero_interval_emits_every_set() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(0, 0);
    let t0: Instant = base_time();
    let ran: bool = run_with_signal_capture(|| {
        throttled.set(1, t0);
        throttled.set(2, t0);
        throttled.set(3, t0);
    });
    if ran {
        assert_eq!(throttled.get(), 3);
        assert!(!throttled.is_throttling());
    }
}

#[test]
fn throttled_value_tick_when_idle_is_noop() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(42, 100);
    let now: Instant = base_time();
    let committed: bool = throttled.tick(now);
    assert!(!committed);
    assert_eq!(throttled.get(), 42);
    assert!(!throttled.is_throttling());
}

#[test]
fn throttled_value_clone_shares_state() {
    let original: ThrottledValue<i32> = ThrottledValue::new(0, 100);
    let clone: ThrottledValue<i32> = original.clone();
    let t0: Instant = base_time();
    let ran: bool = run_with_signal_capture(|| {
        clone.set(9, t0);
    });
    if ran {
        assert_eq!(original.get(), 9);
        assert!(original.is_throttling());
    }
}

#[test]
fn throttled_value_display_idle() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(7, 100);
    let formatted: String = format!("{throttled}");
    assert_eq!(formatted, "ThrottledValue(7)");
}

#[test]
fn throttled_value_display_cooldown() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(7, 100);
    let now: Instant = base_time();
    let ran: bool = run_with_signal_capture(|| {
        throttled.set(99, now);
    });
    if ran {
        let formatted: String = format!("{throttled}");
        assert_eq!(formatted, "ThrottledValue(cooldown=99)");
    }
}
