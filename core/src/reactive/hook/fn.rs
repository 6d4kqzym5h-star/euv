use crate::*;

/// Returns the currently active `HookContext`.
///
/// If no hook context has been set, creates and stores a default one
/// in the global `CURRENT_HOOK_CONTEXT` cell so subsequent calls
/// return the same instance.
///
/// # Returns
///
/// - `HookContext` - The currently active hook context.
pub(crate) fn get_current_hook_context() -> HookContext {
    match try_get_current_hook_context() {
        Some(hook_context_rc) => HookContext::new(hook_context_rc.clone()),
        None => {
            let rc: HookContextRc = Rc::new(RefCell::new(HookContextInner::default()));
            *try_get_current_hook_context_mut() = Some(rc.clone());
            HookContext::new(rc)
        }
    }
}

/// Runs a closure with the given `HookContext` set as the active context.
///
/// Saves the previous context, sets the new one, executes the closure,
/// and restores the previous context afterward.
///
/// # Arguments
///
/// - `HookContext` - The hook context to set as active during closure execution.
/// - `FnOnce() -> R` - The closure to execute with the given context.
///
/// # Returns
///
/// - `R` - The result of the closure execution.
pub(crate) fn with_hook_context<F, R>(context: HookContext, f: F) -> R
where
    F: FnOnce() -> R,
{
    let previous: Option<HookContextRc> = try_get_current_hook_context_mut().take();
    *try_get_current_hook_context_mut() = Some(context.get_inner().clone());
    let result: R = f();
    *try_get_current_hook_context_mut() = previous;
    result
}

/// Creates a new `HookContext` with a fresh inner state.
///
/// Delegates to `HookContext::default()` which initializes an empty
/// hook storage list, a zero hook index, and an empty cleanup list.
///
/// # Returns
///
/// - `HookContext` - A newly created hook context with default state.
pub(crate) fn create_hook_context() -> HookContext {
    HookContext::default()
}

/// Creates a new reactive signal with the given initial value.
///
/// Uses the current `HookContext` to maintain signal identity across
/// re-renders. On the first call at a given hook index, the signal
/// is created with `init()` and stored. On subsequent re-renders,
/// the existing signal at that index is returned unchanged.
///
/// # Arguments
///
/// - `FnOnce() -> T` - A closure that computes the initial value of the signal.
///
/// # Returns
///
/// - `Signal<T>` - A reactive signal containing the initialized or existing value.
pub fn use_signal<T, F>(init: F) -> Signal<T>
where
    T: Clone + PartialEq + 'static,
    F: FnOnce() -> T,
{
    let hook_context: HookContext = get_current_hook_context();
    let Ok(mut inner) = hook_context.get_inner().try_borrow_mut() else {
        return Signal::create(init());
    };
    let index: usize = inner.get_hook_index();
    inner.set_hook_index(index + 1);
    if index < inner.get_hooks().len()
        && let Some(existing) = inner.get_hooks()[index].downcast_ref::<Signal<T>>()
    {
        return *existing;
    }
    let signal: Signal<T> = Signal::create(init());
    // Use `deactivate` on teardown — never free the allocation here.
    //
    // `Signal<T>` is `Copy` (just a `usize` address), so async callbacks
    // spawned from this component — `spawn_local` futures, `setTimeout` /
    // `setInterval` closures, Promise continuations — may still hold copies
    // of this signal after the hook context is torn down (e.g. a `match` arm
    // switch or a route change). Deallocating the `SignalInner` here would
    // turn any still-pending async callback's later `.get()` / `.set()` into
    // a dereference of a dangling pointer (use-after-free).
    //
    // `deactivate` marks the signal inactive and drops its listeners /
    // dependents but keeps the allocation alive, so stale async callbacks
    // become safe no-ops. This mirrors the contract documented on
    // `clear_signal_listeners_by_addr`, which the DOM cleanup path already
    // follows for the same reason.
    inner
        .get_mut_cleanups()
        .push(Box::new(move || signal.deactivate()));
    if index < inner.get_hooks().len() {
        inner.get_mut_hooks()[index] = Box::new(signal);
    } else {
        inner.get_mut_hooks().push(Box::new(signal));
    }
    signal
}

