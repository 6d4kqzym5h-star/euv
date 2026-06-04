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
pub(crate) fn get_signal_inner_ref<T>(addr: usize) -> &'static mut SignalInner<T>
where
    T: Clone + PartialEq + 'static,
{
    unsafe { &mut *(addr as *mut SignalInner<T>) }
}

/// Clears DOM-binding listeners on a signal identified by its inner pointer
/// address, without deactivating the signal itself.
///
/// This function is used during DOM cleanup (`cleanup_dom_subtree`) to
/// release signal listeners that reference DOM elements being removed.
/// Only `Signal<String>` instances are bound to the DOM, so this function
/// only handles that type. If the address does not correspond to a
/// `Signal<String>`, this is a no-op.
///
/// Importantly, this does NOT set `alive = false` or clear `dependents`.
/// A signal may be shared across multiple DOM bindings and DynamicNodes
/// (e.g., a user-created signal used as both a `value:` attribute on a
/// conditionally-rendered element AND as a dependency of another
/// DynamicNode's render function). Deactivating the signal here would
/// permanently break all other dependents — subsequent `set()` calls
/// would be no-ops and `get()` would skip dependency tracking.
///
/// The listeners are cleared to prevent stale callbacks from referencing
/// removed DOM elements. The signal remains fully functional for future
/// `set()` / `get()` calls and dependent DynamicNodes continue to receive
/// updates.
///
/// # Arguments
///
/// - `usize` - The inner pointer address of the signal.
pub(crate) fn clear_signal_listeners_by_addr(addr: usize) {
    let inner: &mut SignalInner<String> = get_signal_inner_ref(addr);
    inner.get_mut_listeners().clear();
    // Remove the attribute signal's update slot from the signal update registry.
    // Attribute signals are registered with the signal address as key (not a
    // dynamic_id), so they are not covered by `cleanup_dynamic_node`.
    cleanup_attr_signal_update_slot(addr);
}

/// Returns whether the signal allocation at `addr` is still present in the
/// global registry (i.e. has not been freed signal inner).
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
    signal_inner_registry_mut().contains(&addr)
}

/// Ensures the signal inner registry is initialized and returns a mutable reference.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
///
/// # Returns
///
/// - `&'static mut HashSet<usize>`: A mutable reference to the signal inner registry.
#[allow(static_mut_refs)]
pub(crate) fn signal_inner_registry_mut() -> &'static mut HashSet<usize> {
    unsafe {
        if (*SIGNAL_INNER_REGISTRY.get_0().get()).is_none() {
            (*SIGNAL_INNER_REGISTRY.get_0().get()) = Some(HashSet::new());
        }
        (*SIGNAL_INNER_REGISTRY.get_0().get())
            .as_mut()
            .unwrap_unchecked()
    }
}
