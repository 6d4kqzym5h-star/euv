use super::*;

impl<T: Clone + PartialEq + 'static> SuspenseHandle<T> {
    /// Creates a new `SuspenseHandle` in the `Pending`
    /// phase.
    pub fn new() -> Self {
        Self {
            phase: Signal::create(SuspensePhase::Pending),
        }
    }

    /// Transitions the phase to `Resolved(value)`. Works
    /// on every target.
    pub fn resolve_sync(&self, value: T) {
        self.get_phase().set(SuspensePhase::Resolved(value));
    }

    /// Transitions the phase to `Failed(message)`. Works
    /// on every target.
    pub fn fail(&self, message: String) {
        self.get_phase().set(SuspensePhase::Failed(message));
    }

    /// Transitions the phase back to `Pending`. Useful
    /// when invalidating the cache (e.g., after a
    /// mutation that requires refetching).
    pub fn reset(&self) {
        self.get_phase().set(SuspensePhase::Pending);
    }
}

impl<T: Clone + PartialEq + 'static> Default for SuspenseHandle<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + PartialEq + Debug + 'static> Display for SuspenseHandle<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "SuspenseHandle({:?})", self.get_phase().get())
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
