use super::*;

#[test]
fn test_signal_get() {
    let signal: Signal<i32> = App::use_signal(|| 42);
    assert_eq!(signal.get(), 42);
}

#[test]
fn test_signal_set() {
    let signal: Signal<i32> = App::use_signal(|| 0);
    signal.set(10);
    assert_eq!(signal.get(), 10);
}

#[test]
fn test_signal_set_no_op_on_equal() {
    let signal: Signal<i32> = App::use_signal(|| 5);
    signal.set(5);
    assert_eq!(signal.get(), 5);
}

#[test]
fn test_signal_subscribe() {
    let signal: Signal<i32> = App::use_signal(|| 0);
    let count: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    signal.subscribe({
        let count: Rc<Cell<i32>> = Rc::clone(&count);
        move || {
            count.set(count.get() + 1);
        }
    });
    signal.set(10);
    assert_eq!(count.get(), 1);
    signal.set(20);
    assert_eq!(count.get(), 2);
}

#[test]
fn test_signal_set_notifies_listeners() {
    let signal: Signal<i32> = App::use_signal(|| 0);
    let count: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    signal.subscribe({
        let count: Rc<Cell<i32>> = Rc::clone(&count);
        move || {
            count.set(count.get() + 1);
        }
    });
    signal.set(10);
    assert_eq!(count.get(), 1);
    signal.set(20);
    assert_eq!(count.get(), 2);
}

#[test]
fn test_signal_set_no_op_on_equal_value() {
    let signal: Signal<i32> = App::use_signal(|| 5);
    let count: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    signal.subscribe({
        let count: Rc<Cell<i32>> = Rc::clone(&count);
        move || {
            count.set(count.get() + 1);
        }
    });
    signal.set(5);
    assert_eq!(count.get(), 0);
}

#[test]
fn test_signal_create_and_get() {
    // Regression test for the bridge signal memory leak fix.
    //
    // Before the fix, `Signal::clear_listeners` (the path used by
    // `cleanup_subtree` to release `data-euv-signal-addrs` bridge
    // signals) only deactivated the signal but left its `Box<SignalInner>`
    // pinned in the global address registry for the lifetime of the page.
    //
    // The fix removes the address from the registry and frees the heap
    // allocation, and guards against double-free by checking
    // `is_alive(addr)` before the `Box::from_raw`. Both behaviors are
    // exercised on the wasm target by `wasm-pack test`; here on native
    // we only verify the public surface (create/get still works).
    let signal: Signal<String> = Signal::create(String::from("hello"));
    assert_eq!(signal.get(), "hello");
}
