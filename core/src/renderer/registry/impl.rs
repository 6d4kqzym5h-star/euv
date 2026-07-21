use super::*;

/// SAFETY: `HandlerRegistryCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for HandlerRegistryCell {}

/// SAFETY: `DelegatedEventsCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for DelegatedEventsCell {}

/// SAFETY: `SignalUpdateRegistryCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for SignalUpdateRegistryCell {}

/// SAFETY: `WindowEventRegistryCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for WindowEventRegistryCell {}

/// Implementation of `From` trait for converting `usize` address into `&'static mut HandlerSlot`.
impl From<usize> for &'static mut HandlerSlot {
    /// Converts a memory address into a mutable reference to `HandlerSlot`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `HandlerSlot` instance.
    ///
    /// # Returns
    ///
    /// - `&'static mut HandlerSlot` - A mutable reference at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `HandlerSlot` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    fn from(address: usize) -> Self {
        unsafe { &mut *(address as *mut HandlerSlot) }
    }
}

/// Static methods for managing framework registries.
///
/// Provides centralized access to event delegation, signal updates, window events,
/// and DOM event handler registries. All methods are thread-safe for single-threaded
/// WASM contexts.
impl Registry {
    /// Returns a shared reference to the delegated events set.
    ///
    /// # Returns
    ///
    /// - `&'static HashSet<&'static str>` - A shared reference to the global set of delegated event names.
    #[allow(static_mut_refs)]
    pub(crate) fn get_delegated_events() -> &'static HashSet<&'static str> {
        unsafe { &*DELEGATED_EVENTS.deref().get_0().get() }
    }

    /// Returns a mutable reference to the delegated events set.
    ///
    /// # Returns
    ///
    /// - `&'static mut HashSet<&'static str>` - A mutable reference to the global set of delegated event names.
    #[allow(static_mut_refs)]
    pub(crate) fn get_mut_delegated_events() -> &'static mut HashSet<&'static str> {
        unsafe { &mut *DELEGATED_EVENTS.deref().get_0().get() }
    }

