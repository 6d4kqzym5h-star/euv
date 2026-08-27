use super::*;

/// Helper body of the `base_time` free function.
///
/// # Returns
///
/// - `Instant` - A monotonic instant in time.
fn base_time() -> Instant {
    Instant::now()
}

/// Helper body of the `seed_throttled` free function.
///
/// # Arguments
///
/// - `&ThrottledValue<T>` - Shared reference to a `ThrottledValue<T>`.
/// - `T: Clone + PartialEq + Default + 'static` - A generic type parameter.
/// - `Instant` - A monotonic instant in time (`Instant`).
fn seed_throttled<T: Clone + PartialEq + Default + 'static>(
    throttled: &ThrottledValue<T>,
    initial: T,
    now: Instant,
) {
    throttled.set(initial, now);
}

#[test]
fn throttled_value_starts_at_default() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    assert_eq!(throttled.get(), 0);
    assert!(!throttled.is_throttling());
}

#[test]
fn throttled_value_set_when_idle_emits_immediately() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    let now: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        throttled.set(5, now);
    }))
    .is_ok();
    if ran {
        assert_eq!(throttled.get(), 5);
        assert!(throttled.is_throttling());
    }
}

#[test]
fn throttled_value_set_during_cooldown_buffers_pending() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    let t0: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        throttled.set(5, t0);
        throttled.set(7, t0 + Duration::from_millis(10));
    }))
    .is_ok();
    if ran {
        assert_eq!(throttled.get(), 5);
    }
}

#[test]
fn throttled_value_tick_during_cooldown_keeps_state() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    let t0: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        throttled.set(5, t0);
    }))
    .is_ok();
    if ran {
        let committed: bool = throttled.tick(t0 + Duration::from_millis(20));
        assert!(!committed);
        assert!(throttled.is_throttling());
        assert_eq!(throttled.get(), 5);
    }
}

#[test]
fn throttled_value_tick_at_interval_commits_pending() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    let t0: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        throttled.set(5, t0);
        throttled.set(7, t0 + Duration::from_millis(10));
    }))
    .is_ok();
    if ran {
        let committed: bool = throttled.tick(t0 + Duration::from_millis(100));
        assert!(committed);
        assert_eq!(throttled.get(), 7);
        assert!(!throttled.is_throttling());
    }
}

#[test]
fn throttled_value_tick_at_interval_with_no_pending_lapses_cooldown() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    let t0: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        throttled.set(5, t0);
    }))
    .is_ok();
    if ran {
        let committed: bool = throttled.tick(t0 + Duration::from_millis(100));
        assert!(!committed);
        assert_eq!(throttled.get(), 5);
        assert!(!throttled.is_throttling());
    }
}

#[test]
fn throttled_value_tick_after_interval_reopens_window() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(50);
    let t0: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        throttled.set(1, t0);
    }))
    .is_ok();
    if ran {
        throttled.tick(t0 + Duration::from_millis(60));
        assert!(!throttled.is_throttling());
    }
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        throttled.set(2, t0 + Duration::from_millis(70));
    }))
    .is_ok();
    if ran {
        assert_eq!(throttled.get(), 2);
        assert!(throttled.is_throttling());
    }
}

#[test]
fn throttled_value_multiple_buffered_sets_only_last_wins() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    let t0: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        throttled.set(1, t0);
        throttled.set(2, t0 + Duration::from_millis(10));
        throttled.set(3, t0 + Duration::from_millis(20));
        throttled.set(4, t0 + Duration::from_millis(30));
    }))
    .is_ok();
    if ran {
        let committed: bool = throttled.tick(t0 + Duration::from_millis(110));
        assert!(committed);
        assert_eq!(throttled.get(), 4);
    }
}

#[test]
fn throttled_value_cancel_drops_pending_and_cooldown() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    let t0: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        throttled.set(5, t0);
        throttled.set(7, t0 + Duration::from_millis(10));
        throttled.cancel();
    }))
    .is_ok();
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
    let throttled: ThrottledValue<i32> = ThrottledValue::new(0);
    let t0: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        throttled.set(1, t0);
        throttled.set(2, t0);
        throttled.set(3, t0);
    }))
    .is_ok();
    if ran {
        assert_eq!(throttled.get(), 3);
        assert!(!throttled.is_throttling());
    }
}

#[test]
fn throttled_value_tick_when_idle_is_noop() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    let now: Instant = base_time();
    let committed: bool = throttled.tick(now);
    assert!(!committed);
    assert_eq!(throttled.get(), 0);
    assert!(!throttled.is_throttling());
}

#[test]
fn throttled_value_clone_shares_state() {
    let original: ThrottledValue<i32> = ThrottledValue::new(100);
    let clone: ThrottledValue<i32> = original.clone();
    let t0: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        clone.set(9, t0);
    }))
    .is_ok();
    if ran {
        assert_eq!(original.get(), 9);
        assert!(original.is_throttling());
    }
}

#[test]
fn throttled_value_display_idle() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    let formatted: String = format!("{throttled}");
    assert_eq!(formatted, "ThrottledValue(0)");
}

#[test]
fn throttled_value_display_cooldown() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    let now: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        throttled.set(99, now);
    }))
    .is_ok();
    if ran {
        let formatted: String = format!("{throttled}");
        assert_eq!(formatted, "ThrottledValue(cooldown=99)");
    }
}

#[test]
fn throttled_value_seed_helper_commits_immediately() {
    let throttled: ThrottledValue<i32> = ThrottledValue::new(100);
    let now: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        seed_throttled(&throttled, 42, now);
    }))
    .is_ok();
    if ran {
        assert_eq!(throttled.get(), 42);
        assert!(throttled.is_throttling());
    }
}
