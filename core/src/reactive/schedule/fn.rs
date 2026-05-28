use crate::*;

/// Ensures the `window.__euv_dispatch` callback is registered.
///
/// Creates a `Closure` that resets the `SCHEDULED` flag and directly
/// invokes `dispatch_signal_update_callbacks`, then stores it on the
/// `window` object so it can be invoked via `queueMicrotask`.
///
/// # Panics
///
/// Panics if `window()` returns `None`.
fn ensure_dispatch_callback() {
    let window_value: Window = window().unwrap();
    let key: JsValue = JsValue::from_str(EUV_DISPATCH);
    if Reflect::get(&window_value, &key)
        .unwrap_or(JsValue::UNDEFINED)
        .is_undefined()
    {
        let closure: closure::Closure<dyn FnMut()> = closure::Closure::wrap(Box::new(|| {
            SCHEDULED.store(false, Ordering::Relaxed);
            dispatch_signal_update_callbacks();
        }));
        let _ = Reflect::set(&window_value, &key, closure.as_ref());
        closure.forget();
    }
}

/// Schedules a deferred `NativeEventName::EuvSignalUpdate.to_string()` event via a microtask.
///
/// If a schedule is already pending (`SCHEDULED` is true) or updates
/// are suppressed (`SUPPRESS_SCHEDULE` is true), this is a no-op.
/// Otherwise, sets `SCHEDULED` to true and queues the
/// `window.__euv_dispatch` callback via `queueMicrotask` on WASM
/// targets. On non-WASM targets, resets `SCHEDULED` immediately
/// since there is no event loop to schedule on.
pub fn schedule_signal_update() {
    if SCHEDULED.load(Ordering::Relaxed) || SUPPRESS_SCHEDULE.load(Ordering::Relaxed) {
        return;
    }
    SCHEDULED.store(true, Ordering::Relaxed);
    let window_option: Option<Window> = window();
    if window_option.is_none() {
        SCHEDULED.store(false, Ordering::Relaxed);
        return;
    }
    ensure_dispatch_callback();
    let window_value: Window = window_option.unwrap();
    let dispatch_fn: JsValue =
        Reflect::get(&window_value, &JsValue::from_str(EUV_DISPATCH)).unwrap_or(JsValue::UNDEFINED);
    if dispatch_fn.is_undefined() {
        SCHEDULED.store(false, Ordering::Relaxed);
        return;
    }
    let queue_microtask_val: JsValue =
        Reflect::get(&window_value, &JsValue::from_str(QUEUE_MICROTASK))
            .unwrap_or(JsValue::UNDEFINED);
    if queue_microtask_val.is_undefined() {
        SCHEDULED.store(false, Ordering::Relaxed);
        return;
    }
    let queue_microtask: Function = queue_microtask_val.into();
    let _ = queue_microtask.call1(&JsValue::NULL, &dispatch_fn);
}

/// Batches signal updates within a closure, deferring DOM synchronization until completion.
///
/// Saves the current `SUPPRESS_SCHEDULE` flag, sets it to `true`,
/// executes the closure, and restores the previous flag value.
/// This prevents `schedule_signal_update` from queuing microtasks
/// during the closure execution, allowing multiple signal mutations
/// to be applied before triggering a single DOM update cycle.
///
/// # Arguments
///
/// - `F`- The closure to execute with batched updates.
///
/// # Returns
///
/// - `R`- The result of the closure execution.
pub fn batch_updates<F, R>(callback: F) -> R
where
    F: FnOnce() -> R,
{
    let previous: bool = SUPPRESS_SCHEDULE.load(Ordering::Relaxed);
    SUPPRESS_SCHEDULE.store(true, Ordering::Relaxed);
    let result: R = callback();
    SUPPRESS_SCHEDULE.store(previous, Ordering::Relaxed);
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
/// - `Signal<String>`- The attribute signal to subscribe.
/// - `F`- A closure that computes the current attribute value string.
pub(crate) fn subscribe_attr_signal<F>(attr_signal: Signal<String>, compute: F)
where
    F: Fn() -> String + 'static,
{
    let signal_key: usize = attr_signal.get_inner_addr();
    let callback: Box<dyn FnMut()> = Box::new(move || {
        let new_value: String = compute();
        attr_signal.set_silent(new_value);
    });
    register_attr_signal_listener(signal_key, callback);
}

/// Converts a bool signal into a reactive `Signal<String>` attribute value.
///
/// Creates a `Signal<String>` initialized with the bool's string
/// representation, then subscribes to the source signal so that
/// whenever the bool changes, the string signal is updated accordingly.
///
/// # Arguments
///
/// - `Signal<bool>`- The source boolean signal.
///
/// # Returns
///
/// - `AttributeValue`- An `AttributeValue::Signal` wrapping the derived string signal.
pub(crate) fn bool_signal_to_string_attribute_value(source: Signal<bool>) -> AttributeValue {
    let initial: String = source.get().to_string();
    let string_signal: Signal<String> = Signal::create(initial);
    let string_signal_clone: Signal<String> = string_signal;
    source.replace_subscribe({
        let source_inner: Signal<bool> = source;
        move || {
            let new_value: String = source_inner.get().to_string();
            string_signal_clone.set_silent(new_value);
        }
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
