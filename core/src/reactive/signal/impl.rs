use crate::*;

/// Implementation of `From` trait for converting `usize` address into `&'static SignalInner<T>`.
impl<T> From<usize> for &'static mut SignalInner<T>
where
    T: Clone + 'static,
{
    /// Converts a memory address into a mutable reference to `SignalInner<T>`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `SignalInner<T>` instance.
    ///
    /// # Returns
    ///
    /// - `&'static mut SignalInner<T>` - A mutable reference to the `SignalInner<T>` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `SignalInner<T>` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> Self {
        unsafe { &mut *(address as *mut SignalInner<T>) }
    }
}

/// Implementation of `From` trait for converting `usize` address into `&'static SignalInner<T>`.
impl<T> From<usize> for &'static SignalInner<T>
where
    T: Clone + 'static,
{
    /// Converts a memory address into a reference to `SignalInner<T>`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `SignalInner<T>` instance.
    ///
    /// # Returns
    ///
    /// - `&'static SignalInner<T>` - A reference to the `SignalInner<T>` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `SignalInner<T>` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> Self {
        unsafe { &*(address as *const SignalInner<T>) }
    }
}

/// Implementation of `From` trait for converting `Signal<T>` into `usize` address.
impl<T> From<Signal<T>> for usize
where
    T: Clone + PartialEq + 'static,
{
    /// Converts a `Signal<T>` into its memory address.
    ///
    /// # Arguments
    ///
    /// - `Signal<T>` - The signal handle.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the signal's inner state.
    #[inline(always)]
    fn from(signal: Signal<T>) -> Self {
        signal.inner as usize
    }
}

/// Implementation of `From` trait for converting `usize` address into `Signal<T>`.
impl<T> From<usize> for Signal<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Converts a memory address into a `Signal<T>` handle.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address previously obtained from `Signal<T>` conversion.
    ///
    /// # Returns
    ///
    /// - `Signal<T>` - A signal handle wrapping the pointer at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `SignalInner<T>` instance
    ///   that was previously converted from a signal handle and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> Self {
        Signal {
            inner: address as *mut SignalInner<T>,
        }
    }
}

/// Implementation of SignalInner construction.
impl<T> SignalInner<T>
where
    T: Clone,
{
    /// Creates a new signal inner with the given initial value and no listeners.
    ///
    /// # Arguments
    ///
    /// - `T` - The initial value of the signal inner.
    ///
    /// # Returns
    ///
    /// - `Self` - A new signal inner with the given value and empty listeners.
    pub fn new(value: T) -> Self {
        let inner: SignalInner<T> = SignalInner {
            value,
            listeners: Vec::new(),
            alive: true,
        };
        inner
    }
}