    /// Returns a shared reference to the signal update registry.
    ///
    /// # Returns
    ///
    /// - `&'static HashMap<usize, SignalUpdateEntry>` - A shared reference to the global signal update registry.
    #[allow(static_mut_refs)]
    pub(crate) fn get_update_registry() -> &'static HashMap<usize, SignalUpdateEntry> {
        unsafe { &*SIGNAL_UPDATE_REGISTRY.deref().get_0().get() }
    }

    /// Returns a mutable reference to the signal update registry.
    ///
    /// # Returns
    ///
    /// - `&'static mut HashMap<usize, SignalUpdateEntry>` - A mutable reference to the global signal update registry.
    #[allow(static_mut_refs)]
    pub(crate) fn get_mut_update_registry() -> &'static mut HashMap<usize, SignalUpdateEntry> {
        unsafe { &mut *SIGNAL_UPDATE_REGISTRY.deref().get_0().get() }
    }

    /// Returns a shared reference to the window event registry.
    ///
    /// # Returns
    ///
    /// - `&'static WindowEventRegistryMap` - A shared reference to the global window event registry.
    #[allow(static_mut_refs)]
    pub(crate) fn get_window_registry() -> &'static WindowEventRegistryMap {
        unsafe { &*WINDOW_EVENT_REGISTRY.deref().get_0().get() }
    }

    /// Returns a mutable reference to the window event registry.
    ///
    /// # Returns
    ///
    /// - `&'static mut WindowEventRegistryMap` - A mutable reference to the global window event registry.
    #[allow(static_mut_refs)]
    pub(crate) fn get_mut_window_registry() -> &'static mut WindowEventRegistryMap {
        unsafe { &mut *WINDOW_EVENT_REGISTRY.deref().get_0().get() }
    }

    /// Returns a shared reference to the handler registry.
    ///
    /// # Returns
    ///
    /// - `&'static HandlerRegistryMap` - A shared reference to the global handler registry.
    #[allow(static_mut_refs)]
    pub(crate) fn get_handler_registry() -> &'static HandlerRegistryMap {
        unsafe { &*HANDLER_REGISTRY.deref().get_0().get() }
    }

    /// Returns a mutable reference to the handler registry.
    ///
    /// # Returns
    ///
    /// - `&'static mut HandlerRegistryMap` - A mutable reference to the global handler registry.
    #[allow(static_mut_refs)]
    pub(crate) fn get_mut_handler_registry() -> &'static mut HandlerRegistryMap {
        unsafe { &mut *HANDLER_REGISTRY.deref().get_0().get() }
    }

    /// Dispatches a delegated event by walking up from `event.target` to
    /// find the nearest element with a `data-euv-id` attribute, then
    /// invoking the matching handler from the global registry.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The DOM event to dispatch.
    /// - `&'static str` - The event name (e.g., "click", "input").
    fn dispatch_delegated_event(event: &Event, event_name: &'static str) {
        let target: EventTarget = match event.target() {
            Some(event_target) => event_target,
            None => return,
        };
        let mut current: Option<Element> = target.dyn_ref::<Element>().cloned().or_else(|| {
            target
                .dyn_ref::<Node>()
                .and_then(|node: &Node| node.parent_node())
                .and_then(|parent: Node| parent.dyn_ref::<Element>().cloned())
        });
        while let Some(element) = current {
            if let Some(euv_id_str) = element.get_attribute(DATA_EUV_ID)
                && let Ok(euv_id) = euv_id_str.parse::<usize>()
            {
                let handler_found: Option<NativeEventHandler> = Self::get_handler_registry()
                    .get(&euv_id)
                    .and_then(|event_map: &HashMap<&'static str, HandlerEntry>| {
                        event_map.get(event_name)
                    })
                    .and_then(|entry: &HandlerEntry| {
                        let slot: &HandlerSlot = unsafe { &**entry };
                        slot.try_get_handler().as_ref().cloned()
                    });
                if let Some(active_handler) = handler_found {
                    active_handler.handle(event.clone());
                    return;
                }
            }
            current = element.parent_element();
        }
    }

    /// Ensures a global capturing-phase listener is registered on `window`
    /// for the given event type.
    ///
    /// Uses event delegation to minimize the number of event listeners attached
    /// to the DOM. All events of the same type are handled by a single window-level
    /// listener that walks the DOM tree to find the appropriate handler.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name to delegate (e.g., "click", "input").
    pub(crate) fn delegation(event_name: &'static str) {
        if Self::is_delegated(event_name) {
            return;
        }
        let closure: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
            Self::dispatch_delegated_event(&event, event_name);
        }));
        let window: Window = match window() {
            Some(window_instance) => window_instance,
            None => return,
        };
        let _ = window.add_event_listener_with_callback_and_bool(
            event_name,
            closure.as_ref().unchecked_ref(),
            true,
        );
        closure.forget();
        Self::mark_delegated(event_name);
    }

    /// Marks the specified dynamic node IDs as dirty, scheduling them for re-render.
    ///
    /// Called when a signal changes to notify all dependent dynamic nodes
    /// that they need to update their DOM representation.
    ///
    /// # Arguments
    ///
    /// - `&[usize]` - The dynamic node IDs to mark as dirty.
    pub(crate) fn mark_dirty(dynamic_ids: &[usize]) {
        let registry: &mut HashMap<usize, SignalUpdateEntry> = Self::get_mut_update_registry();
        for dynamic_id in dynamic_ids {
            if let Some(entry) = registry.get(dynamic_id) {
                let slot: &mut SignalUpdateSlot = unsafe { &mut **entry };
                if !slot.get_removed() {
                    slot.set_dirty(true);
                }
            }
        }
    }

    /// Returns whether the signal update registry contains any dirty slots.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if at least one dynamic node is marked dirty and not removed.
    pub(crate) fn has_dirty() -> bool {
        Self::get_update_registry()
            .values()
            .any(|entry: &SignalUpdateEntry| {
                let slot: &SignalUpdateSlot = unsafe { &**entry };
                slot.get_dirty() && !slot.get_removed()
            })
    }

    /// Registers a signal update callback for a DynamicNode placeholder.
    ///
    /// Associates a re-render callback with a dynamic node ID so that when
    /// the node is marked dirty, the callback can be invoked to update the DOM.
    ///
    /// # Arguments
    ///
    /// - `usize` - The unique dynamic node ID.
    /// - `Box<dyn FnMut()>` - The callback to invoke when the node needs re-rendering.
    pub(crate) fn register_dynamic(dynamic_id: usize, callback: Box<dyn FnMut()>) {
        let slot: Box<SignalUpdateSlot> =
            Box::new(SignalUpdateSlot::new(Some(callback), false, true));
        let entry: SignalUpdateEntry = Box::into_raw(slot);
        if let Some(old_entry) = Self::get_mut_update_registry().insert(dynamic_id, entry) {
            unsafe {
                let _ = Box::from_raw(old_entry);
            }
        }
    }

    /// Registers a signal update callback for an attribute signal.
    ///
    /// Similar to `register_dynamic`, but for attribute-level signals that
    /// need to update DOM element attributes rather than entire subtrees.
    ///
    /// # Arguments
    ///
    /// - `usize` - The signal's inner address used as the registry key.
    /// - `Box<dyn FnMut()>` - The callback to invoke when the attribute needs updating.
    pub(crate) fn register_attr_listener(signal_key: usize, callback: Box<dyn FnMut()>) {
        let slot: Box<SignalUpdateSlot> =
            Box::new(SignalUpdateSlot::new(Some(callback), false, true));
        let entry: SignalUpdateEntry = Box::into_raw(slot);
        if let Some(old_entry) = Self::get_mut_update_registry().insert(signal_key, entry) {
            unsafe {
                let _ = Box::from_raw(old_entry);
            }
        }
    }

    /// Cleans up all handler entries associated with a DOM element.
    ///
    /// Removes all event handlers registered for the given element ID,
    /// detaching any direct event listeners from the DOM.
    ///
    /// # Arguments
    ///
    /// - `usize` - The element's unique `data-euv-id` value.
    pub(crate) fn cleanup_element(euv_id: usize) {
        let registry_ref: &mut HandlerRegistryMap = Self::get_mut_handler_registry();
        let Some(event_map) = registry_ref.remove(&euv_id) else {
            return;
        };
        for (event_name, entry) in event_map {
            let slot: &mut HandlerSlot = unsafe { &mut *entry };
            if let Some(element) = slot.try_get_element().as_ref().cloned()
                && let Some(listener_function) = slot.get_mut_listener_function().take()
            {
                let listener: &Function = listener_function.unchecked_ref::<Function>();
                let _ = element.remove_event_listener_with_callback(event_name, listener);
            }
            slot.set_handler(None);
            unsafe {
                let _ = Box::from_raw(entry);
            }
        }
    }

    /// Cleans up all resources associated with a DynamicNode when its
    /// placeholder element is removed from the DOM.
    ///
    /// Marks the slot as removed and clears its callback to prevent
    /// further updates to a detached DOM subtree.
    ///
    /// # Arguments
    ///
    /// - `usize` - The dynamic node's unique ID.
    pub(crate) fn cleanup_dynamic_node(dynamic_id: usize) {
        if let Some(entry) = Self::get_mut_update_registry().get(&dynamic_id) {
            let slot: &mut SignalUpdateSlot = unsafe { &mut **entry };
            slot.set_removed(true);
            slot.set_callback(None);
        }
    }

    /// Removes the signal update slot for an attribute signal from the registry.
    ///
    /// Marks the attribute slot as removed and clears its callback,
    /// preventing further updates to detached DOM elements.
    ///
    /// # Arguments
    ///
    /// - `usize` - The signal's inner address used as the registry key.
    pub(crate) fn cleanup_attr_slot(addr: usize) {
        if let Some(entry) = Self::get_mut_update_registry().get(&addr) {
            let slot: &mut SignalUpdateSlot = unsafe { &mut **entry };
            slot.set_removed(true);
            slot.set_callback(None);
        }
    }

    /// Returns whether the given event name is a non-bubbling event.
    ///
    /// Non-bubbling events (like "load", "error", "focus") must be attached
    /// directly to elements rather than using event delegation.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name to check.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the event does not bubble up the DOM tree.
    pub(crate) fn is_non_bubbling(event_name: &str) -> bool {
        NON_BUBBLING_EVENTS.contains(&event_name)
    }

    /// Returns whether the event name is already delegated.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name to check.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if a window-level listener already exists for this event type.
    pub(crate) fn is_delegated(event_name: &str) -> bool {
        Self::get_delegated_events().contains(event_name)
    }

    /// Marks an event name as delegated in the global set.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name to mark as delegated.
    pub(crate) fn mark_delegated(event_name: &'static str) {
        Self::get_mut_delegated_events().insert(event_name);
    }

    /// Registers a callback for a window-level event using the proxy pattern.
    ///
    /// Creates a shared window event listener that dispatches to all registered
    /// callbacks for the same event type. Returns a unique handler ID for later
    /// unregistration.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name to listen for (e.g., "resize", "hashchange").
    /// - `F: FnMut() + 'static` - The callback to invoke when the event fires.
    ///
    /// # Returns
    ///
    /// - `usize` - A unique handler ID that can be used to unregister the callback.
    pub(crate) fn register_window_event<F>(event_name: &str, callback: F) -> usize
    where
        F: FnMut() + 'static,
    {
        let handler_id: usize = NEXT_WINDOW_HANDLER_ID.fetch_add(1, Ordering::Relaxed);
        let boxed: Box<Box<dyn FnMut()>> = Box::new(Box::new(callback));
        let entry: WindowEventHandlerEntry = (handler_id, Box::into_raw(boxed));
        let registry: &mut WindowEventRegistryMap = Self::get_mut_window_registry();
        let is_new_event: bool = !registry.contains_key(event_name);
        registry
            .entry(event_name.to_string())
            .or_default()
            .push(entry);
        if is_new_event {
            Self::window_event_listener(event_name);
        }
        handler_id
    }

    /// Unregisters a window event handler by its event name and handler ID.
    ///
    /// Removes the callback from the registry and frees its memory.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name the handler was registered for.
    /// - `usize` - The handler ID returned by `register_window_event`.
    pub(crate) fn unregister_window_event(event_name: &str, handler_id: usize) {
        let registry: &mut WindowEventRegistryMap = Self::get_mut_window_registry();
        if let Some(handlers) = registry.get_mut(event_name) {
            handlers.retain(|(id, ptr): &WindowEventHandlerEntry| {
                if *id == handler_id {
                    unsafe {
                        let _ = Box::from_raw(*ptr);
                    }
                    false
                } else {
                    true
                }
            });
        }
    }

    /// Ensures a single `window.addEventListener` listener is registered
    /// for the given event name that dispatches to all registered callbacks.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name to register the listener for.
    fn window_event_listener(event_name: &str) {
        let event_name_owned: String = event_name.to_string();
        let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let handler_ids: Vec<usize> = match Self::get_window_registry().get(&event_name_owned) {
                Some(handlers) => handlers.iter().map(|(id, _ptr)| *id).collect(),
                None => return,
            };
            for handler_id in handler_ids {
                let callback_ptr: *mut Box<dyn FnMut() + 'static> =
                    match Self::get_window_registry().get(&event_name_owned) {
                        Some(handlers) => {
                            match handlers.iter().find(|(id, _ptr)| *id == handler_id) {
                                Some((_id, ptr)) => *ptr,
                                None => continue,
                            }
                        }
                        None => return,
                    };
                let callback: &mut Box<dyn FnMut() + 'static> = unsafe { &mut *callback_ptr };
                callback();
            }
        }));
        let window: Window = match window() {
            Some(window_instance) => window_instance,
            None => return,
        };
        let _ =
            window.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());
        closure.forget();
    }
}
