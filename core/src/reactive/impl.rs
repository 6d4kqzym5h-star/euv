use crate::*;

/// Implementation of SignalInner construction.
impl<T> SignalInner<T>
where
    T: Clone,
{
    /// Creates a new signal inner with the given initial value and no listeners.
    pub fn new(value: T) -> Self {
        let inner: SignalInner<T> = SignalInner {
            value,
            listeners: Vec::new(),
        };
        inner
    }
}

/// Implementation of signal inner state notification.
impl<T> SignalInner<T>
where
    T: Clone,
{
    /// Notifies all subscribed listeners that the value has changed.
    pub fn notify(&mut self) {
        for listener in self.get_listeners() {
            let mut borrowed = listener.borrow_mut();
            borrowed();
        }
    }
}

/// Implementation of reactive signal operations.
impl<T> Signal<T>
where
    T: Clone + PartialEq,
{
    /// Creates a new `Signal` from an existing raw pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure the pointer was allocated via `Box::leak`
    /// and remains valid for the entire program lifetime.
    pub fn from_inner(inner: *mut SignalInner<T>) -> Self {
        Signal { inner }
    }

    /// Returns a mutable reference to the inner signal state.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other references to the inner state exist.
    /// In single-threaded WASM this is always safe.
    #[allow(clippy::mut_from_ref)]
    fn get_inner_mut(&self) -> &mut SignalInner<T> {
        unsafe { &mut *self.inner }
    }

    /// Returns the current value of the signal.
    pub fn get(&self) -> T {
        self.get_inner_mut().get_value().clone()
    }

    /// Attempts to return the current value of the signal without panicking.
    ///
    /// # Returns
    /// - `Some(T)`: The current value if the borrow succeeds.
    /// - `None`: If the inner value is already mutably borrowed.
    pub fn try_get(&self) -> Option<T> {
        Some(self.get_inner_mut().get_value().clone())
    }

    /// Subscribes a callback to be invoked when the signal changes.
    pub fn subscribe<F>(&self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.get_inner_mut()
            .get_mut_listeners()
            .push(Rc::new(RefCell::new(callback)));
    }

    /// Sets the value of the signal and notifies listeners.
    ///
    /// If the new value is equal to the current value, no update is performed
    /// and no listeners are notified. This prevents unnecessary re-renders and
    /// avoids cascading no-op updates through intermediate signal chains.
    pub fn set(&self, value: T) {
        let inner: &mut SignalInner<T> = self.get_inner_mut();
        if inner.get_value() == &value {
            return;
        }
        let listeners: ListenerList = {
            inner.set_value(value);
            inner.get_listeners().iter().map(Rc::clone).collect()
        };
        for listener in &listeners {
            let mut borrowed = listener.borrow_mut();
            borrowed();
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
    /// # When to use
    ///
    /// Prefer `set` in almost all cases. Only use `set_silent` when the
    /// signal change is guaranteed not to affect any DynamicNode output
    /// (e.g., internal guard flags, derived-value caches already at the
    /// correct value, or within a `with_suppressed_updates` block where
    /// the caller takes responsibility for batching the update).
    ///
    /// If the new value is equal to the current value, no update is performed
    /// and no listeners are notified.
    pub fn set_silent(&self, value: T) {
        let inner: &mut SignalInner<T> = self.get_inner_mut();
        if inner.get_value() == &value {
            return;
        }
        let listeners: ListenerList = {
            inner.set_value(value);
            inner.get_listeners().iter().map(Rc::clone).collect()
        };
        for listener in &listeners {
            let mut borrowed = listener.borrow_mut();
            borrowed();
        }
    }

    /// Attempts to set the value of the signal and notify listeners without panicking.
    ///
    /// If the new value is equal to the current value, no update is performed.
    ///
    /// # Arguments
    /// - `value`: The new value to assign to the signal.
    ///
    /// # Returns
    /// - `true`: If the value was successfully updated and listeners were notified.
    /// - `false`: If the value is unchanged and no update was performed.
    pub fn try_set(&self, value: T) -> bool {
        let inner: &mut SignalInner<T> = self.get_inner_mut();
        if inner.get_value() == &value {
            return false;
        }
        let listeners: ListenerList = {
            inner.set_value(value);
            inner.get_listeners().iter().map(Rc::clone).collect()
        };
        for listener in &listeners {
            listener.borrow_mut()();
        }
        schedule_signal_update();
        true
    }
}

/// Prevents direct dereference of a signal to enforce explicit API usage.
impl<T> Deref for Signal<T>
where
    T: Clone + PartialEq,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        panic!("Signal does not support direct dereference; use .get() instead");
    }
}

/// Prevents direct mutable dereference of a signal to enforce explicit API usage.
impl<T> DerefMut for Signal<T>
where
    T: Clone + PartialEq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        panic!("Signal does not support direct dereference; use .set() instead");
    }
}

