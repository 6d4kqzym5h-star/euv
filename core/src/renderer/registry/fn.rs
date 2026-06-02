use crate::*;

/// Dispatches a delegated event by walking up from `event.target` to
/// find the nearest element with a `data-euv-id` attribute, then
/// invoking the matching handler from the global registry.
///
/// Traverses the DOM tree upward from the event target until an element
/// with a `data-euv-id` attribute is found. Looks up the handler in
/// `HANDLER_REGISTRY` using the `(euv_id, event_name)` key. If found,
/// invokes the handler without stopping propagation, because some
/// browser features (e.g., drag-and-drop) rely on events bubbling
/// to the document level to function correctly.
///
/// # Arguments
///
/// - `&Event` - The native DOM event being dispatched.
/// - `&str` - The name of the event type being delegated.
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
            && let Some(active_handler) = ensure_handler_registry()
                .get(&(euv_id, event_name))
                .and_then(|entry: &HandlerEntry| {
                    entry
                        .try_borrow()
                        .ok()
                        .and_then(|slot: Ref<HandlerSlot>| slot.try_get_handler().as_ref().cloned())
                })
        {
            active_handler.handle(event.clone());
            return;
        }
        current = element.parent_element();
    }
}

/// Ensures a global capturing-phase listener is registered on `window`
/// for the given event type.
///
/// If the event type is already in `DELEGATED_EVENTS`, this is a no-op.
/// Otherwise, creates a `Closure` that calls `dispatch_delegated_event`,
/// registers it as a capturing listener on `window`, and marks the event
/// type as delegated.
///
/// # Arguments
///
/// - `&'static str` - The event type name to listen for (e.g. `"click"`).
pub(crate) fn ensure_delegated_listener(event_name: &'static str) {
    if is_delegated_event(event_name) {
        return;
    }
    let closure: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
        dispatch_delegated_event(&event, event_name);
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
    insert_delegated_event(event_name);
}

/// Invokes all active callbacks in the signal update registry.
///
/// Guards against re-entrant dispatch with `SIGNAL_UPDATE_DISPATCHING`.
/// Drains the registry into a local Vec, invokes each callback, and
/// re-inserts survivors (non-removed entries) back into the registry.
/// This avoids per-key HashMap lookups and a separate keys Vec allocation.
///
/// After completing one pass, checks whether new entries were added during
/// callback execution (e.g., by IntersectionObserver or async callbacks).
/// If so, performs additional passes until the registry stabilizes, up to
/// a maximum iteration limit to prevent infinite loops.
pub(crate) fn dispatch_signal_update_callbacks() {
    if SIGNAL_UPDATE_DISPATCHING.load(Ordering::Relaxed) {
        return;
    }
    SIGNAL_UPDATE_DISPATCHING.store(true, Ordering::Relaxed);
    let mut iterations: usize = 0;
    const MAX_ITERATIONS: usize = 3;
    loop {
        let registry: &mut HashMap<usize, SignalUpdateEntry> = ensure_signal_update_registry_mut();
        let dirty_keys: Vec<usize> = registry
            .iter()
            .filter_map(|(key, entry): (&usize, &SignalUpdateEntry)| {
                let Ok(slot) = entry.try_borrow() else {
                    return None;
                };
                if slot.get_dirty() && !slot.get_removed() {
                    Some(*key)
                } else {
                    None
                }
            })
            .collect();
        if dirty_keys.is_empty() {
            break;
        }
        for key in dirty_keys {
            let entry: SignalUpdateEntry = match ensure_signal_update_registry_mut().remove(&key) {
                Some(removed_entry) => removed_entry,
                None => continue,
            };
            let callback: Option<Box<dyn FnMut()>> = {
                let Ok(mut slot) = entry.try_borrow_mut() else {
                    continue;
                };
                if slot.get_removed() {
                    continue;
                }
                slot.set_dirty(false);
                slot.get_mut_callback().take()
            };
            if let Some(mut callback) = callback {
                callback();
                if let Ok(mut slot) = entry.try_borrow_mut()
                    && !slot.get_removed()
                {
                    slot.set_callback(Some(callback));
                }
            }
            if entry
                .try_borrow()
                .map(|slot: Ref<SignalUpdateSlot>| slot.get_removed())
                .unwrap_or(true)
            {
                continue;
            }
            let registry: &mut HashMap<usize, SignalUpdateEntry> =
                ensure_signal_update_registry_mut();
            if registry.contains_key(&key) {
                continue;
            }
            registry.insert(key, entry);
        }
        iterations += 1;
        if iterations >= MAX_ITERATIONS {
            break;
        }
    }
    SIGNAL_UPDATE_DISPATCHING.store(false, Ordering::Relaxed);
}

