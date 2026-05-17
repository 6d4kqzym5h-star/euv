use crate::*;

/// Implementation of event handler construction, cloning, and invocation.
impl NativeEventHandler {
    /// Creates a new event handler from an `NativeEventName` enum and callback.
    ///
    /// # Arguments
    ///
    /// - `NativeEventName` - The event name enum variant.
    /// - `FnMut(NativeEvent) + 'static` - The callback to invoke when the event fires.
    ///
    /// # Returns
    ///
    /// - `Self` - A new event handler.
    pub fn new<F>(event_name: NativeEventName, callback: F) -> Self
    where
        F: FnMut(NativeEvent) + 'static,
    {
        NativeEventHandler {
            event_name: event_name.as_str().into_owned(),
            callback: Rc::new(RefCell::new(callback)),
        }
    }

    /// Invokes the underlying callback with the given event.
    ///
    /// # Arguments
    ///
    /// - `NativeEvent` - The event to pass to the callback.
    pub fn handle(&self, event: NativeEvent) {
        let mut cb: RefMut<dyn FnMut(NativeEvent)> = self.get_callback().borrow_mut();
        cb(event);
    }
}

/// Clones the event handler, sharing the underlying callback reference.
impl Clone for NativeEventHandler {
    /// Returns a clone of this handler sharing the same callback `Rc`.
    fn clone(&self) -> Self {
        NativeEventHandler {
            event_name: self.get_event_name().clone(),
            callback: Rc::clone(self.get_callback()),
        }
    }
}
