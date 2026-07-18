use crate::*;

/// A single bouncing ball in the 2D physics demo game.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Ball {
    /// The current world-space position of the ball center.
    pub(crate) position: Vector2D,
    /// The current linear velocity in pixels per second.
    pub(crate) velocity: Vector2D,
    /// The radius of the ball in pixels.
    pub(crate) radius: f64,
    /// The CSS color string used to fill the ball.
    pub(crate) color: String,
}

/// Reactive state for the 2D bouncing balls game page.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseGame2D {
    /// Whether the 2D game loop is currently running.
    #[get(type(copy))]
    pub(crate) running: Signal<bool>,
    /// The current frames-per-second measurement.
    #[get(type(copy))]
    pub(crate) fps: Signal<f64>,
    /// The current number of balls on the canvas.
    #[get(type(copy))]
    pub(crate) ball_count: Signal<usize>,
    /// The total number of balls spawned since the 2D game started.
    #[get(type(copy))]
    pub(crate) total_spawned: Signal<usize>,
    /// Whether the canvas has finished loading and is ready for interaction.
    #[get(type(copy))]
    pub(crate) loaded: Signal<bool>,
}

/// A persistent wrapper for the ball list that survives component re-renders.
#[derive(Clone, Debug)]
pub(crate) struct BallStore(pub(crate) Rc<RefCell<Vec<Ball>>>);

/// A cached reference to the 2D game canvas element used for efficient coordinate mapping.
#[derive(Clone, Debug)]
pub(crate) struct CanvasCache(pub(crate) Rc<RefCell<Option<HtmlCanvasElement>>>);

/// Reactive state for the 2D WebGPU demo page.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseGame2DWebGpu {
    /// The current frames-per-second measurement.
    #[get(type(copy))]
    pub(crate) fps: Signal<f64>,
    /// Whether the WebGPU renderer has finished initializing (success or failure).
    #[get(type(copy))]
    pub(crate) loaded: Signal<bool>,
    /// Whether the WebGPU renderer is active and rendering.
    #[get(type(copy))]
    pub(crate) active: Signal<bool>,
    /// Whether the WebGPU render loop has been kicked off in this component tree.
    #[get(type(copy))]
    pub(crate) loop_started: Signal<bool>,
}
