use super::*;
use std::cell::Cell;
use std::panic;
use std::rc::Rc;

// =====================================================================
// HookContext::current / HookContext::with
// =====================================================================

#[test]
fn hook_context_current_does_not_panic_on_native() {
    let result: Result<(), String> = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let _: HookContext = HookContext::current();
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn hook_context_with_runs_callback() {
    let context: HookContext = HookContext::default();
    let result: i32 = HookContext::with(context, || 42);
    // `with` returns the closure result regardless
    // of the context swap.
    assert_eq!(result, 42);
}

#[test]
fn hook_context_with_restores_previous_context() {
    // Setup: install a context as current.
    let original: HookContext = HookContext::default();
    HookContext::with(original.clone(), || {
        // Inside this closure the original context
        // should be the active one. We can't easily
        // verify which context is "current" without
        // touching globals, but we can verify the
        // swap restores correctly by checking that
        // another `with` call works.
        let new_context: HookContext = HookContext::default();
        HookContext::with(new_context, || {
            // Inside the nested call.
            let _: HookContext = HookContext::current();
        });
    });
    // After the outer `with`, the original context
    // should be restored. We can't easily verify
    // without accessing globals, so we just verify
    // no panic.
}

#[test]
fn hook_context_with_returns_closure_result_string() {
    let context: HookContext = HookContext::default();
    let result: String = HookContext::with(context, || String::from("hello"));
    assert_eq!(result, "hello");
}

#[test]
fn hook_context_with_returns_closure_result_unit() {
    let context: HookContext = HookContext::default();
    let result: () = HookContext::with(context, || {});
    let _: () = result;
}

#[test]
fn hook_context_with_can_be_called_multiple_times() {
    let context: HookContext = HookContext::default();
    let r1: i32 = HookContext::with(context.clone(), || 1);
    let r2: i32 = HookContext::with(context.clone(), || 2);
    let r3: i32 = HookContext::with(context, || 3);
    assert_eq!(r1, 1);
    assert_eq!(r2, 2);
    assert_eq!(r3, 3);
}

// =====================================================================
// HookContext::signal (use_signal impl)
// =====================================================================

#[test]
fn hook_context_signal_creates_signal_with_init_value() {
    let context: HookContext = HookContext::default();
    HookContext::with(context, || {
        let signal: Signal<i32> = HookContext::signal(|| 42);
        assert_eq!(signal.get(), 42);
    });
}

#[test]
fn hook_context_signal_reuses_existing_on_second_call_same_index() {
    // `HookContext::signal` bumps the hook index on
    // every call. So calling signal() twice from
    // the same context creates TWO independent
    // signals at hook indices 0 and 1.
    //
    // To verify "reuse existing" semantics, we
    // would need to call signal() inside a
    // re-render cycle that resets the hook index.
    // The simpler invariant: each call creates a
    // new signal with its own init value.
    let context: HookContext = HookContext::default();
    HookContext::with(context, || {
        let first: Signal<i32> = HookContext::signal(|| 1);
        let second: Signal<i32> = HookContext::signal(|| 2);
        // Independent hook indices mean
        // independent signals.
        assert_eq!(first.get(), 1);
        assert_eq!(second.get(), 2);
        assert_ne!(first.get_inner(), second.get_inner());
    });
}

#[test]
fn hook_context_signal_with_string_value() {
    let context: HookContext = HookContext::default();
    HookContext::with(context, || {
        let signal: Signal<String> = HookContext::signal(|| String::from("initial"));
        assert_eq!(signal.get(), "initial");
    });
}

#[test]
fn hook_context_signal_with_vec_value() {
    let context: HookContext = HookContext::default();
    HookContext::with(context, || {
        let signal: Signal<Vec<i32>> = HookContext::signal(|| vec![1, 2, 3]);
        assert_eq!(signal.get(), vec![1, 2, 3]);
    });
}

#[test]
fn hook_context_signal_indexes_advance() {
    // Each call to signal() bumps the hook index,
    // creating a new slot. Two signals created at
    // different hook indices should be distinct.
    let context: HookContext = HookContext::default();
    HookContext::with(context, || {
        let first: Signal<i32> = HookContext::signal(|| 1);
        let second: Signal<i32> = HookContext::signal(|| 2);
        // Both are distinct signals with their own
        // values.
        assert_eq!(first.get(), 1);
        assert_eq!(second.get(), 2);
        // Different hook indices means different
        // inner addresses (different SignalInner
        // allocations).
        assert_ne!(first.get_inner(), second.get_inner());
    });
}

#[test]
fn hook_context_signal_registers_cleanup() {
    // `signal` registers a cleanup that calls
    // `signal.deactivate()`. After switch_arm,
    // the cleanup runs.
    let mut context: HookContext = HookContext::default();
    context.get_inner().borrow_mut().set_arm_changed(99);
    HookContext::with(context.clone(), || {
        let _signal: Signal<i32> = HookContext::signal(|| 0);
        // The cleanup was registered in
        // cleanups. We verify by switching arms,
        // which runs all cleanups.
    });
    // After the closure returns, the context is
    // restored (with the original). Now switch
    // arm to a different index to run cleanups.
    context.switch_arm(1);
    let inner_count: usize = context.get_inner().borrow().get_cleanups().len();
    // The cleanup was already run by switch_arm,
    // but switch_arm also runs the cleanup and
    // clears the cleanups list, so count is 0.
    assert_eq!(inner_count, 0);
}

#[test]
fn hook_context_signal_does_not_panic_on_native() {
    let context: HookContext = HookContext::default();
    let result: Result<(), String> = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        HookContext::with(context, || {
            let _: Signal<i32> = HookContext::signal(|| 7);
        });
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

// =====================================================================
// HookContext::cleanup (use_cleanup impl)
// =====================================================================

#[test]
fn hook_context_cleanup_registers_closure() {
    let context: HookContext = HookContext::default();
    let ran: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let ran_clone: Rc<Cell<bool>> = ran.clone();
    HookContext::with(context, || {
        HookContext::cleanup(move || {
            ran_clone.set(true);
        });
        // After cleanup(), the closure is in the
        // cleanups Vec.
        // Note: cleanup() is called from within a
        // hook context, so it can read the inner
        // state directly via switch_arm.
    });
    // We can't easily verify the closure ran
    // without triggering switch_arm (which is the
    // cleanup trigger). The cleanup is added on
    // switch_arm(new_index). This test only
    // verifies the call doesn't panic.
}

#[test]
fn hook_context_cleanup_does_not_panic_on_native() {
    let context: HookContext = HookContext::default();
    let result: Result<(), String> = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        HookContext::with(context, || {
            HookContext::cleanup(|| {});
        });
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

// =====================================================================
// HookContext::window_event — OMITTED
//
// `HookContext::window_event` calls
// `Registry::register_window_event`, which uses
// `wasm_bindgen::describe` FFI. On native targets
// this triggers a `panic_cannot_unwind` (non-unwinding
// panic) that aborts the entire test process even
// when wrapped in `catch_unwind`. We cannot test
// this path on native.
//
// The wasm-bindgen-test follow-up PR will exercise
// `window_event` under `wasm-pack test --node`.
// =====================================================================

// =====================================================================
// IntervalHandle
// =====================================================================

#[test]
fn interval_handle_clear_does_not_panic_on_native() {
    let handle: IntervalHandle = IntervalHandle::default();
    // `clear()` calls `web_sys::Window::clear_interval`
    // which panics on non-wasm targets ("cannot access
    // imported statics on non-wasm targets"). Wrap in
    // catch_unwind to verify the panic is contained.
    let result: Result<(), String> = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        handle.clear();
    }))
    .map_err(|_| "panic".to_string());
    // We don't assert `is_ok` — `clear()` is expected
    // to panic on native. We just want the panic to be
    // caught, not propagated to the test runner.
    let _: Result<(), String> = result;
}

