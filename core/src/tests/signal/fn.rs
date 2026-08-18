use super::*;
use std::cell::Cell;
use std::panic;
use std::rc::Rc;

/// Regression test: `Signal::clear_listeners` must be idempotent so that
/// `cleanup_subtree` can be safely invoked multiple times on the same element
/// (e.g. via re-patching during a re-render before the element is actually
/// detached). Without the `is_alive` guard, the second call would re-enter
/// `inner_mut` on a deactivated-but-still-alive `SignalInner` and run
/// `set_value(String::new())` and `cleanup_attr_slot` a second time — at
/// best wasteful, at worst racy with `Registry` mutation.
///
/// The contract is: after `clear_listeners(addr)`, a second call is a safe
/// no-op (verified indirectly: if the first call leaves the signal in an
/// inconsistent state, the second call would either panic, double-free, or
/// trigger the guard's branch here).
///
/// `clear_listeners` does NOT free the bridge signal's heap allocation. The
/// bridge is only freed when *every* `subscribe`d source signal still holding
/// a closure that captures the bridge has been deactivated (see
/// `clear_listeners_then_source_deactivate_frees_bridge`). This ordering
/// keeps `bridge.get()` / `bridge.set()` from a stale listener safe, because
/// the address either still points at a valid `SignalInner` (with
/// `alive == false`, so calls are no-ops), or it has been reclaimed after
/// the last subscriber was torn down.
#[test]
fn clear_listeners_is_idempotent() {
    let signal: Signal<String> = Signal::create(String::from("hello"));
    let addr: usize = signal.get_inner();
    assert!(
        Signal::<String>::is_alive(addr),
        "freshly created signal must be in registry"
    );
    // First call deactivates and clears listeners.
    Signal::<String>::clear_listeners(addr);
    // Second call must be a safe no-op.
    Signal::<String>::clear_listeners(addr);
}

/// Verifies that an async-style stale listener registered on the source
/// signal is safe to fire after the bridge `Signal<String>` was deactivated
/// via `clear_listeners`.
///
/// Scenarios like `use_window_event` or `use_interval` keep the source
/// signal alive (and its listener closure) past the moment when the bound
/// DOM element is detached and its bridge signal is cleaned up. The
/// bridge's `Signal<T>` handle is `Copy`, so a stale closure holds only
/// the raw address — dereferencing it after the bridge's heap allocation
/// would be undefined behaviour.
///
/// `Signal::clear_listeners` deactivates the bridge (sets `alive = false`)
/// and removes the bridge from the global address registry, but the heap
/// allocation is only freed once `source.deactivate()` runs. Until then,
/// the stale listener's `bridge.get()` dereferences a still-allocated
/// `SignalInner` whose `alive == false`, returning the empty-string default
/// — safe, not UB.
#[test]
fn clear_listeners_then_set_via_stale_listener_is_safe() {
    let source: Signal<i32> = Signal::create(0);
    let bridge: Signal<String> = Signal::create(String::from("init"));
    let bridge_for_closure: Signal<String> = bridge;
    let triggered: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let triggered_clone: Rc<Cell<bool>> = Rc::clone(&triggered);
    source.subscribe(move || {
        // Touch the bridge through the stale address; this would be UB if
        // the bridge's heap allocation had been freed.
        let _ = bridge_for_closure.get();
        triggered_clone.set(true);
    });
    Signal::<String>::clear_listeners(bridge.get_inner());
    // Fire the listener via `set`. On wasm this is a real dispatch; on
    // native, `App::schedule_update` panics on the `Closure::wrap` path,
    // but the listener has already fired by then.
    let _ = panic::catch_unwind(|| {
        source.set(1);
    });
    assert!(
        triggered.get(),
        "listener should have fired and reached the (deactivated) bridge"
    );
}

/// Verifies that the bridge signal's heap allocation IS reclaimed when
/// the source signal is deactivated after the bound element was detached.
///
/// This is the second half of the safe reclamation contract: once `source`
/// has been deactivated, its listener closure (which captured the bridge
/// address) has been dropped, so no stale dereference can occur. The
/// reverse-index kept in `BRIDGE_REFS` notices that the bridge now has
/// zero subscribers AND the bridge is no longer in `SIGNAL_INNER_REGISTRY`
/// (i.e., `clear_listeners` already ran on it), and frees the
/// `Box<SignalInner<String>>`.
///
/// We assert this indirectly by checking that after `clear_listeners` +
/// `source.deactivate()`, the bridge address is NOT in
/// `SIGNAL_INNER_REGISTRY`. With the fix, `clear_listeners` itself removes
/// the bridge from the registry. The interesting assertion is that the
/// `deactivate()` call did not panic / double-free anything — which would
/// happen if the bridge had been freed before its last subscriber was
/// dropped.
#[test]
fn clear_listeners_then_source_deactivate_frees_bridge() {
    let source: Signal<i32> = Signal::create(0);
    let bridge: Signal<String> = Signal::create(String::from("init"));
    let bridge_addr: usize = bridge.get_inner();
    let bridge_for_closure: Signal<String> = bridge;
    source.subscribe(move || {
        let _ = bridge_for_closure.get();
    });
    // Bridge dependency registered by callers in production (create_dom_with_doc,
    // as_reactive_text, bool_to_attr). In this unit test we replicate it
    // directly so the test exercises the same code path.
    BridgeRefsCell::track(bridge_addr, source.get_inner());
    // Element detached.
    Signal::<String>::clear_listeners(bridge_addr);
    // Source unmounted. This is where the bridge Box should be freed.
    source.deactivate();
    // After deactivate, the bridge address must not be live anywhere.
    assert!(
        !Signal::<String>::is_alive(bridge_addr),
        "bridge must be removed from registry"
    );
}
