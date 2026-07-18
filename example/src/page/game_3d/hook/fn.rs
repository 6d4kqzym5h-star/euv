use crate::*;

/// Creates the 3D game reactive state signals wrapped in a `UseGame3D` struct.
///
/// # Returns
///
/// - `UseGame3D` - The 3D game state.
pub(crate) fn use_game_3d_state() -> UseGame3D {
    UseGame3D {
        running: App::use_signal(|| true),
        fps: App::use_signal(|| 0.0),
        cube_count: App::use_signal(|| 0),
        auto_rotate: App::use_signal(|| true),
        loaded: App::use_signal(|| false),
    }
}

/// Creates the initial set of cubes for the 3D scene.
///
/// # Returns
///
/// - `Vec<Cube3D>` - The initial cube list.
pub(crate) fn create_initial_cubes() -> Vec<Cube3D> {
    vec![
        Cube3D {
            position: Vector3D::new(0.0, 0.0, 0.0),
            rotation: Quaternion::identity(),
            angular_velocity: Vector3D::new(0.5, 1.0, 0.3),
            scale: 1.5,
            face_color: GAME_3D_CUBE_FACE_COLOR.to_string(),
            edge_color: GAME_3D_CUBE_EDGE_COLOR.to_string(),
        },
        Cube3D {
            position: Vector3D::new(-3.0, 0.0, 0.0),
            rotation: Quaternion::from_euler(0.0, 0.5, 0.0),
            angular_velocity: Vector3D::new(0.3, -0.7, 0.5),
            scale: 0.8,
            face_color: "#6c5ce7".to_string(),
            edge_color: "#fd79a8".to_string(),
        },
        Cube3D {
            position: Vector3D::new(3.0, 0.0, 0.0),
            rotation: Quaternion::from_euler(0.5, 0.0, 0.0),
            angular_velocity: Vector3D::new(-0.4, 0.6, -0.2),
            scale: 0.8,
            face_color: "#f5b461".to_string(),
            edge_color: "#00cec9".to_string(),
        },
        Cube3D {
            position: Vector3D::new(0.0, 2.5, 0.0),
            rotation: Quaternion::identity(),
            angular_velocity: Vector3D::new(0.8, 0.2, -0.6),
            scale: 0.6,
            face_color: "#ec524b".to_string(),
            edge_color: "#41b883".to_string(),
        },
    ]
}

/// Creates a `Camera3D` from the current yaw and pitch orbit angles.
///
/// # Arguments
///
/// - `f64` - The orbit yaw angle in radians.
/// - `f64` - The orbit pitch angle in radians.
///
/// # Returns
///
/// - `Camera3D` - The configured camera.
pub(crate) fn create_orbit_camera(yaw: f64, pitch: f64) -> Camera3D {
    let cos_pitch: f64 = pitch.cos();
    let position: Vector3D = Vector3D::new(
        GAME_3D_CAMERA_DISTANCE * yaw.sin() * cos_pitch,
        GAME_3D_CAMERA_DISTANCE * pitch.sin(),
        GAME_3D_CAMERA_DISTANCE * yaw.cos() * cos_pitch,
    );
    Camera3D::create(
        position,
        Vector3D::zero(),
        GAME_3D_CANVAS_WIDTH,
        GAME_3D_CANVAS_HEIGHT,
    )
}

/// Transforms a cube's local vertex to world space.
///
/// # Arguments
///
/// - `&Cube3D` - The cube instance.
/// - `Vector3D` - The local-space vertex.
///
/// # Returns
///
/// - `Vector3D` - The world-space vertex.
pub(crate) fn transform_cube_vertex(cube: &Cube3D, local: Vector3D) -> Vector3D {
    let scaled: Vector3D = Vector3D::new(
        local.get_x() * cube.scale * GAME_3D_CUBE_HALF_SIZE,
        local.get_y() * cube.scale * GAME_3D_CUBE_HALF_SIZE,
        local.get_z() * cube.scale * GAME_3D_CUBE_HALF_SIZE,
    );
    scaled.rotated_by(cube.rotation) + cube.position
}

/// Computes the average depth of a cube face's vertices in camera space.
///
/// # Arguments
///
/// - `&[Vector3D]` - The world-space vertices of the face.
/// - `&Camera3D` - The camera.
///
/// # Returns
///
/// - `f64` - The average z depth (negative is farther away).
pub(crate) fn face_average_depth(world_vertices: &[Vector3D], camera: &Camera3D) -> f64 {
    let view_matrix: Matrix4x4 = camera.view_matrix();
    let mut sum_z: f64 = 0.0;
    for vertex in world_vertices {
        let view_vertex: Vector3D = view_matrix.transform_point(*vertex);
        sum_z += view_vertex.get_z();
    }
    sum_z / world_vertices.len() as f64
}

