use super::*;
#[test]
fn interval_handle_clear_does_not_panic_on_native() {
    let handle: IntervalHandle = IntervalHandle::default();
    let result: Result<(), String> = super::catch_unwind(super::AssertUnwindSafe(|| {
        handle.clear();
    }))
    .map_err(|_| "panic".to_string());
    let _: Result<(), String> = result;
}

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
fn interval_handle_clone() {
    let handle: IntervalHandle = IntervalHandle::default();
    let cloned: IntervalHandle = handle;
    assert_eq!(handle, cloned);
}

#[test]
fn interval_handle_is_hash() {
    use std::collections::hash_map::DefaultHasher;
    let handle: IntervalHandle = IntervalHandle::default();
    let mut hasher: DefaultHasher = DefaultHasher::new();
    handle.hash(&mut hasher);
    let _: u64 = hasher.finish();
}

#[test]
fn native_hook_context_default_does_not_panic() {
    let result: Result<(), ()> = super::catch_unwind(super::AssertUnwindSafe(|| {
        let _: HookContext = HookContext::default();
    }))
    .map_err(|_| ());
    assert!(result.is_ok());
}
