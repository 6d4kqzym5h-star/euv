use super::*;

impl<T: Clone + PartialEq + 'static> SuspenseHandle<T> {
    /// Creates a new `SuspenseHandle` in the `Pending`
    /// phase.
    pub fn new() -> Self {
        Self {
            phase: Signal::create(SuspensePhase::Pending),
        }
    }

    /// Returns the underlying phase signal.
    pub fn state(&self) -> Signal<SuspensePhase<T>> {
        self.phase
    }

    /// Returns a snapshot of the current phase (no
    /// signal read).
    pub fn current(&self) -> SuspensePhase<T> {
        self.phase.get()
    }

    /// Returns `true` if the phase is `Pending`.
    pub fn is_pending(&self) -> bool {
        matches!(self.phase.get(), SuspensePhase::Pending)
    }

    /// Returns `true` if the phase is `Resolved`.
    pub fn is_resolved(&self) -> bool {
        matches!(self.phase.get(), SuspensePhase::Resolved(_))
    }

    /// Returns `true` if the phase is `Failed`.
    pub fn is_failed(&self) -> bool {
        matches!(self.phase.get(), SuspensePhase::Failed(_))
    }

    /// Transitions the phase to `Resolved(value)`. Works
    /// on every target.
    pub fn resolve_sync(&self, value: T) {
        self.phase.set(SuspensePhase::Resolved(value));
    }

    /// Transitions the phase to `Failed(message)`. Works
    /// on every target.
    pub fn fail(&self, message: String) {
        self.phase.set(SuspensePhase::Failed(message));
    }

    /// Transitions the phase back to `Pending`. Useful
    /// when invalidating the cache (e.g., after a
    /// mutation that requires refetching).
    pub fn reset(&self) {
        self.phase.set(SuspensePhase::Pending);
    }
}

impl<T: Clone + PartialEq + 'static> Default for SuspenseHandle<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + PartialEq + std::fmt::Debug + 'static> std::fmt::Display for SuspenseHandle<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "SuspenseHandle({:?})", self.phase.get())
    }
}

impl<T: PartialEq> PartialEq for SuspensePhase<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SuspensePhase::Pending, SuspensePhase::Pending) => true,
            (SuspensePhase::Resolved(a), SuspensePhase::Resolved(b)) => a == b,
            (SuspensePhase::Failed(a), SuspensePhase::Failed(b)) => a == b,
            _ => false,
        }
    }
}
