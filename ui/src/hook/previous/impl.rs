use super::*;

impl<T: Clone + PartialEq + 'static> Previous<T> {
    /// Creates a new `Previous` with no recorded value.
    /// The `previous` signal starts at `None`.
    pub fn new() -> Self {
        Self {
            previous: Signal::create(None),
        }
    }

    /// Records `current` as the new previous value. The
    /// next call to `get_previous_snapshot()` will return
    /// `Some(current)`.
    ///
    /// This is typically called at the top of a render
    /// closure so the signal stores the value just seen.
    pub fn record(&self, current: T) {
        self.get_previous().set(Some(current));
    }

    /// Returns a snapshot of the previously recorded
    /// value, or `None` if no value has been recorded yet.
    pub fn get_previous_snapshot(&self) -> Option<T> {
        self.get_previous().get()
    }

    /// Clears the recorded previous value, returning the
    /// tracker to the `None` state.
    pub fn clear(&self) {
        self.get_previous().set(None);
    }
}

impl<T: Clone + PartialEq + Debug + 'static> Display for Previous<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        match self.get_previous().get() {
            Some(value) => write!(formatter, "Previous(Some({value:?}))"),
            None => write!(formatter, "Previous(None)"),
        }
    }
}

impl<T: Clone + PartialEq + 'static> Default for Previous<T> {
    fn default() -> Self {
        Self::new()
    }
}