/// Marks all non-removed slots in the signal update registry as dirty.
///
/// Called by `schedule_signal_update` to indicate that at least one signal
/// has changed and all dynamic nodes need to check for updates on the next
/// dispatch cycle.
pub(crate) fn mark_all_slots_dirty() {
    let registry: &mut HashMap<usize, SignalUpdateEntry> = ensure_signal_update_registry_mut();
    for entry in registry.values() {
        let Ok(mut slot) = entry.try_borrow_mut() else {
            continue;
        };
        if !slot.get_removed() {
            slot.set_dirty(true);
        }
    }
}

/// Registers a signal update callback for a DynamicNode placeholder.
///
/// On WASM targets, ensures the signal update listener is active first.
/// Then inserts the callback into `SIGNAL_UPDATE_REGISTRY` keyed by
/// `dynamic_id`, wrapped in a `SignalUpdateSlot`.
///
/// # Arguments
///
/// - `usize` - The unique ID of the `DynamicNode`.
/// - `Box<dyn FnMut()>` - The callback to invoke on signal updates.
pub(crate) fn register_dynamic_listener(dynamic_id: usize, callback: Box<dyn FnMut()>) {
    let entry: SignalUpdateEntry = Rc::new(RefCell::new(SignalUpdateSlot::new(
        Some(callback),
        false,
        true,
    )));
    ensure_signal_update_registry_mut().insert(dynamic_id, entry);
}

/// Registers a signal update callback for an attribute signal.
///
/// Inserts the callback into `SIGNAL_UPDATE_REGISTRY` keyed by
/// `signal_key` (the signal's inner address), wrapped in a `SignalUpdateSlot`.
///
/// # Arguments
///
/// - `usize` - The inner address of the attribute signal.
/// - `Box<dyn FnMut()>` - The callback to invoke on signal updates.
pub(crate) fn register_attr_signal_listener(signal_key: usize, callback: Box<dyn FnMut()>) {
    let entry: SignalUpdateEntry = Rc::new(RefCell::new(SignalUpdateSlot::new(
        Some(callback),
        false,
        true,
    )));
    ensure_signal_update_registry_mut().insert(signal_key, entry);
}

