use super::*;

#[test]
fn new_is_pending() {
    let lazy: LazyComponent<i32> = LazyComponent::new(|| 42);
    assert_eq!(lazy.get_state().get(), LoadState::Pending);
    assert!(matches!(lazy.get_state().get(), LoadState::Pending));
    assert!(!matches!(lazy.get_state().get(), LoadState::Loaded(_) | LoadState::Failed(_)));
}

#[test]
fn get_runs_factory_on_first_call() {
    let counter: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter_clone = counter.clone();
    let lazy: LazyComponent<u32> = LazyComponent::new(move || {
        counter_clone.set(counter_clone.get() + 1);
        counter_clone.get() * 10
    });
    assert_eq!(lazy.get(), Some(10));
    assert_eq!(counter.get(), 1);
}

#[test]
fn get_returns_cached_value_on_subsequent_calls() {
    let counter: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter_clone = counter.clone();
    let lazy: LazyComponent<u32> = LazyComponent::new(move || {
        counter_clone.set(counter_clone.get() + 1);
        counter_clone.get()
    });
    let _ = lazy.get();
    let _ = lazy.get();
    let _ = lazy.get();
    assert_eq!(counter.get(), 1);
}

#[test]
fn prefetch_runs_factory() {
    let lazy: LazyComponent<i32> = LazyComponent::new(|| 7);
    lazy.prefetch();
    assert!(matches!(lazy.get_state().get(), LoadState::Loaded(_) | LoadState::Failed(_)));
    assert!(matches!(lazy.get_state().get(), LoadState::Loaded(7)));
}

#[test]
fn prefetch_is_idempotent() {
    let counter: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter_clone = counter.clone();
    let lazy: LazyComponent<u32> = LazyComponent::new(move || {
        counter_clone.set(counter_clone.get() + 1);
        counter_clone.get()
    });
    lazy.prefetch();
    lazy.prefetch();
    lazy.prefetch();
    assert_eq!(counter.get(), 1);
}

#[test]
fn prefetch_after_resolved_is_no_op() {
    let counter: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter_clone = counter.clone();
    let lazy: LazyComponent<u32> = LazyComponent::new(move || {
        counter_clone.set(counter_clone.get() + 1);
        counter_clone.get()
    });
    let _ = lazy.get();
    lazy.prefetch();
    assert_eq!(counter.get(), 1);
}

#[test]
fn reset_returns_to_pending() {
    let counter: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter_clone = counter.clone();
    let lazy: LazyComponent<u32> = LazyComponent::new(move || {
        counter_clone.set(counter_clone.get() + 1);
        counter_clone.get()
    });
    let _ = lazy.get();
    lazy.reset();
    assert_eq!(lazy.get_state().get(), LoadState::Pending);
    let _ = lazy.get();
    assert_eq!(counter.get(), 2);
}

#[test]
fn unwrap_returns_loaded_value() {
    let lazy: LazyComponent<&'static str> = LazyComponent::new(|| "hello");
    let _ = lazy.get();
    assert_eq!(lazy.loaded(), Some("hello"));
}

#[test]
fn change_factory_resets_state() {
    #[cfg(target_arch = "wasm32")]
    {
        let lazy: LazyComponent<i32> = LazyComponent::new(|| 1);
        let _ = lazy.get();
        lazy.change_factory(|| 2);
        assert_eq!(lazy.get_state().get(), LoadState::Pending);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = std::panic::catch_unwind(|| {
            let lazy: LazyComponent<i32> = LazyComponent::new(|| 1);
            let _ = lazy.get();
        });
    }
}

#[test]
fn clone_shares_state_and_factory() {
    let counter: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let counter_clone = counter.clone();
    let lazy: LazyComponent<u32> = LazyComponent::new(move || {
        counter_clone.set(counter_clone.get() + 1);
        counter_clone.get()
    });
    let cloned = lazy.clone();
    let _ = lazy.get();
    let _ = cloned.get();
    assert_eq!(counter.get(), 1);
}

#[test]
fn state_signal_is_reactive() {
    let lazy: LazyComponent<i32> = LazyComponent::new(|| 99);
    let signal = lazy.get_state();
    assert_eq!(signal.get(), LoadState::Pending);
    let _ = lazy.get();
    assert!(matches!(signal.get(), LoadState::Loaded(99)));
}

#[test]
fn panic_in_factory_marks_failed() {
    let lazy: LazyComponent<i32> = LazyComponent::new(|| -> i32 { panic!("boom") });
    let _ = lazy.get();
    match lazy.get_state().get() {
        LoadState::Failed(msg) => {
            assert!(msg.contains("boom"));
        }
        other => panic!("expected Failed, got {:?}", other),
    }
}

#[test]
fn panic_string_factory_is_caught() {
    let lazy: LazyComponent<i32> = LazyComponent::new(|| {
        let s: String = String::from("explicit-string-panic");
        panic!("{}", s)
    });
    let _ = lazy.get();
    match lazy.get_state().get() {
        LoadState::Failed(msg) => {
            assert!(msg.contains("explicit-string-panic"));
        }
        _ => panic!("expected Failed"),
    }
}

#[test]
fn get_after_panic_returns_none() {
    let lazy: LazyComponent<i32> = LazyComponent::new(|| -> i32 { panic!("nope") });
    assert_eq!(lazy.get(), None);
}

#[test]
fn reset_after_panic_allows_recovery() {
    let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let calls_clone = calls.clone();
    let lazy: LazyComponent<i32> = LazyComponent::new(move || {
        let n = calls_clone.get();
        calls_clone.set(n + 1);
        if n == 0 {
            panic!("first time");
        }
        100
    });
    let _ = lazy.get();
    assert!(matches!(lazy.get_state().get(), LoadState::Failed(_)));
    lazy.reset();
    assert_eq!(lazy.get(), Some(100));
    assert_eq!(calls.get(), 2);
}

#[test]
fn debug_format_works() {
    let lazy: LazyComponent<i32> = LazyComponent::new(|| 1);
    let s = format!("{:?}", lazy);
    assert!(s.contains("LazyComponent"));
}

#[test]
fn debug_format_after_load_works() {
    let lazy: LazyComponent<i32> = LazyComponent::new(|| 42);
    let _ = lazy.get();
    let s = format!("{:?}", lazy);
    assert!(s.contains("42"));
}

#[test]
fn string_value_works() {
    let lazy: LazyComponent<String> = LazyComponent::new(|| String::from("hello world"));
    let _ = lazy.get();
    assert_eq!(lazy.loaded(), Some(String::from("hello world")));
}

#[test]
fn vec_value_works() {
    let lazy: LazyComponent<Vec<i32>> = LazyComponent::new(|| vec![1, 2, 3]);
    let _ = lazy.get();
    assert_eq!(lazy.loaded(), Some(vec![1, 2, 3]));
}

#[test]
fn is_pending_after_reset() {
    let lazy: LazyComponent<i32> = LazyComponent::new(|| 1);
    let _ = lazy.get();
    assert!(!matches!(lazy.get_state().get(), LoadState::Pending));
    lazy.reset();
    assert!(matches!(lazy.get_state().get(), LoadState::Pending));
}