/// Computes the normal of a cube face using the cross product of two edges.
///
/// # Arguments
///
/// - `&[Vector3D]` - The world-space vertices of the face (at least 3).
///
/// # Returns
///
/// - `Vector3D` - The face normal.
pub(crate) fn face_normal(world_vertices: &[Vector3D]) -> Vector3D {
    let edge_a: Vector3D = world_vertices[1] - world_vertices[0];
    let edge_b: Vector3D = world_vertices[2] - world_vertices[0];
    edge_a.cross(edge_b).normalized()
}

/// Determines whether a face is visible (back-face culling).
///
/// # Arguments
///
/// - `&[Vector3D]` - The world-space vertices of the face.
/// - `&Camera3D` - The camera.
///
/// # Returns
///
/// - `bool` - True if the face should be rendered.
pub(crate) fn is_face_visible(world_vertices: &[Vector3D], camera: &Camera3D) -> bool {
    let normal: Vector3D = face_normal(world_vertices);
    let face_center: Vector3D = world_vertices
        .iter()
        .fold(Vector3D::zero(), |acc: Vector3D, vertex: &Vector3D| {
            acc + *vertex
        })
        .scaled(1.0 / world_vertices.len() as f64);
    let view_direction: Vector3D = (face_center - camera.get_position()).normalized();
    normal.dot(view_direction) < 0.0
}

/// Renders the 3D scene onto the SSAA offscreen canvas and presents it to the display.
///
/// Clears the offscreen canvas to transparency so the CSS `background`
/// property (set to `var(--accent)`) shows through on the display canvas,
/// draws the world axes, then for each cube (back-to-front via painter's
/// algorithm) draws the visible face fills and finally the unique visible
/// edges as a separate wireframe pass. The fill/stroke separation avoids
/// stroking each shared cube edge twice (which would otherwise appear as
/// thicker lines near the inner corner where three visible faces meet).
/// Calls `present()` to downscale the high-resolution buffer onto the
/// visible canvas with high-quality image smoothing for SSAA anti-aliasing.
///
/// # Arguments
///
/// - `&SsaaCanvas` - The SSAA canvas wrapper.
/// - `&[Cube3D]` - The cube list to render.
/// - `&Camera3D` - The camera.
pub(crate) fn render_scene(ssaa_canvas: &SsaaCanvas, cubes: &[Cube3D], camera: &Camera3D) {
    let context: &CanvasRenderingContext2d = ssaa_canvas.get_offscreen_context();
    context.clear_rect(0.0, 0.0, GAME_3D_CANVAS_WIDTH, GAME_3D_CANVAS_HEIGHT);
    let mut cube_batches: Vec<(f64, &Cube3D, Vec<Vector3D>)> = cubes
        .iter()
        .map(|cube: &Cube3D| {
            let world_vertices: Vec<Vector3D> = GAME_3D_CUBE_VERTICES
                .iter()
                .map(|(vx, vy, vz): &(f64, f64, f64)| {
                    transform_cube_vertex(cube, Vector3D::new(*vx, *vy, *vz))
                })
                .collect();
            let depth: f64 = face_average_depth(&world_vertices, camera);
            (depth, cube, world_vertices)
        })
        .collect();
    cube_batches.sort_by(
        |a: &(f64, &Cube3D, Vec<Vector3D>), b: &(f64, &Cube3D, Vec<Vector3D>)| {
            a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)
        },
    );
    for (_cube_depth, cube, world_vertices) in &cube_batches {
        let mut face_batches: Vec<(f64, Vec<Vector3D>)> = Vec::new();
        for (i0, i1, i2, i3) in GAME_3D_CUBE_FACES {
            let face_world: Vec<Vector3D> = vec![
                world_vertices[i0],
                world_vertices[i1],
                world_vertices[i2],
                world_vertices[i3],
            ];
            if !is_face_visible(&face_world, camera) {
                continue;
            }
            let depth: f64 = face_average_depth(&face_world, camera);
            face_batches.push((depth, face_world));
        }
        face_batches.sort_by(|a: &(f64, Vec<Vector3D>), b: &(f64, Vec<Vector3D>)| {
            a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)
        });
        let _ = Reflect::set(
            context,
            &JsValue::from_str(GAME_3D_PROPERTY_FILL_STYLE),
            &JsValue::from_str(&cube.face_color),
        );
        for (_depth, face_world) in &face_batches {
            let screen_vertices: Vec<Vector3D> = face_world
                .iter()
                .map(|world: &Vector3D| camera.world_to_screen(*world))
                .collect();
            context.begin_path();
            context.move_to(screen_vertices[0].get_x(), screen_vertices[0].get_y());
            for screen_vertex in screen_vertices.iter().skip(1) {
                context.line_to(screen_vertex.get_x(), screen_vertex.get_y());
            }
            context.close_path();
            context.fill();
        }
        let visible_edges: Vec<(usize, usize)> = collect_visible_edges(world_vertices, camera);
        let _ = Reflect::set(
            context,
            &JsValue::from_str(GAME_3D_PROPERTY_STROKE_STYLE),
            &JsValue::from_str(&cube.edge_color),
        );
        context.set_line_width(1.5);
        context.set_line_join("miter");
        for (i_a, i_b) in &visible_edges {
            let v_a: Vector3D = world_vertices[*i_a];
            let v_b: Vector3D = world_vertices[*i_b];
            let s_a: Vector3D = camera.world_to_screen(v_a);
            let s_b: Vector3D = camera.world_to_screen(v_b);
            context.begin_path();
            context.move_to(s_a.get_x(), s_a.get_y());
            context.line_to(s_b.get_x(), s_b.get_y());
            context.stroke();
        }
    }
    ssaa_canvas.present();
}

