use super::*;

/// The internal state of a [`ThrottledValue`].
///
/// - `Idle` — no throttle window is active.
/// - `Cooldown(Instant)` — the most recent `set` happened
///   at `Instant`. While in cooldown, `set` calls are
///   accepted into the `pending` slot but do NOT update
///   the emitted value. When the cooldown expires
///   (next `tick` call at or after `Instant + interval`),
///   any pending value is committed and the state returns
///   to `Idle`.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum ThrottleState {
    /// No active cooldown.
    #[default]
    Idle,
    /// Cooldown is in effect since `Instant`.
    Cooldown(Instant),
}

/// A value that emits the most recent input at most once
/// per `interval_ms`.
///
/// Constructed via `ThrottledValue::new(interval_ms)`
/// (Lombok `New`); the emitted value starts at
/// `T::default()` and the throttle state starts at
/// `Idle`. Use [`ThrottledValue::set`] to seed the
/// emitted value.
///
/// Unlike [`DebouncedValue`], which waits for a quiet
/// period, a throttled value commits a snapshot every
/// `interval_ms` regardless of how often `set` was called.
///
/// Pair with `App::use_interval` — the interval callback
/// calls `tick(Instant::now())` every `interval_ms`. The
/// caller picks the time source so the hook stays free
/// of browser / timer dependencies.
#[derive(Clone, Data, Debug, New)]
pub struct ThrottledValue<T: Clone + PartialEq + Default + 'static> {
    /// The emitted value signal. Defaults to
    /// `Signal::create(T::default())` via
    /// `#[new(skip)]`.
    #[new(skip)]
    pub(crate) value: Signal<T>,
    /// The latest input waiting for the next commit.
    /// Defaults to `Signal::create(None)` via
    /// `#[new(skip)]`.
    #[new(skip)]
    pub(crate) pending: Signal<Option<T>>,
    /// The internal idle/cooldown state. Defaults to
    /// `Signal::create(ThrottleState::Idle)` via
    /// `#[new(skip)]`.
    #[new(skip)]
    pub(crate) state: Signal<ThrottleState>,
    /// The throttle window in milliseconds.
    pub(crate) interval_ms: u32,
}
