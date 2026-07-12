use crate::*;

/// Implementation of reactive signal operations.
impl<T> Signal<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Returns a shared reference to the signal inner registry.
    ///
    /// # Returns
    ///
    /// - `&'static HashSet<usize>` - A shared reference to the global signal address registry.
    #[allow(static_mut_refs)]
    fn registry() -> &'static HashSet<usize> {
        unsafe { &*SIGNAL_INNER_REGISTRY.deref().get_0().get() }
    }

    /// Returns a mutable reference to the signal inner registry.
    ///
    /// # Returns
    ///
    /// - `&'static mut HashSet<usize>` - A mutable reference to the global signal address registry.
    #[allow(static_mut_refs)]
    fn registry_mut() -> &'static mut HashSet<usize> {
        unsafe { &mut *SIGNAL_INNER_REGISTRY.deref().get_0().get() }
    }

    /// Creates a new `Signal` with the given initial value.
    ///
    /// Allocates `SignalInner<T>` on the heap via `Box`, stores the raw pointer
    /// address, and registers it in the global registry for lifecycle tracking.
    ///
    /// # Arguments
    ///
    /// - `T: Clone + PartialEq + 'static` - The initial value of the signal.
    ///
    /// # Returns
    ///
    /// - `Self` - A handle to the newly created reactive signal.
    pub fn create(value: T) -> Self {
        let mut inner: SignalInner<T> = SignalInner::new(value, Vec::new(), true);
        inner.set_listeners_replaced(false);
        let boxed: Box<SignalInner<T>> = Box::new(inner);
        let ptr: *mut SignalInner<T> = Box::into_raw(boxed);
        let addr: usize = ptr as usize;
        Self::registry_mut().insert(addr);
        let mut signal: Self = Self::new(0, std::marker::PhantomData);
        signal.set_inner(addr);
        signal
    }

    /// Returns the current value of the signal.
    ///
    /// Directly reads the value from the heap-allocated inner state via raw
    /// pointer dereference. No runtime borrow checking overhead.
    ///
    /// If the signal has been marked inactive (`alive == false`), returns the
    /// last stored value without registering tracking dependencies. This
    /// ensures that stale async callbacks (e.g., orphaned `setInterval`)
    /// holding a `Signal` copy can still call `.get()` safely without
    /// triggering side effects or panics.
    ///
    /// If a tracking context is active (i.e., a DynamicNode is being rendered),
    /// automatically registers the current dynamic node as a dependent of
    /// this signal for precise reactive updates.
    ///
    /// # Returns
    ///
    /// - `T: Clone + PartialEq + 'static` - The current value of the signal.
    pub fn get(&self) -> T {
        let inner: &mut SignalInner<T> = Self::inner_mut(self.get_inner());
        if !inner.get_alive() {
            return inner.get_value().clone();
        }
        let tracking_id: usize = CURRENT_TRACKING_DYNAMIC_ID.load(Ordering::Relaxed);
        if tracking_id != usize::MAX {
            self.add_dependent(tracking_id);
        }
        inner.get_value().clone()
    }

    /// Subscribes a callback to be invoked when the signal changes.
    ///
    /// # Arguments
    ///
    /// - `FnMut() + 'static` - The callback to invoke when the signal changes.
    pub fn subscribe<F>(&self, callback: F)
    where
        F: FnMut() + 'static,
    {
        Self::inner_mut(self.get_inner())
            .get_mut_listeners()
            .push(Box::new(callback));
    }

    /// Replaces all listeners with a single new callback.
    ///
    /// Unlike `subscribe`, which appends a listener, this method clears any
    /// existing listeners first and then adds the new one.
    ///
    /// # Arguments
    ///
    /// - `FnMut() + 'static` - The callback to invoke when the signal changes.
    pub(crate) fn replace_listener<F>(&self, callback: F)
    where
        F: FnMut() + 'static,
    {
        let inner: &mut SignalInner<T> = Self::inner_mut(self.get_inner());
        inner.get_mut_listeners().clear();
        inner.get_mut_listeners().push(Box::new(callback));
        inner.set_listeners_replaced(true);
    }

    /// Detaches this signal from the reactive system without freeing memory.
    ///
    /// Marks the signal inactive and clears its listeners and dependents, but
    /// intentionally keeps the heap allocation alive.
    ///
    /// This is the only supported teardown path for a signal, and is used by
    /// both DOM-bound subscribe closures (when their node is removed) and the
    /// `use_signal` hook cleanup (when a component unmounts or a `match` arm
    /// switches). Freeing the allocation is deliberately never done at these
    /// points because `Signal<T>` is `Copy` (just a `usize` address): async
    /// callbacks (`spawn_local` futures, `setTimeout` / `setInterval`
    /// closures, Promise continuations) may still hold copies of the signal,
    /// and freeing would turn their later `.get()` / `.set()` calls into a
    /// use-after-free. Deactivating instead makes those stale calls safe
    /// no-ops.
    ///
    /// The allocation remains valid until the page unloads. For SPAs this is
    /// acceptable; a long-lived app could add a periodic sweep that frees
    /// `alive == false` entries once no async references remain. This mirrors
    /// the contract documented on `clear_signal_listeners`.
    pub(crate) fn deactivate(&self) {
        let inner: &mut SignalInner<T> = Self::inner_mut(self.get_inner());
        inner.set_alive(false);
        inner.get_mut_listeners().clear();
        inner.get_mut_dependents().clear();
    }

    /// Core implementation of value update and listener notification.
    ///
    /// Returns `true` if the value was updated and listeners were notified.
    /// Returns `false` if the signal is inactive or the value is unchanged.
    ///
    /// Uses a swap-out pattern for listeners: moves all listeners into a local
    /// `Vec`, drops the mutable reference to inner state, then invokes each
    /// listener. After invocation, listeners are moved back. This prevents
    /// issues with re-entrant access during listener callbacks.
    fn update(&self, value: T) -> bool {
        let inner: &mut SignalInner<T> = Self::inner_mut(self.get_inner());
        if !inner.get_alive() {
            return false;
        }
        if *inner.get_value() == value {
            return false;
        }
        inner.set_value(value);
        inner.set_listeners_replaced(false);
        let mut listeners: Vec<Box<dyn FnMut()>> = Vec::new();
        swap(inner.get_mut_listeners(), &mut listeners);
        for listener in listeners.iter_mut() {
            listener();
        }
        if !Self::is_alive(self.get_inner()) {
            return true;
        }
        let inner: &mut SignalInner<T> = Self::inner_mut(self.get_inner());
        if inner.get_alive() {
            if inner.get_listeners_replaced() {
                inner.set_listeners_replaced(false);
            } else {
                let new_listeners: &mut Vec<Box<dyn FnMut()>> = inner.get_mut_listeners();
                if new_listeners.is_empty() {
                    swap(new_listeners, &mut listeners);
                } else {
                    listeners.append(new_listeners);
                    swap(new_listeners, &mut listeners);
                }
            }
        }
        true
    }

    /// Registers a dynamic node ID as a dependent of this signal.
    ///
    /// When this signal changes, only its registered dependents will be
    /// marked dirty for re-rendering, enabling precise updates instead
    /// of broadcasting to all dynamic nodes.
    ///
    /// # Arguments
    ///
    /// - `usize` - The dynamic node ID to register as a dependent.
    pub(crate) fn add_dependent(&self, dynamic_id: usize) {
        let deps: &mut Vec<usize> = Self::inner_mut(self.get_inner()).get_mut_dependents();
        if !deps.contains(&dynamic_id) {
            deps.push(dynamic_id);
        }
    }

    /// Returns the list of dependent dynamic node IDs for this signal.
    ///
    /// # Returns
    ///
    /// - `Vec<usize>` - Clone of the dependents list.
    pub(crate) fn get_dependents(&self) -> Vec<usize> {
        Self::inner_mut(self.get_inner()).get_dependents().clone()
    }

    /// Sets the value of the signal and notifies listeners.
    ///
    /// Uses precise dirty marking: only dynamic nodes that depend on
    /// this signal are marked dirty, avoiding full broadcast.
    ///
    /// When called inside `batch`, the dispatch is
    /// deferred (dirty slots are still marked precisely), and the
    /// outermost `set()` call outside the suppressed scope will
    /// trigger the actual dispatch cycle.
    ///
    /// # Arguments
    ///
    /// - `T: Clone + PartialEq + 'static` - The new value to assign to the signal.
    pub fn set(&self, value: T) {
        if self.update(value) {
            let dependents: Vec<usize> = self.get_dependents();
            App::schedule_update(&dependents);
        }
    }

    /// Retrieves a mutable pointer to `SignalInner<T>` directly from the
    /// signal's stored address.
    ///
    /// SAFETY: The address stored in `Signal::inner` is always a valid pointer
    /// to a `SignalInner<T>` that is kept alive by the global registry. Since
    /// WASM is single-threaded, the pointer is always valid as long as the
    /// signal has not been explicitly freed.
    fn inner_mut(addr: usize) -> &'static mut SignalInner<T> {
        unsafe { &mut *(addr as *mut SignalInner<T>) }
    }

    /// Returns whether the signal allocation at `addr` is still present in
    /// the global registry (i.e. has not been freed).
    fn is_alive(addr: usize) -> bool {
        Self::registry().contains(&addr)
    }
}

