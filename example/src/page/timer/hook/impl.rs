use crate::*;

/// Implementation of interval handle lifecycle management.
impl IntervalHandle {
    /// Cancels the associated browser interval timer.
    ///
    /// Calls `window.clearInterval` with the stored interval ID.
    /// After calling this method the interval callback will no longer fire.
    ///
    /// # Panics
    ///
    /// Panics if `window()` is unavailable on the current platform.
    pub fn clear(&self) {
        let window: Window = window().expect("no global window exists");
        window.clear_interval_with_handle(self.get_interval_id());
    }
}

/// Compares interval handles by their interval ID.
impl PartialEq for IntervalHandle {
    fn eq(&self, other: &Self) -> bool {
        self.get_interval_id() == other.get_interval_id()
    }
}
