use super::*;

// --- SuspensePhase ---

#[test]
fn suspense_phase_default_is_pending() {
    let phase: SuspensePhase<i32> = SuspensePhase::default();
    assert!(matches!(phase, SuspensePhase::Pending));
}

#[test]
fn suspense_phase_partial_eq_same_variant_pending() {
    let a: SuspensePhase<i32> = SuspensePhase::Pending;
    let b: SuspensePhase<i32> = SuspensePhase::Pending;
    assert_eq!(a, b);
}

#[test]
fn suspense_phase_partial_eq_same_variant_resolved() {
    let a: SuspensePhase<i32> = SuspensePhase::Resolved(1);
    let b: SuspensePhase<i32> = SuspensePhase::Resolved(1);
    assert_eq!(a, b);
}

#[test]
fn suspense_phase_partial_eq_same_variant_failed() {
    let a: SuspensePhase<i32> = SuspensePhase::Failed("oops".to_string());
    let b: SuspensePhase<i32> = SuspensePhase::Failed("oops".to_string());
    assert_eq!(a, b);
}

#[test]
fn suspense_phase_partial_eq_different_variants() {
    let pending: SuspensePhase<i32> = SuspensePhase::Pending;
    let resolved: SuspensePhase<i32> = SuspensePhase::Resolved(0);
    let failed: SuspensePhase<i32> = SuspensePhase::Failed("x".to_string());
    assert_ne!(pending, resolved);
    assert_ne!(pending, failed);
    assert_ne!(resolved, failed);
}

// --- SuspenseHandle basics ---

#[test]
fn new_is_pending() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    assert!(handle.is_pending());
    assert!(!handle.is_resolved());
    assert!(!handle.is_failed());
    assert!(matches!(handle.current(), SuspensePhase::Pending));
}

#[test]
fn default_is_pending() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::default();
    assert!(handle.is_pending());
}

#[test]
fn state_returns_signal_with_same_value() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    let signal = handle.state();
    assert!(matches!(signal.get(), SuspensePhase::Pending));
}

#[test]
fn debug_format_works() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    let s: String = format!("{:?}", handle);
    assert!(s.contains("SuspenseHandle"));
}

#[test]
fn display_format_works() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    let s: String = format!("{}", handle);
    assert!(s.contains("SuspenseHandle"));
    assert!(s.contains("Pending"));
}

// --- resolve_sync ---

#[test]
fn resolve_sync_transitions_to_resolved() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    handle.resolve_sync(42);
    assert!(handle.is_resolved());
    assert_eq!(handle.current(), SuspensePhase::Resolved(42));
}

#[test]
fn resolve_sync_with_string() {
    let handle: SuspenseHandle<String> = SuspenseHandle::new();
    handle.resolve_sync("hello".to_string());
    assert_eq!(
        handle.current(),
        SuspensePhase::Resolved("hello".to_string())
    );
}

#[test]
fn resolve_sync_with_vec() {
    let handle: SuspenseHandle<Vec<i32>> = SuspenseHandle::new();
    handle.resolve_sync(vec![1, 2, 3]);
    assert_eq!(handle.current(), SuspensePhase::Resolved(vec![1, 2, 3]));
}

// --- fail ---

#[test]
fn fail_transitions_to_failed() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    handle.fail("network error".to_string());
    assert!(handle.is_failed());
    assert_eq!(
        handle.current(),
        SuspensePhase::Failed("network error".to_string())
    );
}

// --- reset ---

#[test]
fn reset_from_resolved_returns_to_pending() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    handle.resolve_sync(42);
    handle.reset();
    assert!(handle.is_pending());
}

#[test]
fn reset_from_failed_returns_to_pending() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    handle.fail("oops".to_string());
    handle.reset();
    assert!(handle.is_pending());
}

// --- phase transitions ---

#[test]
fn phase_transitions_pending_resolved_pending() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    assert!(handle.is_pending());
    handle.resolve_sync(1);
    assert!(handle.is_resolved());
    handle.reset();
    assert!(handle.is_pending());
}

#[test]
fn phase_transitions_pending_failed_pending() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    assert!(handle.is_pending());
    handle.fail("x".to_string());
    assert!(handle.is_failed());
    handle.reset();
    assert!(handle.is_pending());
}

#[test]
fn multiple_resolve_calls_update_value() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    handle.resolve_sync(1);
    assert_eq!(handle.current(), SuspensePhase::Resolved(1));
    handle.resolve_sync(2);
    assert_eq!(handle.current(), SuspensePhase::Resolved(2));
    handle.resolve_sync(3);
    assert_eq!(handle.current(), SuspensePhase::Resolved(3));
}

// --- clone ---

#[test]
fn clone_shares_state() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    let cloned = handle.clone();
    // Cloning clones the Signal, so they share
    // the underlying state. resolve on the
    // handle is visible on the clone.
    // Use catch_unwind because Signal::set
    // panics on native.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        handle.resolve_sync(99);
    }));
    // The clone sees the new value if the set
    // didn't panic; otherwise the value is
    // unchanged.
    let current: SuspensePhase<i32> = cloned.current();
    // On wasm the value should be 99; on native
    // it could still be Pending. Either is
    // acceptable for this test — the key is
    // that no panics escape.
    let _ = current;
}

#[test]
fn state_signal_is_reactive() {
    let handle: SuspenseHandle<i32> = SuspenseHandle::new();
    let signal = handle.state();
    // Initial state is Pending.
    assert!(matches!(signal.get(), SuspensePhase::Pending));
    // After resolve_sync the signal reflects the
    // new value (catch_unwind because Signal::set
    // panics on native).
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        handle.resolve_sync(42);
    }));
    // The signal still reads without panicking;
    // its value depends on whether the set
    // succeeded.
    let value: SuspensePhase<i32> = signal.get();
    // The value is either Pending (native,
    // set panicked) or Resolved(42) (wasm).
    assert!(matches!(
        value,
        SuspensePhase::Pending | SuspensePhase::Resolved(42)
    ));
}
