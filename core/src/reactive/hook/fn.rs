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
    match current_hook_context() {
        Some(hook_context_rc) => HookContext::new(hook_context_rc.clone()),
        None => {
            let rc: HookContextRc = Rc::new(RefCell::new(HookContextInner::default()));
            *current_hook_context_mut() = Some(rc.clone());
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
    let previous: Option<HookContextRc> = current_hook_context_mut().take();
    *current_hook_context_mut() = Some(context.get_inner().clone());
    let result: R = f();
    *current_hook_context_mut() = previous;
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
    let mut inner: RefMut<HookContextInner> = hook_context.get_inner().borrow_mut();
    let index: usize = inner.get_hook_index();
    inner.set_hook_index(index + 1);
    if index < inner.get_hooks().len()
        && let Some(existing) = inner.get_hooks()[index].downcast_ref::<Signal<T>>()
    {
        return *existing;
    }
    let signal: Signal<T> = Signal::create(init());
    inner
        .get_mut_cleanups()
        .push(Box::new(move || signal.clear_listeners()));
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
    let mut inner: RefMut<HookContextInner> = hook_context.get_inner().borrow_mut();
    let index: usize = inner.get_hook_index();
    inner.set_hook_index(index + 1);
    if index < inner.get_hooks().len() {
        return;
    }
    inner.get_mut_cleanups().push(Box::new(cleanup));
    inner.get_mut_hooks().push(Box::new(()));
}
