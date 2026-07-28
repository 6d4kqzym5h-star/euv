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

/// The delay in milliseconds before starting the 3D game loop after page mount.
///
/// Defers the heavy `requestAnimationFrame` rendering loop to avoid competing
/// with the mobile drawer close animation for main thread time, preventing
/// sidebar animation stutter on page transitions.
pub(crate) const GAME_3D_LOOP_START_DELAY_MILLIS: i32 = 360;

/// The JavaScript property name for the canvas fill style.
pub(crate) const GAME_3D_PROPERTY_FILL_STYLE: &str = "fillStyle";

/// The loading text displayed on the canvas before the game loop starts.
pub(crate) const GAME_3D_LOADING_TEXT: &str = "Loading...";

/// The CSS font family used for the loading text on the canvas.
pub(crate) const GAME_3D_LOADING_FONT_FAMILY: &str = "sans-serif";

/// The ratio of the loading font size to the canvas height.
pub(crate) const GAME_3D_LOADING_FONT_SIZE_RATIO: f64 = 0.04;

/// The CSS variable name for the loading text color on the canvas.
///
/// Uses `--text-on-accent` because the canvas background is `var!(accent)`,
/// and `text-on-accent` is the theme variable that contrasts with the accent
/// color (foreground/background equal accent in this monochrome design).
pub(crate) const GAME_3D_LOADING_COLOR_VAR: &str = "--text-on-accent";

/// The JavaScript property name for the canvas stroke style.
pub(crate) const GAME_3D_PROPERTY_STROKE_STYLE: &str = "strokeStyle";

/// The CSS color used for cube faces.
pub(crate) const GAME_3D_CUBE_FACE_COLOR: &str = "#16c79a";

/// The CSS color used for cube edges.
pub(crate) const GAME_3D_CUBE_EDGE_COLOR: &str = "#e94560";

/// The JavaScript property name for the touch list `touches` on a `TouchEvent`.
pub(crate) const GAME_3D_EVENT_PROPERTY_TOUCHES: &str = "touches";

/// The JavaScript property name for the client X coordinate on a `Touch` object.
pub(crate) const GAME_3D_EVENT_PROPERTY_CLIENT_X: &str = "clientX";

/// The JavaScript property name for the client Y coordinate on a `Touch` object.
pub(crate) const GAME_3D_EVENT_PROPERTY_CLIENT_Y: &str = "clientY";

/// The JavaScript event name for the wheel event, used to register a
/// non-passive listener directly on the canvas element to prevent page
/// scrolling when the mouse wheel is scrolled over the canvas.
pub(crate) const GAME_3D_EVENT_WHEEL: &str = "wheel";

/// The JavaScript event name for the touchstart event, used to register a
/// non-passive listener directly on the canvas element to prevent page
/// scrolling when a finger touches the canvas on mobile devices.
pub(crate) const GAME_3D_EVENT_TOUCH_START: &str = "touchstart";

/// The JavaScript event name for the touchmove event, used to register a
/// non-passive listener directly on the canvas element to prevent page
/// scrolling when a finger drags across the canvas on mobile devices.
pub(crate) const GAME_3D_EVENT_TOUCH_MOVE: &str = "touchmove";

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

/// The 12 unique edges of a unit cube, defined by pairs of vertex indices.
///
/// Used to draw the cube wireframe without duplicating the shared edge
/// between two adjacent visible faces — without deduplication the three
/// edges meeting at the front-most vertex are each stroked twice (once per
/// face), which shows up as visible "extra lines" near the cube's inner
/// corner after SSAA downscaling.
pub(crate) const GAME_3D_CUBE_EDGES: [(usize, usize); 12] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 0),
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 4),
    (0, 4),
    (1, 5),
    (2, 6),
    (3, 7),
];

/// The HTML `id` attribute value for the 3D WebGPU canvas element.
pub(crate) const GAME_3D_WEBGPU_CANVAS_ID: &str = "game-3d-webgpu-canvas";

/// The CSS selector used to query the 3D WebGPU canvas element from the DOM.
pub(crate) const GAME_3D_WEBGPU_CANVAS_SELECTOR: &str = "#game-3d-webgpu-canvas";

/// The WGSL shader source for the 3D WebGPU demo.
///
/// Renders a triangle with pseudo-3D perspective by dividing the x and y
/// coordinates by the negative z coordinate. Vertex colors are interpolated
/// across the triangle surface. A `vec2<f32>` uniform at
/// `@group(0) @binding(0)` carries (yaw, pitch) drag angles: the vertices
/// are rotated around the triangle centroid before the perspective divide,
/// so dragging on the canvas orbits the triangle in place.
pub(crate) const GAME_3D_WEBGPU_SHADER: &str = r#"
struct CameraUniforms {
    rotation: vec2<f32>,
};