/// Collects the unique edges of a cube that belong to at least one
/// visible (front-facing) face.
///
/// Iterates the 12 cube edges in `GAME_3D_CUBE_EDGES` and returns those
/// that are referenced by a face passing the back-face culling test. The
/// returned edges are deduplicated (an edge shared by two visible faces
/// appears only once) so the wireframe pass strokes each silhouette edge
/// exactly once, avoiding the doubled strokes that would otherwise appear
/// as "extra lines" at the inner corner of a cube's visible silhouette.
///
/// # Arguments
///
/// - `&[Vector3D]` - The cube's 8 world-space vertex positions.
/// - `&Camera3D` - The camera used for back-face culling.
///
/// # Returns
///
/// - `Vec<(usize, usize)>` - The list of unique visible edge index pairs.
fn collect_visible_edges(world_vertices: &[Vector3D], camera: &Camera3D) -> Vec<(usize, usize)> {
    let mut visible_face_edges: std::collections::HashSet<(usize, usize)> =
        std::collections::HashSet::new();
    for (i0, i1, i2, i3) in GAME_3D_CUBE_FACES {
        let face_world: Vec<Vector3D> = vec![
            world_vertices[i0],
            world_vertices[i1],
            world_vertices[i2],
            world_vertices[i3],
        ];
        if !is_face_visible(&face_world, camera) {
            continue;
        }
        let mut add = |a: usize, b: usize| {
            let key: (usize, usize) = if a < b { (a, b) } else { (b, a) };
            visible_face_edges.insert(key);
        };
        add(i0, i1);
        add(i1, i2);
        add(i2, i3);
        add(i3, i0);
    }
    GAME_3D_CUBE_EDGES
        .iter()
        .copied()
        .filter(|(a, b): &(usize, usize)| {
            let key: (usize, usize) = if a < b { (*a, *b) } else { (*b, *a) };
            visible_face_edges.contains(&key)
        })
        .collect()
}

/// Performs one physics update step on all cubes.
///
/// Integrates angular velocity into quaternion rotation for each cube.
///
/// # Arguments
///
/// - `&mut [Cube3D]` - The mutable cube slice.
/// - `f64` - The delta time in seconds.
pub(crate) fn update_cubes(cubes: &mut [Cube3D], delta_time: f64) {
    for cube in cubes.iter_mut() {
        let rotation_delta: Quaternion = Quaternion::new(
            cube.angular_velocity.get_x() * delta_time * 0.5,
            cube.angular_velocity.get_y() * delta_time * 0.5,
            cube.angular_velocity.get_z() * delta_time * 0.5,
            1.0,
        );
        cube.rotation = (rotation_delta * cube.rotation).normalized();
    }
}

/// Queries the canvas element and creates an `SsaaCanvas` for high-quality rendering.
///
/// # Returns
///
/// - `Option<SsaaCanvas>` - The SSAA canvas, or `None` if unavailable.
pub(crate) fn acquire_game_3d_ssaa_canvas() -> Option<SsaaCanvas> {
    let window_value: Window = window().expect("no global window exists");
    let is_mobile: bool = window_value
        .inner_width()
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .is_some_and(|width: f64| width < 768.0);
    let scale_factor: f64 = if is_mobile { 1.0 } else { 2.0 };
    SsaaCanvas::from_selector_with_scale(
        GAME_3D_CANVAS_SELECTOR,
        GAME_3D_CANVAS_WIDTH,
        GAME_3D_CANVAS_HEIGHT,
        scale_factor,
    )
}

