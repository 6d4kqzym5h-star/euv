use crate::*;

/// Rendering configuration controlling the backend type, canvas target,
/// viewport dimensions, quality, and backend-specific options.
///
/// Passed as part of `EngineConfig` when instantiating the engine.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct RenderConfig {
    /// The rendering backend to use (Canvas 2D or WebGPU).
    #[get(type(copy))]
    pub(crate) backend: RenderBackendType,
    /// The CSS selector used to query the canvas element from the DOM.
    #[get(type(clone))]
    pub(crate) canvas_selector: String,
    /// The logical viewport width in CSS pixels.
    #[get(type(copy))]
    pub(crate) width: f64,
    /// The logical viewport height in CSS pixels.
    #[get(type(copy))]
    pub(crate) height: f64,
    /// The rendering quality preset for canvas smoothing and SSAA downscaling.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) quality: RenderQuality,
    /// Whether MSAA anti-aliasing is enabled for the WebGPU backend.
    ///
    /// Ignored when `backend` is `Canvas2D`.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) antialias: bool,
    /// The power preference hint for WebGPU adapter selection.
    ///
    /// Ignored when `backend` is `Canvas2D`.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) power_preference: GpuPowerPreference,
    /// The SSAA supersampling scale factor for the Canvas 2D backend.
    ///
    /// A value of 2.0 renders at 4x resolution and downscales for smoother
    /// edges. Ignored when `backend` is `WebGpu`.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) ssaa_scale_factor: f64,
}

/// The top-level engine configuration containing rendering and scheduler settings.
///
/// Passed to `Engine::new` to configure the engine instance.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct EngineConfig {
    /// The rendering configuration.
    #[get(type(clone))]
    pub(crate) render: RenderConfig,
    /// The scheduler configuration controlling the fixed-timestep game loop.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) scheduler: SchedulerConfig,
}
