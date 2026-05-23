use crate::*;

/// Retrieves a cloned `Rc<RefCell<SignalInner<T>>>` from the global registry.
///
/// # Arguments
///
/// - `usize` - The pointer address of the signal's inner state.
///
/// # Returns
///
/// - `Rc<RefCell<SignalInner<T>>>` - A cloned reference to the signal's inner state.
///
/// # Panics
///
/// Panics if the address is not found in the global signal registry.
pub(crate) fn get_signal_inner_rc<T>(addr: usize) -> Rc<RefCell<SignalInner<T>>>
where
    T: Clone + PartialEq + 'static,
{
    let registry_ref: &HashMap<usize, Rc<dyn Any>> = signal_inner_registry();
    let any_rc: &Rc<dyn Any> = registry_ref
        .get(&addr)
        .expect("Signal inner not found in registry");
    let any_ptr: *const dyn Any = Rc::as_ptr(any_rc);
    let concrete_ptr: *const RefCell<SignalInner<T>> = any_ptr as *const RefCell<SignalInner<T>>;
    let concrete_rc: Rc<RefCell<SignalInner<T>>> = unsafe { Rc::from_raw(concrete_ptr) };
    let result: Rc<RefCell<SignalInner<T>>> = concrete_rc.clone();
    let _ = Rc::into_raw(concrete_rc);
    result
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
    let registry_ref: &HashMap<usize, Rc<dyn Any>> = signal_inner_registry();
    if let Some(any_rc) = registry_ref.get(&addr) {
        let any_ptr: *const dyn Any = Rc::as_ptr(any_rc);
        let concrete_ptr: *const RefCell<SignalInner<String>> =
            any_ptr as *const RefCell<SignalInner<String>>;
        let concrete_rc: Rc<RefCell<SignalInner<String>>> = unsafe { Rc::from_raw(concrete_ptr) };
        {
            let mut inner: RefMut<SignalInner<String>> = concrete_rc.borrow_mut();
            inner.set_alive(false);
            inner.get_mut_listeners().clear();
        }
        let _ = Rc::into_raw(concrete_rc);
    }
}

/// Ensures the signal inner registry is initialized and returns a shared reference.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
#[allow(static_mut_refs)]
fn ensure_signal_inner_registry() -> &'static HashMap<usize, Rc<dyn Any>> {
    unsafe {
        if (*SIGNAL_INNER_REGISTRY.get_0().get()).is_none() {
            (*SIGNAL_INNER_REGISTRY.get_0().get()) = Some(HashMap::new());
        }
        (*SIGNAL_INNER_REGISTRY.get_0().get())
            .as_ref()
            .unwrap_unchecked()
    }
}

/// Ensures the signal inner registry is initialized and returns a mutable reference.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
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

/// Returns a shared reference to the signal inner registry.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
pub(crate) fn signal_inner_registry() -> &'static HashMap<usize, Rc<dyn Any>> {
    ensure_signal_inner_registry()
}

/// Returns a mutable reference to the signal inner registry.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
pub(crate) fn signal_inner_registry_mut() -> &'static mut HashMap<usize, Rc<dyn Any>> {
    ensure_signal_inner_registry_mut()
}
