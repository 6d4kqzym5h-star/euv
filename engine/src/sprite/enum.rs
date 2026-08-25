use super::*;

/// Defines how an animation behaves when it reaches the last frame.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AnimationMode {
    /// The animation loops back to the first frame after the last frame.
    #[default]
    Loop,
    /// The animation plays once and stops on the last frame.
    Once,
    /// The animation plays forward then backward alternately.
    PingPong,
}

/// Represents the current playback state of an animation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AnimationState {
    /// The animation is currently playing.
    #[default]
    Playing,
    /// The animation is paused and can be resumed.
    Paused,
    /// The animation has finished (reached the end in `Once` mode).
    Finished,
}