/// Registers non-passive event listeners directly on the 3D game canvas
/// element to prevent the page from scrolling when the mouse wheel or touch
/// gesture is used over the canvas.
///
/// The framework's event delegation system registers bubbling events on
/// `window` with the capture phase, which Chrome treats as passive by
/// default for `wheel`, `touchstart`, and `touchmove` events, making
/// `preventDefault()` ineffective. This function bypasses the framework and
/// attaches listeners directly on the element, where `preventDefault()`
/// works correctly. On desktop this prevents wheel scrolling; on mobile this
/// prevents touch scrolling as a belt-and-suspenders complement to the
/// `touch-action: none` CSS property.
///
/// # Returns
///
/// - `Option<CanvasGuardEntry>` - The listener closures and element for cleanup, or `None` if the canvas was not found.
pub(crate) fn register_canvas_scroll_guard() -> Option<CanvasGuardEntry> {
    let window: Window = window().expect("no global window exists");
    let document: Document = window.document().expect("should have a document");
    let canvas: Element = document
        .query_selector(GAME_3D_CANVAS_SELECTOR)
        .ok()
        .flatten()?;
    let wheel_closure: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
        event.prevent_default();
    }));
    let _ = canvas.add_event_listener_with_callback(
        GAME_3D_EVENT_WHEEL,
        wheel_closure.as_ref().unchecked_ref(),
    );
    let touch_start_closure: Closure<dyn FnMut(Event)> =
        Closure::wrap(Box::new(move |event: Event| {
            event.prevent_default();
        }));
    let _ = canvas.add_event_listener_with_callback(
        GAME_3D_EVENT_TOUCH_START,
        touch_start_closure.as_ref().unchecked_ref(),
    );
    let touch_move_closure: Closure<dyn FnMut(Event)> =
        Closure::wrap(Box::new(move |event: Event| {
            event.prevent_default();
        }));
    let _ = canvas.add_event_listener_with_callback(
        GAME_3D_EVENT_TOUCH_MOVE,
        touch_move_closure.as_ref().unchecked_ref(),
    );
    Some((
        vec![
            (wheel_closure, GAME_3D_EVENT_WHEEL),
            (touch_start_closure, GAME_3D_EVENT_TOUCH_START),
            (touch_move_closure, GAME_3D_EVENT_TOUCH_MOVE),
        ],
        canvas,
    ))
}

/// Draws the loading text centered on the 3D game canvas using SSAA.
///
/// Called during the startup delay before the game loop begins, so the
/// canvas shows a loading message instead of being blank. Uses an
/// `SsaaCanvas` with a 2x scale factor on desktop and 1x on mobile for
/// crisp text rendering.
pub(crate) fn draw_game_3d_loading() {
    let window_value: Window = window().expect("no global window exists");
    let is_mobile: bool = window_value
        .inner_width()
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .is_some_and(|width: f64| width < 768.0);
    let scale_factor: f64 = if is_mobile { 1.0 } else { 2.0 };
    let Some(ssaa_canvas) = SsaaCanvas::from_selector_with_scale(
        GAME_3D_CANVAS_SELECTOR,
        GAME_3D_CANVAS_WIDTH,
        GAME_3D_CANVAS_HEIGHT,
        scale_factor,
    ) else {
        return;
    };
    let context: &CanvasRenderingContext2d = ssaa_canvas.get_offscreen_context();
    context.clear_rect(0.0, 0.0, GAME_3D_CANVAS_WIDTH, GAME_3D_CANVAS_HEIGHT);
    let font_size: f64 = GAME_3D_CANVAS_HEIGHT * GAME_3D_LOADING_FONT_SIZE_RATIO;
    let font: String = format!("{font_size}px {GAME_3D_LOADING_FONT_FAMILY}");
    // Read the loading text color from the CSS variable via getComputedStyle.
    // Query the canvas element itself so the theme variable (defined on a
    // parent container, not on the document root) is inherited correctly.
    let loading_color: String = window_value
        .document()
        .expect("should have a document")
        .query_selector(GAME_3D_CANVAS_SELECTOR)
        .ok()
        .flatten()
        .and_then(|element: Element| {
            window_value
                .get_computed_style(&element)
                .ok()
                .flatten()
                .and_then(|style| style.get_property_value(GAME_3D_LOADING_COLOR_VAR).ok())
        })
        .unwrap_or_else(|| "#ffffff".to_string());
    let fill_style_key: JsValue = JsValue::from_str(GAME_3D_PROPERTY_FILL_STYLE);
    let _ = Reflect::set(context, &fill_style_key, &JsValue::from_str(&loading_color));
    context.set_font(&font);
    context.set_text_align("center");
    context.set_text_baseline("middle");
    let _ = context.fill_text(
        GAME_3D_LOADING_TEXT,
        GAME_3D_CANVAS_WIDTH * 0.5,
        GAME_3D_CANVAS_HEIGHT * 0.5,
    );
    ssaa_canvas.present();
}

