use super::*;

impl ErrorBoundary {
    /// Creates a new `ErrorBoundary` in the `Healthy`
    /// phase.
    pub fn new() -> Self {
        Self {
            phase: Signal::create(ErrorBoundaryPhase::Healthy),
        }
    }

    /// Returns the underlying phase signal.
    pub fn phase(&self) -> Signal<ErrorBoundaryPhase> {
        *self.get_phase()
    }

    /// Returns a snapshot of the current phase.
    pub fn current(&self) -> ErrorBoundaryPhase {
        self.get_phase().get()
    }

    /// Returns `true` if no child has thrown yet.
    pub fn is_healthy(&self) -> bool {
        matches!(self.get_phase().get(), ErrorBoundaryPhase::Healthy)
    }

    /// Returns `true` if a child has thrown.
    pub fn is_caught(&self) -> bool {
        matches!(self.get_phase().get(), ErrorBoundaryPhase::Caught(_))
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
        F: FnOnce() -> R + std::panic::UnwindSafe,
    {
        match std::panic::catch_unwind(closure) {
            Ok(value) => Ok(value),
            Err(payload) => {
                let message = extract_message(&payload);
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    self.get_phase().set(ErrorBoundaryPhase::Caught(message.clone()));
                }));
                Err(message)
            }
        }
    }

    /// Transitions the boundary back to `Healthy`.
    /// Useful when invalidating the cache (e.g.,
    /// after a retry).
    pub fn reset(&self) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.get_phase().set(ErrorBoundaryPhase::Healthy);
        }));
    }
}

impl Default for ErrorBoundary {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ErrorBoundary {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "ErrorBoundary({:?})", self.get_phase().get())
    }
}

impl PartialEq for ErrorBoundaryPhase {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ErrorBoundaryPhase::Healthy, ErrorBoundaryPhase::Healthy) => true,
            (ErrorBoundaryPhase::Caught(a), ErrorBoundaryPhase::Caught(b)) => a == b,
            _ => false,
        }
    }
}
