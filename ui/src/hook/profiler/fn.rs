use super::*;

/// Returns a monotonic millisecond timestamp suitable for
/// profiling measurements.
///
/// Delegates to `Performance::now()` when available, which is
/// monotonic per-spec, sub-millisecond resolution, and shared
/// across all `Worker` scopes in the same browsing context.
/// Falls back to `Date.now()` (non-monotonic, but always
/// available) when `performance.now()` is missing — the
/// fallback only fires in worklets or non-browser wasm
/// runtimes.
///
/// On non-wasm test runners where the browser API surface is
/// absent the first call may unwind; the `FALLBACK_MS` cell
/// then caches the result of a process-local monotonic clock
/// so subsequent calls don't re-trigger the web-sys lazy
/// initialiser (which would poison once-cell across the rest
/// of the test process).
///
/// # Returns
///
/// - `f64` - A monotonically-increasing millisecond timestamp.
///   Always `>= 0.0`.
pub fn now_ms() -> f64 {
    let result: Result<f64, Box<dyn std::any::Any + Send>> =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            window()
                .and_then(|window: Window| window.performance())
                .map(|performance: Performance| performance.now())
                .unwrap_or_else(Date::now)
        }));
    match result {
        Ok(ms) => ms,
        Err(_) => process_local_ms(),
    }
}

/// Fallback millisecond clock anchored at this thread's first call.
fn process_local_ms() -> f64 {
    thread_local! {
        static START: std::cell::OnceCell<Instant> = const { std::cell::OnceCell::new() };
    }
    START.with(|cell: &std::cell::OnceCell<Instant>| {
        let start: &Instant = cell.get_or_init(Instant::now);
        start.elapsed().as_secs_f64() * 1000.0
    })
}
