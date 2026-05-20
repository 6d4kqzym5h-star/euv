use crate::*;

/// Implementation of `From` trait for converting `usize` address into `&'static mut NativeEventCallbackInner`.
impl From<usize> for &'static mut NativeEventCallbackInner {
    /// Converts a memory address into a mutable reference to `NativeEventCallbackInner`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `NativeEventCallbackInner` instance.
    ///
    /// # Returns
    ///
    /// - `&'static mut NativeEventCallbackInner` - A mutable reference to the `NativeEventCallbackInner` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `NativeEventCallbackInner` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> Self {
        unsafe { &mut *(address as *mut NativeEventCallbackInner) }
    }
}

/// Implementation of `From` trait for converting `usize` address into `&'static NativeEventCallbackInner`.
impl From<usize> for &'static NativeEventCallbackInner {
    /// Converts a memory address into a reference to `NativeEventCallbackInner`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `NativeEventCallbackInner` instance.
    ///
    /// # Returns
    ///
    /// - `&'static NativeEventCallbackInner` - A reference to the `NativeEventCallbackInner` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `NativeEventCallbackInner` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> Self {
        unsafe { &*(address as *const NativeEventCallbackInner) }
    }
}

/// Implementation of `From` trait for converting `&NativeEventHandler` into `usize` address.
impl From<&NativeEventHandler> for usize {
    /// Converts a reference to `NativeEventHandler` into its callback pointer address.
    ///
    /// # Arguments
    ///
    /// - `&NativeEventHandler` - The reference to the event handler.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the callback pointer.
    #[inline(always)]
    fn from(handler: &NativeEventHandler) -> Self {
        handler.callback as usize
    }
}

/// Implementation of event handler construction, cloning, and invocation.
impl NativeEventHandler {
    /// Creates a new event handler from an `NativeEventName` enum and callback.
    ///
    /// # Arguments
    ///
    /// - `NativeEventName` - The event name enum variant.
    /// - `FnMut(Event) + 'static` - The callback to invoke when the event fires.
    ///
    /// # Returns
    ///
    /// - `Self` - A new event handler.
    pub fn new<F>(event_name: NativeEventName, callback: F) -> Self
    where
        F: FnMut(Event) + 'static,
    {
        let inner: Box<NativeEventCallbackInner> = Box::new(NativeEventCallbackInner {
            callback: Box::new(callback),
        });
        NativeEventHandler {
            event_name: event_name.as_str().into_owned(),
            callback: Box::leak(inner) as *mut NativeEventCallbackInner,
        }
    }

    /// Returns a mutable reference to the inner callback closure state by going
    /// through `usize` intermediate conversion.
    ///
    /// # Returns
    ///
    /// - `&'static mut NativeEventCallbackInner` - A mutable reference to the inner callback state.
    pub(crate) fn leak_mut(&self) -> &'static mut NativeEventCallbackInner {
        let address: usize = self.into();
        address.into()
    }

    /// Invokes the underlying callback with the given event.
    ///
    /// # Arguments
    ///
    /// - `Event` - The event to pass to the callback.
    pub fn handle(&self, event: Event) {
        let inner: &mut NativeEventCallbackInner = self.leak_mut();
        (inner.callback)(event);
    }
}
