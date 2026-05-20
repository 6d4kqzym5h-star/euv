use crate::*;

/// Returns a mutable reference to the global effect subscriber registry.
///
/// Lazily initializes the registry on first access via `Box::leak`.
fn get_effect_subscriber_registry() -> &'static mut HashMap<usize, Vec<usize>> {
    let addr: usize = unsafe {
        match EFFECT_SUBSCRIBERS {
            Some(addr) => addr,
            None => {
                let registry: Box<HashMap<usize, Vec<usize>>> = Box::default();
                let leaked: &mut HashMap<usize, Vec<usize>> = Box::leak(registry);
                let addr: usize = leaked as *mut HashMap<usize, Vec<usize>> as usize;
                EFFECT_SUBSCRIBERS = Some(addr);
                addr
            }
        }
    };
    unsafe { &mut *(addr as *mut HashMap<usize, Vec<usize>>) }
}

/// Returns a mutable reference to the pending effects queue.
///
/// Lazily initializes the queue on first access via `Box::leak`.
fn get_pending_effects() -> &'static mut Vec<usize> {
    let addr: usize = unsafe {
        match PENDING_EFFECTS {
            Some(addr) => addr,
            None => {
                let queue: Box<Vec<usize>> = Box::default();
                let leaked: &mut Vec<usize> = Box::leak(queue);
                let addr: usize = leaked as *mut Vec<usize> as usize;
                PENDING_EFFECTS = Some(addr);
                addr
            }
        }
    };
    unsafe { &mut *(addr as *mut Vec<usize>) }
}

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
    RenderEffect::new(effect_fn)
}

/// Registers the current RenderEffect as a subscriber on the given signal.
///
/// Called by `Signal::get()` when a RenderEffect is active, establishing a
/// dependency so the effect re-runs when the signal changes. If this effect
/// has already subscribed to this signal, the subscription is skipped to
/// prevent duplicate entries.
///
/// # Arguments
///
/// - `&Signal<T>` - The signal to track.
/// - `usize` - The memory address of the RenderEffect's inner state.
pub(crate) fn track_signal<T>(signal: &Signal<T>, effect_addr: usize)
where
    T: Clone + PartialEq + 'static,
{
    let signal_addr: usize = signal.get_inner() as usize;
    let registry: &mut HashMap<usize, Vec<usize>> = get_effect_subscriber_registry();
    let subscribers: &mut Vec<usize> = registry.entry(signal_addr).or_default();
    if !subscribers.contains(&effect_addr) {
        subscribers.push(effect_addr);
    }
    let effect: RenderEffect = RenderEffect { inner: effect_addr };
    let inner: &mut RenderEffectInner = effect.leak_mut();
    if !inner.get_dependencies().contains(&signal_addr) {
        inner.get_mut_dependencies().push(signal_addr);
    }
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
    let registry: &mut HashMap<usize, Vec<usize>> = get_effect_subscriber_registry();
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

/// Checks whether RenderEffect notifications are currently paused.
///
/// # Returns
///
/// - `bool` - `true` if notifications are paused.
fn is_effect_notifications_paused() -> bool {
    unsafe { EFFECT_NOTIFICATIONS_PAUSED }
}

/// Sets the RenderEffect notifications paused state.
///
/// # Arguments
///
/// - `bool` - `true` to pause notifications, `false` to resume.
fn set_effect_notifications_paused(paused: bool) {
    unsafe {
        EFFECT_NOTIFICATIONS_PAUSED = paused;
    }
}

/// Checks whether an effect flush microtask has already been scheduled.
///
/// # Returns
///
/// - `bool` - `true` if a flush is already scheduled.
fn is_effect_flush_scheduled() -> bool {
    unsafe { EFFECT_FLUSH_SCHEDULED }
}

/// Sets the effect flush scheduled flag.
///
/// # Arguments
///
/// - `bool` - `true` to mark a flush as scheduled, `false` to clear.
fn set_effect_flush_scheduled(scheduled: bool) {
    unsafe {
        EFFECT_FLUSH_SCHEDULED = scheduled;
    }
}

/// Schedules all RenderEffects that depend on the given signal for
/// deferred execution via `queueMicrotask`.
///
/// Called by `Signal::set()` after notifying regular `listeners`. Instead
/// of executing effects synchronously (which can cause deep call stacks
/// and iterator invalidation), effect addresses are added to a pending
/// queue. A single `queueMicrotask` callback is scheduled to drain the
/// queue at the end of the current synchronous tick.
///
/// # Arguments
///
/// - `usize` - The signal's inner pointer address.
pub(crate) fn notify_effect_subscribers(signal_addr: usize) {
    if is_effect_notifications_paused() {
        let paused: &mut Vec<usize> = get_paused_effect_signals();
        if !paused.contains(&signal_addr) {
            paused.push(signal_addr);
        }
        return;
    }
    let effect_addrs: Vec<usize> = {
        let registry: &mut HashMap<usize, Vec<usize>> = get_effect_subscriber_registry();
        match registry.get(&signal_addr) {
            Some(subscribers) => subscribers.clone(),
            None => return,
        }
    };
    let pending: &mut Vec<usize> = get_pending_effects();
    for effect_addr in effect_addrs {
        if !pending.contains(&effect_addr) {
            pending.push(effect_addr);
        }
    }
    schedule_effect_flush();
}

/// Schedules a microtask to drain the pending effects queue.
///
/// Only one microtask is scheduled at a time; subsequent calls while a
/// microtask is pending are no-ops. The microtask callback takes ownership
/// of the current queue contents, runs each effect, and then checks for
/// any new effects that were added during execution.
fn schedule_effect_flush() {
    if is_effect_flush_scheduled() {
        return;
    }
    set_effect_flush_scheduled(true);
    #[cfg(target_arch = "wasm32")]
    {
        let callback: &'static js_sys::Function = get_or_create_flush_closure();
        let _ = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("queueMicrotask"))
            .and_then(|qmt| {
                let qmt_fn: js_sys::Function = qmt.into();
                qmt_fn.call1(&JsValue::NULL, callback)
            });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        flush_pending_effects();
    }
}