/// Provides a safe default for `Signal<T>` by creating a valid signal
/// initialized with `T::default()`.
///
/// This prevents the creation of invalid signals with `inner = 0` (null
/// pointer), which would cause a panic when `.get()` is called.
///
/// # Returns
///
/// - `Self` - A valid signal initialized with `T::default()`.
impl<T> Default for Signal<T>
where
    T: Clone + Default + PartialEq + 'static,
{
    fn default() -> Self {
        Self::create(T::default())
    }
}

/// Clones the signal, sharing the same inner state.
///
/// Since `Signal` is `Copy`, this simply returns `*self`.
///
/// # Returns
///
/// - `Self` - A copy of the signal handle sharing the same inner state.
impl<T> Clone for Signal<T>
where
    T: Clone + PartialEq + 'static,
{
    fn clone(&self) -> Self {
        *self
    }
}

/// Copies the signal, sharing the same inner state.
///
/// Safe because only the inner address (a `usize`) is copied;
/// the actual heap allocation is owned by the global signal registry.
impl<T> Copy for Signal<T> where T: Clone + PartialEq + 'static {}

/// Marks `SignalCell` as `Sync` for single-threaded WASM contexts.
///
/// SAFETY: `SignalCell` is only used in single-threaded WASM contexts.
/// Concurrent access from multiple threads would be undefined behavior.
unsafe impl<T> Sync for SignalCell<T> where T: Clone + PartialEq + 'static {}

