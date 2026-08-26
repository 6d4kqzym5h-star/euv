use super::*;

impl Counter {
    /// Creates a counter at `initial` with no bounds and
    /// `step = 1`.
    pub fn new(initial: i32) -> Self {
        Self {
            value: Signal::create(initial),
            min: None,
            max: None,
            step: 1,
        }
    }

    /// Creates a counter at `initial`, clamped into
    /// `[min, max]`, with the given `step`.
    ///
    /// `step` must be positive; a non-positive step is
    /// clamped to `1`.
    pub fn with_bounds(initial: i32, min: i32, max: i32, step: i32) -> Self {
        let safe_step: i32 = if step > 0 { step } else { 1 };
        let clamped_initial: i32 = initial.clamp(min, max);
        Self {
            value: Signal::create(clamped_initial),
            min: Some(min),
            max: Some(max),
            step: safe_step,
        }
    }

    /// Adds `step` to the value, clamping into `[min, max]`
    /// afterwards. If the counter is already at `max`, the
    /// value is left at `max`.
    pub fn increment(&self) {
        let current: i32 = self.get_value().get();
        let mut next: i32 = current.saturating_add(self.step);
        if let Some(max) = self.max {
            next = next.min(max);
        }
        if let Some(min) = self.min {
            next = next.max(min);
        }
        self.get_value().set(next);
    }

    /// Subtracts `step` from the value, clamping into
    /// `[min, max]` afterwards. If the counter is already
    /// at `min`, the value is left at `min`.
    pub fn decrement(&self) {
        let current: i32 = self.get_value().get();
        let mut next: i32 = current.saturating_sub(self.step);
        if let Some(min) = self.min {
            next = next.max(min);
        }
        if let Some(max) = self.max {
            next = next.min(max);
        }
        self.get_value().set(next);
    }

    /// Replaces the value with `next`, clamping into
    /// `[min, max]` if bounds are set.
    pub fn set(&self, next: i32) {
        let clamped: i32 = match (self.min, self.max) {
            (Some(min), Some(max)) => next.clamp(min, max),
            (Some(min), None) => next.max(min),
            (None, Some(max)) => next.min(max),
            (None, None) => next,
        };
        self.get_value().set(clamped);
    }

    /// Resets the value to whatever `initial` was passed
    /// to the constructor (the closest thing to a stored
    /// "initial" the public API exposes), clamped into
    /// bounds.
    ///
    /// `Counter::new(7).reset()` returns to `7`.
    /// `Counter::with_bounds(100, 0, 10, 1).reset()`
    /// returns to `10` (the upper bound, because `100`
    /// was clamped on construction).
    pub fn reset(&self) {
        let current: i32 = self.get_value().get();
        self.set(current);
    }

    /// Returns `true` when the value is at the configured
    /// `max` (or always `false` if unbounded above).
    pub fn is_at_max(&self) -> bool {
        match self.max {
            Some(max) => self.get_value().get() >= max,
            None => false,
        }
    }

    /// Returns `true` when the value is at the configured
    /// `min` (or always `false` if unbounded below).
    pub fn is_at_min(&self) -> bool {
        match self.min {
            Some(min) => self.get_value().get() <= min,
            None => false,
        }
    }

    /// Returns the current value as a snapshot.
    pub fn get(&self) -> i32 {
        self.get_value().get()
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Display for Counter {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "Counter({})", self.get_value().get())
    }
}
