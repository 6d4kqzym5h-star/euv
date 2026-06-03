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
                    let slot: &HandlerSlot = unsafe { &**entry };
                    slot.try_get_handler().as_ref().cloned()
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
/// Iterates dirty slots, takes their callbacks, invokes them, and puts
/// them back. Uses direct pointer access for zero-overhead slot manipulation.
///
/// After completing one pass, checks whether new entries were added during
/// callback execution (e.g., by IntersectionObserver or async callbacks).
/// If so, performs additional passes until the registry stabilizes, up to
/// a maximum iteration limit to prevent infinite loops.
///
/// After all dispatch passes complete, sweeps the registry to remove entries
/// that have been marked as `removed`, preventing memory leaks from
/// accumulated dead DynamicNode entries.
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
                let slot: &SignalUpdateSlot = unsafe { &**entry };
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
            let slot: &mut SignalUpdateSlot = unsafe { &mut *entry };
            if slot.get_removed() {
                // Free the allocation since it's removed.
                unsafe {
                    let _ = Box::from_raw(entry);
                }
                continue;
            }
            slot.set_dirty(false);
            let callback: Option<Box<dyn FnMut()>> = slot.get_mut_callback().take();
            if let Some(mut callback) = callback {
                callback();
                let slot: &mut SignalUpdateSlot = unsafe { &mut *entry };
                if !slot.get_removed() {
                    slot.set_callback(Some(callback));
                }
            }
            let slot: &SignalUpdateSlot = unsafe { &*entry };
            if slot.get_removed() {
                unsafe {
                    let _ = Box::from_raw(entry);
                }
                continue;
            }
            let registry: &mut HashMap<usize, SignalUpdateEntry> =
                ensure_signal_update_registry_mut();
            if registry.contains_key(&key) {
                // Another entry was inserted for this key during callback; free ours.
                unsafe {
                    let _ = Box::from_raw(entry);
                }
                continue;
            }
            registry.insert(key, entry);
        }
        iterations += 1;
        if iterations >= MAX_ITERATIONS {
            break;
        }
    }
    sweep_removed_signal_update_entries();
    SIGNAL_UPDATE_DISPATCHING.store(false, Ordering::Relaxed);
}

/// Removes all entries from the signal update registry that have been marked
/// as `removed`. This prevents unbounded memory growth from accumulated
/// dead DynamicNode entries that are no longer connected to the DOM.
///
/// Called after each dispatch cycle completes to clean up stale entries.
fn sweep_removed_signal_update_entries() {
    ensure_signal_update_registry_mut().retain(|_key, entry| {
        let slot: &SignalUpdateSlot = unsafe { &**entry };
        if slot.get_removed() {
            unsafe {
                let _ = Box::from_raw(*entry);
            }
            false
        } else {
            true
        }
    });
}

/// Marks all non-removed slots in the signal update registry as dirty.
///
/// Called by `schedule_signal_update` to indicate that at least one signal
/// has changed and all dynamic nodes need to check for updates on the next
/// dispatch cycle. This is the fallback when no dependency tracking info
/// is available (e.g., during `batch_updates`).
pub(crate) fn mark_all_slots_dirty() {
    let registry: &mut HashMap<usize, SignalUpdateEntry> = ensure_signal_update_registry_mut();
    for entry in registry.values() {
        let slot: &mut SignalUpdateSlot = unsafe { &mut **entry };
        if !slot.get_removed() {
            slot.set_dirty(true);
        }
    }
}

/// Marks only the specified dynamic node slots as dirty.
///
/// This enables precise reactive updates: when a signal changes, only
/// the dynamic nodes that actually depend on that signal are scheduled
/// for re-rendering, avoiding O(N) full broadcast to all dynamic nodes.
///
/// # Arguments
///
/// - `&[usize]` - The dynamic node IDs to mark as dirty.
pub(crate) fn mark_slots_dirty_targeted(dynamic_ids: &[usize]) {
    let registry: &mut HashMap<usize, SignalUpdateEntry> = ensure_signal_update_registry_mut();
    for dynamic_id in dynamic_ids {
        if let Some(entry) = registry.get(dynamic_id) {
            let slot: &mut SignalUpdateSlot = unsafe { &mut **entry };
            if !slot.get_removed() {
                slot.set_dirty(true);
            }
        }
    }
}

/// Registers a signal update callback for a DynamicNode placeholder.
///
/// Allocates a `SignalUpdateSlot` on the heap and inserts the raw pointer
/// into `SIGNAL_UPDATE_REGISTRY` keyed by `dynamic_id`.
///
/// # Arguments
///
/// - `usize` - The unique ID of the `DynamicNode`.
/// - `Box<dyn FnMut()>` - The callback to invoke on signal updates.
pub(crate) fn register_dynamic_listener(dynamic_id: usize, callback: Box<dyn FnMut()>) {
    let slot: Box<SignalUpdateSlot> = Box::new(SignalUpdateSlot::new(Some(callback), false, true));
    let entry: SignalUpdateEntry = Box::into_raw(slot);
    // If there's an existing entry for this key, free it first.
    if let Some(old_entry) = ensure_signal_update_registry_mut().insert(dynamic_id, entry) {
        unsafe {
            let _ = Box::from_raw(old_entry);
        }
    }
}

