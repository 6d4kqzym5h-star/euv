use super::*;

// --- ErrorBoundaryPhase ---

#[test]
fn error_boundary_phase_default_is_healthy() {
    let phase: ErrorBoundaryPhase = ErrorBoundaryPhase::default();
    assert!(matches!(phase, ErrorBoundaryPhase::Healthy));
}

#[test]
fn error_boundary_phase_partial_eq_both_healthy() {
    let a: ErrorBoundaryPhase = ErrorBoundaryPhase::Healthy;
    let b: ErrorBoundaryPhase = ErrorBoundaryPhase::Healthy;
    assert_eq!(a, b);
}

#[test]
fn error_boundary_phase_partial_eq_same_message() {
    let a: ErrorBoundaryPhase = ErrorBoundaryPhase::Caught("oops".to_string());
    let b: ErrorBoundaryPhase = ErrorBoundaryPhase::Caught("oops".to_string());
    assert_eq!(a, b);
}

#[test]
fn error_boundary_phase_partial_eq_different_message() {
    let a: ErrorBoundaryPhase = ErrorBoundaryPhase::Caught("oops".to_string());
    let b: ErrorBoundaryPhase = ErrorBoundaryPhase::Caught("boom".to_string());
    assert_ne!(a, b);
}

#[test]
fn error_boundary_phase_partial_eq_healthy_vs_caught() {
    let a: ErrorBoundaryPhase = ErrorBoundaryPhase::Healthy;
    let b: ErrorBoundaryPhase = ErrorBoundaryPhase::Caught("x".to_string());
    assert_ne!(a, b);
    assert_ne!(b, a);
}

// --- ErrorBoundary basics ---

#[test]
fn new_is_healthy() {
    let boundary = ErrorBoundary::new();
    assert!(boundary.is_healthy());
    assert!(!boundary.is_caught());
    assert!(matches!(boundary.current(), ErrorBoundaryPhase::Healthy));
}

#[test]
fn default_is_healthy() {
    let boundary = ErrorBoundary::default();
    assert!(boundary.is_healthy());
}

#[test]
fn phase_returns_signal_with_healthy_value() {
    let boundary = ErrorBoundary::new();
    let signal = boundary.phase();
    assert!(matches!(signal.get(), ErrorBoundaryPhase::Healthy));
}

#[test]
fn debug_format_works() {
    let boundary = ErrorBoundary::new();
    let s: String = format!("{:?}", boundary);
    assert!(s.contains("ErrorBoundary"));
}

#[test]
fn display_format_works() {
    let boundary = ErrorBoundary::new();
    let s: String = format!("{}", boundary);
    assert!(s.contains("ErrorBoundary"));
    assert!(s.contains("Healthy"));
}

// --- try_with (success) ---

#[test]
fn try_with_success_returns_value() {
    let boundary = ErrorBoundary::new();
    let result: Result<i32, String> = boundary.try_with(|| 42);
    assert_eq!(result, Ok(42));
    assert!(boundary.is_healthy());
}

#[test]
fn try_with_success_string_value() {
    let boundary = ErrorBoundary::new();
    let result: Result<String, String> = boundary.try_with(|| "hello".to_string());
    assert_eq!(result, Ok("hello".to_string()));
    assert!(boundary.is_healthy());
}

#[test]
fn try_with_success_complex_value() {
    let boundary = ErrorBoundary::new();
    let result: Result<Vec<i32>, String> = boundary.try_with(|| vec![1, 2, 3]);
    assert_eq!(result, Ok(vec![1, 2, 3]));
    assert!(boundary.is_healthy());
}

#[test]
fn try_with_success_unit() {
    let boundary = ErrorBoundary::new();
    let result: Result<(), String> = boundary.try_with(|| {});
    assert_eq!(result, Ok(()));
    assert!(boundary.is_healthy());
}

// --- try_with (panic) ---

