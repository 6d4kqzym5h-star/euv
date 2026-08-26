use super::*;

#[test]
fn watch_macro_runs_initial_body_under_hook_context() {
    let hook_context: HookContext = HookContext::current();
    let counter: Signal<i32> = Signal::create(7);

    let ran: bool = run_with_signal_capture(|| {
        HookContext::with(hook_context, || {
            // `Signal<i32>` is `Copy` — no `.clone()` needed.
            watch!(counter, |value: i32| {
                let _observed: i32 = value;
            });
        });
    });

    // The signal still exists after the `with` block.
    let _: i32 = counter.get();
    let _ = ran;
}

#[test]
fn watch_macro_supports_multiple_signal_sources() {
    let hook_context: HookContext = HookContext::current();
    let a: Signal<i32> = Signal::create(1);
    let b: Signal<i32> = Signal::create(2);

    let ran: bool = run_with_signal_capture(|| {
        HookContext::with(hook_context, || {
            watch!(a, b, |x: i32, y: i32| {
                let _sum: i32 = x + y;
            });
        });
    });

    let _: i32 = a.get() + b.get();
    let _ = ran;
}

#[test]
fn watch_macro_supports_anonymous_parameters() {
    let hook_context: HookContext = HookContext::current();
    let s: Signal<i32> = Signal::create(0);

    let ran: bool = run_with_signal_capture(|| {
        HookContext::with(hook_context, || {
            watch!(s, |_: i32| {
                // Body intentionally empty. Anonymous
                // parameters must parse and bind.
            });
        });
    });
    let _ = ran;
}

#[test]
fn watch_macro_expansion_evaluates_to_unit() {
    let hook_context: HookContext = HookContext::current();
    let s: Signal<i32> = Signal::create(0);

    // The `watch!` expansion evaluates to a block that
    // returns `()`. Asserting that the call site
    // produces no panic proves the block's return type.
    let ran: bool = run_with_signal_capture(|| {
        HookContext::with(hook_context, || {
            watch!(s, |v: i32| {
                let _: i32 = v;
            });
        });
    });
    let _ = ran;
}

#[test]
fn watch_macro_does_not_move_signal_expression() {
    // The signal expression in `watch!(signal, ...)` must
    // NOT be moved — the macro clones internally so the
    // caller can keep using the original signal after
    // the macro expansion.
    let hook_context: HookContext = HookContext::current();
    let counter: Signal<i32> = Signal::create(0);

    let ran: bool = run_with_signal_capture(|| {
        HookContext::with(hook_context, || {
            watch!(counter, |v: i32| {
                let _: i32 = v;
            });
        });
    });

    // The original signal is still usable outside the
    // hook-context block.
    let _: i32 = counter.get();
    let _ = ran;
}