/// Registers a signal update callback for an attribute signal.
///
/// Allocates a `SignalUpdateSlot` on the heap and inserts the raw pointer
/// into `SIGNAL_UPDATE_REGISTRY` keyed by `signal_key` (the signal's inner address).
///
/// # Arguments
///
/// - `usize` - The inner address of the attribute signal.
/// - `Box<dyn FnMut()>` - The callback to invoke on signal updates.
pub(crate) fn register_attr_signal_listener(signal_key: usize, callback: Box<dyn FnMut()>) {
    let slot: Box<SignalUpdateSlot> = Box::new(SignalUpdateSlot::new(Some(callback), false, true));
    let entry: SignalUpdateEntry = Box::into_raw(slot);
    if let Some(old_entry) = ensure_signal_update_registry_mut().insert(signal_key, entry) {
        unsafe {
            let _ = Box::from_raw(old_entry);
        }
    }
}

/// Cleans up all handler entries associated with a DOM element.
///
/// Collects all registry keys whose element ID matches `euv_id` and
/// removes them from `HANDLER_REGISTRY`, freeing the heap allocations.
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
        if let Some(entry) = registry_ref.remove(&key) {
            unsafe {
                let _ = Box::from_raw(entry);
            }
        }
    }
}

/// Cleans up all resources associated with a DynamicNode when its
/// placeholder element is removed from the DOM.
///
/// Marks the `SignalUpdateSlot` as removed and clears its callback so it
/// will not be invoked in future dispatches.
///
/// # Arguments
///
/// - `usize` - The unique ID of the `DynamicNode` being removed.
pub(crate) fn cleanup_dynamic_node(dynamic_id: usize) {
    if let Some(entry) = ensure_signal_update_registry().get(&dynamic_id) {
        let slot: &mut SignalUpdateSlot = unsafe { &mut **entry };
        slot.set_removed(true);
        slot.set_callback(None);
    }
}

/// Removes the signal update slot for an attribute signal from the registry.
///
/// Attribute signals are registered with the signal's inner address as key
/// (via `register_attr_signal_listener`). This function marks the slot as
/// removed and clears its callback, mirroring `cleanup_dynamic_node` but
/// for attribute-level signals. The entry will be swept on the next dispatch.
///
/// # Arguments
///
/// - `usize` - The inner pointer address of the attribute signal.
pub(crate) fn cleanup_attr_signal_update_slot(addr: usize) {
    if let Some(entry) = ensure_signal_update_registry().get(&addr) {
        let slot: &mut SignalUpdateSlot = unsafe { &mut **entry };
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
/// - `FnMut() + 'static` - The callback to invoke when the event fires.
///
/// # Returns
///
/// - `usize` - A unique handler ID for later unregistration.
pub(crate) fn register_window_event_handler<F>(event_name: &str, callback: F) -> usize
where
    F: FnMut() + 'static,
{
    let handler_id: usize = NEXT_WINDOW_HANDLER_ID.fetch_add(1, Ordering::Relaxed);
    let boxed: Box<Box<dyn FnMut()>> = Box::new(Box::new(callback) as Box<dyn FnMut()>);
    let entry: WindowEventHandlerEntry = (handler_id, Box::into_raw(boxed));
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
/// Removes the callback entry from the proxy registry and frees the
/// heap allocation. The shared `window.addEventListener` listener is NOT
/// removed even if no handlers remain, because removing and re-adding
/// listeners is more expensive than keeping an empty dispatch loop.
///
/// # Arguments
///
/// - `&str` - The event name the handler was registered for.
/// - `usize` - The handler ID returned by `register_window_event_handler`.
pub(crate) fn unregister_window_event_handler(event_name: &str, handler_id: usize) {
    let registry: &mut WindowEventRegistryMap = ensure_window_event_registry_mut();
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
        // Snapshot the handler IDs before invoking any callback.
        //
        // A callback may, while running, register or unregister handlers for
        // this same event name (e.g. a route change tears down the old page's
        // hooks and `unregister_window_event_handler` frees an entry, or a new
        // page's `use_window_event` pushes one). Mutating the `Vec` mid-iteration
        // would either invalidate references into it or free a `*mut Box<dyn FnMut()>`
        // that we are about to dereference (use-after-free). By snapshotting the
        // IDs and re-looking-up each handler immediately before calling it, we
        // skip any handler that was removed during dispatch and never touch a
        // freed allocation or a reallocated buffer.
        let handler_ids: Vec<usize> =
            match ensure_window_event_registry_mut().get(&event_name_owned) {
                Some(handlers) => handlers.iter().map(|(id, _ptr)| *id).collect(),
                None => return,
            };
        for handler_id in handler_ids {
            let callback_ptr: *mut Box<dyn FnMut()> =
                match ensure_window_event_registry_mut().get(&event_name_owned) {
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
    let _ = window.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());
    closure.forget();
}