/// Registers a cleanup callback that will be executed when the current
/// hook context is cleared (e.g., when a `match` arm switches).
///
/// This is useful for cleaning up side effects like intervals, timeouts,
/// or subscriptions that are not automatically managed by signals.
///
/// The cleanup callback is only registered once on the first render.
/// On subsequent re-renders at the same hook index, this is a no-op.
///
/// # Arguments
///
/// - `FnOnce() + 'static` - The cleanup callback to execute on context teardown.
pub fn use_cleanup<F>(cleanup: F)
where
    F: FnOnce() + 'static,
{
    let hook_context: HookContext = get_current_hook_context();
    let Ok(mut inner) = hook_context.get_inner().try_borrow_mut() else {
        return;
    };
    let index: usize = inner.get_hook_index();
    inner.set_hook_index(index + 1);
    if index < inner.get_hooks().len() {
        return;
    }
    inner.get_mut_cleanups().push(Box::new(cleanup));
    inner.get_mut_hooks().push(Box::new(()));
}

/// Registers a `window.addEventListener` callback using event delegation,
/// automatically removed when the hook context is cleared.
///
/// Uses the global window event proxy registry so that only one
/// `window.addEventListener` call is made per event name regardless of
/// how many components listen to the same event. On cleanup, only the
/// handler entry is removed from the proxy registry; the shared window
/// listener remains active for other consumers.
///
/// The event listener is only registered once on the first render.
/// On subsequent re-renders at the same hook index, this is a no-op.
///
/// # Arguments
///
/// - `E: AsRef<str>` - The event name to listen for (e.g., "hashchange", "popstate", "resize").
/// - `FnMut() + 'static` - The callback to invoke when the event fires.
pub fn use_window_event<E, F>(event_name: E, callback: F)
where
    E: AsRef<str>,
    F: FnMut() + 'static,
{
    let event_name: &str = event_name.as_ref();
    let hook_context: HookContext = get_current_hook_context();
    let Ok(mut inner) = hook_context.get_inner().try_borrow_mut() else {
        return;
    };
    let index: usize = inner.get_hook_index();
    inner.set_hook_index(index + 1);
    if index < inner.get_hooks().len() {
        return;
    }
    let event_name_owned: String = event_name.to_owned();
    let handler_id: usize = register_window_event_handler(event_name, callback);
    inner.get_mut_cleanups().push(Box::new(move || {
        unregister_window_event_handler(&event_name_owned, handler_id);
    }));
    inner.get_mut_hooks().push(Box::new(()));
}

/// Creates a recurring interval that invokes the given closure at the
/// specified period, returning an `IntervalHandle` that is automatically
/// cleared when the hook context is cleared (i.e., when the component
/// unmounts or a `match` arm switches).
///
/// Unlike calling `set_interval_with_callback_and_timeout_and_arguments_0`
/// + `Closure::forget()` manually, this hook ensures the interval is
///   properly cleaned up, preventing memory leaks and stale callbacks.
///
/// The interval is only created once on the first render.
/// On subsequent re-renders at the same hook index, the existing handle
/// is returned unchanged.
///
/// # Arguments
///
/// - `i32` - The interval period in milliseconds.
/// - `FnMut() + 'static` - The closure to invoke on each interval tick.
///
/// # Returns
///
/// - `IntervalHandle` - A handle that can be used to cancel the interval early.
///
/// # Panics
///
/// Panics if `window()` is unavailable on the current platform.
pub fn use_interval<F>(millis: i32, callback: F) -> IntervalHandle
where
    F: FnMut() + 'static,
{
    let hook_context: HookContext = get_current_hook_context();
    let Ok(mut inner) = hook_context.get_inner().try_borrow_mut() else {
        return IntervalHandle::new(0);
    };
    let index: usize = inner.get_hook_index();
    inner.set_hook_index(index + 1);
    if index < inner.get_hooks().len()
        && let Some(existing) = inner.get_hooks()[index].downcast_ref::<IntervalHandle>()
    {
        return *existing;
    }
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(callback));
    let window: Window = window().expect("no global window exists");
    let interval_id: i32 = window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            millis,
        )
        .expect("failed to set interval");
    closure.forget();
    let handle: IntervalHandle = IntervalHandle::new(interval_id);
    inner.get_mut_cleanups().push(Box::new(move || {
        let Some(cleanup_window) = web_sys::window() else {
            return;
        };
        cleanup_window.clear_interval_with_handle(interval_id);
    }));
    if index < inner.get_hooks().len() {
        inner.get_mut_hooks()[index] = Box::new(handle);
    } else {
        inner.get_mut_hooks().push(Box::new(handle));
    }
    handle
}
