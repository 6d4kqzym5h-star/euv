use super::*;

/// A bounded reactive counter.
///
/// Holds an `i32` value that is clamped into
/// `[min, max]` (when those bounds are set) on every
/// mutation. `step` controls the default increment / decrement
/// amount.
///
/// Constructed via `Counter::new(initial)`,
/// `Counter::with_bounds(initial, min, max, step)`, or
/// `Counter::default()` (which gives `0` with no bounds
/// and `step = 1`).
///
/// The current value is exposed as a `Signal<i32>` accessed
/// through `get_value()`. Mutators
/// (`increment`, `decrement`, `set`, `reset`) write through
/// that signal, so any render closure that reads
/// `counter.get_value().get()` re-renders when the value
/// changes.
#[derive(Clone, Data, Debug)]
pub struct Counter {
    /// The current value signal.
    pub(crate) value: Signal<i32>,
    /// Optional minimum bound. `None` means unbounded below.
    pub(crate) min: Option<i32>,
    /// Optional maximum bound. `None` means unbounded above.
    pub(crate) max: Option<i32>,
    /// Step size used by `increment` / `decrement`.
    pub(crate) step: i32,
}
