use super::*;

/// The internal state of a [`DebouncedValue`].
///
/// - `Idle` — no value is pending. `get()` returns the
///   last emitted value.
/// - `Pending(Instant, T)` — `set(T)` was called at
///   `Instant`. If `tick()` is called before
///   `Instant + delay`, nothing happens. If `tick()` is
///   called at or after `Instant + delay`, the pending
///   value is emitted and the state returns to `Idle`.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum DebounceState<T> {
    /// No pending value.
    #[default]
    Idle,
    /// A pending value with the timestamp it was set at.
    Pending(Instant, T),
}
