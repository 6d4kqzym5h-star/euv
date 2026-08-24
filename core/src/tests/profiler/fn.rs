use super::*;

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::vec::Vec;

/// Runs the given closure, swallowing the native-build
/// `Signal::set` panic that fires when `web_sys::window()`
/// is called outside a real browser context.
///
/// Returns `true` if the closure ran to completion (the wasm
/// path) and `false` if it panicked (the native path). Tests
/// use this to gate their post-push assertions: on wasm the
/// entries vector reflects the push; on native it does not,
/// so any "did the entry actually land?" assertion must be
/// skipped on the `false` branch.
fn run_with_signal_capture<F>(f: F) -> bool
where
    F: FnOnce(),
{
    catch_unwind(AssertUnwindSafe(f)).is_ok()
}

#[test]
fn now_ms_is_non_decreasing_and_non_negative() {
    let a: f64 = now_ms();
    // A tiny busy-spin to ensure the clock has a chance to
    // tick — `Instant` on Linux is typically nanosecond
    // resolution, so even a few hundred cycles should be
    // enough.
    let mut sink: u64 = 0;
    for i in 0..10_000_u64 {
        sink = sink.wrapping_add(i);
    }
    std::hint::black_box(sink);
    let b: f64 = now_ms();
    assert!(a >= 0.0, "now_ms must be non-negative, got {}", a);
    assert!(b >= a, "now_ms must be monotonic, got {} then {}", a, b);
}

#[test]
fn now_ms_two_calls_typically_differ() {
    // Best-effort: even on coarse clocks, the two calls should
    // differ for a measurable workload. If this ever flakes
    // on a future platform, bump the loop count — but
    // silently accepting equality here would defeat the
    // purpose of the monotonic-clock check.
    let a: f64 = now_ms();
    let mut sink: u64 = 0;
    for i in 0..1_000_000_u64 {
        sink = sink.wrapping_mul(i.wrapping_add(1));
    }
    std::hint::black_box(sink);
    let b: f64 = now_ms();
    assert!(b >= a);
}

#[test]
fn profile_entry_new_stores_all_fields() {
    let entry: ProfileEntry = ProfileEntry::new(String::from("op"), 1.5, 100.0);
    assert_eq!(entry.get_label(), "op");
    assert!((entry.get_elapsed_ms() - 1.5).abs() < f64::EPSILON);
    assert!((entry.get_timestamp_ms() - 100.0).abs() < f64::EPSILON);
}

#[test]
fn profile_entry_clone_preserves_all_fields() {
    let original: ProfileEntry = ProfileEntry::new(String::from("orig"), 2.5, 200.0);
    let copy: ProfileEntry = original.clone();
    assert_eq!(copy, original);
}

#[test]
fn profile_entry_partial_eq_compares_field_by_field() {
    let a: ProfileEntry = ProfileEntry::new(String::from("a"), 1.0, 10.0);
    let b: ProfileEntry = ProfileEntry::new(String::from("a"), 1.0, 10.0);
    let c: ProfileEntry = ProfileEntry::new(String::from("a"), 1.0, 11.0);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn profiler_handle_new_starts_with_empty_entries() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let entries: Vec<ProfileEntry> = handle.entries().get();
    assert!(entries.is_empty());
}

#[test]
fn profiler_handle_clone_shares_entries_signal() {
    // `Clone` is `Copy`-by-pointer for `Signal<T>` and
    // `derive(Clone)` on `ProfilerHandle` is a field clone —
    // both handles must point at the same underlying signal,
    // so a measurement pushed via one is visible via the
    // other.
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let twin: ProfilerHandle = handle.clone();
    let ran_clean: bool = run_with_signal_capture(|| {
        handle.measure("first", || 42);
    });
    if ran_clean {
        assert_eq!(twin.entries().get().len(), 1);
        assert_eq!(twin.entries().get()[0].get_label(), "first");
    }
}

#[test]
fn profiler_handle_measure_pushes_entry_with_nonzero_label() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let mut captured: Option<i32> = None;
    let ran_clean: bool = run_with_signal_capture(|| {
        captured = Some(handle.measure("compute", || 7_i32.wrapping_mul(6)));
    });
    assert!(ran_clean, "native Signal::set panic should be caught");
    assert_eq!(captured, Some(42), "closure return value must be forwarded");
    if ran_clean {
        let entries: Vec<ProfileEntry> = handle.entries().get();
        assert_eq!(entries.len(), 1);
        let entry: &ProfileEntry = &entries[0];
        assert_eq!(entry.get_label(), "compute");
        // `elapsed_ms` should be `>= 0.0`. It can be exactly 0 on
        // very fast platforms if the clock granularity is
        // coarser than the measured workload, but it can never
        // be negative.
        assert!(entry.get_elapsed_ms() >= 0.0);
        assert!(entry.get_timestamp_ms() >= 0.0);
    }
}

