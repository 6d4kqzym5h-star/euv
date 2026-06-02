use crate::*;

/// Retrieves a mutable pointer to `SignalInner<T>` directly from the signal's
/// stored address.
///
/// SAFETY: The address stored in `Signal::inner` is always a valid pointer
/// to a `SignalInner<T>` that is kept alive by the global registry. Since
/// WASM is single-threaded, the pointer is always valid as long as the signal
/// has not been explicitly freed.
///
/// # Arguments
///
/// - `usize` - The pointer address of the signal's inner state.
///
/// # Returns
///
/// - `&'static mut SignalInner<T>` - A mutable reference to the signal's inner state.
#[inline(always)]
pub(crate) fn get_signal_inner_ref<T>(addr: usize) -> &'static mut SignalInner<T>
where
    T: Clone + PartialEq + 'static,
{
    unsafe { &mut *(addr as *mut SignalInner<T>) }
}

/// Clears all listeners on a signal identified by its inner pointer address
/// and marks it as inactive.
///
/// This function is used during DOM cleanup (`cleanup_dom_subtree`) to
/// release signal listeners that reference DOM elements being removed.
/// Only `Signal<String>` instances are bound to the DOM, so this function
/// only handles that type. If the address does not correspond to a
/// `Signal<String>`, this is a no-op.
///
/// Also clears the dependents list to stop precise dirty marking for
/// dynamic nodes that depended on this signal.
///
/// The heap allocation is intentionally NOT freed here. Because `Signal<T>`
/// is `Copy` (just a `usize` address), async callbacks (e.g., `setInterval`,
/// `setTimeout`, Promises) may still hold copies of the address after the
/// owning DOM subtree is removed. If we freed the memory, those callbacks
/// would dereference a dangling pointer (use-after-free / UB).
///
/// Instead, we only mark `alive = false`:
/// - `set()` / `update_and_notify()` check `alive` and become no-ops.
/// - `get()` still returns the last stored value safely (memory is intact).
/// - The memory remains allocated until the page is unloaded. For SPAs this
///   is acceptable; for long-lived apps a periodic GC pass can sweep all
///   `alive == false` entries from the registry once no async references
///   remain.
///
/// # Arguments
///
/// - `usize` - The inner pointer address of the signal.
pub(crate) fn clear_signal_listeners_by_addr(addr: usize) {
    let inner: &mut SignalInner<String> = get_signal_inner_ref(addr);
    inner.set_alive(false);
    inner.get_mut_listeners().clear();
    inner.get_mut_dependents().clear();
    // Do NOT call free_signal_inner(addr) — the memory must remain valid
    // for any async callbacks that still hold a copy of this signal address.
    // Remove the attribute signal's update slot from the signal update registry.
    // Attribute signals are registered with the signal address as key (not a
    // dynamic_id), so they are not covered by `cleanup_dynamic_node`.
    cleanup_attr_signal_update_slot(addr);
}

/// Returns whether the signal allocation at `addr` is still present in the
/// global registry (i.e. has not been freed via `free_signal_inner`).
///
/// Used by `update_and_notify` to avoid re-borrowing a `SignalInner` pointer
/// after running listeners, since a listener may have freed the allocation
/// during its execution. Probing the registry is the only safe way to detect
/// this, because the raw address itself carries no liveness information.
///
/// # Arguments
///
/// - `usize` - The inner pointer address of the signal.
///
/// # Returns
///
/// - `bool` - `true` if the allocation is still registered (safe to deref).
pub(crate) fn is_signal_inner_alive(addr: usize) -> bool {
    signal_inner_registry_mut().contains_key(&addr)
}

/// Frees the heap allocation for a signal and removes it from the registry.
///
/// Calls the type-erased drop function stored in the registry entry to
/// correctly reconstruct and drop the `Box<SignalInner<T>>`.
///
/// # Arguments
///
/// - `usize` - The inner pointer address of the signal.
pub(crate) fn free_signal_inner(addr: usize) {
    if let Some(entry) = signal_inner_registry_mut().remove(&addr) {
        unsafe { (entry.get_drop_fn())(entry.get_ptr()) };
    }
}

/// Type-erased drop function for `SignalInner<T>`.
///
/// Reconstructs the `Box<SignalInner<T>>` from the raw pointer and drops it,
/// freeing the heap allocation.
///
/// # Safety
///
/// The pointer must have been created by `Box::into_raw(Box::new(SignalInner<T>))`
/// and must not have been freed previously.
pub(crate) unsafe fn drop_signal_inner<T: Clone + PartialEq + 'static>(ptr: *mut ()) {
    unsafe {
        let _ = Box::from_raw(ptr as *mut SignalInner<T>);
    }
}

/// Ensures the signal inner registry is initialized and returns a mutable reference.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
///
/// # Returns
///
/// - `&'static mut HashMap<usize, SignalRegistryEntry>`: A mutable reference to the signal inner registry.
#[allow(static_mut_refs)]
fn ensure_signal_inner_registry_mut() -> &'static mut HashMap<usize, SignalRegistryEntry> {
    unsafe {
        if (*SIGNAL_INNER_REGISTRY.get_0().get()).is_none() {
            (*SIGNAL_INNER_REGISTRY.get_0().get()) = Some(HashMap::new());
        }
        (*SIGNAL_INNER_REGISTRY.get_0().get())
            .as_mut()
            .unwrap_unchecked()
    }
}

/// Returns a mutable reference to the signal inner registry.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
///
/// # Returns
///
/// - `&'static mut HashMap<usize, SignalRegistryEntry>`: A mutable reference to the signal inner registry.
pub(crate) fn signal_inner_registry_mut() -> &'static mut HashMap<usize, SignalRegistryEntry> {
    ensure_signal_inner_registry_mut()
}