/// Cleans up all handler entries associated with a DOM element.
///
/// Collects all registry keys whose element ID matches `euv_id` and
/// removes them from `HANDLER_REGISTRY`.
///
/// # Arguments
///
/// - `usize` - The euv ID of the DOM element being removed.
pub(crate) fn cleanup_element_handlers(euv_id: usize) {
    let registry_ref: &mut HandlerRegistryMap = ensure_handler_registry_mut();
    let keys_to_remove: Vec<(usize, &'static str)> = registry_ref
        .keys()
        .filter(|(id, _): &&(usize, &'static str)| *id == euv_id)
        .copied()
        .collect();
    for key in keys_to_remove {
        registry_ref.remove(&key);
    }
}

/// Cleans up all resources associated with a DynamicNode when its
/// placeholder element is removed from the DOM.
///
/// On WASM targets, marks the `SignalUpdateSlot` as removed and
/// clears its callback so it will not be invoked in future dispatches.
///
/// # Arguments
///
/// - `usize` - The unique ID of the `DynamicNode` being removed.
pub(crate) fn cleanup_dynamic_node(dynamic_id: usize) {
    if let Some(entry) = ensure_signal_update_registry().get(&dynamic_id)
        && let Ok(mut slot) = entry.try_borrow_mut()
    {
        slot.set_removed(true);
        slot.set_callback(None);
    }
}

/// Ensures the handler registry is initialized and returns a shared reference.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
#[allow(static_mut_refs)]
fn ensure_handler_registry() -> &'static HandlerRegistryMap {
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
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
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

/// Returns whether the event name is already delegated.
///
/// # Arguments
///
/// - `&str` - The event name to check.
///
/// # Returns
///
/// - `bool` - Whether the event is already delegated.
#[allow(static_mut_refs)]
pub(crate) fn is_delegated_event(event_name: &str) -> bool {
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

/// Inserts an event name into the delegated events set.
///
/// # Arguments
///
/// - `&'static str` - The event name to insert.
#[allow(static_mut_refs)]
pub(crate) fn insert_delegated_event(event_name: &'static str) {
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
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
#[allow(static_mut_refs)]
fn ensure_signal_update_registry() -> &'static HashMap<usize, SignalUpdateEntry> {
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
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
#[allow(static_mut_refs)]
fn ensure_signal_update_registry_mut() -> &'static mut HashMap<usize, SignalUpdateEntry> {
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
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
#[allow(static_mut_refs)]
pub(crate) fn ensure_window_event_registry_mut() -> &'static mut WindowEventRegistryMap {
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
///
/// Ensures a single `window.addEventListener` listener exists for the given
/// event name. The proxy listener dispatches to all registered callbacks
/// for that event. Returns a unique handler ID that can be used to unregister
/// the callback later via `unregister_window_event_handler`.
///
/// # Arguments
///
/// - `&str` - The event name to listen for (e.g., "hashchange", "popstate", "resize").
/// - `Box<dyn FnMut()>` - The callback to invoke when the event fires.
///
/// # Returns
///
/// - `usize` - A unique handler ID for later unregistration.
pub(crate) fn register_window_event_handler<F>(event_name: &str, callback: F) -> usize
where
    F: FnMut() + 'static,
{
    let handler_id: usize = NEXT_WINDOW_HANDLER_ID.fetch_add(1, Ordering::Relaxed);
    let entry: WindowEventHandlerEntry = (
        handler_id,
        Rc::new(RefCell::new(Box::new(callback) as Box<dyn FnMut()>)),
    );
    let registry: &mut WindowEventRegistryMap = ensure_window_event_registry_mut();
    let is_new_event: bool = !registry.contains_key(event_name);
    registry
        .entry(event_name.to_string())
        .or_default()
        .push(entry);
    if is_new_event {
        ensure_window_event_listener(event_name);
    }
    handler_id
}

/// Unregisters a window event handler by its event name and handler ID.
///
/// Removes the callback entry from the proxy registry. The shared
/// `window.addEventListener` listener is NOT removed even if no handlers
/// remain, because removing and re-adding listeners is more expensive
/// than keeping an empty dispatch loop.
///
/// # Arguments
///
/// - `&str` - The event name the handler was registered for.
/// - `usize` - The handler ID returned by `register_window_event_handler`.
pub(crate) fn unregister_window_event_handler(event_name: &str, handler_id: usize) {
    let registry: &mut WindowEventRegistryMap = ensure_window_event_registry_mut();
    if let Some(handlers) = registry.get_mut(event_name) {
        handlers.retain(|(id, _): &WindowEventHandlerEntry| *id != handler_id);
    }
}

/// Ensures a single `window.addEventListener` listener is registered
/// for the given event name that dispatches to all registered callbacks.
///
/// Creates a `Closure` that looks up all handlers for the event name in
/// `WINDOW_EVENT_REGISTRY` and invokes each one. The closure is forgotten
/// after registration so it lives for the lifetime of the application.
///
/// # Arguments
///
/// - `&str` - The event name to create a proxy listener for.
fn ensure_window_event_listener(event_name: &str) {
    let event_name_owned: String = event_name.to_string();
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let registry: &WindowEventRegistryMap = ensure_window_event_registry_mut();
        if let Some(handlers) = registry.get(&event_name_owned) {
            for (_handler_id, callback_rc) in handlers {
                if let Ok(mut callback_ref) = callback_rc.try_borrow_mut() {
                    callback_ref();
                }
            }
        }
    }));
    let window: Window = match window() {
        Some(window_instance) => window_instance,
        None => return,
    };
    let _ = window.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());
    closure.forget();
}
