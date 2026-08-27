use super::*;

/// Inherent implementation of [`ErrorBoundary`].
impl ErrorBoundary {
    /// Creates a new `ErrorBoundary` in the `Healthy`
    /// phase.
    pub fn new() -> Self {
        Self {
            phase: Signal::create(ErrorBoundaryPhase::Healthy),
        }
    }

    /// Runs a closure and, if it panics, transitions
    /// the boundary to `Caught` and returns `Err`.
    ///
    /// On success, the closure's return value is
    /// returned wrapped in `Ok`. The closure is
    /// wrapped in `AssertUnwindSafe` so it does not
    /// have to satisfy `UnwindSafe`.
    pub fn try_with<F, R>(&self, closure: F) -> Result<R, String>
    where
        F: FnOnce() -> R + UnwindSafe,
    {
        match catch_unwind(closure) {
            Ok(value) => Ok(value),
            Err(payload) => {
                let message: String = extract_message(&payload);
                let _ = catch_unwind(AssertUnwindSafe(|| {
                    self.get_phase()
                        .set(ErrorBoundaryPhase::Caught(message.clone()));
                }));
                Err(message)
            }
        }
    }

    /// Transitions the boundary back to `Healthy`.
    /// Useful when invalidating the cache (e.g.,
    /// after a retry).
    pub fn reset(&self) {
        let _ = catch_unwind(AssertUnwindSafe(|| {
            self.get_phase().set(ErrorBoundaryPhase::Healthy);
        }));
    }
}

/// Default-construction for [`ErrorBoundary`].
impl Default for ErrorBoundary {
    /// Constructs a default [`ErrorBoundary`] value.
    fn default() -> Self {
        Self::new()
    }
}

/// Formatting / debug-printing for [`ErrorBoundary`].
impl Display for ErrorBoundary {
    /// Formats the [`ErrorBoundary`] via the supplied formatter.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "ErrorBoundary({:?})", self.get_phase().get())
    }
}

/// Equality comparison for [`ErrorBoundaryPhase`].
impl PartialEq for ErrorBoundaryPhase {
    /// Returns `true` when `self` and `other` are equivalent by the [`PartialEq`] contract.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ErrorBoundaryPhase::Healthy, ErrorBoundaryPhase::Healthy) => true,
            (ErrorBoundaryPhase::Caught(a), ErrorBoundaryPhase::Caught(b)) => a == b,
            _ => false,
        }
    }
}
