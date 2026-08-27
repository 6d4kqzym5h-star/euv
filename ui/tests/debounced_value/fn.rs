use super::*;

/// Helper body of the `base_time` free function.
///
/// # Returns
///
/// - `Instant` - A monotonic instant in time.
fn base_time() -> Instant {
    Instant::now()
}

/// Helper body of the `seed_debounced` free function.
///
/// # Arguments
///
/// - `&DebouncedValue<T>` - Shared reference to a `DebouncedValue<T>`.
/// - `T: Clone + PartialEq + Default + 'static` - A generic type parameter.
/// - `u32` - A 32-bit unsigned integer (`u32`).
/// - `Instant` - A monotonic instant in time (`Instant`).
fn seed_debounced<T: Clone + PartialEq + Default + 'static>(
    debounced: &DebouncedValue<T>,
    initial: T,
    delay_ms: u32,
    now: Instant,
) {
    debounced.set(initial, now);
    debounced.tick(now + Duration::from_millis(u64::from(delay_ms) + 1));
}

#[test]
fn debounced_value_starts_at_default() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    assert_eq!(debounced.get(), 0);
    assert!(!debounced.is_pending());
}

#[test]
fn debounced_value_set_marks_pending() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    let now: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        debounced.set(5, now);
    }))
    .is_ok();
    if ran {
        assert!(debounced.is_pending());
        assert_eq!(debounced.get(), 0);
    }
}

#[test]
fn debounced_value_tick_before_delay_keeps_default() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    let now: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        debounced.set(5, now);
    }))
    .is_ok();
    if ran {
        let emitted: bool = debounced.tick(now);
        assert!(!emitted);
        assert!(debounced.is_pending());
        assert_eq!(debounced.get(), 0);
    }
}

#[test]
fn debounced_value_tick_at_delay_emits_pending() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    let now: Instant = base_time();
    let later: Instant = now + Duration::from_millis(100);
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        debounced.set(5, now);
    }))
    .is_ok();
    if ran {
        let emitted: bool = debounced.tick(later);
        assert!(emitted);
        assert!(!debounced.is_pending());
        assert_eq!(debounced.get(), 5);
    }
}

#[test]
fn debounced_value_tick_past_delay_emits_pending() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    let now: Instant = base_time();
    let later: Instant = now + Duration::from_millis(250);
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        debounced.set(7, now);
    }))
    .is_ok();
    if ran {
        let emitted: bool = debounced.tick(later);
        assert!(emitted);
        assert_eq!(debounced.get(), 7);
    }
}

#[test]
fn debounced_value_rapid_sets_only_last_wins() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    let t0: Instant = base_time();
    let t_emit: Instant = t0 + Duration::from_millis(150);
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        debounced.set(1, t0);
        debounced.set(2, t0 + Duration::from_millis(10));
        debounced.set(3, t0 + Duration::from_millis(20));
    }))
    .is_ok();
    if ran {
        let emitted: bool = debounced.tick(t_emit);
        assert!(emitted);
        assert_eq!(debounced.get(), 3);
    }
}

#[test]
fn debounced_value_cancel_drops_pending() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    let now: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        debounced.set(5, now);
        debounced.cancel();
    }))
    .is_ok();
    if ran {
        assert!(!debounced.is_pending());
        assert_eq!(debounced.get(), 0);
    }
}

#[test]
fn debounced_value_zero_delay_emits_immediately() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(0);
    let now: Instant = base_time();
    let _ran: bool = catch_unwind(AssertUnwindSafe(|| {
        debounced.set(5, now);
        let emitted: bool = debounced.tick(now);
        assert!(emitted);
        assert_eq!(debounced.get(), 5);
    }))
    .is_ok();
}

#[test]
fn debounced_value_tick_when_idle_is_noop() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    let now: Instant = base_time();
    let emitted: bool = debounced.tick(now);
    assert!(!emitted);
    assert_eq!(debounced.get(), 0);
}

#[test]
fn debounced_value_two_pending_cycles() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(50);
    let t0: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        debounced.set(1, t0);
    }))
    .is_ok();
    if ran {
        let t_lapse: Instant = t0 + Duration::from_millis(60);
        assert!(debounced.tick(t_lapse));
        assert_eq!(debounced.get(), 1);
    }
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        debounced.set(2, t0 + Duration::from_millis(70));
    }))
    .is_ok();
    if ran {
        let t_mid: Instant = t0 + Duration::from_millis(80);
        let t_done: Instant = t0 + Duration::from_millis(130);
        assert!(!debounced.tick(t_mid));
        assert!(debounced.tick(t_done));
        assert_eq!(debounced.get(), 2);
    }
}

#[test]
fn debounced_value_clone_shares_state() {
    let original: DebouncedValue<i32> = DebouncedValue::new(100);
    let clone: DebouncedValue<i32> = original.clone();
    let now: Instant = base_time();
    let later: Instant = now + Duration::from_millis(150);
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        clone.set(9, now);
    }))
    .is_ok();
    if ran {
        assert!(original.is_pending());
        assert!(original.tick(later));
        assert_eq!(clone.get(), 9);
    }
}

#[test]
fn debounced_value_string_round_trip() {
    let debounced: DebouncedValue<String> = DebouncedValue::new(10);
    let now: Instant = base_time();
    let later: Instant = now + Duration::from_millis(20);
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        debounced.set(String::from("hello"), now);
    }))
    .is_ok();
    if ran {
        assert!(debounced.tick(later));
        assert_eq!(debounced.get(), String::from("hello"));
    }
}

#[test]
fn debounced_value_display_idle() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    let formatted: String = format!("{debounced}");
    assert_eq!(formatted, "DebouncedValue(0)");
}

#[test]
fn debounced_value_display_pending() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(100);
    let now: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        debounced.set(99, now);
    }))
    .is_ok();
    if ran {
        let formatted: String = format!("{debounced}");
        assert_eq!(formatted, "DebouncedValue(pending=99)");
    }
}

#[test]
fn debounced_value_seed_helper_commits_immediately() {
    let debounced: DebouncedValue<i32> = DebouncedValue::new(10);
    let now: Instant = base_time();
    let ran: bool = catch_unwind(AssertUnwindSafe(|| {
        seed_debounced(&debounced, 42, 10, now);
    }))
    .is_ok();
    if ran {
        assert_eq!(debounced.get(), 42);
        assert!(!debounced.is_pending());
    }
}
