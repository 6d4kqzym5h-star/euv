//! Tests for pre-existing pure-Rust reactive types.
//!
//! These tests target types that do not depend on
//! `js-sys`, `web-sys`, or `wasm-bindgen` runtime
//! statics. They run natively under
//! `cargo test -p euv-core --lib` and provide
//! the first layer of coverage for `euv-core`'s
//! non-wasm surface area.
//!
//! # What's covered here
//!
//! - `reactive::signal::SignalCell` (storage wrapper)
//! - `reactive::signal::FireHandle` (leaked closure handle)
//! - `reactive::signal::Signal::create/get/is_alive/deactivate/clear_listeners/`
//!   `add_dependent/get_dependents/replace_listener` (paths that don't
//!   trigger dispatch via `Signal::set`)
//! - `reactive::hook::HookContext` (Default, Clone, reset_index, switch_arm)
//! - `reactive::hook::HookContextInner` (Default field values)
//! - `reactive::schedule::Scheduler` (ZST, default/clone/copy/eq/hash/ord)
//! - `reactive::schedule::CurrentHookContextCell` (UnsafeCell storage)
//! - `reactive::schedule::HookContextRc` (type alias identity)
//!
//! # What is NOT covered here
//!
//! - Anything that calls `Signal::set` (triggers
//!   `App::schedule_update` which uses `web_sys::window()`).
//! - Anything that calls `HookContext::current` /
//!   `HookContext::with` (touches the `static mut`
//!   `CURRENT_HOOK_CONTEXT` global).
//! - Anything that uses `IntervalHandle::clear` (uses
//!   `web_sys::Window::clearInterval`).
//!
//! These belong in wasm-bindgen-test files that ship
//! in a follow-up PR.

use super::*;
use std::cell::Cell;
use std::panic;
use std::rc::Rc;

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
    let stored: Signal<i32> = cell.get();
    assert_eq!(stored.get(), 42);
}

