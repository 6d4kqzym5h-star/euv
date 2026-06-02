use crate::*;

/// Implementation of reactive signal operations.
impl<T> Signal<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Creates a new `Signal` with the given initial value.
    ///
    /// # Arguments
    ///
    /// - `T` - The initial value of the signal.
    ///
    /// # Returns
    ///
    /// - `Self` - A handle to the newly created reactive signal.
    pub fn create(value: T) -> Self {
        let signal_inner: Rc<RefCell<SignalInner<T>>> =
            Rc::new(RefCell::new(SignalInner::new(value, Vec::new(), true)));
        let addr: usize = Rc::as_ptr(&signal_inner) as usize;
        signal_inner_registry_mut().insert(addr, signal_inner as Rc<dyn Any>);
        let mut signal: Self = Self::new(0, std::marker::PhantomData);
        signal.set_inner(addr);
        signal
    }

    /// Returns a reference to the inner `RefCell` for this signal.
    ///
    /// # Returns
    ///
    /// - `&'static RefCell<SignalInner<T>>` - A reference to the inner state.
    fn inner_ref(&self) -> &'static RefCell<SignalInner<T>> {
        get_signal_inner_ref(self.get_inner())
    }

    /// Returns the current value of the signal.
    ///
    /// Uses `try_borrow` to avoid panicking when the inner `RefCell` is
    /// already mutably borrowed (e.g., during a `set()` notification cycle).
    /// In that case, falls back to an unsafe direct read of the value,
    /// which is safe in single-threaded WASM contexts because the value
    /// itself is not being mutated at the point of read (only the `RefCell`
    /// borrow flag is contested).
    ///
    /// If a tracking context is active (i.e., a DynamicNode is being rendered),
    /// automatically registers the current dynamic node as a dependent of
    /// this signal for precise reactive updates.
    ///
    /// # Returns
    ///
    /// - `T` - The current value of the signal.
    pub fn get(&self) -> T {
        let tracking_id: usize = CURRENT_TRACKING_DYNAMIC_ID.load(Ordering::Relaxed);
        if tracking_id != usize::MAX {
            self.add_dependent(tracking_id);
        }
        let inner_ref: &RefCell<SignalInner<T>> = self.inner_ref();
        let Ok(inner) = inner_ref.try_borrow() else {
            return unsafe { (*inner_ref.as_ptr()).get_value().clone() };
        };
        inner.get_value().clone()
    }

    /// Subscribes a callback to be invoked when the signal changes.
    ///
    /// If the inner `RefCell` is already borrowed, this is a no-op to avoid
    /// panicking during re-entrant signal updates.
    ///
    /// # Arguments
    ///
    /// - `FnMut() + 'static` - The callback to invoke when the signal changes.
    pub fn subscribe<F>(&self, callback: F)
    where
        F: FnMut() + 'static,
    {
        if let Ok(mut inner) = self.inner_ref().try_borrow_mut() {
            inner.get_mut_listeners().push(Box::new(callback));
        }
    }

    /// Replaces all listeners with a single new callback.
    ///
    /// Unlike `subscribe`, which appends a listener, this method clears any
    /// existing listeners first and then adds the new one. If the inner
    /// `RefCell` is already borrowed, this is a no-op.
    ///
    /// # Arguments
    ///
    /// - `FnMut() + 'static` - The callback to invoke when the signal changes.
    pub(crate) fn replace_subscribe<F>(&self, callback: F)
    where
        F: FnMut() + 'static,
    {
        if let Ok(mut inner) = self.inner_ref().try_borrow_mut() {
            let listeners: &mut Vec<Box<dyn FnMut()>> = inner.get_mut_listeners();
            listeners.clear();
            listeners.push(Box::new(callback));
        }
    }

    /// Removes all subscribed listeners from this signal, clears its
    /// dependent dynamic node list, and marks it as inactive.
    /// If the inner `RefCell` is already borrowed, this is a no-op.
    ///
    /// NOTE: Does NOT remove from `SIGNAL_INNER_REGISTRY`. The registry
    /// must keep the `Rc` alive because `Signal` is `Copy` and other copies
    /// may still hold the raw address. Removing would cause use-after-free.
    pub(crate) fn clear_listeners(&self) {
        if let Ok(mut inner) = self.inner_ref().try_borrow_mut() {
            inner.set_alive(false);
            inner.get_mut_listeners().clear();
            inner.get_mut_dependents().clear();
        }
    }

    /// Core implementation of value update and listener notification.
    ///
    /// Returns `true` if the value was updated and listeners were notified.
    /// Returns `false` if the inner `RefCell` is already mutably borrowed
    /// (re-entrant access), the signal is inactive, or the value is unchanged.
    fn update_and_notify(&self, value: T) -> bool {
        let inner_ref: &RefCell<SignalInner<T>> = self.inner_ref();
        let mut listeners: Vec<Box<dyn FnMut()>> = Vec::new();
        let Ok(mut inner) = inner_ref.try_borrow_mut() else {
            return false;
        };
        if !inner.get_alive() {
            return false;
        }
        if *inner.get_value() == value {
            return false;
        }
        inner.set_value(value);
        swap(inner.get_mut_listeners(), &mut listeners);
        drop(inner);
        for mut listener in listeners {
            listener();
            if let Ok(mut inner) = inner_ref.try_borrow_mut() {
                inner.get_mut_listeners().push(listener);
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
        if let Ok(mut inner) = self.inner_ref().try_borrow_mut() {
            let deps: &mut Vec<usize> = inner.get_mut_dependents();
            if !deps.contains(&dynamic_id) {
                deps.push(dynamic_id);
            }
        }
    }

    /// Removes a dynamic node ID from the dependents list of this signal.
    ///
    /// Called during cleanup when a dynamic node is removed from the DOM
    /// and its dependency relationships need to be severed.
    ///
    /// # Arguments
    ///
    /// - `usize` - The dynamic node ID to remove.
    #[allow(dead_code)]
    pub(crate) fn remove_dependent(&self, dynamic_id: usize) {
        if let Ok(mut inner) = self.inner_ref().try_borrow_mut() {
            inner.get_mut_dependents().retain(|id| *id != dynamic_id);
        }
    }

    /// Returns the list of dependent dynamic node IDs for this signal.
    ///
    /// # Returns
    ///
    /// - `Vec<usize>` - Clone of the dependents list.
    pub(crate) fn get_dependents(&self) -> Vec<usize> {
        if let Ok(inner) = self.inner_ref().try_borrow() {
            inner.get_dependents().clone()
        } else {
            Vec::new()
        }
    }

    /// Sets the value of the signal and notifies listeners.
    ///
    /// Uses precise dirty marking: only dynamic nodes that depend on
    /// this signal are marked dirty, avoiding full broadcast.
    ///
    /// # Arguments
    ///
    /// - `T` - The new value to assign to the signal.
    pub fn set(&self, value: T) {
        if self.update_and_notify(value) {
            let dependents: Vec<usize> = self.get_dependents();
            schedule_signal_update_targeted(&dependents);
        }
    }

    /// Sets the value of the signal and notifies listeners without scheduling
    /// a global DOM update dispatch.
    ///
    /// # Arguments
    ///
    /// - `T` - The new value to assign to the signal.
    pub fn set_silent(&self, value: T) {
        self.update_and_notify(value);
    }

    /// Sets the value of the signal without notifying listeners or scheduling
    /// a DOM update. This is useful for breaking circular watch dependencies
    /// where two signals watch each other and would otherwise recurse infinitely.
    /// If the inner `RefCell` is already borrowed, this is a no-op.
    ///
    /// # Arguments
    ///
    /// - `T` - The new value to assign to the signal.
    pub fn set_untracked(&self, value: T) {
        let inner_ref: &RefCell<SignalInner<T>> = self.inner_ref();
        if let Ok(mut inner) = inner_ref.try_borrow_mut() {
            inner.set_value(value);
        }
    }
}

/// Clones the signal, sharing the same inner state.
///
/// Since `Signal` is `Copy`, this simply returns `*self`.
///
/// # Returns
///
/// - `Self`: A copy of the signal handle sharing the same inner state.
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
/// the actual `Rc` reference is held by the global signal registry.
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
    /// - `Self`: An empty `SignalCell` with `None` stored in the inner `UnsafeCell`.
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
/// - `Self`: An empty `SignalCell` with no signal stored.
impl<T> Default for SignalCell<T>
where
    T: Clone + PartialEq + 'static,
{
    fn default() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }
}

/// Marks `SignalInnerRegistryCell` as `Sync` for single-threaded WASM contexts.
///
/// SAFETY: `SignalInnerRegistryCell` is only used in single-threaded WASM contexts.
/// Concurrent access from multiple threads would be undefined behavior.
unsafe impl Sync for SignalInnerRegistryCell {}