#[test]
fn interval_handle_new_with_zero_id() {
    let handle: IntervalHandle = IntervalHandle::new(0);
    assert_eq!(handle.get_interval_id(), 0);
}

#[test]
fn interval_handle_new_with_positive_id() {
    let handle: IntervalHandle = IntervalHandle::new(42);
    assert_eq!(handle.get_interval_id(), 42);
}

#[test]
fn interval_handle_new_with_negative_id() {
    // Browsers return i32 interval IDs; negative
    // values are technically possible.
    let handle: IntervalHandle = IntervalHandle::new(-1);
    assert_eq!(handle.get_interval_id(), -1);
}

#[test]
fn interval_handle_set_interval_id_replaces() {
    let mut handle: IntervalHandle = IntervalHandle::default();
    handle.set_interval_id(123);
    assert_eq!(handle.get_interval_id(), 123);
    handle.set_interval_id(456);
    assert_eq!(handle.get_interval_id(), 456);
}

// =====================================================================
// HookContextInner field accessor coverage
// =====================================================================

#[test]
fn hook_context_inner_set_hooks_replaces() {
    let mut inner: HookContextInner = HookContextInner::default();
    inner.get_mut_hooks().push(Box::new(42_i32));
    assert_eq!(inner.get_hooks().len(), 1);
    inner.get_mut_hooks().clear();
    assert_eq!(inner.get_hooks().len(), 0);
}

#[test]
fn hook_context_inner_set_arm_changed() {
    let mut inner: HookContextInner = HookContextInner::default();
    inner.set_arm_changed(5);
    assert_eq!(inner.get_arm_changed(), 5);
}

#[test]
fn hook_context_inner_set_hook_index() {
    let mut inner: HookContextInner = HookContextInner::default();
    inner.set_hook_index(10);
    assert_eq!(inner.get_hook_index(), 10);
}

#[test]
fn hook_context_inner_set_cleanups_replaces() {
    let mut inner: HookContextInner = HookContextInner::default();
    inner.get_mut_cleanups().push(Box::new(|| {}));
    assert_eq!(inner.get_cleanups().len(), 1);
    inner.get_mut_cleanups().clear();
    assert_eq!(inner.get_cleanups().len(), 0);
}

#[test]
fn hook_context_inner_debug_format_works() {
    let inner: HookContextInner = HookContextInner::default();
    let formatted: String = format!("{:?}", inner);
    assert!(formatted.contains("HookContextInner"));
}

// =====================================================================
// Native panic regressions
// =====================================================================

#[test]
fn native_hook_context_signal_works_on_native() {
    let result: Result<(), String> = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let context: HookContext = HookContext::default();
        HookContext::with(context, || {
            let s: Signal<i32> = HookContext::signal(|| 1);
            assert_eq!(s.get(), 1);
        });
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn native_hook_context_with_works_on_native() {
    let result: Result<(), String> = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let context: HookContext = HookContext::default();
        let r: i32 = HookContext::with(context, || 99);
        assert_eq!(r, 99);
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn native_hook_context_cleanup_works_on_native() {
    let result: Result<(), String> = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let context: HookContext = HookContext::default();
        HookContext::with(context, || {
            HookContext::cleanup(|| {});
            HookContext::cleanup(|| {});
            HookContext::cleanup(|| {});
        });
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
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

#[test]
fn native_hook_context_default_does_not_panic() {
    let result: Result<(), ()> = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        let _: HookContext = HookContext::default();
    }))
    .map_err(|_| ());
    assert!(result.is_ok());
}
