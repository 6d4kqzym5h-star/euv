/// The HTML `id` attribute value for the 2D game canvas element.
pub(crate) const GAME_2D_CANVAS_ID: &str = "game-2d-canvas";

/// The CSS selector used to query the 2D game canvas element from the DOM.
pub(crate) const GAME_2D_CANVAS_SELECTOR: &str = "#game-2d-canvas";

/// The default canvas width in CSS pixels.
pub(crate) const GAME_2D_CANVAS_WIDTH: f64 = 600.0;

/// The default canvas height in CSS pixels.
pub(crate) const GAME_2D_CANVAS_HEIGHT: f64 = 400.0;

/// The gravitational acceleration in pixels per second squared.
pub(crate) const GAME_2D_GRAVITY: f64 = 600.0;

/// The minimum radius of a ball in pixels.
pub(crate) const GAME_2D_BALL_MIN_RADIUS: f64 = 8.0;

/// The maximum radius of a ball in pixels.
pub(crate) const GAME_2D_BALL_MAX_RADIUS: f64 = 30.0;

/// The restitution (bounciness) coefficient for wall and ball collisions.
pub(crate) const GAME_2D_RESTITUTION: f64 = 0.85;

/// The linear damping coefficient applied per second to simulate air resistance.
pub(crate) const GAME_2D_LINEAR_DAMPING: f64 = 0.1;

/// The initial upward velocity magnitude when spawning a ball.
pub(crate) const GAME_2D_SPAWN_VELOCITY: f64 = 200.0;

/// The fixed timestep for the 2D game loop in seconds (60 FPS).
pub(crate) const GAME_2D_FIXED_TIMESTEP: f64 = 1.0 / 60.0;

/// The maximum number of balls allowed simultaneously.
pub(crate) const GAME_2D_MAX_BALLS: usize = 100;

/// The debounce interval in milliseconds for the resize event handler.
pub(crate) const GAME_2D_RESIZE_DEBOUNCE_MILLIS: i32 = 100;

/// The delay in milliseconds before starting the 2D game loop after page mount.
///
/// Defers the heavy `requestAnimationFrame` rendering loop to avoid competing
/// with the mobile drawer close animation for main thread time, preventing
/// sidebar animation stutter on page transitions.
pub(crate) const GAME_2D_LOOP_START_DELAY_MILLIS: i32 = 360;

/// The JavaScript property name for the canvas fill style.
pub(crate) const GAME_2D_PROPERTY_FILL_STYLE: &str = "fillStyle";

/// The loading text displayed on the canvas before the game loop starts.
pub(crate) const GAME_2D_LOADING_TEXT: &str = "Loading...";

/// The CSS font family used for the loading text on the canvas.
pub(crate) const GAME_2D_LOADING_FONT_FAMILY: &str = "sans-serif";

/// The ratio of the loading font size to the canvas height.
pub(crate) const GAME_2D_LOADING_FONT_SIZE_RATIO: f64 = 0.04;

/// The CSS variable name for the loading text color on the canvas.
///
/// Uses `--text-on-accent` because the canvas background is `var!(accent)`,
/// and `text-on-accent` is the theme variable that contrasts with the accent
/// color (foreground/background equal accent in this monochrome design).
pub(crate) const GAME_2D_LOADING_COLOR_VAR: &str = "--text-on-accent";

/// The palette of ball colors used for random color assignment.
pub(crate) const GAME_2D_BALL_COLORS: &[&str] = &[
    "#e94560", "#0f3460", "#16c79a", "#f5b461", "#ec524b", "#41b883", "#6c5ce7", "#fd79a8",
    "#00cec9", "#fab1a0",
];

/// The number of physics substeps performed per fixed timestep.
///
/// Splits the fixed `1/60s` timestep into smaller slices so a fast ball never
/// moves more than its own radius between collision checks, preventing the
/// "tunneling" effect where one ball passes through another in a single step.
pub(crate) const GAME_2D_PHYSICS_SUBSTEPS: usize = 4;

/// The number of ball-to-ball collision resolution passes per substep.
///
/// A single pass is insufficient when a ball is squeezed between two or more
/// other balls: resolving the overlap with ball A can leave the ball still
/// overlapping with ball C. Repeating the pass lets the correction propagate
/// through the contact graph until every overlap is cleared (or a stable
/// stacking configuration is reached).
pub(crate) const GAME_2D_COLLISION_ITERATIONS: usize = 4;

/// The HTML `id` attribute value for the 2D WebGPU canvas element.
pub(crate) const GAME_2D_WEBGPU_CANVAS_ID: &str = "game-2d-webgpu-canvas";

/// The CSS selector used to query the 2D WebGPU canvas element from the DOM.
pub(crate) const GAME_2D_WEBGPU_CANVAS_SELECTOR: &str = "#game-2d-webgpu-canvas";

/// The WGSL shader source for the 2D WebGPU demo.
///
/// Renders a classic RGB triangle (red, green, blue vertices) with
/// interpolated vertex colors. Vertex positions are derived from
/// `@builtin(vertex_index)` so no vertex buffer is needed.
pub(crate) const GAME_2D_WEBGPU_SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VertexOutput {
    var p = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5),
    );
    var c = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0),
    );
    var out: VertexOutput;
    out.position = vec4<f32>(p[vi], 0.0, 1.0);
    out.color = c[vi];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;
