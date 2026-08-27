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