/// Implementation of SignalCell construction and access.
impl<T> SignalCell<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Creates a new `SignalCell` with no signal stored.
    ///
    /// # Returns
    ///
    /// - `Self` - An empty `SignalCell` with `None` stored in the inner `UnsafeCell`.
    pub const fn none() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }

    /// Stores a signal into the cell.
    ///
    /// # Arguments
    ///
    /// - `Signal<T>` - The signal to store.
    ///
    /// # Panics
    ///
    /// Panics if a signal has already been stored.
    pub fn set(&self, signal: Signal<T>) {
        unsafe {
            let ptr: &mut Option<Signal<T>> = &mut *self.get_inner().get();
            if ptr.is_some() {
                panic!("SignalCell::set called on an already-initialized cell");
            }
            *ptr = Some(signal);
        }
    }

    /// Returns the signal stored in the cell.
    ///
    /// # Returns
    ///
    /// - `Signal<T>` - The stored signal.
    ///
    /// # Panics
    ///
    /// Panics if no signal has been stored via `set`.
    pub fn get(&self) -> Signal<T> {
        unsafe {
            let ptr: &Option<Signal<T>> = &*self.get_inner().get();
            match ptr {
                Some(signal) => *signal,
                None => panic!("SignalCell::get called on an uninitialized cell"),
            }
        }
    }
}

/// Provides a default empty `SignalCell`.
///
/// Creates a `SignalCell` with `None` stored in the inner `UnsafeCell`.
///
/// # Returns
///
/// - `Self` - An empty `SignalCell` with no signal stored.
impl<T> Default for SignalCell<T>
where
    T: Clone + PartialEq + 'static,
{
    fn default() -> Self {
        Self::new(UnsafeCell::new(None))
    }
}

