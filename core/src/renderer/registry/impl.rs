use crate::*;

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

/// SAFETY: `HandlerRegistryCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for HandlerRegistryCell {}

/// SAFETY: `DelegatedEventsCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for DelegatedEventsCell {}

/// SAFETY: `SignalUpdateRegistryCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for SignalUpdateRegistryCell {}

/// SAFETY: `WindowEventRegistryCell` is only used in single-threaded WASM contexts.
unsafe impl Sync for WindowEventRegistryCell {}

/// Static methods for managing framework registries.
impl Registry {
    /// Dispatches a delegated event by walking up from `event.target` to
    /// find the nearest element with a `data-euv-id` attribute, then
    /// invoking the matching handler from the global registry.
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
                let handler_found: Option<NativeEventHandler> = Self::ensure_handler_registry()
                    .get(&(euv_id, event_name))
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
    pub(crate) fn ensure_delegation(event_name: &'static str) {
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

    /// Registers a signal update callback for a DynamicNode placeholder.
    pub(crate) fn mark_dirty(dynamic_ids: &[usize]) {
        let registry: &mut HashMap<usize, SignalUpdateEntry> = Self::ensure_update_registry_mut();
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
    pub(crate) fn has_dirty() -> bool {
        Self::ensure_update_registry_mut()
            .values()
            .any(|entry: &SignalUpdateEntry| {
                let slot: &SignalUpdateSlot = unsafe { &**entry };
                slot.get_dirty() && !slot.get_removed()
            })
    }

    /// Registers a signal update callback for a DynamicNode placeholder.
    pub(crate) fn register_dynamic(dynamic_id: usize, callback: Box<dyn FnMut()>) {
        let slot: Box<SignalUpdateSlot> =
            Box::new(SignalUpdateSlot::new(Some(callback), false, true));
        let entry: SignalUpdateEntry = Box::into_raw(slot);
        if let Some(old_entry) = Self::ensure_update_registry_mut().insert(dynamic_id, entry) {
            unsafe {
                let _ = Box::from_raw(old_entry);
            }
        }
    }

    /// Registers a signal update callback for an attribute signal.
    pub(crate) fn register_attr_listener(signal_key: usize, callback: Box<dyn FnMut()>) {
        let slot: Box<SignalUpdateSlot> =
            Box::new(SignalUpdateSlot::new(Some(callback), false, true));
        let entry: SignalUpdateEntry = Box::into_raw(slot);
        if let Some(old_entry) = Self::ensure_update_registry_mut().insert(signal_key, entry) {
            unsafe {
                let _ = Box::from_raw(old_entry);
            }
        }
    }

    /// Cleans up all handler entries associated with a DOM element.
    pub(crate) fn cleanup_element(euv_id: usize) {
        let registry_ref: &mut HandlerRegistryMap = Self::ensure_handler_registry_mut();
        let keys_to_remove: Vec<(usize, &'static str)> = registry_ref
            .keys()
            .filter(|(id, _): &&(usize, &'static str)| *id == euv_id)
            .copied()
            .collect();
        for key in keys_to_remove {
            if let Some(entry) = registry_ref.remove(&key) {
                let slot: &mut HandlerSlot = unsafe { &mut *entry };
                if let Some(element) = slot.try_get_element().as_ref().cloned()
                    && let Some(listener_function) = slot.get_mut_listener_function().take()
                {
                    let event_name: &str = key.1;
                    let listener: &Function = listener_function.unchecked_ref::<Function>();
                    let _ = element.remove_event_listener_with_callback(event_name, listener);
                }
                slot.set_handler(None);
                unsafe {
                    let _ = Box::from_raw(entry);
                }
            }
        }
    }

    /// Cleans up all resources associated with a DynamicNode when its
    /// placeholder element is removed from the DOM.
    pub(crate) fn cleanup_dynamic_node(dynamic_id: usize) {
        if let Some(entry) = Self::ensure_update_registry().get(&dynamic_id) {
            let slot: &mut SignalUpdateSlot = unsafe { &mut **entry };
            slot.set_removed(true);
            slot.set_callback(None);
        }
    }

    /// Removes the signal update slot for an attribute signal from the registry.
    pub(crate) fn cleanup_attr_slot(addr: usize) {
        if let Some(entry) = Self::ensure_update_registry().get(&addr) {
            let slot: &mut SignalUpdateSlot = unsafe { &mut **entry };
            slot.set_removed(true);
            slot.set_callback(None);
        }
    }

    /// Ensures the handler registry is initialized and returns a shared reference.
    #[allow(static_mut_refs)]
    pub(crate) fn ensure_handler_registry() -> &'static HandlerRegistryMap {
        unsafe {
            if (*HANDLER_REGISTRY.get_0().get()).is_none() {
                (*HANDLER_REGISTRY.get_0().get()) = Some(HashMap::new());
            }
            (*HANDLER_REGISTRY.get_0().get())
                .as_ref()
                .unwrap_unchecked()
        }
    }

    /// Ensures the handler registry is initialized and returns a mutable reference.
    #[allow(static_mut_refs)]
    pub(crate) fn ensure_handler_registry_mut() -> &'static mut HandlerRegistryMap {
        unsafe {
            if (*HANDLER_REGISTRY.get_0().get()).is_none() {
                (*HANDLER_REGISTRY.get_0().get()) = Some(HashMap::new());
            }
            (*HANDLER_REGISTRY.get_0().get())
                .as_mut()
                .unwrap_unchecked()
        }
    }

    /// Returns whether the given event name is a non-bubbling event.
    pub(crate) fn is_non_bubbling(event_name: &str) -> bool {
        NON_BUBBLING_EVENTS.contains(&event_name)
    }

    /// Returns whether the event name is already delegated.
    #[allow(static_mut_refs)]
    pub(crate) fn is_delegated(event_name: &str) -> bool {
        unsafe {
            if (*DELEGATED_EVENTS.get_0().get()).is_none() {
                return false;
            }
            (*DELEGATED_EVENTS.get_0().get())
                .as_ref()
                .unwrap_unchecked()
                .contains(event_name)
        }
    }

    /// Marks an event name as delegated in the global set.
    #[allow(static_mut_refs)]
    pub(crate) fn mark_delegated(event_name: &'static str) {
        unsafe {
            if (*DELEGATED_EVENTS.get_0().get()).is_none() {
                (*DELEGATED_EVENTS.get_0().get()) = Some(HashSet::new());
            }
            (*DELEGATED_EVENTS.get_0().get())
                .as_mut()
                .unwrap_unchecked()
                .insert(event_name);
        }
    }

    /// Ensures the signal update registry is initialized and returns a shared reference.
    #[allow(static_mut_refs)]
    pub(crate) fn ensure_update_registry() -> &'static HashMap<usize, SignalUpdateEntry> {
        unsafe {
            if (*SIGNAL_UPDATE_REGISTRY.get_0().get()).is_none() {
                (*SIGNAL_UPDATE_REGISTRY.get_0().get()) = Some(HashMap::new());
            }
            (*SIGNAL_UPDATE_REGISTRY.get_0().get())
                .as_ref()
                .unwrap_unchecked()
        }
    }

    /// Ensures the signal update registry is initialized and returns a mutable reference.
    #[allow(static_mut_refs)]
    pub(crate) fn ensure_update_registry_mut() -> &'static mut HashMap<usize, SignalUpdateEntry> {
        unsafe {
            if (*SIGNAL_UPDATE_REGISTRY.get_0().get()).is_none() {
                (*SIGNAL_UPDATE_REGISTRY.get_0().get()) = Some(HashMap::new());
            }
            (*SIGNAL_UPDATE_REGISTRY.get_0().get())
                .as_mut()
                .unwrap_unchecked()
        }
    }

    /// Ensures the window event proxy registry is initialized and returns a mutable reference.
    #[allow(static_mut_refs)]
    pub(crate) fn ensure_window_registry_mut() -> &'static mut WindowEventRegistryMap {
        unsafe {
            if (*WINDOW_EVENT_REGISTRY.get_0().get()).is_none() {
                (*WINDOW_EVENT_REGISTRY.get_0().get()) = Some(HashMap::new());
            }
            (*WINDOW_EVENT_REGISTRY.get_0().get())
                .as_mut()
                .unwrap_unchecked()
        }
    }