/// Starts the 3D game loop driven by `requestAnimationFrame`.
///
/// Runs a fixed-timestep accumulator loop that updates cube rotation at a
/// constant rate and renders every frame. The canvas context is cached
/// once at startup. Updates the FPS signal approximately every second.
///
/// # Arguments
///
/// - `UseGame3D` - The game state for signal updates.
/// - `Rc<RefCell<Vec<Cube3D>>>` - The shared cube list.
/// - `CameraAngles` - The non-reactive camera orbit angles.
pub(crate) fn start_game_3d_loop(
    state: UseGame3D,
    cubes: Rc<RefCell<Vec<Cube3D>>>,
    angles: CameraAngles,
) {
    let canvas_ssaa: Rc<RefCell<Option<SsaaCanvas>>> = Rc::new(RefCell::new(None));
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let accumulator: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
    let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(RefCell::new(None));
    let guard_cell: CanvasGuardCell = Rc::new(RefCell::new(None));
    let state_clone: UseGame3D = state;
    let cubes_clone: Rc<RefCell<Vec<Cube3D>>> = cubes;
    let angles_clone: CameraAngles = angles;
    let acc_clone: Rc<Cell<f64>> = accumulator.clone();
    let last_clone: Rc<Cell<f64>> = last_time.clone();
    let frame_clone: Rc<Cell<u32>> = frame_count.clone();
    let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
    let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_clone: RafClosureCell = closure_cell.clone();
    let context_clone: Rc<RefCell<Option<SsaaCanvas>>> = canvas_ssaa.clone();
    let dirty_clone: Rc<Cell<bool>> = resize_dirty.clone();
    let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let window_value: Window = window().expect("no global window exists");
        let performance: Performance = window_value
            .performance()
            .expect("performance should exist");
        let current_time: f64 = performance.now() / 1000.0;
        let prev: f64 = last_clone.get();
        let frame_time: f64 = if prev < 0.0 {
            GAME_3D_FIXED_TIMESTEP
        } else {
            (current_time - prev).min(0.25)
        };
        last_clone.set(current_time);
        acc_clone.set(acc_clone.get() + frame_time);
        if state_clone.get_running().get() {
            if state_clone.get_auto_rotate().get() {
                let yaw: f64 = angles_clone.yaw.get() + GAME_3D_AUTO_YAW_SPEED * frame_time;
                angles_clone.yaw.set(yaw);
            }
            while acc_clone.get() >= GAME_3D_FIXED_TIMESTEP {
                update_cubes(&mut cubes_clone.borrow_mut(), GAME_3D_FIXED_TIMESTEP);
                acc_clone.set(acc_clone.get() - GAME_3D_FIXED_TIMESTEP);
            }
        }
        if dirty_clone.get() {
            *context_clone.borrow_mut() = None;
            dirty_clone.set(false);
        }
        if context_clone.borrow().is_none() {
            *context_clone.borrow_mut() = acquire_game_3d_ssaa_canvas();
        }
        if let Some(ssaa_canvas) = context_clone.borrow().as_ref() {
            let camera: Camera3D =
                create_orbit_camera(angles_clone.yaw.get(), angles_clone.pitch.get());
            render_scene(ssaa_canvas, &cubes_clone.borrow(), &camera);
        }
        frame_clone.set(frame_clone.get() + 1);
        fps_clone.set(fps_clone.get() + frame_time);
        if fps_clone.get() >= 1.0 {
            let fps: f64 = f64::from(frame_clone.get()) / fps_clone.get();
            state_clone.get_fps().set(fps);
            frame_clone.set(0);
            fps_clone.set(0.0);
        }
        let next_id: i32 = window_value
            .request_animation_frame(
                cell_clone
                    .borrow()
                    .as_ref()
                    .expect("raf closure should exist")
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap_or(0);
        raf_clone.set(Some(next_id));
    }));
    *closure_cell.borrow_mut() = Some(raf_closure);
    let start_timeout_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let start_timeout_clone: Rc<Cell<Option<i32>>> = start_timeout_id.clone();
    let raf_for_start: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_start: RafClosureCell = closure_cell.clone();
    let guard_for_start: CanvasGuardCell = guard_cell.clone();
    let state_for_start: UseGame3D = state;
    let start_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        state_for_start.get_loaded().set(true);
        *guard_for_start.borrow_mut() = register_canvas_scroll_guard();
        let start_window: Window = window().expect("no global window exists");
        let start_id: i32 = start_window
            .request_animation_frame(
                cell_for_start
                    .borrow()
                    .as_ref()
                    .expect("raf closure should exist")
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap_or(0);
        raf_for_start.set(Some(start_id));
    }));
    let start_callback: Function = start_closure.as_ref().unchecked_ref::<Function>().clone();
    start_closure.forget();
    let window_value: Window = window().expect("no global window exists");
    let timeout_id: i32 = window_value
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            &start_callback,
            GAME_3D_LOOP_START_DELAY_MILLIS,
        )
        .unwrap_or(0);
    start_timeout_clone.set(Some(timeout_id));
    let loading_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        draw_game_3d_loading();
    }));
    let loading_callback: Function = loading_closure.as_ref().unchecked_ref::<Function>().clone();
    loading_closure.forget();
    let _ =
        window_value.set_timeout_with_callback_and_timeout_and_arguments_0(&loading_callback, 0);
    let debounce_timer: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let dirty_for_event: Rc<Cell<bool>> = resize_dirty.clone();
    let timer_for_event: Rc<Cell<Option<i32>>> = debounce_timer.clone();
    let debounce_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        dirty_for_event.set(true);
    }));
    let debounce_callback: Function = debounce_closure
        .as_ref()
        .unchecked_ref::<Function>()
        .clone();
    debounce_closure.forget();
    let timeout_window: Window = window().expect("no global window exists");
    App::use_window_event("resize", move || {
        let old_timer: Option<i32> = timer_for_event.get();
        if let Some(timer_id) = old_timer {
            timeout_window.clear_timeout_with_handle(timer_id);
        }
        let new_timer: i32 = timeout_window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &debounce_callback,
                GAME_3D_RESIZE_DEBOUNCE_MILLIS,
            )
            .unwrap_or_default();
        timer_for_event.set(Some(new_timer));
    });
    let guard_for_cleanup: CanvasGuardCell = guard_cell.clone();
    App::use_cleanup(move || {
        if let Some(cancel_id) = raf_id.get() {
            let window_value: Window = window().expect("no global window exists");
            let _ = window_value.cancel_animation_frame(cancel_id);
        }
        if let Some(timeout_id) = start_timeout_id.get() {
            let window_value: Window = window().expect("no global window exists");
            window_value.clear_timeout_with_handle(timeout_id);
        }
        if let Some(timer_id) = debounce_timer.get() {
            let window_value: Window = window().expect("no global window exists");
            window_value.clear_timeout_with_handle(timer_id);
        }
        closure_cell.borrow_mut().take();
        if let Some((listeners, element)) = guard_for_cleanup.borrow_mut().take() {
            for (closure, event_name) in listeners {
                let _ = element.remove_event_listener_with_callback(
                    event_name,
                    closure.as_ref().unchecked_ref(),
                );
            }
        }
    });
}

