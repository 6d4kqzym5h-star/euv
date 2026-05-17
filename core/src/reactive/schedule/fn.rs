use crate::*;

/// Dispatches the global `__euv_signal_update__` event on the window.
///
/// This triggers any `DynamicNode` listeners that are subscribed via
/// `window.addEventListener("__euv_signal_update__", ...)` so that
/// dynamic virtual DOM nodes can re-render when signal values change.
///
/// # Panics
///
/// Panics if `Event::new("__euv_signal_update__")` fails, which should
/// never happen for a valid event type string.
#[cfg(target_arch = "wasm32")]
pub(crate) fn dispatch_signal_update() {
    if let Some(win) = window() {
        let event: Event = Event::new("__euv_signal_update__").unwrap();
        let _ = win.dispatch_event(&event);
    }
}

/// Ensures the global `window.__euv_dispatch` callback is registered.
///
/// On first call, creates a single `Closure` that resets the `SCHEDULED`
/// flag and dispatches the signal update event, then installs it as
/// `window.__euv_dispatch` via `Reflect::set`. Subsequent calls
/// are no-ops because the function is already present on the window object.
///
/// This avoids the per-call `Closure::forget()` memory leak that would
/// otherwise occur if a new closure were allocated on every signal update.
///
/// # Panics
///
/// Panics if `window()` returns `None` or if `Reflect::set` fails.
#[cfg(target_arch = "wasm32")]
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
/// Batches multiple signal updates within the same synchronous tick into
/// a single dispatch that runs after all local listeners have completed,
/// preventing DynamicNode re-renders from interfering with in-flight
/// signal updates.
///
/// When `SUPPRESS_SCHEDULE` is `true`, this function is a no-op so that
/// internal operations (such as `watch!` initialisation) can perform
/// signal mutations without triggering premature DOM re-renders.
///
/// Uses `queueMicrotask` to schedule the pre-registered
/// `window.__euv_dispatch` callback, avoiding repeated `Closure::forget()`
/// memory leaks. The dispatch callback is registered once via
/// `ensure_dispatch_callback` and reused on every call.
///
/// # Panics
///
/// Panics if `window()` returns `None` or if `Reflect::get` / `Function::call` fails.
pub(crate) fn schedule_signal_update() {
    if SCHEDULED.load(Ordering::Relaxed) || SUPPRESS_SCHEDULE.load(Ordering::Relaxed) {
        return;
    }
    SCHEDULED.store(true, Ordering::Relaxed);
    #[cfg(target_arch = "wasm32")]
    {
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
    #[cfg(not(target_arch = "wasm32"))]
    {
        SCHEDULED.store(false, Ordering::Relaxed);
    }
}

/// Executes a closure with signal update scheduling suppressed.
///
/// Any `schedule_signal_update()` calls that occur within the closure
/// (including those triggered by `Signal::set()`) are silently ignored.
/// After the closure returns, the suppress flag is restored to its
/// previous value.
///
/// This is used internally by `watch!` to prevent its initial body
/// execution from triggering unnecessary DynamicNode re-renders.
///
/// # Arguments
///
/// - `FnOnce() -> R` - The closure to execute with suppressed scheduling.
///
/// # Returns
///
/// - `R` - The return value of the closure.
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

/// Subscribes an attribute signal to the global `__euv_signal_update__` event so that
/// whenever any signal changes, the attribute value is recomputed and the attribute
/// signal is updated. This enables reactive `if` conditions inside any HTML attribute,
/// including `style`, `class`, and others.
///
/// Works identically to the DOM-level `if {expr} { children }` mechanism: both
/// re-evaluate their condition expressions when any signal dispatches the global
/// update event, then apply only the minimal diff to the DOM.
///
/// # Arguments
///
/// - `Signal<String>` - The attribute signal to update when signals change.
/// - `Fn() -> String + 'static` - A closure that recomputes the attribute value string.
///
/// # Panics
///
/// Panics if `window()` is unavailable on the current platform.
pub fn subscribe_attr_signal<F>(attr_signal: Signal<String>, compute: F)
where
    F: Fn() -> String + 'static,
{
    #[cfg(target_arch = "wasm32")]
    {
        let signal_key: usize = Into::<usize>::into(attr_signal);
        let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let new_value: String = compute();
            attr_signal.set(new_value);
        }));
        register_attr_signal_listener(signal_key, closure);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = attr_signal;
        let _ = compute;
    }
}

/// Converts a bool signal into a reactive `Signal<String>` that
/// yields `"true"` or `"false"`, enabling boolean attributes like `checked` to
/// reactively update the DOM.
///
/// # Arguments
///
/// - `Signal<bool>` - The source boolean signal to convert.
///
/// # Returns
///
/// - `AttributeValue` - A signal-backed attribute value that reactively mirrors the boolean as a string.
pub(crate) fn bool_signal_to_string_attribute_value(source: Signal<bool>) -> AttributeValue {
    let initial: String = source.get().to_string();
    let string_signal: Signal<String> = Signal::new(initial);
    let string_signal_clone: Signal<String> = string_signal;
    source.subscribe({
        let source_inner: Signal<bool> = source;
        move || {
            let new_value: String = source_inner.get().to_string();
            string_signal_clone.set(new_value);
        }
    });
    AttributeValue::Signal(string_signal)
}

/// Initializes `CURRENT_HOOK_CONTEXT` to point to `DEFAULT_HOOK_CONTEXT`.
///
/// Must be called once at program startup before any hook context operations.
pub(crate) fn init_current_hook_context() {
    unsafe {
        CURRENT_HOOK_CONTEXT = DEFAULT_HOOK_CONTEXT.get_inner() as *mut HookContextInner as usize;
    }
}
