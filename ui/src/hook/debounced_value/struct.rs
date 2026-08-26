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

/// A value that only emits after a quiet period of
/// `delay` since its most recent `set`.
///
/// Constructed via `DebouncedValue::new(delay_ms)`
/// (Lombok `New`); the emitted value starts at
/// `T::default()` and the throttle state starts at
/// `Idle`. Use [`DebouncedValue::set`] (or
/// [`DebouncedValue::tick`] with a backdated `Instant`)
/// to seed the emitted value.
///
/// Typical use: pair with `App::use_interval` — the
/// interval callback calls `tick(Instant::now())` every
/// N milliseconds. After `delay_ms` without a fresh
/// `set`, the pending value is committed.
///
/// This shape keeps the hook free of any browser /
/// timer dependency so the same code runs in
/// `cargo test` and in `wasm32-unknown-unknown` — the
/// caller supplies the time source.
#[derive(Clone, Data, Debug, New)]
pub struct DebouncedValue<T: Clone + PartialEq + Default + 'static> {
    /// The emitted value signal. Defaults to
    /// `Signal::create(T::default())` via
    /// `#[new(skip)]`.
    #[new(skip)]
    pub(crate) value: Signal<T>,
    /// The internal pending/empty state. Defaults to
    /// `Signal::create(DebounceState::Idle)` via
    /// `#[new(skip)]`.
    #[new(skip)]
    pub(crate) state: Signal<DebounceState<T>>,
    /// The quiet period in milliseconds.
    pub(crate) delay_ms: u32,
}