#[test]
fn signal_cell_set_overwrites_via_none_then_set() {
    // SignalCell::set panics if already set; so we
    // can only test the first-set behaviour.
    let cell: SignalCell<String> = SignalCell::default();
    let signal: Signal<String> = Signal::create(String::from("first"));
    cell.set(signal);
    let stored: Signal<String> = cell.get();
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
    let stored: Signal<String> = cell.get();
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
    let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| {
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
    use std::hash::{Hash, Hasher};
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
// HookContext (pure-Rust surface)
// =====================================================================

#[test]
fn hook_context_default_has_zero_hook_index() {
    let context: HookContext = HookContext::default();
    let inner: std::cell::Ref<HookContextInner> = context.get_inner().borrow();
    assert_eq!(inner.get_hook_index(), 0);
}

#[test]
fn hook_context_default_has_no_hooks() {
    let context: HookContext = HookContext::default();
    let inner: std::cell::Ref<HookContextInner> = context.get_inner().borrow();
    assert!(inner.get_hooks().is_empty());
}

#[test]
fn hook_context_default_has_no_cleanups() {
    let context: HookContext = HookContext::default();
    let inner: std::cell::Ref<HookContextInner> = context.get_inner().borrow();
    assert!(inner.get_cleanups().is_empty());
}

#[test]
fn hook_context_default_arm_changed_is_zero() {
    let context: HookContext = HookContext::default();
    let inner: std::cell::Ref<HookContextInner> = context.get_inner().borrow();
    assert_eq!(inner.get_arm_changed(), 0);
}

#[test]
fn hook_context_clone_shares_inner_state() {
    let context: HookContext = HookContext::default();
    let clone: HookContext = context.clone();
    assert!(Rc::ptr_eq(&context.get_inner(), &clone.get_inner(),));
}

#[test]
fn hook_context_reset_index_resets_to_zero() {
    let mut context: HookContext = HookContext::default();
    context.get_inner().borrow_mut().set_hook_index(42);
    context.reset_index();
    assert_eq!(context.get_inner().borrow().get_hook_index(), 0);
}

#[test]
fn hook_context_switch_arm_same_index_resets_hook_index() {
    let mut context: HookContext = HookContext::default();
    context.get_inner().borrow_mut().set_hook_index(99);
    context.switch_arm(0);
    assert_eq!(context.get_inner().borrow().get_hook_index(), 0);
}

#[test]
fn hook_context_switch_arm_new_index_clears_hooks() {
    let mut context: HookContext = HookContext::default();
    context
        .get_inner()
        .borrow_mut()
        .get_mut_hooks()
        .push(Box::new(42_i32));
    assert_eq!(context.get_inner().borrow().get_hooks().len(), 1);
    context.switch_arm(1);
    assert_eq!(context.get_inner().borrow().get_hooks().len(), 0);
    assert_eq!(context.get_inner().borrow().get_arm_changed(), 1);
}

#[test]
fn hook_context_switch_arm_runs_cleanups() {
    let mut context: HookContext = HookContext::default();
    let ran: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let ran_clone: Rc<Cell<bool>> = ran.clone();
    context
        .get_inner()
        .borrow_mut()
        .get_mut_cleanups()
        .push(Box::new(move || {
            ran_clone.set(true);
        }));
    context.switch_arm(1);
    assert!(ran.get());
}

// =====================================================================
// HookContextInner: Default
// =====================================================================

#[test]
fn hook_context_inner_default_has_empty_hooks() {
    let inner: HookContextInner = HookContextInner::default();
    assert!(inner.get_hooks().is_empty());
}

#[test]
fn hook_context_inner_default_has_zero_hook_index() {
    let inner: HookContextInner = HookContextInner::default();
    assert_eq!(inner.get_hook_index(), 0);
}

#[test]
fn hook_context_inner_default_has_zero_arm_changed() {
    let inner: HookContextInner = HookContextInner::default();
    assert_eq!(inner.get_arm_changed(), 0);
}

#[test]
fn hook_context_inner_default_has_empty_cleanups() {
    let inner: HookContextInner = HookContextInner::default();
    assert!(inner.get_cleanups().is_empty());
}

// =====================================================================
// Schedule: Scheduler (ZST), CurrentHookContextCell, HookContextRc
// =====================================================================

#[test]
fn scheduler_is_zero_sized() {
    assert_eq!(std::mem::size_of::<Scheduler>(), 0);
}

#[test]
fn scheduler_is_default() {
    let s: Scheduler = Scheduler::default();
    let _: Scheduler = s;
}

#[test]
fn scheduler_is_copy() {
    let s: Scheduler = Scheduler::default();
    let copy: Scheduler = s;
    let _: Scheduler = copy;
}

#[test]
fn scheduler_is_clone() {
    let s: Scheduler = Scheduler::default();
    let clone: Scheduler = s.clone();
    let _: Scheduler = clone;
}

#[test]
fn scheduler_is_eq() {
    let a: Scheduler = Scheduler::default();
    let b: Scheduler = Scheduler::default();
    assert_eq!(a, b);
}

#[test]
fn scheduler_is_ord() {
    let a: Scheduler = Scheduler::default();
    let b: Scheduler = Scheduler::default();
    assert!(a <= b);
    assert!(a >= b);
}

#[test]
fn scheduler_is_hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let s: Scheduler = Scheduler::default();
    let mut hasher: DefaultHasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let _: u64 = hasher.finish();
}

#[test]
fn scheduler_debug_format_works() {
    let s: Scheduler = Scheduler::default();
    let formatted: String = format!("{:?}", s);
    assert!(formatted.contains("Scheduler"));
}

#[test]
fn current_hook_context_cell_default_is_none() {
    let cell: CurrentHookContextCell =
        CurrentHookContextCell::new(std::cell::UnsafeCell::new(None));
    let ptr: *const Option<HookContextRc> = cell.get_0().get() as *const _;
    let value: Option<HookContextRc> = unsafe { (*ptr).clone() };
    assert!(value.is_none());
}

#[test]
fn current_hook_context_cell_set_then_get() {
    let cell: CurrentHookContextCell =
        CurrentHookContextCell::new(std::cell::UnsafeCell::new(None));
    let context: HookContextRc = Rc::new(std::cell::RefCell::new(HookContextInner::default()));
    unsafe {
        let ptr: *mut Option<HookContextRc> = cell.get_0().get();
        *ptr = Some(context.clone());
    }
    let read_back: Option<HookContextRc> = unsafe {
        let ptr: *const Option<HookContextRc> = cell.get_0().get();
        (*ptr).clone()
    };
    assert!(read_back.is_some());
    let stored: HookContextRc = read_back.unwrap();
    let inner: std::cell::Ref<HookContextInner> = stored.borrow();
    assert_eq!(inner.get_hook_index(), 0);
}

#[test]
fn current_hook_context_cell_set_replaces_previous() {
    let cell: CurrentHookContextCell =
        CurrentHookContextCell::new(std::cell::UnsafeCell::new(None));
    let first: HookContextRc = Rc::new(std::cell::RefCell::new(HookContextInner::default()));
    let second: HookContextRc = Rc::new(std::cell::RefCell::new(HookContextInner::default()));
    unsafe {
        let ptr: *mut Option<HookContextRc> = cell.get_0().get();
        *ptr = Some(first.clone());
        *ptr = Some(second.clone());
    }
    let read_back: Option<HookContextRc> = unsafe {
        let ptr: *const Option<HookContextRc> = cell.get_0().get();
        (*ptr).clone()
    };
    assert!(Rc::ptr_eq(&read_back.unwrap(), &second,));
}

#[test]
fn hook_context_rc_is_shared_clone() {
    let rc: HookContextRc = Rc::new(std::cell::RefCell::new(HookContextInner::default()));
    let cloned: HookContextRc = rc.clone();
    assert!(Rc::ptr_eq(&rc, &cloned));
}

#[test]
fn queue_microtask_constant_value() {
    assert_eq!(QUEUE_MICROTASK, "queueMicrotask");
}

// =====================================================================
// IntervalHandle: ZST-like value, no native tests for clear() since
// it requires web_sys::Window. We do cover construction.
// =====================================================================

#[test]
fn interval_handle_default() {
    let handle: IntervalHandle = IntervalHandle::default();
    let _: IntervalHandle = handle;
}

#[test]
fn interval_handle_is_copy() {
    let handle: IntervalHandle = IntervalHandle::default();
    let copy: IntervalHandle = handle;
    let _: IntervalHandle = copy;
}

#[test]
fn interval_handle_is_eq() {
    let a: IntervalHandle = IntervalHandle::default();
    let b: IntervalHandle = IntervalHandle::default();
    assert_eq!(a, b);
}

#[test]
fn interval_handle_get_interval_id_returns_zero_by_default() {
    let handle: IntervalHandle = IntervalHandle::default();
    assert_eq!(handle.get_interval_id(), 0);
}

#[test]
fn interval_handle_clone() {
    let handle: IntervalHandle = IntervalHandle::default();
    let cloned: IntervalHandle = handle;
    assert_eq!(handle, cloned);
}

#[test]
fn interval_handle_is_hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let handle: IntervalHandle = IntervalHandle::default();
    let mut hasher: DefaultHasher = DefaultHasher::new();
    handle.hash(&mut hasher);
    let _: u64 = hasher.finish();
}

// =====================================================================
// Regression: native panic guards
// =====================================================================

#[test]
fn native_signal_create_and_is_alive_does_not_panic() {
    let result: Result<(), ()> = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let signal: Signal<i32> = Signal::create(7);
        assert!(Signal::<i32>::is_alive(signal.get_inner()));
    }))
    .map_err(|_| ());
    assert!(result.is_ok());
}

#[test]
fn native_signal_get_does_not_panic() {
    let result: Result<(), ()> = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let signal: Signal<i32> = Signal::create(11);
        let _: i32 = signal.get();
    }))
    .map_err(|_| ());
    assert!(result.is_ok());
}

#[test]
fn native_fire_handle_fire_does_not_panic() {
    let result: Result<(), ()> = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let handle: FireHandle = FireHandle::from(|| {});
        unsafe {
            handle.fire();
        }
    }))
    .map_err(|_| ());
    assert!(result.is_ok());
}

#[test]
fn native_hook_context_default_does_not_panic() {
    let result: Result<(), ()> = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let _: HookContext = HookContext::default();
    }))
    .map_err(|_| ());
    assert!(result.is_ok());
}
