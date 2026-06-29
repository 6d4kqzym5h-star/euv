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
pub(crate) fn get_signal_inner_mut<T>(addr: usize) -> &'static mut SignalInner<T>
where
    T: Clone + PartialEq + 'static,
{
    unsafe { &mut *(addr as *mut SignalInner<T>) }
}

/// Clears DOM-binding listeners on a bridge signal identified by its inner
/// pointer address, deactivates the bridge signal, and releases its value memory.
///
/// This function is used during DOM cleanup (`cleanup_dom_subtree`) to
/// release bridge `Signal<String>` instances that are no longer needed.
/// Only `Signal<String>` instances are bound to the DOM, so this function
/// only handles that type. If the address does not correspond to a
/// `Signal<String>`, this is a no-op.
///
/// Bridge signals are internal `Signal<String>` instances created by
/// `as_reactive_text` and `AttributeValue::Signal` for DOM binding.
/// They have exactly one consumer (the DOM element), so deactivating them
/// is safe when the element is removed. User-created source signals are
/// never passed to this function — they are tracked by `SignalInner.dependents`
/// and cleaned up by `use_signal`'s `deactivate()` on hook context teardown.
///
/// The bridge signal's value is replaced with `String::new()` to release
/// the original string data, and `alive` is set to `false` so that any
/// stale async references become safe no-ops.
///
/// # Arguments
///
/// - `usize` - The inner pointer address of the bridge signal.
pub(crate) fn clear_signal_listeners(addr: usize) {
    let inner: &mut SignalInner<String> = get_signal_inner_mut(addr);
    inner.get_mut_listeners().clear();
    inner.set_alive(false);
    inner.set_value(String::new());
    cleanup_attr_slot(addr);
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
pub(crate) fn is_signal_alive(addr: usize) -> bool {
    get_signal_inner_registry_mut().contains(&addr)
}

/// Ensures the signal inner registry is initialized and returns a mutable reference.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
///
/// # Returns
///
/// - `&'static mut HashSet<usize>`: A mutable reference to the signal inner registry.
#[allow(static_mut_refs)]
pub(crate) fn get_signal_inner_registry_mut() -> &'static mut HashSet<usize> {
    unsafe {
        if (*SIGNAL_INNER_REGISTRY.get_0().get()).is_none() {
            (*SIGNAL_INNER_REGISTRY.get_0().get()) = Some(HashSet::new());
        }
        (*SIGNAL_INNER_REGISTRY.get_0().get())
            .as_mut()
            .unwrap_unchecked()
    }
}
