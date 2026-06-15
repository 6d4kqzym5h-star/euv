use crate::*;

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

/// Schedules a deferred signal update with precise dirty marking.
///
/// Marks only the specified dynamic node IDs as dirty, then queues a
/// single microtask dispatch if one is not already pending. When
/// `SUPPRESS_SCHEDULE` is `true`, slots are still marked dirty but no
/// dispatch is scheduled, allowing `batch` to batch
/// precise dirty marks without triggering premature DOM updates.
///
/// # Arguments
///
/// - `&[usize]` - Dynamic node IDs to mark dirty.
pub fn schedule_update(dependents: &[usize]) {
    mark_slots_dirty_targeted(dependents);
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

/// Batches signal updates within a closure, deferring DOM dispatch until the
/// outermost batch completes.
///
/// Sets `SUPPRESS_SCHEDULE` to `true` so that any `Signal::set()` calls
/// inside the closure mark their dependents dirty precisely but do not
/// queue a microtask dispatch. When the outermost batch completes,
/// a single dispatch is scheduled if any dirty slots were accumulated
/// during the batch, ensuring that all pending updates are processed.
///
/// Unlike the legacy full-broadcast approach, this uses precise dependency
/// tracking: only the dynamic nodes that actually depend on the changed
/// signals are marked dirty and re-rendered.
///
/// # Arguments
///
/// - `FnOnce() -> R` - The closure to execute with batched updates.
///
/// # Returns
///
/// - `R` - The result of the closure execution.
pub fn batch<F, R>(callback: F) -> R
where
    F: FnOnce() -> R,
{
    let was_outermost: bool = !SUPPRESS_SCHEDULE.load(Ordering::Relaxed);
    SUPPRESS_SCHEDULE.store(true, Ordering::Relaxed);
    let result: R = callback();
    SUPPRESS_SCHEDULE.store(!was_outermost, Ordering::Relaxed);
    if was_outermost && has_dirty_slots() {
        schedule_update(&[]);
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
pub(crate) fn subscribe_attr<F>(attr_signal: Signal<String>, compute: F)
where
    F: Fn() -> String + 'static,
{
    register_attr_signal_listener(
        attr_signal.get_inner(),
        Box::new(move || {
            attr_signal.set(compute());
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
pub(crate) fn bool_to_attr(source: Signal<bool>) -> AttributeValue {
    let string_signal: Signal<String> = Signal::create(source.get().to_string());
    let string_signal_clone: Signal<String> = string_signal;
    let source_for_sub: Signal<bool> = source;
    source_for_sub.subscribe(move || {
        string_signal_clone.set(source_for_sub.get().to_string());
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
pub(crate) fn try_get_current_hook_context_mut() -> &'static mut Option<HookContextRc> {
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
pub(crate) fn try_get_current_hook_context() -> &'static Option<HookContextRc> {
    unsafe { &*CURRENT_HOOK_CONTEXT.get_0().get() }
}