/// Returns the cached flush closure address, or `None` if not yet created.
///
/// # Returns
///
/// - `Option<usize>` - The address of the cached JS function, or `None`.
#[cfg(target_arch = "wasm32")]
fn get_flush_closure_addr() -> Option<usize> {
    unsafe { FLUSH_CLOSURE_FN }
}

/// Sets the cached flush closure address.
///
/// # Arguments
///
/// - `Option<usize>` - The address of the JS function to cache, or `None` to clear.
#[cfg(target_arch = "wasm32")]
fn set_flush_closure_addr(addr: Option<usize>) {
    unsafe {
        FLUSH_CLOSURE_FN = addr;
    }
}

/// Returns a static reference to the flush callback JS function.
///
/// On first call, creates the `Closure<dyn FnMut()>`, converts it to a
/// `js_sys::Function`, calls `Closure::forget()` to prevent dropping, and
/// caches the function reference in `FLUSH_CLOSURE_FN`. Subsequent calls
/// return the cached reference directly.
#[cfg(target_arch = "wasm32")]
fn get_or_create_flush_closure() -> &'static js_sys::Function {
    let addr: usize = match get_flush_closure_addr() {
        Some(addr) => addr,
        None => {
            let closure: wasm_bindgen::closure::Closure<dyn FnMut()> =
                wasm_bindgen::closure::Closure::wrap(Box::new(|| {
                    flush_pending_effects();
                }));
            let func: js_sys::Function =
                closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
            closure.forget();
            let leaked: &js_sys::Function = Box::leak(Box::new(func));
            let addr: usize = leaked as *const js_sys::Function as usize;
            set_flush_closure_addr(Some(addr));
            addr
        }
    };
    unsafe { &*(addr as *const js_sys::Function) }
}

