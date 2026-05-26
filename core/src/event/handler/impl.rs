use crate::*;

/// Implementation of event handler construction and invocation.
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
    pub fn create<F>(event_name: NativeEventName, callback: F) -> Self
    where
        F: FnMut(Event) + 'static,
    {
        let callback_inner: Rc<RefCell<NativeEventCallbackInner>> = Rc::new(RefCell::new(
            NativeEventCallbackInner::new(Box::new(callback)),
        ));
        Self::new(event_name.to_string(), callback_inner)
    }

    /// Invokes the underlying callback with the given event.
    ///
    /// # Arguments
    ///
    /// - `Event` - The event to pass to the callback.
    pub fn handle(&self, event: Event) {
        let mut inner: RefMut<NativeEventCallbackInner> = self.get_callback().borrow_mut();
        (inner.get_mut_callback())(event);
    }
}