/// Clones the signal, sharing the same inner state.
impl<T> Clone for Signal<T>
where
    T: Clone + PartialEq,
{
    fn clone(&self) -> Self {
        *self
    }
}

/// Copies the signal, sharing the same inner state.
///
/// A `Signal` is just a raw pointer; copying it is a trivial bitwise copy.
impl<T> Copy for Signal<T> where T: Clone + PartialEq {}

/// Converts a static `String` into a text attribute value.
impl IntoReactiveValue for String {
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Text(self)
    }
}

/// Converts a string slice into a text attribute value.
impl IntoReactiveValue for &str {
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Text(self.to_string())
    }
}

/// Converts a string signal into a reactive attribute value.
impl IntoReactiveValue for Signal<String> {
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Signal(self)
    }
}

/// Converts a mutable bool signal into a reactive attribute value.
///
/// The signal is mapped to a `Signal<String>` that yields `"true"` or `"false"`,
/// enabling boolean attributes like `checked` to reactively update the DOM.
impl IntoReactiveValue for Signal<bool> {
    fn into_reactive_value(self) -> AttributeValue {
        bool_signal_to_string_attribute_value(self)
    }
}

/// Converts a CSS class reference into an attribute value.
impl IntoReactiveValue for crate::vdom::CssClass {
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Css(self)
    }
}

/// Converts a reference to a CSS class into an attribute value by cloning.
impl IntoReactiveValue for &'static crate::vdom::CssClass {
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Css(self.clone())
    }
}

/// Converts a closure into a callback attribute value.
impl<F> IntoCallbackAttribute for F
where
    F: FnMut(NativeEvent) + 'static,
{
    fn into_callback_attribute(self) -> AttributeValue {
        AttributeValue::Event(NativeEventHandler::new(
            NativeEventName::Other("callback".to_string()),
            self,
        ))
    }
}

/// Converts an owned event handler into a callback attribute value.
impl IntoCallbackAttribute for NativeEventHandler {
    fn into_callback_attribute(self) -> AttributeValue {
        AttributeValue::Event(self)
    }
}

/// Converts an optional event handler into a callback attribute value.
impl IntoCallbackAttribute for Option<NativeEventHandler> {
    fn into_callback_attribute(self) -> AttributeValue {
        match self {
            Some(handler) => AttributeValue::Event(handler),
            None => AttributeValue::Text(String::new()),
        }
    }
}

/// Implementation of hook context lifecycle and hook index management.
impl HookContext {
    /// Creates a new `HookContext` from an existing raw pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure the pointer was allocated via `Box::leak`
    /// and remains valid for the entire program lifetime.
    pub fn from_inner(inner: *mut HookContextInner) -> Self {
        HookContext { inner }
    }

    /// Returns a mutable reference to the inner hook context state.
    ///
    /// # Safety
    ///
    /// The caller must ensure no other references to the inner state exist.
    /// In single-threaded WASM this is always safe.
    #[allow(clippy::mut_from_ref)]
    fn get_inner_mut(&self) -> &mut HookContextInner {
        unsafe { &mut *self.inner }
    }

    /// Returns the current hook index.
    pub fn get_hook_index(&self) -> usize {
        *self.get_inner_mut().get_hook_index()
    }

    /// Sets the hook index.
    pub fn set_hook_index(&mut self, index: usize) {
        self.get_inner_mut().set_hook_index(index);
    }

    /// Returns a reference to the hooks storage.
    pub fn get_hooks(&self) -> &Vec<Box<dyn Any>> {
        self.get_inner_mut().get_hooks()
    }

    /// Returns a mutable reference to the hooks storage.
    pub fn get_mut_hooks(&mut self) -> &mut Vec<Box<dyn Any>> {
        self.get_inner_mut().get_mut_hooks()
    }

    /// Resets the hook index for a new render cycle.
    pub fn reset_hook_index(&mut self) {
        self.set_hook_index(0_usize);
    }
}

/// Clones the hook context, sharing the same inner state.
impl Clone for HookContext {
    fn clone(&self) -> Self {
        *self
    }
}

/// Copies the hook context, sharing the same inner state.
///
/// A `HookContext` is just a raw pointer; copying it is a trivial bitwise copy.
impl Copy for HookContext {}

/// Provides a default empty hook context.
impl Default for HookContext {
    fn default() -> Self {
        let boxed: Box<HookContextInner> = Box::default();
        HookContext::from_inner(Box::leak(boxed) as *mut HookContextInner)
    }
}

/// Implementation of HookContextInner construction.
impl HookContextInner {
    /// Creates a new empty hook context inner.
    pub const fn new() -> Self {
        HookContextInner {
            hooks: Vec::new(),
            hook_index: 0_usize,
        }
    }
}

/// Provides a default empty hook context inner.
impl Default for HookContextInner {
    fn default() -> Self {
        Self::new()
    }
}

/// SAFETY: `HookContextCell` is only used in single-threaded WASM contexts.
/// Concurrent access from multiple threads would be undefined behavior.
unsafe impl Sync for HookContextCell {}
