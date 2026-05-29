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

    /// Returns the raw inner pointer address for identity comparison.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the inner `Rc`.
    pub(crate) fn get_inner_addr(&self) -> usize {
        self.get_inner()
    }

    /// Returns a reference to the inner `RefCell` for this signal.
    ///
    /// # Returns
    ///
    /// - `&'static RefCell<SignalInner<T>>` - A reference to the inner state.
    #[inline(always)]
    fn inner_ref(&self) -> &'static RefCell<SignalInner<T>> {
        get_signal_inner_ref(self.get_inner())
    }

    /// Returns the current value of the signal.
    ///
    /// # Returns
    ///
    /// - `T` - The current value of the signal.
    #[inline]
    pub fn get(&self) -> T {
        self.inner_ref().borrow().get_value().clone()
    }

    /// Attempts to return the current value of the signal without panicking.
    ///
    /// Unlike `get`, this method uses `try_borrow` and returns `None` if the
    /// inner `RefCell` is already mutably borrowed, avoiding a panic.
    ///
    /// # Returns
    ///
    /// - `Some(T)` - The current value if the borrow succeeds.
    /// - `None` - If the inner value is already mutably borrowed.
    pub fn try_get(&self) -> Option<T> {
        self.inner_ref()
            .try_borrow()
            .ok()
            .map(|inner: Ref<SignalInner<T>>| inner.get_value().clone())
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
        self.inner_ref()
            .borrow_mut()
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
    pub(crate) fn replace_subscribe<F>(&self, callback: F)
    where
        F: FnMut() + 'static,
    {
        let mut inner: RefMut<SignalInner<T>> = self.inner_ref().borrow_mut();
        let listeners: &mut Vec<Box<dyn FnMut()>> = inner.get_mut_listeners();
        listeners.clear();
        listeners.push(Box::new(callback));
    }

    /// Removes all subscribed listeners from this signal and marks it as
    /// inactive.
    pub(crate) fn clear_listeners(&self) {
        let mut inner: RefMut<SignalInner<T>> = self.inner_ref().borrow_mut();
        inner.set_alive(false);
        inner.get_mut_listeners().clear();
    }

    /// Core implementation of value update and listener notification.
    ///
    /// Returns `true` if the value was updated and listeners were notified.
    #[inline]
    fn update_and_notify(&self, value: T) -> bool {
        let inner_ref: &RefCell<SignalInner<T>> = self.inner_ref();
        let mut listeners: Vec<Box<dyn FnMut()>> = Vec::new();
        {
            let mut inner: RefMut<SignalInner<T>> = inner_ref.borrow_mut();
            if !inner.get_alive() {
                return false;
            }
            if *inner.get_value() == value {
                return false;
            }
            inner.set_value(value);
            swap(inner.get_mut_listeners(), &mut listeners);
        }
        for listener in listeners.iter_mut() {
            listener();
        }
        {
            let mut inner: RefMut<SignalInner<T>> = inner_ref.borrow_mut();
            swap(inner.get_mut_listeners(), &mut listeners);
        }
        true
    }

    /// Sets the value of the signal and notifies listeners.
    ///
    /// # Arguments
    ///
    /// - `T` - The new value to assign to the signal.
    #[inline]
    pub fn set(&self, value: T) {
        if self.update_and_notify(value) {
            schedule_signal_update();
        }
    }

    /// Sets the value of the signal and notifies listeners without scheduling
    /// a global DOM update dispatch.
    ///
    /// # Arguments
    ///
    /// - `T` - The new value to assign to the signal.
    #[inline]
    pub fn set_silent(&self, value: T) {
        self.update_and_notify(value);
    }

    /// Sets the value of the signal without notifying listeners or scheduling
    /// a DOM update. This is useful for breaking circular watch dependencies
    /// where two signals watch each other and would otherwise recurse infinitely.
    ///
    /// # Arguments
    ///
    /// - `T` - The new value to assign to the signal.
    #[inline]
    pub fn set_untracked(&self, value: T) {
        let inner_ref: &RefCell<SignalInner<T>> = self.inner_ref();
        let mut inner: RefMut<SignalInner<T>> = inner_ref.borrow_mut();
        inner.set_value(value);
    }

    /// Attempts to set the value of the signal and notify listeners without panicking.
    ///
    /// Unlike `set`, this method uses `try_borrow_mut` and returns `false` if
    /// the inner `RefCell` is already borrowed, avoiding a panic.
    ///
    /// # Arguments
    ///
    /// - `T` - The new value to assign to the signal.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the value was successfully updated and listeners were notified, `false` if unchanged, inactive, or already borrowed.
    pub fn try_set(&self, value: T) -> bool {
        let inner_ref: &RefCell<SignalInner<T>> = self.inner_ref();
        let mut listeners: Vec<Box<dyn FnMut()>> = Vec::new();
        {
            let mut inner: RefMut<SignalInner<T>> = match inner_ref.try_borrow_mut() {
                Ok(inner) => inner,
                Err(_) => return false,
            };
            if !inner.get_alive() {
                return false;
            }
            if *inner.get_value() == value {
                return false;
            }
            inner.set_value(value);
            swap(inner.get_mut_listeners(), &mut listeners);
        }
        for listener in listeners.iter_mut() {
            listener();
        }
        {
            let mut inner: RefMut<SignalInner<T>> = inner_ref.borrow_mut();
            swap(inner.get_mut_listeners(), &mut listeners);
        }
        schedule_signal_update();
        true
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
