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
/// - `&Event`: The native DOM event being dispatched.
/// - `&str`: The name of the event type being delegated.
fn dispatch_delegated_event(event: &Event, event_name: &str) {
    let target: EventTarget = match event.target() {
        Some(t) => t,
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
            let key: (usize, String) = (euv_id, event_name.to_string());
            let found: Option<NativeEventHandler> = {
                let registry: &HashMap<(usize, String), HandlerEntry> = ensure_handler_registry();
                registry.get(&key).and_then(|entry| {
                    let slot: Ref<HandlerSlot> = entry.borrow();
                    slot.try_get_handler().as_ref().cloned()
                })
            };
            if let Some(active_handler) = found {
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
/// If the event type is already in `DELEGATED_EVENTS`, this is a no-op.
/// Otherwise, creates a `Closure` that calls `dispatch_delegated_event`,
/// registers it as a capturing listener on `window`, and marks the event
/// type as delegated.
///
/// # Arguments
///
/// - `String`: The event type name to listen for (e.g. `"click"`).
///
/// # Panics
///
/// Panics if `window()` returns `None` or if `add_event_listener_with_callback_and_bool` fails.
pub(crate) fn ensure_delegated_listener(event_name: String) {
    let already_delegated: bool = is_delegated_event(&event_name);
    if already_delegated {
        return;
    }
    let event_name_clone: String = event_name.clone();
    let closure: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
        dispatch_delegated_event(&event, &event_name_clone);
    }));
    let window: Window = window().unwrap();
    window
        .add_event_listener_with_callback_and_bool(
            &event_name,
            closure.as_ref().unchecked_ref(),
            true,
        )
        .unwrap();
    closure.forget();
    insert_delegated_event(event_name);
}

/// One-time initialization of global event delegation.
///
/// Iterates over all `DELEGATABLE_EVENT_NAMES`
/// and calls `ensure_delegated_listener` for each one, registering
/// capturing-phase listeners on `window`.
pub(crate) fn init_event_delegation() {
    for event_name_str in DELEGATABLE_EVENT_NAMES {
        let event_name: String = event_name_str.to_string();
        ensure_delegated_listener(event_name);
    }
}

/// Ensures the global `NativeEventName::EuvSignalUpdate.to_string()` listener is registered on window.
///
/// If already registered (`SIGNAL_UPDATE_LISTENER_REGISTERED` is true),
/// this is a no-op. Otherwise, creates a `Closure` that calls
/// `dispatch_signal_update_callbacks`, registers it as a listener for
/// the `NativeEventName::EuvSignalUpdate.to_string()` event on `window`, and sets the flag.
///
/// # Panics
///
/// Panics if `window()` returns `None` or if `add_event_listener_with_callback` fails.
pub(crate) fn ensure_signal_update_listener() {
    let already_registered: bool = is_signal_update_listener_registered();
    if already_registered {
        return;
    }
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {
        dispatch_signal_update_callbacks();
    }));
    let event_name: String = NativeEventName::EuvSignalUpdate.to_string();
    let window: Window = window().unwrap();
    window
        .add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
    set_signal_update_listener_registered(true);
}

/// Invokes all active callbacks in the signal update registry.
///
/// Guards against re-entrant dispatch with `SIGNAL_UPDATE_DISPATCHING`.
/// Iterates over all registry keys, takes each callback out of its slot,
/// invokes it, and puts it back (unless the slot has been marked as
/// removed during execution). Removed entries are cleaned up afterward.
fn dispatch_signal_update_callbacks() {
    if SIGNAL_UPDATE_DISPATCHING.load(Ordering::Relaxed) {
        return;
    }
    SIGNAL_UPDATE_DISPATCHING.store(true, Ordering::Relaxed);
    let keys: Vec<usize> = ensure_signal_update_registry().keys().cloned().collect();
    let mut keys_to_cleanup: Vec<usize> = Vec::new();
    for key in keys {
        let callback: Option<Box<dyn FnMut()>> = {
            let registry_ref: &HashMap<usize, SignalUpdateEntry> = ensure_signal_update_registry();
            let Some(entry) = registry_ref.get(&key) else {
                continue;
            };
            let mut slot: RefMut<SignalUpdateSlot> = entry.borrow_mut();
            if slot.get_removed() {
                keys_to_cleanup.push(key);
                continue;
            }
            slot.get_mut_callback().take()
        };
        if let Some(mut callback) = callback {
            callback();
            let registry_ref: &mut HashMap<usize, SignalUpdateEntry> =
                ensure_signal_update_registry_mut();
            if let Some(entry) = registry_ref.get(&key) {
                let mut slot: RefMut<SignalUpdateSlot> = entry.borrow_mut();
                if !slot.get_removed() {
                    slot.set_callback(Some(callback));
                }
            }
        }
    }
    if !keys_to_cleanup.is_empty() {
        let registry_ref: &mut HashMap<usize, SignalUpdateEntry> =
            ensure_signal_update_registry_mut();
        for key in keys_to_cleanup {
            registry_ref.remove(&key);
        }
    }
    SIGNAL_UPDATE_DISPATCHING.store(false, Ordering::Relaxed);
}

