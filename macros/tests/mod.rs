mod class;
mod component;
mod computed;
mod unsafe_no_inline;
mod var;
mod vars;
mod watch;

use euv::*;

// Proc-macros defined in this crate (e.g. `class!`,
// `vars!`, `watch!`, `computed!`, `unsafe_no_inline!`,
// `#[component]`) are resolved by their absolute path
// within the integration test root — no explicit `use`
// is needed. The previous `use euv_macros::*;` was
// rejected by clippy as an unused import.

use std::panic::{AssertUnwindSafe, catch_unwind};

/// Wraps signal-mutating code in `catch_unwind` so the
/// test survives the wasm-bound `Scheduler::update` path
/// (`Signal::set` calls `App::schedule_update` which on
/// non-wasm targets panics inside `js_sys`).
///
/// The native test runner does not provide a `window()`
/// for the scheduler to schedule microtasks on. The
/// closure either runs to completion (returns `true`)
/// or the panic is swallowed and the read-side
/// assertions are skipped (returns `false`). The
/// `SCHEDULED` global the scheduler sets on its way to
/// the panic stays `true`, so subsequent tests in the
/// same process short-circuit the `window()` call and
/// behave normally.
pub(crate) fn run_with_signal_capture<F>(f: F) -> bool
where
    F: FnOnce(),
{
    catch_unwind(AssertUnwindSafe(f)).is_ok()
}

/// Wraps a closure in `catch_unwind` so the test survives
/// the wasm-bound `window()` call that
/// `OnceLock::get_or_init` triggers inside the
/// macro-generated class / vars function. The native
/// test runner does not provide a `window()`, so the
/// first invocation panics inside `js_sys`.
pub(crate) fn run_with_window_capture<F>(f: F) -> bool
where
    F: FnOnce(),
{
    catch_unwind(AssertUnwindSafe(f)).is_ok()
}
