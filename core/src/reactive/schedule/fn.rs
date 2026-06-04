use crate::*;

thread_local! {
    /// The persistent dispatch `Closure`, kept alive for the lifetime of the
    /// program so it can be handed to `setTimeout` repeatedly.
    ///
    /// The closure resets the `SCHEDULED` flag and then runs the queued signal
    /// update callbacks. Resetting `SCHEDULED` here is what allows the next
    /// `schedule_signal_update_targeted` call to schedule a fresh dispatch; if
    /// this never ran, the flag would stay `true` forever and every reactive
    /// update would be silently dropped.
    static DISPATCH_CLOSURE: closure::Closure<dyn FnMut()> =
        closure::Closure::wrap(Box::new(|| {
            SCHEDULED.store(false, Ordering::Relaxed);
            dispatch_signal_update_callbacks();
        }) as Box<dyn FnMut()>);
}

/// Invokes `callback` with a reference to the persistent dispatch `Function`.
///
/// The dispatch closure is created once per thread and stored in
/// `DISPATCH_CLOSURE`. This helper exposes it as a `&Function` so it can be
/// passed to the various browser scheduling APIs (`setTimeout`,
/// `queueMicrotask`, `requestAnimationFrame`) without recreating the closure
/// on every schedule.
///
/// # Arguments
///
/// - `FnOnce(&Function) -> R` - Receives the dispatch function reference.
///
/// # Returns
///
/// - `R` - The value returned by `callback`.
fn with_dispatch_function<F, R>(callback: F) -> R
where
    F: FnOnce(&Function) -> R,
{
    DISPATCH_CLOSURE.with(|dispatch_closure| {
        let dispatch_function: &Function = dispatch_closure.as_ref().unchecked_ref::<Function>();
        callback(dispatch_function)
    })
}

/// Schedules a deferred signal update event.
///
/// If a schedule is already pending (`SCHEDULED` is true) or updates
/// are suppressed (`SUPPRESS_SCHEDULE` is true), this is a no-op.
/// Otherwise, sets `SCHEDULED` to true and queues the dispatch callback
/// (preferring `queueMicrotask`) on WASM targets. This ensures that no
/// matter how many signal updates occur within a single task, only one
/// dispatch cycle runs, preventing CPU spikes during rapid input events
/// (e.g., slider dragging).
///
/// On non-WASM targets, resets `SCHEDULED` immediately since there is
/// no event loop to schedule on.
pub fn schedule_signal_update() {
    schedule_signal_update_targeted(&[]);
}

