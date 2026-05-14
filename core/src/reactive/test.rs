use crate::*;

#[test]
fn test_signal_get() {
    let signal = use_signal(|| 42);
    assert_eq!(signal.get(), 42);
}

#[test]
fn test_signal_set() {
    let signal = use_signal(|| 0);
    signal.set(10);
    assert_eq!(signal.get(), 10);
}

#[test]
fn test_signal_clone() {
    let signal = use_signal(|| "hello".to_string());
    let cloned = signal;
    signal.set("world".to_string());
    assert_eq!(cloned.get(), "world");
}

#[test]
fn test_signal_subscribe() {
    let signal = use_signal(|| 0);
    let triggered: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    signal.subscribe({
        let triggered: Rc<Cell<bool>> = Rc::clone(&triggered);
        move || {
            triggered.set(true);
        }
    });
    signal.set(5);
    assert!(triggered.get());
}
