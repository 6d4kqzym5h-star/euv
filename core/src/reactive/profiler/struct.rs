//! Profiler data types.
//!
//! Pure data containers for the profiler (`ProfileEntry` and friends);
//! no dependency on parent-module symbols, so `super::*` is not imported.

use lombok_macros::{Data, New};
/// A single recorded measurement.
///
/// Pushed onto the `ProfilerHandle`'s entries signal every time
/// the user calls `ProfilerHandle::measure(label, f)` (or
/// manually `begin` / `end`). Cheap to `Clone` (the inner
/// strings are small and `f64` is `Copy`).
///
/// Field-level semantics:
///
/// - `label`: free-form identifier — typically the call site name
///   (`"render-list"`, `"fetch-posts"`). Empty strings are
///   allowed but render as an empty chip in the UI, which makes
///   misconfigured measurements obvious in a profiler readout.
/// - `elapsed_ms`: wall-clock time between `begin()` and the
///   matching `end()` (or the duration of the measured closure),
///   in milliseconds. Always `>= 0.0` — `begin` is captured
///   before any user code runs, so the subtraction cannot
///   underflow.
/// - `timestamp_ms`: the wall-clock `now_ms()` value at the
///   instant the entry was recorded (NOT the start of the
///   measurement). This lets the UI sort / filter entries by
///   when they were committed, not by when the user started
///   the timer — which matters when entries are kept around
///   for "last N measurements" readouts.
#[derive(Clone, Debug, PartialEq, Data, New)]
pub struct ProfileEntry {
    /// The free-form label passed to `measure` / `begin`.
    pub label: String,
    /// Duration of the measured operation, in milliseconds.
    #[get(type(copy))]
    pub elapsed_ms: f64,
    /// Wall-clock time at which the entry was recorded.
    #[get(type(copy))]
    pub timestamp_ms: f64,
}
