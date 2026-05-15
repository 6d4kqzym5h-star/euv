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

/// Schedules a deferred `__euv_signal_update__` event via a microtask.
///
/// Batches multiple signal updates within the same synchronous tick into
/// a single dispatch that runs after all local listeners have completed,
/// preventing DynamicNode re-renders from interfering with in-flight
/// signal updates.
///
/// # Panics
///
/// Panics if `Promise::new()` or `Event::new()` fails.
pub(crate) fn schedule_signal_update() {
    if SCHEDULED.load(Ordering::Relaxed) {
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
        let promise: js_sys::Promise = js_sys::Promise::resolve(&wasm_bindgen::JsValue::NULL);
        let closure: wasm_bindgen::closure::Closure<dyn FnMut(wasm_bindgen::JsValue)> =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |_value: wasm_bindgen::JsValue| {
                SCHEDULED.store(false, Ordering::Relaxed);
                dispatch_signal_update();
            }));
        let _ = promise.then(&closure);
        closure.forget();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        SCHEDULED.store(false, Ordering::Relaxed);
    }
}

/// Dispatches the global `__euv_signal_update__` event on the window.
///
/// This is the public entry point for manually triggering a signal update
/// cycle, which causes all `DynamicNode` instances to re-render.
///
/// # Panics
///
/// Panics if `Event::new("__euv_signal_update__")` fails.
pub fn trigger_update() {
    schedule_signal_update();
}

/// Returns the currently active `HookContext`.
///
/// When called outside a `with_hook_context` scope, returns a reference
/// to the default empty context.
///
/// # Returns
///
/// - `HookContext`: The currently active hook context.
fn get_current_hook_context() -> HookContext {
    unsafe { HookContext::from_inner(CURRENT_HOOK_CONTEXT) }
}

/// Runs a closure with the given `HookContext` set as the active context.
///
/// This is called by the renderer before invoking a `DynamicNode`'s
/// render function, enabling `use_signal` and other hooks to access
/// and persist state across re-renders.
///
/// # Arguments
///
/// - `HookContext`: The hook context to set as active.
/// - `F`: The closure to execute with the active context.
///
/// # Returns
///
/// - `R`: The return value of the closure.
pub fn with_hook_context<F, R>(context: HookContext, f: F) -> R
where
    F: FnOnce() -> R,
{
    // SAFETY: WASM is single-threaded; no data race on CURRENT_HOOK_CONTEXT.
    let previous: *mut HookContextInner = unsafe { CURRENT_HOOK_CONTEXT };
    // SAFETY: Same as above.
    unsafe {
        CURRENT_HOOK_CONTEXT = context.inner;
    }
    let result: R = f();
    // SAFETY: Same as above.
    unsafe {
        CURRENT_HOOK_CONTEXT = previous;
    }
    result
}

/// Creates a new `HookContext` allocated via `Box::leak`.
///
/// The allocated memory lives for the remainder of the program and will
/// never be freed. This is acceptable for WASM single-threaded contexts
/// where `DynamicNode` instances persist for the application lifetime.
///
/// # Returns
///
/// - `HookContext`: A handle to the newly allocated hook context.
pub fn create_hook_context() -> HookContext {
    let ctx: Box<HookContextInner> = Box::default();
    HookContext::from_inner(Box::leak(ctx) as *mut HookContextInner)
}

/// Creates a new reactive signal with the given initial value.
///
/// When called inside a `DynamicNode` render function (within a
/// `with_hook_context` scope), the signal state is persisted across
/// re-renders by storing it in the active `HookContext`. Subsequent
/// re-renders return the same signal handle, preserving its current value.
///
/// When called outside a hook context, a fresh signal is created each time.
///
/// # Arguments
///
/// - `FnOnce() -> T`: A closure that returns the initial value of the signal.
///
/// # Returns
///
/// - `Signal<T>`: A mutable handle to the newly created or persisted reactive signal.
pub fn use_signal<T, F>(init: F) -> Signal<T>
where
    T: Clone + PartialEq + 'static,
    F: FnOnce() -> T,
{
    let mut ctx: HookContext = get_current_hook_context();
    let index: usize = ctx.get_hook_index();
    ctx.set_hook_index(index + 1_usize);
    if index < ctx.get_hooks().len()
        && let Some(existing) = ctx.get_hooks()[index].downcast_ref::<Signal<T>>()
    {
        return *existing;
    }
    let signal: Signal<T> = {
        let boxed: Box<SignalInner<T>> = Box::new(SignalInner::new(init()));
        Signal::from_inner(Box::leak(boxed) as *mut SignalInner<T>)
    };
    if index < ctx.get_hooks().len() {
        ctx.get_mut_hooks()[index] = Box::new(signal);
    } else {
        ctx.get_mut_hooks().push(Box::new(signal));
    }
    signal
}

/// Converts a bool signal into a reactive `Signal<String>` that
/// yields `"true"` or `"false"`, enabling boolean attributes like `checked` to
/// reactively update the DOM.
///
/// # Arguments
///
/// - `Signal<bool>`: The source boolean signal to convert.
///
/// # Returns
///
/// - `AttributeValue`: A signal-backed attribute value that reactively mirrors the boolean as a string.
pub(crate) fn bool_signal_to_string_attribute_value(source: Signal<bool>) -> AttributeValue {
    let initial: String = source.get().to_string();
    let string_signal: Signal<String> = {
        let inner: SignalInner<String> = SignalInner::new(initial);
        let boxed: Box<SignalInner<String>> = Box::new(inner);
        Signal::from_inner(Box::leak(boxed) as *mut SignalInner<String>)
    };
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
