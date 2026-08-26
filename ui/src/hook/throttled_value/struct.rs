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
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ThrottleState {
    /// No active cooldown.
    Idle,
    /// Cooldown is in effect since `Instant`.
    Cooldown(Instant),
}

/// A value that emits the most recent input at most once
/// per `interval_ms`.
///
/// Unlike [`DebouncedValue`], which waits for a quiet
/// period, a throttled value commits a snapshot every
/// `interval_ms` regardless of how often `set` was called.
///
/// Pair with `App::use_interval` — the interval callback
/// calls `tick(Instant::now())` every `interval_ms`. The
/// caller picks the time source so the hook stays free
/// of browser / timer dependencies.
#[derive(Clone, Data, Debug)]
pub struct ThrottledValue<T: Clone + PartialEq + 'static> {
    /// The emitted value signal.
    pub(crate) value: Signal<T>,
    /// The latest input waiting for the next commit.
    /// `None` when no input is queued.
    pub(crate) pending: Signal<Option<T>>,
    /// The internal idle/cooldown state.
    pub(crate) state: Signal<ThrottleState>,
    /// The throttle window in milliseconds.
    pub(crate) interval_ms: u32,
}
