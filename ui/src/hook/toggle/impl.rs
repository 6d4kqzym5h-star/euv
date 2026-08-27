use super::*;

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

impl Display for Toggle {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "Toggle({})", self.get_value().get())
    }
}

impl PartialEq for Toggle {
    fn eq(&self, other: &Self) -> bool {
        self.get_value().get() == other.get_value().get()
    }
}