/// Registers a signal update callback for a DynamicNode placeholder.
///
/// On WASM targets, ensures the signal update listener is active first.
/// Then inserts the callback into `SIGNAL_UPDATE_REGISTRY` keyed by
/// `dynamic_id`, wrapped in a `SignalUpdateSlot`.
///
/// # Arguments
///
/// - `usize`: The unique ID of the `DynamicNode`.
/// - `Box<dyn FnMut()>`: The callback to invoke on signal updates.
pub(crate) fn register_dynamic_listener(dynamic_id: usize, callback: Box<dyn FnMut()>) {
    ensure_signal_update_listener();
    let registry_ref: &mut HashMap<usize, SignalUpdateEntry> = ensure_signal_update_registry_mut();
    let entry: SignalUpdateEntry =
        Rc::new(RefCell::new(SignalUpdateSlot::new(Some(callback), false)));
    registry_ref.insert(dynamic_id, entry);
}

/// Registers a signal update callback for an attribute signal.
///
/// Inserts the callback into `SIGNAL_UPDATE_REGISTRY` keyed by
/// `signal_key` (the signal's inner address), wrapped in a `SignalUpdateSlot`.
///
/// # Arguments
///
/// - `usize`: The inner address of the attribute signal.
/// - `Box<dyn FnMut()>`: The callback to invoke on signal updates.
pub(crate) fn register_attr_signal_listener(signal_key: usize, callback: Box<dyn FnMut()>) {
    let registry_ref: &mut HashMap<usize, SignalUpdateEntry> = ensure_signal_update_registry_mut();
    let entry: SignalUpdateEntry =
        Rc::new(RefCell::new(SignalUpdateSlot::new(Some(callback), false)));
    registry_ref.insert(signal_key, entry);
}

/// Removes a single handler entry identified by its element ID and event name.
///
/// Looks up the `(_euv_id, event_name)` key in `HANDLER_REGISTRY` and
/// removes it if present.
///
/// # Arguments
///
/// - `usize`: The euv ID of the DOM element.
/// - `&str`: The event name of the handler to remove.
pub(crate) fn cleanup_event_handler(_euv_id: usize, event_name: &str) {
    let registry_ref: &mut HashMap<(usize, String), HandlerEntry> = ensure_handler_registry_mut();
    let key: (usize, String) = (_euv_id, event_name.to_string());
    registry_ref.remove(&key);
}

/// Cleans up all handler entries associated with a DOM element.
///
/// Collects all registry keys whose element ID matches `euv_id` and
/// removes them from `HANDLER_REGISTRY`.
///
/// # Arguments
///
/// - `usize`: The euv ID of the DOM element being removed.
pub(crate) fn cleanup_element_handlers(euv_id: usize) {
    let registry_ref: &mut HashMap<(usize, String), HandlerEntry> = ensure_handler_registry_mut();
    let keys_to_remove: Vec<(usize, String)> = registry_ref
        .keys()
        .filter(|(id, _)| *id == euv_id)
        .cloned()
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
/// - `usize`: The unique ID of the `DynamicNode` being removed.
pub(crate) fn cleanup_dynamic_node(dynamic_id: usize) {
    let registry_ref: &HashMap<usize, SignalUpdateEntry> = ensure_signal_update_registry();
    if let Some(entry) = registry_ref.get(&dynamic_id) {
        let mut slot: RefMut<SignalUpdateSlot> = entry.borrow_mut();
        slot.set_removed(true);
        slot.set_callback(None);
    }
}

/// No-op for text node signal cleanup.
///
/// Text node signal listeners are cleaned up lazily via the `is_node_connected`
/// guard in the `replace_subscribe` callback. When the signal fires and the
/// text node is no longer in the DOM, the listener self-clears by calling
/// `clear_listeners()`. This provides automatic cleanup without needing
/// a separate registry for text-node-to-signal mappings.
///
/// # Arguments
///
/// - `&Text` - The text node being removed (unused, present for API consistency).
#[allow(unused_variables)]
pub(crate) fn cleanup_text_signal_listeners(text: &Text) {}

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
/// - `&str`: The event name to check.
///
/// # Returns
///
/// - `bool`: Whether the event is already delegated.
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
/// - `String`: The event name to insert.
#[allow(static_mut_refs)]
pub(crate) fn insert_delegated_event(event_name: String) {
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

/// Returns whether the signal update listener has been registered.
///
/// # Returns
///
/// - `bool`: Whether the listener is registered.
#[allow(static_mut_refs)]
pub(crate) fn is_signal_update_listener_registered() -> bool {
    unsafe { *SIGNAL_UPDATE_LISTENER_REGISTERED.get_0().get() }
}

/// Sets the signal update listener registered flag.
///
/// # Arguments
///
/// - `bool`: The new flag value.
#[allow(static_mut_refs)]
pub(crate) fn set_signal_update_listener_registered(value: bool) {
    unsafe {
        *SIGNAL_UPDATE_LISTENER_REGISTERED.get_0().get() = value;
    }
}
