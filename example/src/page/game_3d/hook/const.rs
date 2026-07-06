/// The HTML `id` attribute value for the 3D game canvas element.
pub(crate) const GAME_3D_CANVAS_ID: &str = "game-3d-canvas";

/// The CSS selector used to query the 3D game canvas element from the DOM.
pub(crate) const GAME_3D_CANVAS_SELECTOR: &str = "#game-3d-canvas";

/// The default canvas width in CSS pixels.
pub(crate) const GAME_3D_CANVAS_WIDTH: f64 = 600.0;

/// The default canvas height in CSS pixels.
pub(crate) const GAME_3D_CANVAS_HEIGHT: f64 = 400.0;

/// The fixed timestep for the game loop in seconds (60 FPS).
pub(crate) const GAME_3D_FIXED_TIMESTEP: f64 = 1.0 / 60.0;

/// The half-size of a cube edge, used to define cube vertices relative to center.
pub(crate) const GAME_3D_CUBE_HALF_SIZE: f64 = 1.0;

/// The distance of the camera from the origin.
pub(crate) const GAME_3D_CAMERA_DISTANCE: f64 = 8.0;

/// The orbit yaw speed in radians per second for auto-rotation.
pub(crate) const GAME_3D_AUTO_YAW_SPEED: f64 = 0.5;

/// The minimum angle in radians between the camera pitch and ±π/2.
///
/// This prevents the orbit camera from looking straight up or down, which
/// would make the `forward` vector parallel to the `up` vector and cause
/// the `right = forward × up` cross product to degenerate, producing a
/// zero vector after normalization and collapsing the view matrix.
pub(crate) const GAME_3D_PITCH_CLAMP: f64 = 0.01;

/// The debounce interval in milliseconds for the resize event handler.
pub(crate) const GAME_3D_RESIZE_DEBOUNCE_MILLIS: i32 = 100;

/// The JavaScript property name for the canvas fill style.
pub(crate) const GAME_3D_PROPERTY_FILL_STYLE: &str = "fillStyle";

/// The JavaScript property name for the canvas stroke style.
pub(crate) const GAME_3D_PROPERTY_STROKE_STYLE: &str = "strokeStyle";

/// The canvas background fill color as a CSS color string.
pub(crate) const GAME_3D_BACKGROUND_COLOR: &str = "#0f0f23";

/// The CSS color used for cube faces.
pub(crate) const GAME_3D_CUBE_FACE_COLOR: &str = "#16c79a";

/// The CSS color used for cube edges.
pub(crate) const GAME_3D_CUBE_EDGE_COLOR: &str = "#e94560";

/// The CSS color used for the world origin axes.
pub(crate) const GAME_3D_AXIS_COLOR: &str = "#6366f1";

/// The JavaScript property name for the touch list `touches` on a `TouchEvent`.
pub(crate) const GAME_3D_EVENT_PROPERTY_TOUCHES: &str = "touches";

/// The JavaScript property name for the client X coordinate on a `Touch` object.
pub(crate) const GAME_3D_EVENT_PROPERTY_CLIENT_X: &str = "clientX";

/// The JavaScript property name for the client Y coordinate on a `Touch` object.
pub(crate) const GAME_3D_EVENT_PROPERTY_CLIENT_Y: &str = "clientY";

/// The cube vertex offsets relative to center, defining the 8 corners of a unit cube.
pub(crate) const GAME_3D_CUBE_VERTICES: [(f64, f64, f64); 8] = [
    (-1.0, -1.0, -1.0),
    (1.0, -1.0, -1.0),
    (1.0, 1.0, -1.0),
    (-1.0, 1.0, -1.0),
    (-1.0, -1.0, 1.0),
    (1.0, -1.0, 1.0),
    (1.0, 1.0, 1.0),
    (-1.0, 1.0, 1.0),
];

/// The cube face indices, each defining a quad by referencing 4 vertex indices.
/// Winding order is counter-clockwise when viewed from outside the cube,
/// ensuring that face normals point outward for correct back-face culling.
pub(crate) const GAME_3D_CUBE_FACES: [(usize, usize, usize, usize); 6] = [
    (0, 3, 2, 1),
    (4, 5, 6, 7),
    (0, 1, 5, 4),
    (2, 3, 7, 6),
    (0, 4, 7, 3),
    (1, 2, 6, 5),
];