/// Creates a click event handler that toggles the game between running and paused.
///
/// # Arguments
///
/// - `UseGame3D` - The game state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_3d_on_toggle_pause(state: UseGame3D) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_running().get();
        state.get_running().set(!current);
    }))
}

/// Creates a click event handler that toggles auto-rotation.
///
/// # Arguments
///
/// - `UseGame3D` - The game state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_3d_on_toggle_auto_rotate(state: UseGame3D) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_auto_rotate().get();
        state.get_auto_rotate().set(!current);
    }))
}

/// Creates a click event handler that resets the camera orbit angles.
///
/// # Arguments
///
/// - `CameraAngles` - The non-reactive camera orbit angles.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_3d_on_reset_camera(angles: CameraAngles) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        angles.yaw.set(0.3);
        angles.pitch.set(0.4);
    }))
}

/// Creates a pointer event handler that updates orbit angles based on drag movement.
///
/// # Arguments
///
/// - `CameraAngles` - The non-reactive camera orbit angles.
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A pointer move handler.
pub(crate) fn game_3d_on_pointer_move(
    angles: CameraAngles,
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let last: Option<(f64, f64)> = last_pointer.get();
        let Some((last_x, last_y)) = last else {
            return;
        };
        let client_x: f64 = Reflect::get(event.as_ref(), &JsValue::from_str("clientX"))
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .unwrap_or(0.0);
        let client_y: f64 = Reflect::get(event.as_ref(), &JsValue::from_str("clientY"))
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .unwrap_or(0.0);
        let dx: f64 = client_x - last_x;
        let dy: f64 = client_y - last_y;
        last_pointer.set(Some((client_x, client_y)));
        let yaw: f64 = angles.yaw.get() - dx * 0.01;
        let pitch: f64 = (angles.pitch.get() + dy * 0.01).clamp(
            -HALF_PI + GAME_3D_PITCH_CLAMP,
            HALF_PI - GAME_3D_PITCH_CLAMP,
        );
        angles.yaw.set(yaw);
        angles.pitch.set(pitch);
    }))
}

/// Creates a pointer event handler that records the pointer start position.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A pointer down handler.
pub(crate) fn game_3d_on_pointer_down(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let client_x: f64 = Reflect::get(event.as_ref(), &JsValue::from_str("clientX"))
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .unwrap_or(0.0);
        let client_y: f64 = Reflect::get(event.as_ref(), &JsValue::from_str("clientY"))
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .unwrap_or(0.0);
        last_pointer.set(Some((client_x, client_y)));
    }))
}

/// Creates a pointer event handler that clears the pointer position.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A pointer up handler.
pub(crate) fn game_3d_on_pointer_up(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        last_pointer.set(None);
    }))
}

/// Extracts the client coordinates of the first active touch from a `TouchEvent`.
///
/// Reads `touches[0].clientX` and `touches[0].clientY` from the event via
/// JavaScript reflection. Used by touch-specific camera drag handlers since
/// `TouchEvent` does not expose `clientX`/`clientY` directly on the event object.
///
/// # Arguments
///
/// - `&Event` - The native touch event.
///
/// # Returns
///
/// - `(f64, f64)` - The `(client_x, client_y)` coordinates of the first touch.
pub(crate) fn extract_first_touch_client(event: &Event) -> (f64, f64) {
    let touches_value: JsValue = Reflect::get(
        event.as_ref(),
        &JsValue::from_str(GAME_3D_EVENT_PROPERTY_TOUCHES),
    )
    .ok()
    .unwrap_or(JsValue::NULL);
    let touches: Array = touches_value.unchecked_into();
    if touches.length() == 0 {
        return (0.0, 0.0);
    }
    let touch: JsValue = touches.get(0);
    let client_x: f64 = Reflect::get(&touch, &JsValue::from_str(GAME_3D_EVENT_PROPERTY_CLIENT_X))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or(0.0);
    let client_y: f64 = Reflect::get(&touch, &JsValue::from_str(GAME_3D_EVENT_PROPERTY_CLIENT_Y))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or(0.0);
    (client_x, client_y)
}

