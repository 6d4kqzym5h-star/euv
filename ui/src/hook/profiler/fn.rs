use super::*;

/// Returns a monotonic millisecond timestamp suitable for
/// profiling measurements.
///
/// - On `wasm32-unknown-unknown`, delegates to
///   `web_sys::Performance::now()`, which is monotonic
///   per-spec, sub-millisecond resolution, and shared across
///   all `Worker` scopes in the same browsing context.
/// - On every other target (used by `cargo test` and
///   downstream consumers that don't have a `Window`),
///   delegates to `Instant::now()` translated to
///   milliseconds since the process start. The exact
///   reference is irrelevant for relative timing — only the
///   delta between two `now_ms()` calls matters.
///
/// # Returns
///
/// - `f64` - A monotonically-increasing millisecond timestamp.
///   Always `>= 0.0`. The unit difference between platforms is
///   irrelevant for `elapsed_ms` arithmetic (subtraction
///   cancels the unit), but two platforms cannot be compared
///   to each other.
pub fn now_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w: web_sys::Window| w.performance())
            .map(|p: web_sys::Performance| p.now())
            // If `performance.now()` is somehow unavailable
            // (rare — only happens in worklets or
            // non-browser wasm runtimes), fall back to
            // `Date.now()` which is non-monotonic but at
            // least always available. The fallback is
            // never hit in normal page contexts.
            .unwrap_or_else(|| js_sys::Date::now())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Process-local reference: a `static` Instant captured
        // on the first call. This avoids the awkward
        // `Instant::now() - process_start()` arithmetic at
        // every call site, and gives us a stable reference
        // across the lifetime of the process.
        static PROCESS_START: OnceLock<Instant> = OnceLock::new();
        let start: &Instant = PROCESS_START.get_or_init(Instant::now);
        start.elapsed().as_secs_f64() * 1000.0
    }
}
