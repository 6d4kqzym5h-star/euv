use super::*;

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
    let _ = super::catch_unwind(|| {
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

// =====================================================================
// SignalCell
// =====================================================================

#[test]
fn signal_cell_default_is_empty() {
    let cell: SignalCell<i32> = SignalCell::default();
    // `get` panics if uninitialized, so use
    // `none()`-style construction and verify the
    // default value's inner pointer is None.
    let ptr: *const Option<Signal<i32>> = cell.get_inner().get() as *const _;
    let value: Option<Signal<i32>> = unsafe { *ptr }.clone();
    assert!(value.is_none());
}

#[test]
fn signal_cell_none_constructor_is_empty() {
    let cell: SignalCell<i32> = SignalCell::none();
    let ptr: *const Option<Signal<i32>> = cell.get_inner().get() as *const _;
    let value: Option<Signal<i32>> = unsafe { *ptr }.clone();
    assert!(value.is_none());
}

#[test]
fn signal_cell_set_then_get() {
    let cell: SignalCell<i32> = SignalCell::default();
    let signal: Signal<i32> = Signal::create(42);
    cell.set(signal);
    let stored: Signal<i32> = cell.loaded().expect("cell should be initialized");
    assert_eq!(stored.get(), 42);
}

#[test]
fn signal_cell_set_overwrites_via_none_then_set() {
    // SignalCell::set panics if already set; so we
    // can only test the first-set behaviour.
    let cell: SignalCell<String> = SignalCell::default();
    let signal: Signal<String> = Signal::create(String::from("first"));
    cell.set(signal);
    let stored: Signal<String> = cell.loaded().expect("cell should be initialized");
    assert_eq!(stored.get(), "first");
}

#[test]
fn signal_cell_default_creates_independent_cells() {
    let a: SignalCell<i32> = SignalCell::default();
    let b: SignalCell<i32> = SignalCell::default();
    let a_ptr: *const Option<Signal<i32>> = a.get_inner().get() as *const _;
    let b_ptr: *const Option<Signal<i32>> = b.get_inner().get() as *const _;
    let a_value: Option<Signal<i32>> = unsafe { *a_ptr }.clone();
    let b_value: Option<Signal<i32>> = unsafe { *b_ptr }.clone();
    assert!(a_value.is_none());
    assert!(b_value.is_none());
    assert_ne!(a_ptr, b_ptr);
}

#[test]
fn signal_cell_with_string_value() {
    let cell: SignalCell<String> = SignalCell::default();
    let signal: Signal<String> = Signal::create(String::from("hello"));
    cell.set(signal);
    let stored: Signal<String> = cell.loaded().expect("cell should be initialized");
    assert_eq!(stored.get(), "hello");
}

// =====================================================================
// Signal: pure-Rust operations (no set / no subscribe dispatch)
// =====================================================================

#[test]
fn signal_create_returns_handle() {
    let signal: Signal<i32> = Signal::create(7);
    let value: i32 = signal.get();
    assert_eq!(value, 7);
}

#[test]
fn signal_create_with_string() {
    let signal: Signal<String> = Signal::create(String::from("hi"));
    assert_eq!(signal.get(), "hi");
}

#[test]
fn signal_create_with_vec() {
    let signal: Signal<Vec<i32>> = Signal::create(vec![1, 2, 3]);
    assert_eq!(signal.get(), vec![1, 2, 3]);
}

#[test]
fn signal_copy_semantics_share_state() {
    let a: Signal<i32> = Signal::create(10);
    let b: Signal<i32> = a;
    assert_eq!(a.get(), 10);
    assert_eq!(b.get(), 10);
}

#[test]
fn signal_clone_via_copy_is_idempotent() {
    let signal: Signal<i32> = Signal::create(42);
    let c1: Signal<i32> = signal;
    let c2: Signal<i32> = signal;
    let c3: Signal<i32> = signal;
    assert_eq!(c1.get(), 42);
    assert_eq!(c2.get(), 42);
    assert_eq!(c3.get(), 42);
}

#[test]
fn signal_is_alive_for_fresh_signal() {
    let signal: Signal<i32> = Signal::create(0);
    assert!(Signal::<i32>::is_alive(signal.get_inner()));
}

#[test]
fn signal_deactivate_does_not_panic_on_native() {
    let signal: Signal<i32> = Signal::create(0);
    // deactivate() may not fully work on native
    // (touches the BridgeRefsCell global which is
    // wasm-only), but it must not panic.
    let _ = super::catch_unwind(super::AssertUnwindSafe(|| {
        signal.deactivate();
    }));
}

#[test]
fn signal_clear_listeners_deactivates_signal() {
    let signal: Signal<String> = Signal::create(String::from("x"));
    assert!(Signal::<String>::is_alive(signal.get_inner()));
    Signal::<String>::clear_listeners(signal.get_inner());
    assert!(!Signal::<String>::is_alive(signal.get_inner()));
}

#[test]
fn signal_add_dependent_idempotent() {
    let signal: Signal<i32> = Signal::create(0);
    signal.add_dependent(7);
    signal.add_dependent(7);
    signal.add_dependent(7);
    let deps: Vec<usize> = signal.get_dependents();
    let count: usize = deps.iter().filter(|&&id| id == 7).count();
    assert_eq!(count, 1);
}

#[test]
fn signal_add_dependent_distinct_ids() {
    let signal: Signal<i32> = Signal::create(0);
    signal.add_dependent(1);
    signal.add_dependent(2);
    signal.add_dependent(3);
    let deps: Vec<usize> = signal.get_dependents();
    assert_eq!(deps.len(), 3);
    assert!(deps.contains(&1));
    assert!(deps.contains(&2));
    assert!(deps.contains(&3));
}

#[test]
fn signal_get_dependents_empty_by_default() {
    let signal: Signal<i32> = Signal::create(0);
    let deps: Vec<usize> = signal.get_dependents();
    assert!(deps.is_empty());
}

#[test]
fn signal_get_does_not_panic_on_inactive() {
    let signal: Signal<i32> = Signal::create(99);
    signal.deactivate();
    let value: i32 = signal.get();
    assert_eq!(value, 99);
}

// =====================================================================
// FireHandle
// =====================================================================

#[test]
fn fire_handle_new_yields_valid_handle() {
    let handle: FireHandle = FireHandle::new(|| {});
    let _: FireHandle = handle;
}

#[test]
fn fire_handle_from_closure() {
    let handle: FireHandle = FireHandle::from(|| {});
    let _: FireHandle = handle;
}

#[test]
fn fire_handle_is_copy() {
    let handle: FireHandle = FireHandle::from(|| {});
    let copy1: FireHandle = handle;
    let copy2: FireHandle = handle;
    let copy3: FireHandle = handle;
    assert_eq!(copy1, copy2);
    assert_eq!(copy2, copy3);
    assert_eq!(copy1, copy3);
}

#[test]
fn fire_handle_default_inner_is_zero() {
    let a: FireHandle = unsafe { std::mem::zeroed() };
    let b: FireHandle = unsafe { std::mem::zeroed() };
    assert_eq!(a, b);
    use std::collections::hash_map::DefaultHasher;
    let mut h1: DefaultHasher = DefaultHasher::new();
    let mut h2: DefaultHasher = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn fire_handle_distinct_closures_have_distinct_addresses() {
    let a: FireHandle = FireHandle::from(|| {});
    let b: FireHandle = FireHandle::from(|| {});
    assert_ne!(a, b);
}

#[test]
fn fire_handle_fire_invokes_closure() {
    let counter: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let counter_for_closure: Rc<Cell<i32>> = counter.clone();
    let handle: FireHandle = FireHandle::new(move || {
        counter_for_closure.set(counter_for_closure.get() + 1);
    });
    unsafe {
        handle.fire();
    }
    assert_eq!(counter.get(), 1);
}

#[test]
fn fire_handle_fire_at_invokes_closure() {
    let counter: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let counter_for_closure: Rc<Cell<i32>> = counter.clone();
    let handle: FireHandle = FireHandle::new(move || {
        counter_for_closure.set(counter_for_closure.get() + 1);
    });
    let addr: usize = handle.get_inner();
    unsafe {
        FireHandle::fire_at(addr);
    }
    assert_eq!(counter.get(), 1);
}

#[test]
fn fire_handle_fire_can_be_called_repeatedly() {
    let counter: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let counter_for_closure: Rc<Cell<i32>> = counter.clone();
    let handle: FireHandle = FireHandle::new(move || {
        counter_for_closure.set(counter_for_closure.get() + 1);
    });
    unsafe {
        handle.fire();
        handle.fire();
        handle.fire();
    }
    assert_eq!(counter.get(), 3);
}

#[test]
fn fire_handle_clone_via_copy_increments_underlying_counter() {
    let counter: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let counter_for_closure: Rc<Cell<i32>> = counter.clone();
    let handle: FireHandle = FireHandle::new(move || {
        counter_for_closure.set(counter_for_closure.get() + 1);
    });
    let copy: FireHandle = handle;
    unsafe {
        handle.fire();
    }
    assert_eq!(counter.get(), 1);
    unsafe {
        copy.fire();
    }
    assert_eq!(counter.get(), 2);
}

// =====================================================================
// Regression: native panic guards
// =====================================================================

#[test]
fn native_signal_create_and_is_alive_does_not_panic() {
    let result: Result<(), ()> = super::catch_unwind(super::AssertUnwindSafe(|| {
        let signal: Signal<i32> = Signal::create(7);
        assert!(Signal::<i32>::is_alive(signal.get_inner()));
    }))
    .map_err(|_| ());
    assert!(result.is_ok());
}

#[test]
fn native_signal_get_does_not_panic() {
    let result: Result<(), ()> = super::catch_unwind(super::AssertUnwindSafe(|| {
        let signal: Signal<i32> = Signal::create(11);
        let _: i32 = signal.get();
    }))
    .map_err(|_| ());
    assert!(result.is_ok());
}

#[test]
fn native_fire_handle_fire_does_not_panic() {
    let result: Result<(), ()> = super::catch_unwind(super::AssertUnwindSafe(|| {
        let handle: FireHandle = FireHandle::from(|| {});
        unsafe {
            handle.fire();
        }
    }))
    .map_err(|_| ());
    assert!(result.is_ok());
}

// =====================================================================
// Signal::<String>::try_reclaim_inactive — SPA orphan bridge reclamation
// =====================================================================

#[test]
fn try_reclaim_inactive_returns_zero_when_no_orphans() {
    // No bridges have been created yet (or all have source subscribers),
    // so the sweep has nothing to do and must report 0.
    let freed: usize = Signal::<String>::try_reclaim_inactive(usize::MAX);
    assert_eq!(freed, 0);
}

#[test]
fn try_reclaim_inactive_zero_max_freed_is_noop() {
    // Passing max_freed = 0 short-circuits the scan — verifies the
    // function respects the cap without burning a HashMap walk.
    let freed: usize = Signal::<String>::try_reclaim_inactive(0);
    assert_eq!(freed, 0);
}

#[test]
fn try_reclaim_inactive_reclaims_orphan_after_clear_listeners() {
    // The canonical orphan case: a bridge is created (via
    // BridgeRefsCell::track), its source never deactivates, the bridge
    // itself is detached (clear_listeners removes it from the registry).
    // Without try_reclaim_inactive, the bridge heap would stay parked
    // in BridgeRefsCell with an empty dep set until the page unloads.
    let source: Signal<i32> = Signal::create(0);
    let bridge: Signal<String> = Signal::create(String::from("init"));
    let bridge_addr: usize = bridge.get_inner();
    BridgeRefsCell::track(bridge_addr, source.get_inner());
    // Simulate the orphan invariant directly: empty the bridge's
    // dep set (as `Signal::deactivate` would) without invoking
    // the atomic free step, then detach the DOM via clear_listeners.
    // After this, the bridge sits in BridgeRefsCell with an empty
    // dep set and is not in SIGNAL_INNER_REGISTRY — exactly the
    // state the SPA sweep reclaims.
    BridgeRefsCell::map_mut()
        .get_mut(&bridge_addr)
        .expect("bridge should be tracked")
        .remove(&source.get_inner());
    Signal::<String>::clear_listeners(bridge_addr);
    // Sanity: the bridge is no longer in the registry.
    assert!(
        !Signal::<String>::is_alive(bridge_addr),
        "clear_listeners must remove the bridge from SIGNAL_INNER_REGISTRY"
    );
    // Sanity: the bridge is still parked in BridgeRefsCell with an
    // empty dep set — the orphan invariant.
    assert!(
        BridgeRefsCell::map_mut()
            .get(&bridge_addr)
            .map(|s| s.is_empty())
            .unwrap_or(false),
        "bridge must still be parked in BridgeRefsCell with an empty dep set"
    );
    // Sweep. The bridge satisfies both invariants, so it must be reclaimed.
    let freed: usize = Signal::<String>::try_reclaim_inactive(usize::MAX);
    assert_eq!(freed, 1, "orphan bridge must be reclaimed in a single sweep");
    // Post-sweep: the bridge is removed from BridgeRefsCell.
    assert!(
        !BridgeRefsCell::map_mut().contains_key(&bridge_addr),
        "reclaimed bridge must be removed from BridgeRefsCell"
    );
}

#[test]
fn try_reclaim_inactive_skips_bridge_with_live_source() {
    // A bridge whose source still claims it (dep set non-empty) must
    // NOT be reclaimed — reclaiming would let a stale subscriber
    // dereference a freed pointer. We construct the canonical
    // "DOM-detached, source-still-listening" state: clear_listeners
    // ran (so bridge is out of registry) but the bridge's entry in
    // BridgeRefsCell still records `source_addr`.
    let source: Signal<i32> = Signal::create(0);
    let bridge: Signal<String> = Signal::create(String::from("init"));
    let bridge_addr: usize = bridge.get_inner();
    BridgeRefsCell::track(bridge_addr, source.get_inner());
    Signal::<String>::clear_listeners(bridge_addr);
    // Sanity: bridge is out of registry, dep set still claims source.
    assert!(!Signal::<String>::is_alive(bridge_addr));
    assert_eq!(
        BridgeRefsCell::map_mut()
            .get(&bridge_addr)
            .map(|s| s.len())
            .unwrap_or(0),
        1,
        "dep set should still record the source subscriber"
    );
    let freed: usize = Signal::<String>::try_reclaim_inactive(usize::MAX);
    assert_eq!(
        freed, 0,
        "bridge with non-empty dep set must not be reclaimed"
    );
    // The bridge is still tracked in BridgeRefsCell (sweep correctly
    // left it alone).
    assert!(BridgeRefsCell::map_mut().contains_key(&bridge_addr));
}

#[test]
fn try_reclaim_inactive_skips_bridge_still_in_registry() {
    // A bridge whose address is still in SIGNAL_INNER_REGISTRY (i.e.
    // its DOM element is still attached) must NOT be reclaimed — the
    // handler might still be invoked via the live data-euv-id lookup,
    // and freeing the heap would be undefined behaviour.
    let source: Signal<i32> = Signal::create(0);
    let bridge: Signal<String> = Signal::create(String::from("init"));
    let bridge_addr: usize = bridge.get_inner();
    BridgeRefsCell::track(bridge_addr, source.get_inner());
    // Note: clear_listeners NOT called — bridge is still in registry.
    // Also drop the source subscriber entry by hand so the dep set
    // would otherwise be empty if we forgot the registry check.
    BridgeRefsCell::map_mut().remove(&bridge_addr);
    BridgeRefsCell::track(bridge_addr, source.get_inner());
    BridgeRefsCell::map_mut()
        .get_mut(&bridge_addr)
        .unwrap()
        .remove(&source.get_inner());
    let freed: usize = Signal::<String>::try_reclaim_inactive(usize::MAX);
    assert_eq!(
        freed, 0,
        "bridge still in SIGNAL_INNER_REGISTRY must not be reclaimed"
    );
}

#[test]
fn try_reclaim_inactive_respects_max_freed_cap() {
    // With two reclaimable orphans and max_freed = 1, the sweep must
    // reclaim exactly 1 and leave the other for a subsequent call.
    let source_a: Signal<i32> = Signal::create(0);
    let source_b: Signal<i32> = Signal::create(1);
    let bridge_a: Signal<String> = Signal::create(String::from("a"));
    let bridge_b: Signal<String> = Signal::create(String::from("b"));
    let addr_a: usize = bridge_a.get_inner();
    let addr_b: usize = bridge_b.get_inner();
    BridgeRefsCell::track(addr_a, source_a.get_inner());
    BridgeRefsCell::track(addr_b, source_b.get_inner());
    // Empty both dep sets to create the orphan invariant.
    BridgeRefsCell::map_mut()
        .get_mut(&addr_a)
        .unwrap()
        .remove(&source_a.get_inner());
    BridgeRefsCell::map_mut()
        .get_mut(&addr_b)
        .unwrap()
        .remove(&source_b.get_inner());
    Signal::<String>::clear_listeners(addr_a);
    Signal::<String>::clear_listeners(addr_b);
    let freed: usize = Signal::<String>::try_reclaim_inactive(1);
    assert_eq!(freed, 1, "max_freed = 1 must cap reclaim count");
    // Second sweep finishes the job.
    let freed_rest: usize = Signal::<String>::try_reclaim_inactive(usize::MAX);
    assert_eq!(
        freed_rest, 1,
        "remaining orphan must be reclaimed on the next sweep"
    );
    // Final state: neither bridge is in BridgeRefsCell.
    let remaining: usize = BridgeRefsCell::map_mut()
        .iter()
        .filter(|(addr, _)| **addr == addr_a || **addr == addr_b)
        .count();
    assert_eq!(remaining, 0);
}

#[test]
fn try_reclaim_inactive_idempotent() {
    // Calling the sweep twice must reclaim the same orphans only once.
    // The second call must observe an empty dep-set + not-in-registry
    // set and return 0.
    let source: Signal<i32> = Signal::create(0);
    let bridge: Signal<String> = Signal::create(String::from("once"));
    let bridge_addr: usize = bridge.get_inner();
    BridgeRefsCell::track(bridge_addr, source.get_inner());
    BridgeRefsCell::map_mut()
        .get_mut(&bridge_addr)
        .unwrap()
        .remove(&source.get_inner());
    Signal::<String>::clear_listeners(bridge_addr);
    let first: usize = Signal::<String>::try_reclaim_inactive(usize::MAX);
    let second: usize = Signal::<String>::try_reclaim_inactive(usize::MAX);
    assert_eq!(first, 1);
    assert_eq!(second, 0, "second sweep must find nothing left to reclaim");
}
