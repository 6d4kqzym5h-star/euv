use crate::*;

#[test]
fn test_signal_get() {
    let signal: Signal<i32> = use_signal(|| 42);
    assert_eq!(signal.get(), 42);
}

#[test]
fn test_signal_set() {
    let signal: Signal<i32> = use_signal(|| 0);
    signal.set(10);
    assert_eq!(signal.get(), 10);
}

#[test]
fn test_signal_set_no_op_on_equal() {
    let signal: Signal<i32> = use_signal(|| 5);
    signal.set(5);
    assert_eq!(signal.get(), 5);
}

#[test]
fn test_signal_subscribe() {
    let signal: Signal<i32> = use_signal(|| 0);
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
fn test_signal_set_silent_updates_value() {
    let signal: Signal<i32> = use_signal(|| 0);
    signal.set_silent(10);
    assert_eq!(signal.get(), 10);
}

#[test]
fn test_signal_set_silent_notifies_listeners() {
    let signal: Signal<i32> = use_signal(|| 0);
    let count: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    signal.subscribe({
        let count: Rc<Cell<i32>> = Rc::clone(&count);
        move || {
            count.set(count.get() + 1);
        }
    });
    signal.set_silent(10);
    assert_eq!(count.get(), 1);
    signal.set_silent(20);
    assert_eq!(count.get(), 2);
}

#[test]
fn test_signal_set_silent_no_op_on_equal() {
    let signal: Signal<i32> = use_signal(|| 5);
    let count: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    signal.subscribe({
        let count: Rc<Cell<i32>> = Rc::clone(&count);
        move || {
            count.set(count.get() + 1);
        }
    });
    signal.set_silent(5);
    assert_eq!(count.get(), 0);
}