@group(0) @binding(0) var<uniform> u_camera: CameraUniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

fn rotate_y(p: vec3<f32>, angle: f32) -> vec3<f32> {
    let s = sin(angle);
    let c = cos(angle);
    return vec3<f32>(p.x * c + p.z * s, p.y, p.z * c - p.x * s);
}

fn rotate_x(p: vec3<f32>, angle: f32) -> vec3<f32> {
    let s = sin(angle);
    let c = cos(angle);
    return vec3<f32>(p.x, p.y * c - p.z * s, p.y * s + p.z * c);
}

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VertexOutput {
    var p = array<vec3<f32>, 3>(
        vec3<f32>(0.0, 0.5, -1.5),
        vec3<f32>(-0.5, -0.5, -2.5),
        vec3<f32>(0.5, -0.5, -1.0),
    );
    var c = array<vec3<f32>, 3>(
        vec3<f32>(0.2, 0.8, 1.0),
        vec3<f32>(0.8, 0.2, 1.0),
        vec3<f32>(0.2, 1.0, 0.8),
    );
    let center = vec3<f32>(0.0, -0.1667, -1.6667);
    var pos = p[vi] - center;
    pos = rotate_y(pos, u_camera.rotation.x);
    pos = rotate_x(pos, u_camera.rotation.y);
    pos = pos + center;
    var out: VertexOutput;
    out.position = vec4<f32>(pos.x / -pos.z, pos.y / -pos.z, 0.0, 1.0);
    out.color = c[vi];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

/// The HTML `id` attribute value for the 3D WebGL canvas element.
pub(crate) const GAME_3D_WEBGL_CANVAS_ID: &str = "game-3d-webgl-canvas";

/// The CSS selector used to query the 3D WebGL canvas element from the DOM.
pub(crate) const GAME_3D_WEBGL_CANVAS_SELECTOR: &str = "#game-3d-webgl-canvas";

/// The placeholder shown in the pointer readout before the pointer first
/// enters the canvas.
pub(crate) const GAME_3D_POINTER_EMPTY_TEXT: &str = "-";

/// The radians of rotation applied per pixel of pointer drag.
pub(crate) const GAME_3D_DRAG_SENSITIVITY: f64 = 0.01;

/// The absolute pitch limit in radians, keeping the view from flipping
/// over the poles when dragging vertically.
pub(crate) const GAME_3D_PITCH_LIMIT: f64 = 1.5;

/// The GLSL ES 3.00 vertex shader source for the 3D WebGL demo.
///
/// Mirrors [`GAME_3D_WEBGPU_SHADER`]: vertices are generated procedurally
/// from `gl_VertexID` (attribute-less rendering, valid in WebGL 2) and a
/// `vec2` uniform carries the (yaw, pitch) drag angles applied around the
/// triangle centroid before the perspective divide.
pub(crate) const GAME_3D_WEBGL_VERTEX_SHADER: &str = r#"#version 300 es

uniform vec2 u_rotation;

out vec3 v_color;

vec3 rotate_y(vec3 p, float angle) {
    float s = sin(angle);
    float c = cos(angle);
    return vec3(p.x * c + p.z * s, p.y, p.z * c - p.x * s);
}

vec3 rotate_x(vec3 p, float angle) {
    float s = sin(angle);
    float c = cos(angle);
    return vec3(p.x, p.y * c - p.z * s, p.y * s + p.z * c);
}

void main() {
    vec3 p[3] = vec3[3](
        vec3(0.0, 0.5, -1.5),
        vec3(-0.5, -0.5, -2.5),
        vec3(0.5, -0.5, -1.0)
    );
    vec3 c[3] = vec3[3](
        vec3(0.2, 0.8, 1.0),
        vec3(0.8, 0.2, 1.0),
        vec3(0.2, 1.0, 0.8)
    );
    vec3 center = vec3(0.0, -0.1667, -1.6667);
    vec3 pos = p[gl_VertexID] - center;
    pos = rotate_y(pos, u_rotation.x);
    pos = rotate_x(pos, u_rotation.y);
    pos = pos + center;
    gl_Position = vec4(pos.x / -pos.z, pos.y / -pos.z, 0.0, 1.0);
    v_color = c[gl_VertexID];
}
"#;

/// The GLSL ES 3.00 fragment shader source for the 3D WebGL demo.
pub(crate) const GAME_3D_WEBGL_FRAGMENT_SHADER: &str = r#"#version 300 es

precision mediump float;

in vec3 v_color;

out vec4 out_color;

void main() {
    out_color = vec4(v_color, 1.0);
}
"#;
