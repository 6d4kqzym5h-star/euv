use crate::*;

/// Implementation of event handler construction and invocation.
impl NativeEventHandler {
    /// Creates a new event handler from a static event name string and callback.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name (e.g., "click", "input", "hashchange").
    /// - `FnMut(Event) + 'static` - The callback to invoke when the event fires.
    ///
    /// # Returns
    ///
    /// - `Self` - A new event handler.
    pub fn create<F>(event_name: &'static str, callback: F) -> Self
    where
        F: FnMut(Event) + 'static,
    {
        let callback_inner: Rc<RefCell<NativeEventCallbackInner>> = Rc::new(RefCell::new(
            NativeEventCallbackInner::new(Box::new(callback)),
        ));
        Self::new(event_name, callback_inner)
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