    /// Registers a callback for a window-level event using the proxy pattern.
    pub(crate) fn register_window_event<F>(event_name: &str, callback: F) -> usize
    where
        F: FnMut() + 'static,
    {
        let handler_id: usize = NEXT_WINDOW_HANDLER_ID.fetch_add(1, Ordering::Relaxed);
        let boxed: Box<Box<dyn FnMut()>> = Box::new(Box::new(callback) as Box<dyn FnMut()>);
        let entry: WindowEventHandlerEntry = (handler_id, Box::into_raw(boxed));
        let registry: &mut WindowEventRegistryMap = Self::ensure_window_registry_mut();
        let is_new_event: bool = !registry.contains_key(event_name);
        registry
            .entry(event_name.to_string())
            .or_default()
            .push(entry);
        if is_new_event {
            Self::ensure_window_event_listener(event_name);
        }
        handler_id
    }

    /// Unregisters a window event handler by its event name and handler ID.
    pub(crate) fn unregister_window_event(event_name: &str, handler_id: usize) {
        let registry: &mut WindowEventRegistryMap = Self::ensure_window_registry_mut();
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
    fn ensure_window_event_listener(event_name: &str) {
        let event_name_owned: String = event_name.to_string();
        let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let handler_ids: Vec<usize> =
                match Self::ensure_window_registry_mut().get(&event_name_owned) {
                    Some(handlers) => handlers.iter().map(|(id, _ptr)| *id).collect(),
                    None => return,
                };
            for handler_id in handler_ids {
                let callback_ptr: *mut Box<dyn FnMut()> = match Self::ensure_window_registry_mut()
                    .get(&event_name_owned)
                {
                    Some(handlers) => match handlers.iter().find(|(id, _ptr)| *id == handler_id) {
                        Some((_id, ptr)) => *ptr,
                        None => continue,
                    },
                    None => return,
                };
                let callback: &mut Box<dyn FnMut()> = unsafe { &mut *callback_ptr };
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
