use super::*;

/// A generic tween interpolating a value of type `T` from a start to an end
/// value over a fixed duration, driven by an [`Easing`] curve.
///
/// Works with any type implementing [`Interpolable`] — the engine provides
/// implementations for `f64`, [`Vector2D`], [`Vector3D`], and [`Color`].
///
/// ## Why hand-written accessors
///
/// Lombok's `Data` derive is intentionally **not** applied here for the same
/// reason as [`EngineCell`]: the derive does not propagate generic bounds, so
/// deriving on `Tween<T>` would force `T: Default`-style bounds that
/// `Interpolable` types do not carry. The accessor pairs below follow the
/// same naming contract as the Lombok-generated ones (`get_*` / `set_*`).
pub struct Tween<T: Interpolable + Copy> {
    /// The value at the start of the tween.
    pub(crate) from: T,
    /// The value at the end of the tween.
    pub(crate) to: T,
    /// The total interpolation duration in seconds.
    pub(crate) duration: f64,
    /// The easing curve applied to the normalized time.
    pub(crate) easing: Easing,
    /// The start delay in seconds before interpolation begins.
    pub(crate) delay: f64,
    /// The time elapsed since creation (including the delay phase).
    pub(crate) elapsed: f64,
    /// The current playback state.
    pub(crate) state: TweenState,
    /// What happens when the tween reaches the end of its duration.
    pub(crate) mode: AnimationMode,
    /// The current playback direction (1.0 = forward, -1.0 = backward) for ping-pong mode.
    pub(crate) direction: f64,
    /// An optional callback fired when the tween completes a cycle.
    pub(crate) on_complete: Option<Rc<dyn Fn()>>,
}