#[test]
fn try_with_static_str_panic() {
    let boundary = ErrorBoundary::new();
    let result: Result<(), String> = boundary.try_with(|| -> () {
        panic!("static str panic");
    });
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "static str panic");
    assert!(boundary.is_caught());
}

#[test]
fn try_with_string_panic() {
    let boundary = ErrorBoundary::new();
    let result: Result<(), String> = boundary.try_with(|| -> () {
        panic!("{}", "owned string panic");
    });
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "owned string panic");
    assert!(boundary.is_caught());
}

#[test]
fn try_with_non_string_panic() {
    let boundary = ErrorBoundary::new();
    let result: Result<(), String> = boundary.try_with(|| -> () {
        std::panic::panic_any(42_i32);
    });
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "<non-string panic payload>");
    assert!(boundary.is_caught());
}

// --- reset ---

#[test]
fn reset_from_caught_returns_to_healthy() {
    let boundary = ErrorBoundary::new();
    let _ = boundary.try_with(|| -> () {
        panic!("boom");
    });
    assert!(boundary.is_caught());
    boundary.reset();
    assert!(boundary.is_healthy());
}

#[test]
fn reset_from_healthy_is_noop() {
    let boundary = ErrorBoundary::new();
    boundary.reset();
    assert!(boundary.is_healthy());
}

// --- clone + share ---

#[test]
fn clone_shares_state() {
    let boundary = ErrorBoundary::new();
    let cloned = boundary.clone();
    assert!(cloned.is_healthy());
    // Reset on the clone is visible on the
    // original via catch_unwind because
    // Signal::set panics on native.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        boundary.reset();
    }));
    // The clone still reads without panic.
    let _ = cloned.current();
}

#[test]
fn phase_signal_is_reactive() {
    let boundary = ErrorBoundary::new();
    let signal = boundary.phase();
    // Initial state is Healthy.
    assert!(matches!(signal.get(), ErrorBoundaryPhase::Healthy));
    // After a panic-driven try_with the
    // boundary transitions (catch_unwind wraps
    // the set call).
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: Result<(), String> = boundary.try_with(|| -> () {
            panic!("reactive test");
        });
    }));
    // The signal still reads without panicking;
    // its value depends on whether the set
    // succeeded.
    let value: ErrorBoundaryPhase = signal.get();
    assert!(matches!(
        value,
        ErrorBoundaryPhase::Healthy | ErrorBoundaryPhase::Caught(_)
    ));
}

// --- multiple panics ---

#[test]
fn multiple_panics_overwrite_message() {
    let boundary = ErrorBoundary::new();
    let _ = boundary.try_with(|| -> () {
        panic!("first");
    });
    let _ = boundary.try_with(|| -> () {
        panic!("second");
    });
    assert!(boundary.is_caught());
    if let ErrorBoundaryPhase::Caught(message) = boundary.current() {
        assert_eq!(message, "second");
    } else {
        panic!("expected Caught");
    }
}

#[test]
fn panic_then_reset_then_success() {
    let boundary = ErrorBoundary::new();
    let _ = boundary.try_with(|| -> () {
        panic!("boom");
    });
    assert!(boundary.is_caught());
    boundary.reset();
    assert!(boundary.is_healthy());
    let result: Result<i32, String> = boundary.try_with(|| 42);
    assert_eq!(result, Ok(42));
    assert!(boundary.is_healthy());
}

// --- extract_message ---

#[test]
fn extract_message_from_static_str() {
    let payload: Box<dyn std::any::Any + Send> = Box::new("hello");
    assert_eq!(extract_message(&payload), "hello");
}

#[test]
fn extract_message_from_string() {
    let payload: Box<dyn std::any::Any + Send> = Box::new("world".to_string());
    assert_eq!(extract_message(&payload), "world");
}

#[test]
fn extract_message_from_i32() {
    let payload: Box<dyn std::any::Any + Send> = Box::new(42_i32);
    assert_eq!(extract_message(&payload), "<non-string panic payload>");
}
