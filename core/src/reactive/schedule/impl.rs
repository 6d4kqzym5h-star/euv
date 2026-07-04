use crate::*;

/// Marks `CurrentHookContextCell` as `Sync` for single-threaded WASM contexts.
///
/// SAFETY: `CurrentHookContextCell` is only used in single-threaded WASM contexts.
/// Concurrent access from multiple threads would be undefined behavior.
unsafe impl Sync for CurrentHookContextCell {}

/// Static methods for scheduling signal update dispatch and batching.
impl Scheduler {
    /// Removes all entries from the signal update registry that have been
    /// marked as `removed`. Called after each dispatch cycle completes.
    fn sweep_removed_entries() {
        Registry::ensure_update_registry_mut().retain(|_key, entry| {
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

    /// Schedules a deferred signal update with precise dirty marking.
    pub(crate) fn update(dependents: &[usize]) {
        Registry::mark_dirty(dependents);
        if SUPPRESS_SCHEDULE.load(Ordering::Relaxed) {
            return;
        }
        if SCHEDULED.load(Ordering::Relaxed) {
            return;
        }
        SCHEDULED.store(true, Ordering::Relaxed);
        let window_value: Window = match window() {
            Some(window_instance) => window_instance,
            None => {
                SCHEDULED.store(false, Ordering::Relaxed);
                return;
            }
        };
        let queued_microtask: bool =
            DISPATCH_CLOSURE.with(|dispatch_closure: &Closure<dyn FnMut()>| {
                let dispatch_function: &Function =
                    dispatch_closure.as_ref().unchecked_ref::<Function>();
                let queue_microtask_value: JsValue =
                    Reflect::get(&window_value, &JsValue::from_str(QUEUE_MICROTASK))
                        .unwrap_or(JsValue::UNDEFINED);
                matches!(
                    queue_microtask_value.dyn_into::<Function>(),
                    Ok(queue_microtask)
                        if queue_microtask.call1(&window_value, dispatch_function).is_ok()
                )
            });
        if queued_microtask {
            return;
        }
        let scheduled: bool = DISPATCH_CLOSURE.with(|dispatch_closure: &Closure<dyn FnMut()>| {
            let dispatch_function: &Function =
                dispatch_closure.as_ref().unchecked_ref::<Function>();
            window_value
                .set_timeout_with_callback_and_timeout_and_arguments_0(dispatch_function, 0)
                .is_ok()
        });
        if scheduled {
            return;
        }
        let requested_frame: bool =
            DISPATCH_CLOSURE.with(|dispatch_closure: &Closure<dyn FnMut()>| {
                let dispatch_function: &Function =
                    dispatch_closure.as_ref().unchecked_ref::<Function>();
                window_value
                    .request_animation_frame(dispatch_function)
                    .is_ok()
            });
        if requested_frame {
            return;
        }
        SCHEDULED.store(false, Ordering::Relaxed);
    }

    /// Batches signal updates within a closure, deferring DOM dispatch.
    pub(crate) fn batch<F, R>(callback: F) -> R
    where
        F: FnOnce() -> R,
    {
        let was_outermost: bool = !SUPPRESS_SCHEDULE.load(Ordering::Relaxed);
        SUPPRESS_SCHEDULE.store(true, Ordering::Relaxed);
        let result: R = callback();
        SUPPRESS_SCHEDULE.store(!was_outermost, Ordering::Relaxed);
        if was_outermost && Registry::has_dirty() {
            Self::update(&[]);
        }
        result
    }

    /// Invokes all active callbacks in the signal update registry.
    ///
    /// Guards against re-entrant dispatch with `SIGNAL_UPDATE_DISPATCHING`.
    /// Iterates dirty slots, takes their callbacks, invokes them, and puts
    /// them back. After completing one pass, checks whether new entries
    /// were added during callback execution. If so, performs additional
    /// passes until the registry stabilizes, up to a maximum iteration limit.
    ///
    /// After all dispatch passes complete, sweeps the registry to remove
    /// entries that have been marked as `removed`.
    pub(crate) fn dispatch_updates() {
        if SIGNAL_UPDATE_DISPATCHING.load(Ordering::Relaxed) {
            return;
        }
        SIGNAL_UPDATE_DISPATCHING.store(true, Ordering::Relaxed);
        let mut iterations: usize = 0;
        loop {
            let registry: &mut HashMap<usize, SignalUpdateEntry> =
                Registry::ensure_update_registry_mut();
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
                let entry: SignalUpdateEntry =
                    match Registry::ensure_update_registry_mut().remove(&key) {
                        Some(removed_entry) => removed_entry,
                        None => continue,
                    };
                let slot: &mut SignalUpdateSlot = unsafe { &mut *entry };
                if slot.get_removed() {
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
                    Registry::ensure_update_registry_mut();
                if registry.contains_key(&key) {
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
        Self::sweep_removed_entries();
        SIGNAL_UPDATE_DISPATCHING.store(false, Ordering::Relaxed);
    }
}
