use super::*;

/// The playback state of a `Tween`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TweenState {
    /// The tween is waiting for its start delay to elapse.
    #[default]
    Delayed,
    /// The tween is actively interpolating.
    Running,
    /// The tween is paused and can be resumed.
    Paused,
    /// The tween has completed (`AnimationMode::Once` only).
    Finished,
}
