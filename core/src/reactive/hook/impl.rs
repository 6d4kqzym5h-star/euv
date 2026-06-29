use crate::*;

/// Implementation of hook context lifecycle and hook index management.
impl HookContext {
    /// Resets the hook index for a new render cycle.
    ///
    /// Sets the internal hook index back to zero so that subsequent
    /// `use_signal` calls start indexing from the beginning of the hook list.
    pub fn reset_index(&mut self) {
        if let Ok(mut inner) = self.get_inner().try_borrow_mut() {
            inner.set_hook_index(0);
        }
    }

    /// Notifies the hook context that a match arm is being entered.
    ///
    /// If the arm index has changed, all existing hooks and cleanups
    /// are cleared and re-initialized for the new arm. If the arm
    /// is unchanged, only the hook index is reset.
    ///
    /// # Arguments
    ///
    /// - `usize` - The index of the new match arm.
    pub fn switch_arm(&mut self, changed: usize) {
        let cleanups: Vec<Box<dyn FnOnce()>>;
        {
            let Ok(mut inner) = self.get_inner().try_borrow_mut() else {
                return;
            };
            if inner.get_arm_changed() == changed {
                drop(inner);
                self.reset_index();
                return;
            }
            cleanups = take(inner.get_mut_cleanups());
            inner.get_mut_hooks().clear();
            inner.set_arm_changed(changed);
        }
        for cleanup in cleanups {
            cleanup();
        }
        self.reset_index();
    }
}

/// Clones the hook context, sharing the same inner state.
///
/// All clones share the same underlying `Rc<RefCell<HookContextInner>>`,
/// so modifications through one clone are visible through all others.
///
/// # Returns
///
/// - `Self`: A new `HookContext` sharing the same inner state.
impl Clone for HookContext {
    fn clone(&self) -> Self {
        Self::new(self.get_inner().clone())
    }
}

/// Provides a default empty hook context.
///
/// Creates a fresh `Rc<RefCell<HookContextInner>>` with default values
/// (empty hook list, zero hook index, empty cleanup list).
///
/// # Returns
///
/// - `Self`: A new `HookContext` with default inner state.
impl Default for HookContext {
    fn default() -> Self {
        Self::new(Rc::new(RefCell::new(HookContextInner::default())))
    }
}

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
        if let Some(cleanup_window) = web_sys::window() {
            cleanup_window.clear_interval_with_handle(self.get_interval_id());
        }
    }
}

/// Compares interval handles by their interval ID.
impl PartialEq for IntervalHandle {
    /// Compares two interval handles for equality by their interval IDs.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The first interval handle.
    /// - `&Self` - The second interval handle.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if both handles have the same interval ID.
    fn eq(&self, other: &Self) -> bool {
        self.get_interval_id() == other.get_interval_id()
    }
}
