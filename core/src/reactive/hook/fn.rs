use crate::*;

/// Returns the currently active `HookContext`.
///
/// When called outside a `with_hook_context` scope, returns a reference
/// to the default empty context.
///
/// # Returns
///
/// - `HookContext` - The currently active hook context.
pub fn get_current_hook_context() -> HookContext {
    let address: usize = unsafe { CURRENT_HOOK_CONTEXT };
    if address == 0 {
        init_current_hook_context();
    }
    let current: usize = unsafe { CURRENT_HOOK_CONTEXT };
    current.into()
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
    let previous: usize = unsafe { CURRENT_HOOK_CONTEXT };
    unsafe {
        CURRENT_HOOK_CONTEXT = Into::<usize>::into(context);
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
    let ptr: *mut HookContextInner = Box::leak(ctx) as *mut HookContextInner;
    let address: usize = ptr as usize;
    address.into()
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
    let ctx: HookContext = get_current_hook_context();
    let ctx_inner: &mut HookContextInner = ctx.leak_mut();
    let index: usize = ctx_inner.get_hook_index();
    ctx_inner.set_hook_index(index + 1);
    if index < ctx_inner.get_hooks().len()
        && let Some(existing) = ctx_inner.get_hooks()[index].downcast_ref::<Signal<T>>()
    {
        return *existing;
    }
    let signal: Signal<T> = Signal::new(init());
    let cleanup_signal: Signal<T> = signal;
    ctx_inner
        .get_mut_cleanups()
        .push(Box::new(move || cleanup_signal.clear_listeners()));
    if index < ctx_inner.get_hooks().len() {
        ctx_inner.get_mut_hooks()[index] = Box::new(signal);
    } else {
        ctx_inner.get_mut_hooks().push(Box::new(signal));
    }
    signal
}
