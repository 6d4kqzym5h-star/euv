// `macros/tests/fn.rs` is a sub-module included from the
// integration test root (`macros/tests/mod.rs`) via
// `mod r#fn; pub(crate) use r#fn::*;`. Cargo also compiles
// this file as its own standalone test binary (because
// it's a top-level `tests/*.rs`), so we silence the
// dead-code lint that fires when the standalone binary
// has no callers. `catch_unwind` / `AssertUnwindSafe`
// references are written with their full
// `::std::panic::` path so this standalone binary does
// not need a `use` import (it is not allowed under the
// "all `use` imports follow `lib.rs`; other files use
// `use super::*;`" convention).

/// Wraps signal-mutating code in `catch_unwind` so the
/// test survives the wasm-bound `Scheduler::update` path
/// (`Signal::set` calls `App::schedule_update` which on
/// non-wasm targets panics inside `js_sys`).
///
/// The native test runner does not provide a `window()` for
/// the scheduler to schedule microtasks on. The closure
/// either runs to completion (returns `true`) or the panic
/// is swallowed and the read-side assertions are skipped
/// (returns `false`). The `SCHEDULED` global the scheduler
/// sets on its way to the panic stays `true`, so subsequent
/// tests in the same process short-circuit the `window()`
/// call and behave normally.
#[allow(dead_code)]
pub(crate) fn run_with_signal_capture<F>(f: F) -> bool
where
    F: FnOnce(),
{
    ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(f)).is_ok()
}

/// Wraps a closure in `catch_unwind` so the test survives
/// the wasm-bound `window()` call that `OnceLock::get_or_init`
/// triggers inside the macro-generated class / vars
/// function. The native test runner does not provide a
/// `window()`, so the first invocation panics inside
/// `js_sys`.
#[allow(dead_code)]
pub(crate) fn run_with_window_capture<F>(f: F) -> bool
where
    F: FnOnce(),
{
    ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(f)).is_ok()
}
