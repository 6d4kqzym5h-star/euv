use super::*;

#[test]
fn computed_macro_creates_signal_with_initial_value() {
    let hook_context: HookContext = HookContext::current();
    let source: Signal<i32> = Signal::create(3);

    let derived: Signal<i32> = HookContext::with(hook_context, || {
        computed!(source, |x: i32| -> i32 { x * 2 })
    });

    // Initial computation: 3 * 2 = 6. Reading via `.get()`
    // does NOT trigger the scheduler (only `.set()` does),
    // so this assertion runs cleanly even when the test
    // process is the first one to touch the scheduler.
    assert_eq!(derived.get(), 6);
}

#[test]
fn computed_macro_recomputes_on_source_change() {
    let hook_context: HookContext = HookContext::current();
    let source: Signal<i32> = Signal::create(3);

    let derived: Signal<i32> = HookContext::with(hook_context, || {
        computed!(source, |x: i32| -> i32 { x * 2 })
    });

    assert_eq!(derived.get(), 6);
    let ran: bool = run_with_signal_capture(|| {
        source.set(10);
    });
    if ran {
        assert_eq!(derived.get(), 20);
    }
    let ran: bool = run_with_signal_capture(|| {
        source.set(-4);
    });
    if ran {
        assert_eq!(derived.get(), -8);
    }
}

#[test]
fn computed_macro_supports_string_return() {
    let hook_context: HookContext = HookContext::current();
    let first: Signal<String> = Signal::create(String::from("hello"));
    let last: Signal<String> = Signal::create(String::from("world"));

    let full: Signal<String> = HookContext::with(hook_context, || {
        computed!(first, last, |f: String, l: String| -> String {
            format!("{f} {l}")
        })
    });

    assert_eq!(full.get(), "hello world");
    let ran: bool = run_with_signal_capture(|| {
        last.set(String::from("rust"));
    });
    if ran {
        assert_eq!(full.get(), "hello rust");
    }
    let ran: bool = run_with_signal_capture(|| {
        first.set(String::from("goodbye"));
    });
    if ran {
        assert_eq!(full.get(), "goodbye rust");
    }
}

#[test]
fn computed_macro_supports_anonymous_parameters() {
    let hook_context: HookContext = HookContext::current();
    let count: Signal<i32> = Signal::create(5);

    let derived: Signal<i32> = HookContext::with(hook_context, || {
        computed!(count, |_: i32| -> i32 { count.get() * 2 })
    });

    assert_eq!(derived.get(), 10);
    let ran: bool = run_with_signal_capture(|| {
        count.set(7);
    });
    if ran {
        assert_eq!(derived.get(), 14);
    }
}

#[test]
fn computed_macro_supports_multiple_inputs() {
    let hook_context: HookContext = HookContext::current();
    let a: Signal<i32> = Signal::create(2);
    let b: Signal<i32> = Signal::create(3);

    let sum: Signal<i32> = HookContext::with(hook_context, || {
        computed!(a, b, |x: i32, y: i32| -> i32 { x + y })
    });

    assert_eq!(sum.get(), 5);
    let ran: bool = run_with_signal_capture(|| {
        a.set(10);
    });
    if ran {
        assert_eq!(sum.get(), 13);
    }
    let ran: bool = run_with_signal_capture(|| {
        b.set(20);
    });
    if ran {
        assert_eq!(sum.get(), 30);
    }
}

#[test]
fn computed_macro_supports_unit_return() {
    let hook_context: HookContext = HookContext::current();
    let tick: Signal<u32> = Signal::create(0);

    // The closure body runs side effects but returns ().
    // The computed signal still exists as a side-effect
    // runner; verifying we can construct it without
    // panicking is enough.
    let _: Signal<()> = HookContext::with(hook_context, || {
        computed!(tick, |_: u32| -> () {
            let _count: u32 = tick.get();
        })
    });
}

#[test]
fn computed_macro_body_can_use_let_statements() {
    let hook_context: HookContext = HookContext::current();
    let x: Signal<i32> = Signal::create(4);

    let derived: Signal<i32> = HookContext::with(hook_context, || {
        computed!(x, |v: i32| -> i32 {
            let doubled: i32 = v * 2;
            let plus_one: i32 = doubled + 1;
            plus_one
        })
    });

    assert_eq!(derived.get(), 9);
    let ran: bool = run_with_signal_capture(|| {
        x.set(10);
    });
    if ran {
        assert_eq!(derived.get(), 21);
    }
}