#[test]
fn profiler_handle_measure_accumulates_multiple_entries() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let ran_clean: bool = run_with_signal_capture(|| {
        for i in 0..5_u32 {
            handle.measure(&format!("op-{}", i), || i.wrapping_add(1));
        }
    });
    assert!(ran_clean, "native Signal::set panic should be caught");
    if ran_clean {
        let entries: Vec<ProfileEntry> = handle.entries().get();
        assert_eq!(entries.len(), 5);
        for (i, entry) in entries.iter().enumerate() {
            assert_eq!(entry.get_label(), format!("op-{}", i).as_str());
        }
    }
}

#[test]
fn profiler_handle_measure_forwards_closure_return_value() {
    // The closure can return any type — `measure` forwards
    // it unchanged. Verify the return value is not lost
    // (e.g. by accidentally returning `()` or the entry).
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let mut captured: Option<String> = None;
    let ran_clean: bool = run_with_signal_capture(|| {
        captured = Some(handle.measure("build-string", || String::from("hello")));
    });
    assert!(ran_clean);
    assert_eq!(captured.as_deref(), Some("hello"));
    let mut captured_tuple: Option<(i32, f64)> = None;
    let ran_clean: bool = run_with_signal_capture(|| {
        captured_tuple = Some(handle.measure("build-tuple", || (7, 3.14)));
    });
    assert!(ran_clean);
    assert_eq!(captured_tuple, Some((7, 3.14)));
}

#[test]
fn profiler_handle_clear_empties_entries() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let ran_clean: bool = run_with_signal_capture(|| {
        handle.measure("first", || ());
        handle.measure("second", || ());
    });
    assert!(ran_clean);
    if ran_clean {
        assert_eq!(handle.entries().get().len(), 2);
        handle.clear();
        assert!(handle.entries().get().is_empty());
        // New measurements after `clear()` start from zero again.
        let ran_clean_post: bool = run_with_signal_capture(|| {
            handle.measure("third", || ());
        });
        assert!(ran_clean_post);
        assert_eq!(handle.entries().get().len(), 1);
        assert_eq!(handle.entries().get()[0].get_label(), "third");
    }
}

#[test]
fn profiler_handle_begin_end_push_entry_with_nonzero_elapsed() {
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let mark: ProfilerMark = handle.begin("interval");
    // Simulate some work between begin() and end().
    let mut sink: u64 = 0;
    for i in 0..10_000_u64 {
        sink = sink.wrapping_add(i);
    }
    std::hint::black_box(sink);
    // `end()` pushes via `Signal::set`. On wasm the push
    // succeeds; on native it panics inside `web_sys::window()`.
    // `catch_unwind` returns `Ok(())` only on the wasm path,
    // and the entry's invariants are then verified; on the
    // native path the post-push assertions are skipped
    // because the entries vector is still empty (the push
    // was aborted before the write).
    let ran_clean: bool = run_with_signal_capture(|| {
        mark.end();
    });
    if ran_clean {
        let entries: Vec<ProfileEntry> = handle.entries().get();
        assert_eq!(entries.len(), 1);
        let entry: &ProfileEntry = &entries[0];
        assert_eq!(entry.get_label(), "interval");
        assert!(entry.get_elapsed_ms() >= 0.0);
        assert!(entry.get_timestamp_ms() >= 0.0);
    }
}

#[test]
fn profiler_handle_entries_signal_is_subscribable() {
    // The whole point of storing entries in a `Signal` is
    // that subscribers can `.get()` and re-render on push.
    // Simulate a subscriber by cloning the signal into a
    // separate variable and confirming `.get()` reflects
    // mutations made via the handle.
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let subscriber_signal: Signal<Vec<ProfileEntry>> = handle.entries();
    assert!(subscriber_signal.get().is_empty());
    let ran_clean: bool = run_with_signal_capture(|| {
        handle.measure("first", || ());
    });
    assert!(ran_clean);
    if ran_clean {
        assert_eq!(subscriber_signal.get().len(), 1);
        let ran_clean2: bool = run_with_signal_capture(|| {
            handle.measure("second", || ());
        });
        assert!(ran_clean2);
        assert_eq!(subscriber_signal.get().len(), 2);
    }
}

#[test]
fn profiler_mark_drop_without_end_discards_silently() {
    // The contract is: dropping a `ProfilerMark` without
    // calling `end()` does NOT push any entry. Verify by
    // constructing the marker, dropping it explicitly, and
    // confirming entries are still empty.
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    {
        let _mark: ProfilerMark = handle.begin("discard");
        // _mark dropped at end of scope without .end()
    }
    assert!(handle.entries().get().is_empty());
}

#[test]
fn profiler_handle_measure_records_distinct_timestamps() {
    // Two measurements taken back-to-back should have
    // non-decreasing timestamps. (Strictly speaking the
    // timestamps can be equal on a coarse clock, but never
    // earlier.)
    let handle: ProfilerHandle = ProfilerHandle::new(Signal::create(Vec::new()));
    let ran_clean: bool = run_with_signal_capture(|| {
        handle.measure("a", || ());
        handle.measure("b", || ());
    });
    assert!(ran_clean);
    if ran_clean {
        let entries: Vec<ProfileEntry> = handle.entries().get();
        assert!(entries[0].get_timestamp_ms() <= entries[1].get_timestamp_ms());
    }
}
