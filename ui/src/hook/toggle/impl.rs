use super::*;

/// Inherent implementation of [`Toggle`].
impl Toggle {
    /// Sets the value to `true`.
    pub fn set_true(&self) {
        self.get_value().set(true);
    }

    /// Sets the value to `false`.
    pub fn set_false(&self) {
        self.get_value().set(false);
    }

    /// Flips the value: `true` becomes `false`,
    /// `false` becomes `true`.
    pub fn toggle(&self) {
        let current: bool = self.get_value().get();
        self.get_value().set(!current);
    }

    /// Replaces the value with `next`.
    pub fn set(&self, next: bool) {
        self.get_value().set(next);
    }

    /// Returns the current value as a snapshot.
    pub fn get(&self) -> bool {
        self.get_value().get()
    }
}

/// Formatting / debug-printing for [`Toggle`].
impl Display for Toggle {
    /// Formats the [`Toggle`] via the supplied formatter.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "Toggle({})", self.get_value().get())
    }
}

/// Equality comparison for [`Toggle`].
impl PartialEq for Toggle {
    /// Returns `true` when `self` and `other` are equivalent by the [`PartialEq`] contract.
    fn eq(&self, other: &Self) -> bool {
        self.get_value().get() == other.get_value().get()
    }
}
