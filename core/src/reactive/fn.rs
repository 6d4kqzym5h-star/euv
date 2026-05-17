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
        let signal_key: usize = attr_signal.inner as usize;
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

/// Returns the currently active `HookContext`.
///
/// When called outside a `with_hook_context` scope, returns a reference
/// to the default empty context.
///
/// # Returns
///
/// - `HookContext` - The currently active hook context.
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
/// - `HookContext` - The hook context to set as active.
/// - `FnOnce() -> R` - The closure to execute with the active context.
///
/// # Returns
///
/// - `R` - The return value of the closure.
pub fn with_hook_context<F, R>(context: HookContext, f: F) -> R
where
    F: FnOnce() -> R,
{
    let previous: *mut HookContextInner = unsafe { CURRENT_HOOK_CONTEXT };
    unsafe {
        CURRENT_HOOK_CONTEXT = context.inner;
    }
    let result: R = f();
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
/// - `HookContext` - A handle to the newly allocated hook context.
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
/// - `FnOnce() -> T` - A closure that returns the initial value of the signal.
///
/// # Returns
///
/// - `Signal<T>` - A mutable handle to the newly created or persisted reactive signal.
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
    let signal: Signal<T> = Signal::new(init());
    let cleanup_signal: Signal<T> = signal;
    ctx.get_mut_cleanups()
        .push(Box::new(move || cleanup_signal.clear_listeners()));
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
