use crate::*;

/// Creates a new `RenderEffect` that automatically tracks signal dependencies
/// and re-executes when any dependency changes.
///
/// This is the public API for creating reactive effects. During the first
/// execution, any `Signal::get()` calls establish dependencies. When those
/// signals change, the closure is re-executed with fresh dependency tracking.
///
/// # Arguments
///
/// - `F` - The closure to execute reactively.
///
/// # Returns
///
/// - `RenderEffect` - A handle to the created effect.
pub fn create_render_effect<F>(effect_fn: F) -> RenderEffect
where
    F: FnMut() + 'static,
{
    RenderEffect::create(effect_fn)
}

/// Removes all effect subscriber entries for the given effect from the
/// global registry and clears the effect's dependency list.
///
/// Called at the beginning of each `run_once` to clean up old
/// subscriptions before re-tracking fresh dependencies.
///
/// # Arguments
///
/// - `usize` - The effect's inner pointer address.
/// - `&mut Vec<usize>` - The effect's current dependency list (will be cleared).
pub(crate) fn cleanup_effect_dependencies(effect_addr: usize, dependencies: &mut Vec<usize>) {
    let registry: &mut HashMap<usize, Vec<usize>> = effect_subscribers_mut();
    for signal_addr in dependencies.iter() {
        if let Some(subscribers) = registry.get_mut(signal_addr) {
            subscribers.retain(|addr| *addr != effect_addr);
            if subscribers.is_empty() {
                registry.remove(signal_addr);
            }
        }
    }
    dependencies.clear();
}

/// Ensures the effect subscribers registry is initialized and returns a mutable reference.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
#[allow(static_mut_refs)]
fn ensure_effect_subscribers_mut() -> &'static mut HashMap<usize, Vec<usize>> {
    unsafe {
        if (*EFFECT_SUBSCRIBERS.get_0().get()).is_none() {
            (*EFFECT_SUBSCRIBERS.get_0().get()) = Some(HashMap::new());
        }
        (*EFFECT_SUBSCRIBERS.get_0().get())
            .as_mut()
            .unwrap_unchecked()
    }
}

/// Returns a mutable reference to the effect subscribers registry.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
#[allow(static_mut_refs)]
pub(crate) fn effect_subscribers_mut() -> &'static mut HashMap<usize, Vec<usize>> {
    ensure_effect_subscribers_mut()
}

/// Sets the currently active RenderEffect address.
///
/// # Arguments
///
/// - `Option<usize>`: The effect address to set, or `None` to clear.
#[allow(static_mut_refs)]
pub(crate) fn set_current_effect(addr: Option<usize>) {
    unsafe {
        *CURRENT_EFFECT.get_0().get() = addr;
    }
}

/// Atomically swaps the current RenderEffect address with a new value.
///
/// # Arguments
///
/// - `Option<usize>`: The new effect address to set.
///
/// # Returns
///
/// - `Option<usize>`: The previous effect address that was replaced.
#[allow(static_mut_refs)]
pub(crate) fn swap_current_effect(new_addr: Option<usize>) -> Option<usize> {
    unsafe {
        let old: Option<usize> = *CURRENT_EFFECT.get_0().get();
        *CURRENT_EFFECT.get_mut_0().get() = new_addr;
        old
    }
}

/// Returns a mutable reference to the render effect registry.
///
/// SAFETY: Must only be called from the main thread (WASM single-threaded context).
#[allow(static_mut_refs)]
pub(crate) fn render_effect_registry_mut()
-> &'static mut HashMap<usize, Rc<RefCell<RenderEffectInner>>> {
    unsafe {
        if (*RENDER_EFFECT_REGISTRY.get_0().get()).is_none() {
            (*RENDER_EFFECT_REGISTRY.get_0().get()) = Some(HashMap::new());
        }
        (*RENDER_EFFECT_REGISTRY.get_0().get())
            .as_mut()
            .unwrap_unchecked()
    }
}