/// Drains the pending effects queue, executing each effect exactly once.
///
/// Takes ownership of the current queue contents to avoid borrow conflicts
/// when effects modify the queue during execution. After draining, checks
/// if new effects were added during execution and recursively flushes.
/// Aborts after `MAX_FLUSH_ITERATIONS` cycles to prevent infinite loops.
fn flush_pending_effects() {
    set_effect_flush_scheduled(false);
    for _ in 0..MAX_FLUSH_ITERATIONS {
        let batch: Vec<usize> = {
            let pending: &mut Vec<usize> = get_pending_effects();
            take(pending)
        };
        if batch.is_empty() {
            return;
        }
        for effect_addr in batch {
            let effect: RenderEffect = RenderEffect { inner: effect_addr };
            if effect.leak_mut().get_disposed() {
                continue;
            }
            effect.run_once();
        }
    }
}

/// Returns a mutable reference to the paused effect signals queue.
///
/// Lazily initializes the queue on first access via `Box::leak`.
fn get_paused_effect_signals() -> &'static mut Vec<usize> {
    let addr: usize = unsafe {
        match PAUSED_EFFECT_SIGNALS {
            Some(addr) => addr,
            None => {
                let queue: Box<Vec<usize>> = Box::default();
                let leaked: &mut Vec<usize> = Box::leak(queue);
                let addr: usize = leaked as *mut Vec<usize> as usize;
                PAUSED_EFFECT_SIGNALS = Some(addr);
                addr
            }
        }
    };
    unsafe { &mut *(addr as *mut Vec<usize>) }
}

/// Pauses RenderEffect notifications.
///
/// While paused, `notify_effect_subscribers` collects signal addresses
/// but does not schedule effect execution. This is used during event
/// handler dispatch to prevent signal changes from triggering full
/// component re-renders via RenderEffect, while still allowing
/// fine-grained `replace_subscribe` listeners to update individual
/// DOM nodes (e.g., text content, attribute values).
pub(crate) fn pause_effect_notifications() {
    set_effect_notifications_paused(true);
}

/// Resumes RenderEffect notifications and flushes any suppressed signals.
///
/// After resuming, all signal changes that occurred while paused are
/// processed by `notify_effect_subscribers`, which schedules the
/// dependent effects for execution via `queueMicrotask`.
pub(crate) fn resume_effect_notifications() {
    let suppressed: Vec<usize> = {
        let paused: &mut Vec<usize> = get_paused_effect_signals();
        take(paused)
    };
    set_effect_notifications_paused(false);
    for signal_addr in suppressed {
        notify_effect_subscribers(signal_addr);
    }
}

/// Checks if there is currently an active RenderEffect being tracked.
///
/// # Returns
///
/// - `Option<usize>` - The address of the active effect, or `None`.
pub(crate) fn current_effect_addr() -> Option<usize> {
    unsafe { CURRENT_EFFECT }
}

/// Sets the currently active RenderEffect address.
///
/// Used by `Signal::set()` to temporarily clear the current effect before
/// notifying listeners, preventing listener callbacks that call `Signal::get()`
/// from establishing spurious dependencies.
///
/// # Arguments
///
/// - `Option<usize>` - The effect address to set, or `None` to clear.
pub(crate) fn set_current_effect(addr: Option<usize>) {
    unsafe {
        CURRENT_EFFECT = addr;
    }
}

/// Atomically swaps the current RenderEffect address with a new value.
///
/// Used by `RenderEffect::run_once` to save and restore the previous
/// tracking target around effect closure execution.
///
/// # Arguments
///
/// - `Option<usize>` - The new effect address to set.
///
/// # Returns
///
/// - `Option<usize>` - The previous effect address that was replaced.
pub(crate) fn swap_current_effect(new_addr: Option<usize>) -> Option<usize> {
    unsafe {
        let old: Option<usize> = CURRENT_EFFECT;
        CURRENT_EFFECT = new_addr;
        old
    }
}
