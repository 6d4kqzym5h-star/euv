use crate::*;

/// Implementation of hook context lifecycle and hook index management.
impl HookContext {
    /// Creates a new `HookContext` from an existing raw pointer.
    ///
    /// # Arguments
    ///
    /// - `*mut HookContextInner` - A raw pointer to the heap-allocated hook context inner state.
    ///
    /// # Returns
    ///
    /// - `Self` - A hook context handle wrapping the given pointer.
    pub fn from_inner(inner: *mut HookContextInner) -> Self {
        HookContext { inner }
    }

    /// Returns a mutable reference to the inner hook context state.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other references to the inner state exist.
    /// In single-threaded WASM this is always safe.
    #[allow(clippy::mut_from_ref)]
    fn get_inner_mut(&self) -> &mut HookContextInner {
        unsafe { &mut *self.inner }
    }

    /// Returns the current hook index.
    ///
    /// # Returns
    ///
    /// - `usize` - The current hook index.
    pub fn get_hook_index(&self) -> usize {
        self.get_inner_mut().get_hook_index()
    }

    /// Sets the hook index.
    ///
    /// # Arguments
    ///
    /// - `usize` - The new hook index value.
    pub fn set_hook_index(&mut self, index: usize) {
        self.get_inner_mut().set_hook_index(index);
    }

    /// Returns a reference to the hooks storage.
    ///
    /// # Returns
    ///
    /// - `&Vec<Box<dyn Any>>` - A reference to the hooks storage.
    pub fn get_hooks(&self) -> &Vec<Box<dyn Any>> {
        self.get_inner_mut().get_hooks()
    }

    /// Returns a mutable reference to the hooks storage.
    ///
    /// # Returns
    ///
    /// - `&mut Vec<Box<dyn Any>>` - A mutable reference to the hooks storage.
    pub fn get_mut_hooks(&mut self) -> &mut Vec<Box<dyn Any>> {
        self.get_inner_mut().get_mut_hooks()
    }

    /// Returns a mutable reference to the cleanup closures storage.
    ///
    /// # Returns
    ///
    /// - `&mut Vec<Box<dyn FnOnce()>>` - A mutable reference to the cleanup closures storage.
    pub fn get_mut_cleanups(&mut self) -> &mut Vec<Box<dyn FnOnce()>> {
        self.get_inner_mut().get_mut_cleanups()
    }

    /// Resets the hook index for a new render cycle.
    ///
    /// Sets the hook index back to `0` so that subsequent hook calls
    /// re-associate with their stored state by call order.
    pub fn reset_hook_index(&mut self) {
        self.set_hook_index(0_usize);
    }

    /// Notifies the hook context that a match arm is being entered.
    /// Toggles the `arm_changed` flag; if it differs from the previous value,
    /// the hooks array is cleared to prevent signal leakage between arms.
    ///
    /// # Arguments
    ///
    /// - `bool` - The new arm changed state.
    pub fn set_arm_changed(&mut self, changed: bool) {
        let inner: &mut HookContextInner = self.get_inner_mut();
        if inner.get_arm_changed() != changed {
            let cleanups: Vec<Box<dyn FnOnce()>> = std::mem::take(inner.get_mut_cleanups());
            for cleanup in cleanups {
                cleanup();
            }
            inner.get_mut_hooks().clear();
            inner.set_arm_changed(changed);
        }
        self.reset_hook_index();
    }
}

/// Clones the hook context, sharing the same inner state.
impl Clone for HookContext {
    /// Returns a bitwise copy of this hook context.
    fn clone(&self) -> Self {
        *self
    }
}

/// Copies the hook context, sharing the same inner state.
///
/// A `HookContext` is just a raw pointer; copying it is a trivial bitwise copy.
impl Copy for HookContext {}

/// Provides a default empty hook context.
impl Default for HookContext {
    /// Returns a default `HookContext` by allocating a new empty inner via `Box::leak`.
    fn default() -> Self {
        let boxed: Box<HookContextInner> = Box::default();
        HookContext::from_inner(Box::leak(boxed) as *mut HookContextInner)
    }
}

/// Implementation of HookContextInner construction.
impl HookContextInner {
    /// Creates a new empty hook context inner.
    ///
    /// # Returns
    ///
    /// - `Self` - A new hook context inner with empty hooks and cleanups.
    pub const fn new() -> Self {
        HookContextInner {
            hooks: Vec::new(),
            arm_changed: false,
            hook_index: 0_usize,
            cleanups: Vec::new(),
        }
    }
}

/// Provides a default empty hook context inner.
impl Default for HookContextInner {
    /// Returns a default `HookContextInner` by delegating to `HookContextInner::new`.
    fn default() -> Self {
        Self::new()
    }
}

/// SAFETY: `HookContextCell` is only used in single-threaded WASM contexts.
/// Concurrent access from multiple threads would be undefined behavior.
unsafe impl Sync for HookContextCell {}
