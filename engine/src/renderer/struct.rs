use crate::*;

/// A 2D camera that defines the viewport into the game world.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Camera2D {
    /// The world-space position of the camera center.
    #[get(type(copy))]
    pub(crate) position: Vector2D,
    /// The zoom factor (1.0 = no zoom, 2.0 = 2x magnification).
    #[get(type(copy))]
    pub(crate) zoom: f64,
    /// The rotation angle in radians.
    #[get(type(copy))]
    pub(crate) rotation: f64,
    /// The viewport width in screen pixels.
    #[get(type(copy))]
    pub(crate) viewport_width: f64,
    /// The viewport height in screen pixels.
    #[get(type(copy))]
    pub(crate) viewport_height: f64,
}

/// A 3D camera that defines the viewport into a 3D world using perspective
/// or orthographic projection.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Camera3D {
    /// The world-space position of the camera (eye).
    #[get(type(copy))]
    pub(crate) position: Vector3D,
    /// The point the camera is looking at (target).
    #[get(type(copy))]
    pub(crate) target: Vector3D,
    /// The up direction for the camera.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) up: Vector3D,
    /// The vertical field of view in radians.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) fov: f64,
    /// The near clipping plane distance.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) near: f64,
    /// The far clipping plane distance.
    #[get(type(copy))]
    #[new(skip)]
    pub(crate) far: f64,
    /// The viewport width in pixels.
    #[get(type(copy))]
    pub(crate) viewport_width: f64,
    /// The viewport height in pixels.
    #[get(type(copy))]
    pub(crate) viewport_height: f64,
}

/// A wrapper around `CanvasRenderingContext2d` providing convenience
/// drawing methods and camera management for the game engine.
#[derive(Clone, Data, New)]
pub struct CanvasRenderer {
    /// The underlying canvas 2D rendering context.
    pub(crate) context: CanvasRenderingContext2d,
    /// The active camera controlling the viewport.
    #[get(type(copy))]
    pub(crate) camera: Camera2D,
}

/// A supersampling anti-aliasing (SSAA) canvas wrapper that renders at a higher
/// resolution on an offscreen canvas and downscales to the display canvas for
/// smoother polygon edges in software-rendered 3D scenes.
///
/// The offscreen context is scaled by `scale_factor` so that all drawing
/// code can use logical pixel coordinates without modification. After
/// rendering, call `present()` to draw the high-resolution buffer onto the
/// visible canvas with high-quality image smoothing.
#[derive(Clone, Data, New)]
pub struct SsaaCanvas {
    /// The display canvas element visible to the user.
    pub(crate) display_canvas: HtmlCanvasElement,
    /// The 2D rendering context of the display canvas used for final presentation.
    pub(crate) display_context: CanvasRenderingContext2d,
    /// The offscreen canvas used for high-resolution rendering.
    pub(crate) offscreen_canvas: HtmlCanvasElement,
    /// The 2D rendering context of the offscreen canvas, pre-scaled by `scale_factor`.
    pub(crate) offscreen_context: CanvasRenderingContext2d,
    /// The supersampling scale factor (e.g., 2.0 means 4x SSAA).
    #[get(type(copy))]
    pub(crate) scale_factor: f64,
    /// The logical display width in CSS pixels.
    #[get(type(copy))]
    pub(crate) width: f64,
    /// The logical display height in CSS pixels.
    #[get(type(copy))]
    pub(crate) height: f64,
}
