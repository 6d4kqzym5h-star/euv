use super::*;

/// The complete set of Robert Penner easing functions used to warp a linear
/// interpolation parameter into an accelerated / decelerated curve.
///
/// Each variant maps a normalized input `t` in the range 0.0 to 1.0 to an
/// eased output, following the same naming as mainstream engines
/// (Godot `Tween::EaseType`, Phaser `Ease`, CSS `timing-function`).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Easing {
    /// Constant velocity: `f(t) = t`.
    #[default]
    Linear,
    /// Quadratic acceleration from zero velocity.
    InQuad,
    /// Quadratic deceleration to zero velocity.
    OutQuad,
    /// Quadratic acceleration until halfway, then deceleration.
    InOutQuad,
    /// Cubic acceleration from zero velocity.
    InCubic,
    /// Cubic deceleration to zero velocity.
    OutCubic,
    /// Cubic acceleration until halfway, then deceleration.
    InOutCubic,
    /// Quartic acceleration from zero velocity.
    InQuart,
    /// Quartic deceleration to zero velocity.
    OutQuart,
    /// Quartic acceleration until halfway, then deceleration.
    InOutQuart,
    /// Quintic acceleration from zero velocity.
    InQuint,
    /// Quintic deceleration to zero velocity.
    OutQuint,
    /// Quintic acceleration until halfway, then deceleration.
    InOutQuint,
    /// Sinusoidal acceleration from zero velocity.
    InSine,
    /// Sinusoidal deceleration to zero velocity.
    OutSine,
    /// Sinusoidal acceleration until halfway, then deceleration.
    InOutSine,
    /// Exponential acceleration from zero velocity.
    InExpo,
    /// Exponential deceleration to zero velocity.
    OutExpo,
    /// Exponential acceleration until halfway, then deceleration.
    InOutExpo,
    /// Circular acceleration from zero velocity.
    InCirc,
    /// Circular deceleration to zero velocity.
    OutCirc,
    /// Circular acceleration until halfway, then deceleration.
    InOutCirc,
    /// Overshoots backwards before moving forwards.
    InBack,
    /// Overshoots the target then settles back onto it.
    OutBack,
    /// Overshoots backwards then forwards past the target before settling.
    InOutBack,
    /// Elastic oscillation building up from rest.
    InElastic,
    /// Elastic oscillation settling down onto the target.
    OutElastic,
    /// Elastic oscillation at both ends.
    InOutElastic,
    /// Bounces off the start before moving forwards.
    InBounce,
    /// Bounces off the target like a dropped ball.
    OutBounce,
    /// Bounces at both ends.
    InOutBounce,
}