/// Schedules a deferred signal update with precise dirty marking.
///
/// If `dependents` is non-empty, only those dynamic node IDs are marked
/// dirty. If `dependents` is empty, falls back to marking all slots dirty
/// (for backwards compatibility with `batch_updates` and other callers
/// that don't track dependencies).
///
/// # Arguments
///
/// - `&[usize]` - Dynamic node IDs to mark dirty. Empty slice means mark all.
pub fn schedule_signal_update_targeted(dependents: &[usize]) {
    if SUPPRESS_SCHEDULE.load(Ordering::Relaxed) {
        if !dependents.is_empty() {
            mark_slots_dirty_targeted(dependents);
        } else {
            mark_all_slots_dirty();
        }
        return;
    }
    if !dependents.is_empty() {
        mark_slots_dirty_targeted(dependents);
    } else {
        mark_all_slots_dirty();
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
    let queued_microtask: bool = with_dispatch_function(|dispatch_function| {
        let queue_microtask_value: JsValue =
            Reflect::get(&window_value, &JsValue::from_str(QUEUE_MICROTASK))
                .unwrap_or(JsValue::UNDEFINED);
        matches!(
            queue_microtask_value.dyn_into::<Function>(),
            Ok(queue_microtask) if queue_microtask.call1(&window_value, dispatch_function).is_ok()
        )
    });
    if queued_microtask {
        return;
    }
    let scheduled: bool = with_dispatch_function(|dispatch_function| {
        window_value
            .set_timeout_with_callback_and_timeout_and_arguments_0(dispatch_function, 0)
            .is_ok()
    });
    if scheduled {
        return;
    }
    let requested_frame: bool = with_dispatch_function(|dispatch_function| {
        window_value
            .request_animation_frame(dispatch_function)
            .is_ok()
    });
    if requested_frame {
        return;
    }
    SCHEDULED.store(false, Ordering::Relaxed);
}

/// Batches signal updates within a closure, deferring DOM synchronization until completion.
///
/// Saves the current `SUPPRESS_SCHEDULE` flag, sets it to `true`,
/// executes the closure, and restores the previous flag value.
/// This prevents `schedule_signal_update` from queuing microtasks
/// during the closure execution, allowing multiple signal mutations
/// to be applied before triggering a single DOM update cycle.
///
/// When the outermost `batch_updates` call completes (i.e., the previous
/// suppress flag was `false`), a single `schedule_signal_update()` is
/// invoked to ensure that any signal mutations performed inside the
/// closure are reflected in the DOM. This is critical for `watch!`
/// initialisation, where `Console::log` calls mutate the console signal
/// inside the batched block and must still trigger DynamicNode re-renders.
///
/// # Arguments
///
/// - `FnOnce() -> R` - The closure to execute with batched updates.
///
/// # Returns
///
/// - `R` - The result of the closure execution.
pub fn batch_updates<F, R>(callback: F) -> R
where
    F: FnOnce() -> R,
{
    let was_outermost: bool = !SUPPRESS_SCHEDULE.load(Ordering::Relaxed);
    SUPPRESS_SCHEDULE.store(true, Ordering::Relaxed);
    let result: R = callback();
    if was_outermost {
        SUPPRESS_SCHEDULE.store(false, Ordering::Relaxed);
        schedule_signal_update();
    } else {
        SUPPRESS_SCHEDULE.store(false, Ordering::Relaxed);
    }
    result
}

/// Subscribes an attribute signal to the global signal update dispatch cycle.
///
/// Creates a callback that re-computes the attribute value and sets
/// it on the signal whenever a signal update cycle runs. The callback
/// is registered in the signal update registry using the signal's
/// inner address as the key.
///
/// # Arguments
///
/// - `Signal<String>` - The attribute signal to subscribe.
/// - `Fn() -> String + 'static` - A closure that computes the current attribute value string.
pub(crate) fn subscribe_attr_signal<F>(attr_signal: Signal<String>, compute: F)
where
    F: Fn() -> String + 'static,
{
    register_attr_signal_listener(
        attr_signal.get_inner(),
        Box::new(move || {
            attr_signal.set_silent(compute());
        }),
    );
}

/// Converts a bool signal into a reactive `Signal<String>` attribute value.
///
/// Creates a `Signal<String>` initialized with the bool's string
/// representation, then subscribes to the source signal so that
/// whenever the bool changes, the string signal is updated accordingly.
///
/// # Arguments
///
/// - `Signal<bool>` - The source boolean signal.
///
/// # Returns
///
/// - `AttributeValue` - An `AttributeValue::Signal` wrapping the derived string signal.
pub(crate) fn bool_signal_to_string_attribute_value(source: Signal<bool>) -> AttributeValue {
    let string_signal: Signal<String> = Signal::create(source.get().to_string());
    let string_signal_clone: Signal<String> = string_signal;
    let source_for_sub: Signal<bool> = source;
    source_for_sub.replace_subscribe(move || {
        string_signal_clone.set_silent(source_for_sub.get().to_string());
    });
    AttributeValue::Signal(string_signal)
}

/// Returns a mutable reference to the current hook context.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
///
/// # Returns
///
/// - `&'static mut Option<HookContextRc>`: A mutable reference to the global hook context.
#[allow(static_mut_refs)]
pub(crate) fn current_hook_context_mut() -> &'static mut Option<HookContextRc> {
    unsafe { &mut *CURRENT_HOOK_CONTEXT.get_0().get() }
}

/// Returns a shared reference to the current hook context.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
///
/// # Returns
///
/// - `&'static Option<HookContextRc>`: A shared reference to the global hook context.
#[allow(static_mut_refs)]
pub(crate) fn current_hook_context() -> &'static Option<HookContextRc> {
    unsafe { &*CURRENT_HOOK_CONTEXT.get_0().get() }
}
