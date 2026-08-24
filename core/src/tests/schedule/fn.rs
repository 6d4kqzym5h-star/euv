use super::*;

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