/// Implementation of reactive signal operations.
impl<T> Signal<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Creates a new `Signal` with the given initial value.
    ///
    /// The inner state is allocated via `Box::leak` and lives for the
    /// remainder of the program. This is safe in single-threaded WASM
    /// contexts where no concurrent access can occur.
    ///
    /// # Arguments
    ///
    /// - `T` - The initial value of the signal.
    ///
    /// # Returns
    ///
    /// - `Self` - A handle to the newly created reactive signal.
    pub fn new(value: T) -> Self {
        let boxed: Box<SignalInner<T>> = Box::new(SignalInner::new(value));
        Signal {
            inner: Box::leak(boxed) as *mut SignalInner<T>,
        }
    }

    /// Returns a mutable reference to the inner signal state by going through
    /// `usize` intermediate conversion.
    ///
    /// # Returns
    ///
    /// - `&mut SignalInner<T>` - A mutable reference to the inner signal state.
    #[allow(clippy::mut_from_ref)]
    pub(crate) fn leak_mut(&self) -> &'static mut SignalInner<T> {
        let address: usize = (*self).into();
        address.into()
    }

    /// Returns the current value of the signal.
    ///
    /// # Returns
    ///
    /// - `T` - The current value of the signal.
    pub fn get(&self) -> T {
        self.leak_mut().get_value().clone()
    }

    /// Attempts to return the current value of the signal without panicking.
    ///
    /// # Returns
    ///
    /// - `Some(T)` - The current value if the borrow succeeds.
    /// - `None` - If the inner value is already mutably borrowed.
    pub fn try_get(&self) -> Option<T> {
        Some(self.leak_mut().get_value().clone())
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
        self.leak_mut().get_mut_listeners().push(Box::new(callback));
    }

    /// Replaces all listeners with a single new callback.
    ///
    /// Unlike `subscribe`, which appends a listener, this method clears any
    /// existing listeners first and then adds the new one. This prevents
    /// listener accumulation across re-renders: each signal is guaranteed
    /// to have at most one active listener at any time, eliminating
    /// cascading `set()` calls that would otherwise grow exponentially.
    ///
    /// # Arguments
    ///
    /// - `FnMut() + 'static` - The callback to invoke when the signal changes.
    pub fn replace_subscribe<F>(&self, callback: F)
    where
        F: FnMut() + 'static,
    {
        let listeners: &mut Vec<Box<dyn FnMut()>> = self.leak_mut().get_mut_listeners();
        listeners.clear();
        listeners.push(Box::new(callback));
    }

    /// Removes all subscribed listeners from this signal and marks it as
    /// inactive. After calling this method, subsequent `set()` and
    /// `try_set()` calls become complete no-ops: the value is not updated,
    /// no listeners are invoked, and `schedule_signal_update()` is not
    /// called. This is used during hook context cleanup when a `match`
    /// arm switch discards old signals, ensuring that stale `setInterval`
    /// closures referencing these signals become entirely harmless.
    pub fn clear_listeners(&self) {
        let inner: &mut SignalInner<T> = self.leak_mut();
        inner.set_alive(false);
        inner.get_mut_listeners().clear();
    }

    /// Sets the value of the signal and notifies listeners.
    ///
    /// If the signal has been marked as inactive (via `clear_listeners()`),
    /// this method is a complete no-op: the value is not updated, no
    /// listeners are invoked, and no global update is scheduled.
    ///
    /// If the new value is equal to the current value, no update is performed
    /// and no listeners are notified. This prevents unnecessary re-renders and
    /// avoids cascading no-op updates through intermediate signal chains.
    ///
    /// # Arguments
    ///
    /// - `T` - The new value to assign to the signal.
    pub fn set(&self, value: T) {
        let inner: &mut SignalInner<T> = self.leak_mut();
        if !inner.get_alive() {
            return;
        }
        if inner.get_value() == &value {
            return;
        }
        inner.set_value(value);
        for listener in inner.get_mut_listeners().iter_mut() {
            listener();
        }
        schedule_signal_update();
    }

    /// Sets the value of the signal and notifies listeners without scheduling
    /// a global DOM update dispatch.
    ///
    /// This is identical to `set` except it does not call
    /// `schedule_signal_update()`, meaning no `__euv_signal_update__` event
    /// will be dispatched. Use this for internal bookkeeping signals whose
    /// changes should not trigger DynamicNode re-renders.
    ///
    /// If the new value is equal to the current value, no update is performed
    /// and no listeners are notified.
    ///
    /// If the signal has been marked as inactive (via `clear_listeners()`),
    /// this method is a complete no-op.
    ///
    /// # Arguments
    ///
    /// - `T` - The new value to assign to the signal.
    pub fn set_silent(&self, value: T) {
        let inner: &mut SignalInner<T> = self.leak_mut();
        if !inner.get_alive() {
            return;
        }
        if inner.get_value() == &value {
            return;
        }
        inner.set_value(value);
        for listener in inner.get_mut_listeners().iter_mut() {
            listener();
        }
    }

    /// Attempts to set the value of the signal and notify listeners without panicking.
    ///
    /// If the new value is equal to the current value, no update is performed.
    ///
    /// # Arguments
    ///
    /// - `T` - The new value to assign to the signal.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the value was successfully updated and listeners were notified, `false` if unchanged or inactive.
    pub fn try_set(&self, value: T) -> bool {
        let inner: &mut SignalInner<T> = self.leak_mut();
        if !inner.get_alive() {
            return false;
        }
        if inner.get_value() == &value {
            return false;
        }
        inner.set_value(value);
        for listener in inner.get_mut_listeners().iter_mut() {
            listener();
        }
        schedule_signal_update();
        true
    }
}

/// Prevents direct dereference of a signal to enforce explicit API usage.
impl<T> Deref for Signal<T>
where
    T: Clone + PartialEq + 'static,
{
    type Target = T;

    /// Panics with a message directing the caller to use `.get()` instead.
    fn deref(&self) -> &Self::Target {
        panic!("Signal does not support direct dereference; use .get() instead");
    }
}

/// Prevents direct mutable dereference of a signal to enforce explicit API usage.
impl<T> DerefMut for Signal<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Panics with a message directing the caller to use `.set()` instead.
    fn deref_mut(&mut self) -> &mut Self::Target {
        panic!("Signal does not support direct dereference; use .set() instead");
    }
}

/// Clones the signal, sharing the same inner state.
impl<T> Clone for Signal<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Returns a bitwise copy of this signal.
    fn clone(&self) -> Self {
        *self
    }
}

/// Copies the signal, sharing the same inner state.
///
/// A `Signal` is just a raw pointer; copying it is a trivial bitwise copy.
impl<T> Copy for Signal<T> where T: Clone + PartialEq + 'static {}

/// SAFETY: `SignalCell` is only used in single-threaded WASM contexts.
/// Concurrent access from multiple threads would be undefined behavior.
unsafe impl<T> Sync for SignalCell<T> where T: Clone + PartialEq + 'static {}

/// Implementation of SignalCell construction and access.
impl<T> SignalCell<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Creates a new empty `SignalCell` with no signal stored.
    ///
    /// # Returns
    ///
    /// - `Self` - An empty cell ready to hold a signal.
    pub const fn new() -> Self {
        SignalCell {
            inner: UnsafeCell::new(None),
        }
    }

    /// Returns a raw pointer to the inner `Option<Signal<T>>`.
    ///
    /// # Returns
    ///
    /// - `*mut Option<Signal<T>>` - The raw pointer to the inner option.
    pub(crate) fn get_inner(&self) -> *mut Option<Signal<T>> {
        self.inner.get()
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
            let ptr: &mut Option<Signal<T>> = &mut *self.get_inner();
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
            let ptr: &Option<Signal<T>> = &*self.get_inner();
            match ptr {
                Some(signal) => *signal,
                None => panic!("SignalCell::get called on an uninitialized cell"),
            }
        }
    }
}

/// Provides a default empty `SignalCell`.
impl<T> Default for SignalCell<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Returns a default `SignalCell` by delegating to `SignalCell::new`.
    fn default() -> Self {
        Self::new()
    }
}
