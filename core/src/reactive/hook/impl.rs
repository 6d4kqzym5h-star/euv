use crate::*;

/// Implementation of `From` trait for converting `usize` address into `&'static mut HookContextInner`.
impl From<usize> for &'static mut HookContextInner {
    /// Converts a memory address into a mutable reference to `HookContextInner`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `HookContextInner` instance.
    ///
    /// # Returns
    ///
    /// - `&'static mut HookContextInner` - A mutable reference to the `HookContextInner` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `HookContextInner` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> Self {
        unsafe { &mut *(address as *mut HookContextInner) }
    }
}

/// Implementation of `From` trait for converting `usize` address into `&'static HookContextInner`.
impl From<usize> for &'static HookContextInner {
    /// Converts a memory address into a reference to `HookContextInner`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `HookContextInner` instance.
    ///
    /// # Returns
    ///
    /// - `&'static HookContextInner` - A reference to the `HookContextInner` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `HookContextInner` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> Self {
        unsafe { &*(address as *const HookContextInner) }
    }
}

/// Implementation of `From` trait for converting `HookContext` into `usize` address.
impl From<HookContext> for usize {
    /// Converts a `HookContext` into its memory address.
    ///
    /// # Arguments
    ///
    /// - `HookContext` - The hook context handle.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the hook context's inner state.
    #[inline(always)]
    fn from(context: HookContext) -> Self {
        context.inner as usize
    }
}

/// Implementation of `From` trait for converting `usize` address into `HookContext`.
impl From<usize> for HookContext {
    /// Converts a memory address into a `HookContext` handle.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address previously obtained from `HookContext` conversion.
    ///
    /// # Returns
    ///
    /// - `HookContext` - A hook context handle wrapping the pointer at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `HookContextInner` instance
    ///   that was previously converted from a hook context handle and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> Self {
        HookContext {
            inner: address as *mut HookContextInner,
        }
    }
}

/// Implementation of hook context lifecycle and hook index management.
impl HookContext {
    /// Returns a mutable reference to the inner hook context state by going
    /// through `usize` intermediate conversion.
    ///
    /// # Returns
    ///
    /// - `&'static mut HookContextInner` - A mutable reference to the inner state.
    #[allow(clippy::mut_from_ref)]
    pub fn leak_mut(&self) -> &'static mut HookContextInner {
        let address: usize = (*self).into();
        address.into()
    }

    /// Resets the hook index for a new render cycle.
    ///
    /// Sets the hook index back to `0` so that subsequent hook calls
    /// re-associate with their stored state by call order.
    pub fn reset_hook_index(&mut self) {
        self.leak_mut().set_hook_index(0);
    }

    /// Notifies the hook context that a match arm is being entered.
    /// Toggles the `arm_changed` flag; if it differs from the previous value,
    /// the hooks array is cleared to prevent signal leakage between arms.
    ///
    /// # Arguments
    ///
    /// - `bool` - The new arm changed state.
    pub fn set_arm_changed(&mut self, changed: bool) {
        let inner: &mut HookContextInner = self.leak_mut();
        if inner.get_arm_changed() != changed {
            let cleanups: Vec<Box<dyn FnOnce()>> = take(inner.get_mut_cleanups());
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
        let ptr: *mut HookContextInner = Box::leak(boxed) as *mut HookContextInner;
        let address: usize = ptr as usize;
        address.into()
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

/// Implementation of HookContextCell construction and access.
impl HookContextCell {
    /// Returns a mutable reference to the inner `HookContextInner`.
    ///
    /// # Returns
    ///
    /// - `&'static mut HookContextInner` - A mutable reference to the inner state.
    pub const fn get_inner(&self) -> &'static mut HookContextInner {
        unsafe { &mut *self.0.get() }
    }
}

/// SAFETY: `HookContextCell` is only used in single-threaded WASM contexts.
/// Concurrent access from multiple threads would be undefined behavior.
unsafe impl Sync for HookContextCell {}