/// Creates a touch event handler that records the first touch start position and
/// prevents default browser behavior to avoid page scrolling during camera drag.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A touch start handler.
pub(crate) fn game_3d_on_touch_start(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if event.cancelable() {
            event.prevent_default();
        }
        let (client_x, client_y): (f64, f64) = extract_first_touch_client(&event);
        last_pointer.set(Some((client_x, client_y)));
    }))
}

/// Creates a touch event handler that updates orbit angles based on single-finger
/// drag movement and prevents default browser behavior.
///
/// # Arguments
///
/// - `CameraAngles` - The non-reactive camera orbit angles.
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A touch move handler.
pub(crate) fn game_3d_on_touch_move(
    angles: CameraAngles,
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if event.cancelable() {
            event.prevent_default();
        }
        let last: Option<(f64, f64)> = last_pointer.get();
        let Some((last_x, last_y)) = last else {
            return;
        };
        let (client_x, client_y): (f64, f64) = extract_first_touch_client(&event);
        let dx: f64 = client_x - last_x;
        let dy: f64 = client_y - last_y;
        last_pointer.set(Some((client_x, client_y)));
        let yaw: f64 = angles.yaw.get() - dx * 0.01;
        let pitch: f64 = (angles.pitch.get() + dy * 0.01).clamp(
            -HALF_PI + GAME_3D_PITCH_CLAMP,
            HALF_PI - GAME_3D_PITCH_CLAMP,
        );
        angles.yaw.set(yaw);
        angles.pitch.set(pitch);
    }))
}

/// Creates a touch event handler that clears the pointer position and prevents
/// default browser behavior.
///
/// # Arguments
///
/// - `Rc<Cell<Option<(f64, f64)>>>` - The shared last pointer position cell.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A touch end handler.
pub(crate) fn game_3d_on_touch_end(
    last_pointer: Rc<Cell<Option<(f64, f64)>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if event.cancelable() {
            event.prevent_default();
        }
        last_pointer.set(None);
    }))
}

/// Creates the reactive state signals for the 3D WebGPU demo.
///
/// # Returns
///
/// - `UseGame3DWebGpu` - The WebGPU demo state.
pub(crate) fn use_game_3d_webgpu_state() -> UseGame3DWebGpu {
    UseGame3DWebGpu {
        fps: App::use_signal(|| 0.0),
        loaded: App::use_signal(|| false),
        active: App::use_signal(|| false),
        loop_started: App::use_signal(|| false),
    }
}

/// Creates a click event handler that selects a tab on the 3D game page.
///
/// # Arguments
///
/// - `Signal<Game3DTab>` - The tab signal to update.
/// - `Game3DTab` - The tab variant to set.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets the active tab.
pub(crate) fn game_3d_on_tab_select(
    tab: Signal<Game3DTab>,
    value: Game3DTab,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        tab.set(value);
    }))
}