/// Marks `SignalInnerRegistryCell` as `Sync` for single-threaded WASM contexts.
///
/// SAFETY: `SignalInnerRegistryCell` is only used in single-threaded WASM contexts.
/// Concurrent access from multiple threads would be undefined behavior.
unsafe impl Sync for SignalInnerRegistryCell {}

/// String-specific signal operations.
impl Signal<String> {
    /// Clears DOM-binding listeners on a bridge signal identified by its inner
    /// pointer address, deactivates the bridge signal, and releases its value
    /// memory.
    ///
    /// This function is used during DOM cleanup (`cleanup_dom_subtree`) to
    /// release bridge `Signal<String>` instances that are no longer needed.
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
    pub(crate) fn clear_listeners(addr: usize) {
        let inner: &mut SignalInner<String> = Self::inner_mut(addr);
        inner.get_mut_listeners().clear();
        inner.set_alive(false);
        inner.set_value(String::new());
        Registry::cleanup_attr_slot(addr);
    }
}

/// Implementation of `FireHandle` construction, invocation, and conversions.
impl FireHandle {
    /// Leaks the given closure and returns a handle pointing to its heap address.
    ///
    /// The closure is double-boxed (`Box<Box<dyn FnMut()>>`) and leaked so the
    /// inner box's address remains stable for the lifetime of the program.
    /// The address is captured as a `usize` and wrapped in a `FireHandle`.
    ///
    /// # Arguments
    ///
    /// - `F: FnMut() + 'static` - The fire closure to leak.
    ///
    /// # Returns
    ///
    /// - `FireHandle` - A handle holding the leaked closure's address.
    pub fn new<F>(fire: F) -> Self
    where
        F: FnMut() + 'static,
    {
        let leaked: &'static mut Box<dyn FnMut()> =
            Box::leak(Box::new(Box::new(fire) as Box<dyn FnMut()>));
        let addr: usize = leaked as *mut Box<dyn FnMut()> as usize;
        let mut handle: Self = Self { inner: 0 };
        handle.set_inner(addr);
        handle
    }

    /// Invokes the closure pointed to by this handle.
    ///
    /// Takes `self` by value because `FireHandle: Copy` — repeated invocations
    /// on a single captured handle each copy the address and operate on the
    /// same underlying closure.
    ///
    /// # Safety
    ///
    /// The handle must come from `FireHandle::new` (or `From`) and the
    /// underlying boxed closure must still be live.
    pub unsafe fn fire(self) {
        unsafe { Self::fire_at(self.get_inner()) };
    }

    /// Invokes the closure stored at the given address.
    ///
    /// This is the static counterpart of `fire` for call sites that have
    /// only the raw `usize` address (e.g., macro-generated code that
    /// captures the address by `move` into a subscribe closure).
    ///
    /// # Arguments
    ///
    /// - `usize` - The address of a leaked `Box<dyn FnMut()>`.
    ///
    /// # Safety
    ///
    /// `addr` must come from a valid `FireHandle` produced by `new` (or
    /// `From`) and the underlying boxed closure must still be live.
    pub unsafe fn fire_at(addr: usize) {
        let ptr: *mut Box<dyn FnMut()> = addr as *mut Box<dyn FnMut()>;
        unsafe { (&mut *ptr)() };
    }
}

/// Leaks a fire closure into a `FireHandle`.
///
/// This is the canonical `Into` path used by `watch!`/`computed!` macros
/// and the virtual list component to obtain a `FireHandle` from a closure.
impl<F> From<F> for FireHandle
where
    F: FnMut() + 'static,
{
    /// Leaks this closure and stores its address in the returned handle.
    ///
    /// # Returns
    ///
    /// - `FireHandle` - A handle holding the leaked closure's address.
    fn from(fire: F) -> Self {
        Self::new(fire)
    }
}

/// Extracts the raw address from a `FireHandle`.
///
/// This is used by macro-generated code that needs to capture the address
/// (a `Copy` type) into `FnMut() + 'static` subscribe closures.
impl From<FireHandle> for usize {
    /// Returns the leaked closure's heap address.
    ///
    /// # Returns
    ///
    /// - `usize` - The address held by this handle.
    fn from(handle: FireHandle) -> Self {
        handle.get_inner()
    }
}
