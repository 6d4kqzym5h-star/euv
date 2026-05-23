use crate::*;

/// Dispatches a custom `__euv_signal_update__` event on the global window.
///
/// Used by the scheduler to trigger a DOM update cycle after signal changes.
/// Does nothing if the window object is unavailable.
pub(crate) fn dispatch_signal_update() {
    if let Some(win) = window() {
        let event: Event = Event::new("__euv_signal_update__").unwrap();
        let _ = win.dispatch_event(&event);
    }
}

/// Ensures the `window.__euv_dispatch` callback is registered.
///
/// Creates a `Closure` that resets the `SCHEDULED` flag and dispatches
/// the signal update event, then stores it on the `window` object
/// so it can be invoked via `queueMicrotask`.
///
/// # Panics
///
/// Panics if `window()` returns `None`.
fn ensure_dispatch_callback() {
    let win: Window = window().unwrap();
    let key: JsValue = JsValue::from_str("__euv_dispatch");
    if Reflect::get(&win, &key)
        .unwrap_or(JsValue::UNDEFINED)
        .is_undefined()
    {
        let closure: closure::Closure<dyn FnMut()> = closure::Closure::wrap(Box::new(|| {
            SCHEDULED.store(false, Ordering::Relaxed);
            dispatch_signal_update();
        }));
        let _ = Reflect::set(&win, &key, closure.as_ref());
        closure.forget();
    }
}

/// Schedules a deferred `__euv_signal_update__` event via a microtask.
///
/// If a schedule is already pending (`SCHEDULED` is true) or updates
/// are suppressed (`SUPPRESS_SCHEDULE` is true), this is a no-op.
/// Otherwise, sets `SCHEDULED` to true and queues the
/// `window.__euv_dispatch` callback via `queueMicrotask` on WASM
/// targets. On non-WASM targets, resets `SCHEDULED` immediately
/// since there is no event loop to schedule on.
pub(crate) fn schedule_signal_update() {
    if SCHEDULED.load(Ordering::Relaxed) || SUPPRESS_SCHEDULE.load(Ordering::Relaxed) {
        return;
    }
    SCHEDULED.store(true, Ordering::Relaxed);
    let win: Option<Window> = window();
    if win.is_none() {
        SCHEDULED.store(false, Ordering::Relaxed);
        return;
    }
    ensure_dispatch_callback();
    let win: Window = win.unwrap();
    let dispatch_fn: JsValue =
        Reflect::get(&win, &JsValue::from_str("__euv_dispatch")).unwrap_or(JsValue::UNDEFINED);
    if dispatch_fn.is_undefined() {
        SCHEDULED.store(false, Ordering::Relaxed);
        return;
    }
    let queue_microtask_val: JsValue =
        Reflect::get(&win, &JsValue::from_str("queueMicrotask")).unwrap_or(JsValue::UNDEFINED);
    if queue_microtask_val.is_undefined() {
        SCHEDULED.store(false, Ordering::Relaxed);
        return;
    }
    let queue_microtask: Function = queue_microtask_val.into();
    let _ = queue_microtask.call1(&JsValue::NULL, &dispatch_fn);
}

/// Executes a closure with signal update scheduling suppressed.
///
/// Saves the current `SUPPRESS_SCHEDULE` flag, sets it to `true`,
/// executes the closure, and restores the previous flag value.
/// This prevents `schedule_signal_update` from queuing microtasks
/// during the closure execution.
///
/// # Arguments
///
/// - `F`: The closure to execute with suppressed scheduling.
///
/// # Returns
///
/// - `R`: The result of the closure execution.
pub fn with_suppressed_updates<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let previous: bool = SUPPRESS_SCHEDULE.load(Ordering::Relaxed);
    SUPPRESS_SCHEDULE.store(true, Ordering::Relaxed);
    let result: R = f();
    SUPPRESS_SCHEDULE.store(previous, Ordering::Relaxed);
    result
}

/// Subscribes an attribute signal to the global `__euv_signal_update__` event.
///
/// Creates a callback that re-computes the attribute value and sets
/// it on the signal whenever a signal update cycle runs. The callback
/// is registered in the signal update registry using the signal's
/// inner address as the key.
///
/// # Arguments
///
/// - `Signal<String>`: The attribute signal to subscribe.
/// - `F`: A closure that computes the current attribute value string.
pub fn subscribe_attr_signal<F>(attr_signal: Signal<String>, compute: F)
where
    F: Fn() -> String + 'static,
{
    let signal_key: usize = attr_signal.get_inner_addr();
    let callback: Box<dyn FnMut()> = Box::new(move || {
        let new_value: String = compute();
        attr_signal.set(new_value);
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
/// - `Signal<bool>`: The source boolean signal.
///
/// # Returns
///
/// - `AttributeValue`: An `AttributeValue::Signal` wrapping the derived string signal.
pub(crate) fn bool_signal_to_string_attribute_value(source: Signal<bool>) -> AttributeValue {
    let initial: String = source.get().to_string();
    let string_signal: Signal<String> = Signal::create(initial);
    let string_signal_clone: Signal<String> = string_signal;
    source.replace_subscribe({
        let source_inner: Signal<bool> = source;
        move || {
            let new_value: String = source_inner.get().to_string();
            string_signal_clone.set(new_value);
        }
    });
    AttributeValue::Signal(string_signal)
}

/// Resets all scheduling and hook context state to their initial values.
///
/// Sets `SCHEDULED`, `SUPPRESS_SCHEDULE`, and `SIGNAL_UPDATE_DISPATCHING`
/// to `false`, and clears the `CURRENT_HOOK_CONTEXT` cell.
/// Used during application teardown to ensure a clean slate when
/// the WASM module is re-instantiated after a browser refresh.
pub fn reset_schedule_state() {
    SCHEDULED.store(false, Ordering::Relaxed);
    SUPPRESS_SCHEDULE.store(false, Ordering::Relaxed);
    SIGNAL_UPDATE_DISPATCHING.store(false, Ordering::Relaxed);
    *current_hook_context_mut() = None;
}

/// Returns a mutable reference to the current hook context.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
#[allow(static_mut_refs)]
pub(crate) fn current_hook_context_mut() -> &'static mut Option<HookContextRc> {
    unsafe { &mut *CURRENT_HOOK_CONTEXT.get_0().get() }
}

/// Returns a shared reference to the current hook context.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
#[allow(static_mut_refs)]
pub(crate) fn current_hook_context() -> &'static Option<HookContextRc> {
    unsafe { &*CURRENT_HOOK_CONTEXT.get_0().get() }
}
