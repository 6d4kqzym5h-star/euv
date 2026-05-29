use crate::*;

/// Implementation of `From` trait for converting `usize` address into `&'static mut HandlerSlot`.
impl From<usize> for &'static mut HandlerSlot {
    /// Converts a memory address into a mutable reference to `HandlerSlot`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `HandlerSlot` instance.
    ///
    /// # Returns
    ///
    /// - `&'static mut HandlerSlot` - A mutable reference at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `HandlerSlot` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> Self {
        unsafe { &mut *(address as *mut HandlerSlot) }
    }
}

/// SAFETY: `HandlerRegistryCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for HandlerRegistryCell {}

/// SAFETY: `DelegatedEventsCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for DelegatedEventsCell {}

/// SAFETY: `SignalUpdateRegistryCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for SignalUpdateRegistryCell {}
