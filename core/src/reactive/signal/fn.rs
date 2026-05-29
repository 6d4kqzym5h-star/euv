use crate::*;

/// Retrieves a raw pointer reference to `RefCell<SignalInner<T>>` directly
/// from the signal's stored address, bypassing the registry HashMap.
///
/// SAFETY: The address stored in `Signal::inner` is always a valid pointer
/// to an `Rc<RefCell<SignalInner<T>>>` that is kept alive by the global
/// registry. Since WASM is single-threaded, the reference is always valid
/// as long as the signal has not been explicitly cleared.
///
/// # Arguments
///
/// - `usize` - The pointer address of the signal's inner state.
///
/// # Returns
///
/// - `&'static RefCell<SignalInner<T>>` - A reference to the signal's inner state.
#[inline(always)]
pub(crate) fn get_signal_inner_ref<T>(addr: usize) -> &'static RefCell<SignalInner<T>>
where
    T: Clone + PartialEq + 'static,
{
    unsafe { &*(addr as *const RefCell<SignalInner<T>>) }
}

/// Clears all listeners on a signal identified by its inner pointer address.
///
/// This function is used during DOM cleanup (`cleanup_dom_subtree`) to
/// release signal listeners that reference DOM elements being removed.
/// Only `Signal<String>` instances are bound to the DOM, so this function
/// only handles that type. If the address does not correspond to a
/// `Signal<String>`, this is a no-op.
///
/// # Arguments
///
/// - `usize` - The inner pointer address of the signal.
pub(crate) fn clear_signal_listeners_by_addr(addr: usize) {
    let inner_ref: &RefCell<SignalInner<String>> = get_signal_inner_ref(addr);
    let mut inner: RefMut<SignalInner<String>> = inner_ref.borrow_mut();
    inner.set_alive(false);
    inner.get_mut_listeners().clear();
}

/// Ensures the signal inner registry is initialized and returns a mutable reference.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
///
/// # Returns
///
/// - `&'static mut HashMap<usize, Rc<dyn Any>>`: A mutable reference to the signal inner registry.
#[allow(static_mut_refs)]
fn ensure_signal_inner_registry_mut() -> &'static mut HashMap<usize, Rc<dyn Any>> {
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
/// - `&'static mut HashMap<usize, Rc<dyn Any>>`: A mutable reference to the signal inner registry.
pub(crate) fn signal_inner_registry_mut() -> &'static mut HashMap<usize, Rc<dyn Any>> {
    ensure_signal_inner_registry_mut()
}
