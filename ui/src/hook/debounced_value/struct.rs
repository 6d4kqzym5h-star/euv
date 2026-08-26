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
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum DebounceState<T> {
    /// No pending value.
    Idle,
    /// A pending value with the timestamp it was set at.
    Pending(Instant, T),
}

/// A value that only emits after a quiet period of
/// `delay` since its most recent `set`.
///
/// Typical use: pair with `App::use_interval` — the
/// interval callback calls `tick(Instant::now())` every
/// N milliseconds. After `delay` ms without a fresh
/// `set`, the pending value is committed.
///
/// This shape keeps the hook free of any browser /
/// timer dependency so the same code runs in
/// `cargo test` and in `wasm32-unknown-unknown` — the
/// caller supplies the time source.
#[derive(Clone, Data, Debug)]
pub struct DebouncedValue<T: Clone + PartialEq + 'static> {
    /// The last emitted value (also the initial value).
    pub(crate) value: Signal<T>,
    /// The internal pending/empty state.
    pub(crate) state: Signal<DebounceState<T>>,
    /// The quiet period in milliseconds.
    pub(crate) delay_ms: u32,
}