/// Starts the 3D WebGPU render loop driven by `requestAnimationFrame`.
///
/// Asynchronously initializes a `WebGpuRenderer`, creates a render pipeline
/// from a WGSL shader with pseudo-3D perspective, and runs a
/// `requestAnimationFrame` loop that renders the triangle with an animated
/// clear color each frame.
///
/// # Arguments
///
/// - `UseGame3DWebGpu` - The WebGPU demo state for signal updates.
pub(crate) fn start_game_3d_webgpu_loop(state: UseGame3DWebGpu) {
    let init_state: UseGame3DWebGpu = state;
    let loop_state: UseGame3DWebGpu = state;
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(RefCell::new(None));
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let resize_timer: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let renderer_rc: Rc<RefCell<Option<WebGpuRenderer>>> = Rc::new(RefCell::new(None));
    let cancelled: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let resize_dirty_for_event: Rc<Cell<bool>> = resize_dirty.clone();
    let resize_timer_for_event: Rc<Cell<Option<i32>>> = resize_timer.clone();
    let debounce_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        resize_dirty_for_event.set(true);
    }));
    let debounce_callback: Function = debounce_closure
        .as_ref()
        .unchecked_ref::<Function>()
        .clone();
    debounce_closure.forget();
    let resize_window: Window = window().expect("no global window exists");
    App::use_window_event("resize", move || {
        let old_timer: Option<i32> = resize_timer_for_event.get();
        if let Some(timer_id) = old_timer {
            let clear_window: Window = window().expect("no global window exists");
            clear_window.clear_timeout_with_handle(timer_id);
        }
        let new_timer: i32 = resize_window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &debounce_callback,
                GAME_3D_RESIZE_DEBOUNCE_MILLIS,
            )
            .unwrap_or_default();
        resize_timer_for_event.set(Some(new_timer));
    });
    let raf_for_cleanup: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_cleanup: RafClosureCell = closure_cell.clone();
    let renderer_for_cleanup: Rc<RefCell<Option<WebGpuRenderer>>> = renderer_rc.clone();
    let resize_timer_for_cleanup: Rc<Cell<Option<i32>>> = resize_timer.clone();
    let cancelled_for_cleanup: Rc<Cell<bool>> = cancelled.clone();
    App::use_cleanup(move || {
        cancelled_for_cleanup.set(true);
        if let Some(cancel_id) = raf_for_cleanup.get() {
            let window_value: Window = window().expect("no global window exists");
            let _ = window_value.cancel_animation_frame(cancel_id);
        }
        if let Some(timer_id) = resize_timer_for_cleanup.get() {
            let window_value: Window = window().expect("no global window exists");
            window_value.clear_timeout_with_handle(timer_id);
        }
        cell_for_cleanup.borrow_mut().take();
        *renderer_for_cleanup.borrow_mut() = None;
    });
    let cancelled_for_init: Rc<Cell<bool>> = cancelled.clone();
    spawn_local(async move {
        let config: RenderConfig = RenderConfig::webgpu(
            GAME_3D_WEBGPU_CANVAS_SELECTOR,
            GAME_3D_CANVAS_WIDTH,
            GAME_3D_CANVAS_HEIGHT,
        );
        let renderer: Result<WebGpuRenderer, WebGpuInitError> =
            Engine::webgpu_renderer(&config).await;
        if cancelled_for_init.get() {
            return;
        }
        let renderer: WebGpuRenderer = match renderer {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!("[euv-engine][game_3d] webgpu init failed: {error}"));
                init_state.get_loaded().set(true);
                return;
            }
        };
        let pipeline: JsValue = renderer.create_render_pipeline(GAME_3D_WEBGPU_SHADER);
        init_state.get_active().set(true);
        init_state.get_loaded().set(true);
        *renderer_rc.borrow_mut() = Some(renderer);
        let pipeline_rc: Rc<JsValue> = Rc::new(pipeline);
        let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
        let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
        let renderer_for_loop: Rc<RefCell<Option<WebGpuRenderer>>> = renderer_rc.clone();
        let pipeline_for_loop: Rc<JsValue> = pipeline_rc.clone();
        let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_clone: RafClosureCell = closure_cell.clone();
        let last_clone: Rc<Cell<f64>> = last_time.clone();
        let frame_clone: Rc<Cell<u32>> = frame_count.clone();
        let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
        let resize_dirty_for_loop: Rc<Cell<bool>> = resize_dirty.clone();
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let window_value: Window = window().expect("no global window exists");
            let performance: Performance = window_value
                .performance()
                .expect("performance should exist");
            let current_time: f64 = performance.now() / 1000.0;
            let prev: f64 = last_clone.get();
            let frame_time: f64 = if prev < 0.0 {
                GAME_3D_FIXED_TIMESTEP
            } else {
                (current_time - prev).min(0.25)
            };
            last_clone.set(current_time);
            if resize_dirty_for_loop.get() {
                resize_dirty_for_loop.set(false);
                let window_for_dpr: Window = window().expect("no global window exists");
                let dpr: f64 = Reflect::get(
                    window_for_dpr.as_ref(),
                    &JsValue::from_str("devicePixelRatio"),
                )
                .ok()
                .and_then(|value: JsValue| value.as_f64())
                .filter(|value: &f64| value.is_finite() && *value >= 1.0)
                .unwrap_or(1.0);
                let new_physical_width: u32 = (GAME_3D_CANVAS_WIDTH * dpr).round() as u32;
                let new_physical_height: u32 = (GAME_3D_CANVAS_HEIGHT * dpr).round() as u32;
                if let Some(renderer) = renderer_for_loop.borrow_mut().as_mut() {
                    let _ = renderer.resize(new_physical_width, new_physical_height);
                }
            }
            if let Some(renderer) = renderer_for_loop.borrow().as_ref() {
                let t: f64 = current_time;
                let r: f64 = (t * 0.3 + 1.0).sin() * 0.3 + 0.1;
                let g: f64 = (t * 0.5 + 3.0).sin() * 0.3 + 0.1;
                let b: f64 = (t * 0.8).sin() * 0.3 + 0.1;
                renderer.render_frame(&pipeline_for_loop, (r, g, b, 1.0), 3);
            }
            frame_clone.set(frame_clone.get() + 1);
            fps_clone.set(fps_clone.get() + frame_time);
            if fps_clone.get() >= 1.0 {
                let fps: f64 = f64::from(frame_clone.get()) / fps_clone.get();
                loop_state.get_fps().set(fps);
                frame_clone.set(0);
                fps_clone.set(0.0);
            }
            let next_id: i32 = window_value
                .request_animation_frame(
                    cell_clone
                        .borrow()
                        .as_ref()
                        .expect("raf closure should exist")
                        .as_ref()
                        .unchecked_ref(),
                )
                .unwrap_or(0);
            raf_clone.set(Some(next_id));
        }));
        *closure_cell.borrow_mut() = Some(raf_closure);
        let start_window: Window = window().expect("no global window exists");
        let start_id: i32 = start_window
            .request_animation_frame(
                closure_cell
                    .borrow()
                    .as_ref()
                    .expect("raf closure should exist")
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap_or(0);
        raf_id.set(Some(start_id));
    });
}
