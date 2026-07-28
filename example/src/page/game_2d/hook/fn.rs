use super::*;

/// Creates the 2D bouncing balls game reactive state signals wrapped in a `UseGame2D` struct.
///
/// # Returns
///
/// - `UseGame2D` - The 2D game state.
pub(crate) fn use_game_2d_state() -> UseGame2D {
    UseGame2D {
        running: App::use_signal(|| true),
        fps: App::use_signal(|| 0.0),
        ball_count: App::use_signal(|| 0),
        total_spawned: App::use_signal(|| 0),
        loaded: App::use_signal(|| false),
    }
}

/// Returns a random ball color from the predefined palette.
///
/// # Returns
///
/// - `&'static str` - A CSS color string.
pub(crate) fn random_ball_color() -> &'static str {
    let index: usize = (js_sys::Math::random() * GAME_2D_BALL_COLORS.len() as f64) as usize;
    GAME_2D_BALL_COLORS[index % GAME_2D_BALL_COLORS.len()]
}

/// Returns a random ball radius within the allowed range.
///
/// # Returns
///
/// - `f64` - The radius in pixels.
pub(crate) fn random_ball_radius() -> f64 {
    let raw: f64 = js_sys::Math::random();
    GAME_2D_BALL_MIN_RADIUS + raw * (GAME_2D_BALL_MAX_RADIUS - GAME_2D_BALL_MIN_RADIUS)
}

/// Creates a new ball at the given position with a random upward velocity.
///
/// # Arguments
///
/// - `Vector2D` - The spawn position.
///
/// # Returns
///
/// - `Ball` - The newly created ball.
pub(crate) fn create_ball(position: Vector2D) -> Ball {
    let angle: f64 = js_sys::Math::random() * PI - PI * 0.5;
    let speed: f64 = GAME_2D_SPAWN_VELOCITY + js_sys::Math::random() * GAME_2D_SPAWN_VELOCITY;
    Ball {
        position,
        velocity: Vector2D::new(angle.cos() * speed, -angle.sin() * speed.abs()),
        radius: random_ball_radius(),
        color: random_ball_color().to_string(),
    }
}

/// Creates a click event handler that spawns a new ball at the click position.
///
/// # Arguments
///
/// - `UseGame2D` - The 2D game state.
/// - `Rc<RefCell<Vec<Ball>>>` - The shared ball list.
/// - `CanvasCache` - The cached canvas element reference.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_2d_on_spawn_ball(
    state: UseGame2D,
    balls: Rc<RefCell<Vec<Ball>>>,
    canvas_cache: CanvasCache,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let current_count: usize = state.get_ball_count().get();
        if current_count >= GAME_2D_MAX_BALLS {
            return;
        }
        let (client_x, client_y): (f64, f64) = extract_mouse_client_position(&event);
        let Some(canvas_element) = canvas_cache.0.borrow().as_ref().cloned() else {
            return;
        };
        let rect: DomRect = canvas_element.get_bounding_client_rect();
        let position: Vector2D = map_client_to_canvas(client_x, client_y, &rect);
        let ball: Ball = create_ball(position);
        balls.borrow_mut().push(ball);
        let new_count: usize = balls.borrow().len();
        state.get_ball_count().set(new_count);
        let total: usize = state.get_total_spawned().get();
        state.get_total_spawned().set(total + 1);
    }))
}

/// Creates a click event handler that toggles the 2D game between running and paused.
///
/// # Arguments
///
/// - `UseGame2D` - The 2D game state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_2d_on_toggle_pause(state: UseGame2D) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_running().get();
        state.get_running().set(!current);
    }))
}

/// Creates a click event handler that clears all balls from the canvas.
///
/// # Arguments
///
/// - `UseGame2D` - The 2D game state.
/// - `Rc<RefCell<Vec<Ball>>>` - The shared ball list.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler.
pub(crate) fn game_2d_on_clear(
    state: UseGame2D,
    balls: Rc<RefCell<Vec<Ball>>>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        balls.borrow_mut().clear();
        state.get_ball_count().set(0);
    }))
}

/// Extracts the client (viewport) coordinates from a mouse event.
///
/// # Arguments
///
/// - `&Event` - The mouse event.
///
/// # Returns
///
/// - `(f64, f64)` - The `(client_x, client_y)` coordinates.
pub(crate) fn extract_mouse_client_position(event: &Event) -> (f64, f64) {
    let mouse_event: &MouseEvent = event.unchecked_ref();
    (
        f64::from(mouse_event.client_x()),
        f64::from(mouse_event.client_y()),
    )
}

/// Extracts the client coordinates of the first changed touch from a `TouchEvent`.
///
/// Reads `changedTouches[0].clientX` and `changedTouches[0].clientY` from the
/// event via direct cast. Used by the touch spawn handler since
/// `TouchEvent` does not expose `clientX`/`clientY` directly on the event object.
///
/// # Arguments
///
/// - `&Event` - The native touch event.
///
/// # Returns
///
/// - `(f64, f64)` - The `(client_x, client_y)` coordinates of the first changed touch.
pub(crate) fn extract_touch_client_position(event: &Event) -> (f64, f64) {
    let touch_event: &TouchEvent = event.unchecked_ref();
    let touches: TouchList = touch_event.changed_touches();
    if touches.length() == 0 {
        return (0.0, 0.0);
    }
    let touch: Option<Touch> = touches.get(0);
    let Some(touch) = touch else {
        return (0.0, 0.0);
    };
    (f64::from(touch.client_x()), f64::from(touch.client_y()))
}

/// Creates a touch event handler that spawns a new ball at the touch position
/// and prevents default browser behavior to avoid click delay and page scrolling.
///
/// # Arguments
///
/// - `UseGame2D` - The 2D game state.
/// - `Rc<RefCell<Vec<Ball>>>` - The shared ball list.
/// - `CanvasCache` - The cached canvas element reference.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A touch start handler.
pub(crate) fn game_2d_on_touch_spawn_ball(
    state: UseGame2D,
    balls: Rc<RefCell<Vec<Ball>>>,
    canvas_cache: CanvasCache,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if event.cancelable() {
            event.prevent_default();
        }
        let current_count: usize = state.get_ball_count().get();
        if current_count >= GAME_2D_MAX_BALLS {
            return;
        }
        let (client_x, client_y): (f64, f64) = extract_touch_client_position(&event);
        let Some(canvas_element) = canvas_cache.0.borrow().as_ref().cloned() else {
            return;
        };
        let rect: DomRect = canvas_element.get_bounding_client_rect();
        let position: Vector2D = map_client_to_canvas(client_x, client_y, &rect);
        let ball: Ball = create_ball(position);
        balls.borrow_mut().push(ball);
        let new_count: usize = balls.borrow().len();
        state.get_ball_count().set(new_count);
        let total: usize = state.get_total_spawned().get();
        state.get_total_spawned().set(total + 1);
    }))
}

/// Maps viewport client coordinates to canvas-internal coordinates.
///
/// # Arguments
///
/// - `f64` - The client x coordinate.
/// - `f64` - The client y coordinate.
/// - `&DomRect` - The cached canvas bounding rect.
///
/// # Returns
///
/// - `Vector2D` - The canvas-space position.
pub(crate) fn map_client_to_canvas(
    client_x: f64,
    client_y: f64,
    canvas_rect: &DomRect,
) -> Vector2D {
    let rect_width: f64 = canvas_rect.width();
    let rect_height: f64 = canvas_rect.height();
    if rect_width < EPSILON || rect_height < EPSILON {
        return Vector2D::zero();
    }
    let scale_x: f64 = GAME_2D_CANVAS_WIDTH / rect_width;
    let scale_y: f64 = GAME_2D_CANVAS_HEIGHT / rect_height;
    Vector2D::new(
        (client_x - canvas_rect.left()) * scale_x,
        (client_y - canvas_rect.top()) * scale_y,
    )
}

/// Performs one physics update step on all balls.
///
/// Subdivides `delta_time` into `GAME_2D_PHYSICS_SUBSTEPS` smaller slices,
/// applying gravity, integrating velocity and position, handling wall
/// collisions with restitution, and resolving ball-to-ball collisions with
/// impulse-based response in each substep. The ball-to-ball pass is itself
/// repeated `GAME_2D_COLLISION_ITERATIONS` times per substep to converge on a
/// non-overlapping configuration when many balls are in contact.
///
/// # Arguments
///
/// - `&mut [Ball]` - The mutable ball slice.
/// - `f64` - The delta time in seconds.
pub(crate) fn update_balls(balls: &mut [Ball], delta_time: f64) {
    let sub_dt: f64 = delta_time / GAME_2D_PHYSICS_SUBSTEPS as f64;
    let gravity: Vector2D = Vector2D::new(0.0, GAME_2D_GRAVITY);
    for _ in 0..GAME_2D_PHYSICS_SUBSTEPS {
        let damping: f64 = (1.0 - GAME_2D_LINEAR_DAMPING * sub_dt).max(0.0);
        for ball in balls.iter_mut() {
            ball.velocity += gravity.scaled(sub_dt);
            ball.velocity = ball.velocity.scaled(damping);
            ball.position += ball.velocity.scaled(sub_dt);
        }
        for ball in balls.iter_mut() {
            resolve_wall_collision(ball);
        }
        for _ in 0..GAME_2D_COLLISION_ITERATIONS {
            let count: usize = balls.len();
            for i in 0..count {
                for j in (i + 1)..count {
                    let (left, right) = balls.split_at_mut(j);
                    resolve_ball_collision(&mut left[i], &mut right[0]);
                }
            }
        }
    }
}

/// Resolves a collision between a ball and the canvas walls.
///
/// Reflects velocity with restitution and clamps position inside bounds.
///
/// # Arguments
///
/// - `&mut Ball` - The ball to check and correct.
pub(crate) fn resolve_wall_collision(ball: &mut Ball) {
    if ball.position.get_x() - ball.radius < 0.0 {
        ball.position.set_x(ball.radius);
        let velocity_x: f64 = ball.velocity.get_x();
        ball.velocity.set_x(velocity_x.abs() * GAME_2D_RESTITUTION);
    }
    if ball.position.get_x() + ball.radius > GAME_2D_CANVAS_WIDTH {
        ball.position.set_x(GAME_2D_CANVAS_WIDTH - ball.radius);
        let velocity_x: f64 = ball.velocity.get_x();
        ball.velocity.set_x(-velocity_x.abs() * GAME_2D_RESTITUTION);
    }
    if ball.position.get_y() - ball.radius < 0.0 {
        ball.position.set_y(ball.radius);
        let velocity_y: f64 = ball.velocity.get_y();
        ball.velocity.set_y(velocity_y.abs() * GAME_2D_RESTITUTION);
    }
    if ball.position.get_y() + ball.radius > GAME_2D_CANVAS_HEIGHT {
        ball.position.set_y(GAME_2D_CANVAS_HEIGHT - ball.radius);
        let velocity_y: f64 = ball.velocity.get_y();
        ball.velocity.set_y(-velocity_y.abs() * GAME_2D_RESTITUTION);
    }
}

/// Resolves a collision between two balls using impulse-based response.
///
/// Separates overlapping balls along the contact normal and applies velocity
/// changes based on their masses.
///
/// # Arguments
///
/// - `&mut Ball` - The first ball.
/// - `&mut Ball` - The second ball.
pub(crate) fn resolve_ball_collision(a: &mut Ball, b: &mut Ball) {
    let delta: Vector2D = b.position - a.position;
    let distance: f64 = delta.magnitude();
    let radius_sum: f64 = a.radius + b.radius;
    if distance >= radius_sum {
        return;
    }
    let normal: Vector2D = if distance < EPSILON {
        Vector2D::right()
    } else {
        delta.scaled(1.0 / distance)
    };
    let overlap: f64 = radius_sum - distance;
    let mass_a: f64 = a.radius * a.radius;
    let mass_b: f64 = b.radius * b.radius;
    let total_mass: f64 = mass_a + mass_b;
    a.position -= normal.scaled(overlap * (mass_b / total_mass));
    b.position += normal.scaled(overlap * (mass_a / total_mass));
    let relative_velocity: Vector2D = b.velocity - a.velocity;
    let velocity_along_normal: f64 = relative_velocity.dot(normal);
    if velocity_along_normal > 0.0 {
        return;
    }
    let impulse_magnitude: f64 =
        -(1.0 + GAME_2D_RESTITUTION) * velocity_along_normal / (1.0 / mass_a + 1.0 / mass_b);
    let impulse: Vector2D = normal.scaled(impulse_magnitude);
    a.velocity -= impulse.scaled(1.0 / mass_a);
    b.velocity += impulse.scaled(1.0 / mass_b);
}

/// Renders all balls onto the supplied SSAA canvas and presents the result.
///
/// Draws onto the offscreen context using logical CSS-pixel coordinates,
/// then delegates to `present()` for HiDPI-friendly downscaling. The canvas
/// backing store is sized to `devicePixelRatio * scale_factor` automatically
/// by `SsaaCanvas::from_selector_with_scale`.
///
/// # Arguments
///
/// - `&SsaaCanvas` - The SSAA canvas wrapper.
/// - `&[Ball]` - The ball list to render.
pub(crate) fn render_balls_with_ssaa(ssaa_canvas: &SsaaCanvas, balls: &[Ball]) {
    let context: &CanvasRenderingContext2d = ssaa_canvas.get_offscreen_context();
    context.clear_rect(0.0, 0.0, GAME_2D_CANVAS_WIDTH, GAME_2D_CANVAS_HEIGHT);
    let fill_style_key: JsValue = JsValue::from_str(GAME_2D_PROPERTY_FILL_STYLE);
    for ball in balls {
        let _ = Reflect::set(context, &fill_style_key, &JsValue::from_str(&ball.color));
        context.begin_path();
        let _ = context.arc(
            ball.position.get_x(),
            ball.position.get_y(),
            ball.radius,
            0.0,
            std::f64::consts::TAU,
        );
        context.fill();
    }
    ssaa_canvas.present();
}

/// Queries the 2D game canvas element and constructs an SSAA wrapper for it.
///
/// Picks the SSAA scale factor via the same desktop/mobile heuristic used
/// for the 3D game (2x on desktop, 1x on mobile). The DPR multiplier is
/// applied automatically inside `SsaaCanvas::from_selector_with_scale`.
///
/// Returns the underlying display element alongside the SSAA wrapper so
/// that click handlers can map viewport coordinates into canvas space.
///
/// # Returns
///
/// - `Option<(HtmlCanvasElement, SsaaCanvas)>` - The display canvas plus
///   the SSAA wrapper, or `None` if the canvas element was not found.
pub(crate) fn acquire_game_2d_ssaa_canvas() -> Option<(HtmlCanvasElement, SsaaCanvas)> {
    let window_value: Window = window().expect("no global window exists");
    let is_mobile: bool = window_value
        .inner_width()
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .is_some_and(|width: f64| width < 768.0);
    let scale_factor: f64 = if is_mobile { 1.0 } else { 2.0 };
    let ssaa_canvas: SsaaCanvas = SsaaCanvas::from_selector_with_scale(
        GAME_2D_CANVAS_SELECTOR,
        GAME_2D_CANVAS_WIDTH,
        GAME_2D_CANVAS_HEIGHT,
        scale_factor,
    )?;
    let document_value: Document = window_value.document().expect("should have a document");
    let element: Element = document_value
        .query_selector(GAME_2D_CANVAS_SELECTOR)
        .ok()
        .flatten()?;
    let display_canvas: HtmlCanvasElement = element.unchecked_into();
    Some((display_canvas, ssaa_canvas))
}

/// Draws the loading text centered on the 2D game canvas using SSAA.
///
/// Called during the startup delay before the game loop begins, so the
/// canvas shows a loading message instead of being blank. Uses an
/// `SsaaCanvas` with a 2x scale factor on desktop and 1x on mobile for
/// crisp text rendering.
pub(crate) fn draw_game_2d_loading() {
    let window_value: Window = window().expect("no global window exists");
    let is_mobile: bool = window_value
        .inner_width()
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .is_some_and(|width: f64| width < 768.0);
    let scale_factor: f64 = if is_mobile { 1.0 } else { 2.0 };
    let Some(ssaa_canvas) = SsaaCanvas::from_selector_with_scale(
        GAME_2D_CANVAS_SELECTOR,
        GAME_2D_CANVAS_WIDTH,
        GAME_2D_CANVAS_HEIGHT,
        scale_factor,
    ) else {
        return;
    };
    let context: &CanvasRenderingContext2d = ssaa_canvas.get_offscreen_context();
    context.clear_rect(0.0, 0.0, GAME_2D_CANVAS_WIDTH, GAME_2D_CANVAS_HEIGHT);
    let font_size: f64 = GAME_2D_CANVAS_HEIGHT * GAME_2D_LOADING_FONT_SIZE_RATIO;
    let font: String = format!("{font_size}px {GAME_2D_LOADING_FONT_FAMILY}");
    // Read the loading text color from the CSS variable via getComputedStyle.
    // Query the canvas element itself so the theme variable (defined on a
    // parent container, not on the document root) is inherited correctly.
    let loading_color: String = window_value
        .document()
        .expect("should have a document")
        .query_selector(GAME_2D_CANVAS_SELECTOR)
        .ok()
        .flatten()
        .and_then(|element: Element| {
            window_value
                .get_computed_style(&element)
                .ok()
                .flatten()
                .and_then(|style: CssStyleDeclaration| {
                    style.get_property_value(GAME_2D_LOADING_COLOR_VAR).ok()
                })
        })
        .unwrap_or_else(|| "#ffffff".to_string());
    let fill_style_key: JsValue = JsValue::from_str(GAME_2D_PROPERTY_FILL_STYLE);
    let _ = Reflect::set(context, &fill_style_key, &JsValue::from_str(&loading_color));
    context.set_font(&font);
    context.set_text_align("center");
    context.set_text_baseline("middle");
    let _ = context.fill_text(
        GAME_2D_LOADING_TEXT,
        GAME_2D_CANVAS_WIDTH * 0.5,
        GAME_2D_CANVAS_HEIGHT * 0.5,
    );
    ssaa_canvas.present();
}

/// Starts the 2D game loop driven by `requestAnimationFrame`.
///
/// Runs a fixed-timestep accumulator loop that updates physics at a constant
/// rate and renders every frame. The canvas context is cached once at startup
/// to avoid per-frame DOM queries. Updates the FPS signal approximately every
/// second.
///
/// # Arguments
///
/// - `UseGame2D` - The 2D game state for signal updates.
/// - `Rc<RefCell<Vec<Ball>>>` - The shared ball list.
/// - `CanvasCache` - The shared canvas element cache for event handlers.
pub(crate) fn start_game_2d_loop(
    state: UseGame2D,
    balls: Rc<RefCell<Vec<Ball>>>,
    canvas_cache: CanvasCache,
) {
    let canvas_ssaa: Rc<RefCell<Option<SsaaCanvas>>> = Rc::new(RefCell::new(None));
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let accumulator: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
    let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
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
            GAME_2D_FIXED_TIMESTEP
        } else {
            (current_time - prev).min(0.25)
        };
        last_clone.set(current_time);
        acc_clone.set(acc_clone.get() + frame_time);
        if state.get_running().get() {
            while acc_clone.get() >= GAME_2D_FIXED_TIMESTEP {
                update_balls(&mut balls.borrow_mut(), GAME_2D_FIXED_TIMESTEP);
                acc_clone.set(acc_clone.get() - GAME_2D_FIXED_TIMESTEP);
            }
        }
        if dirty_clone.get() {
            *context_clone.borrow_mut() = None;
            *canvas_cache.0.borrow_mut() = None;
            dirty_clone.set(false);
        }
        if context_clone.borrow().is_none()
            && let Some((canvas_el, ssaa_canvas)) = acquire_game_2d_ssaa_canvas()
        {
            *canvas_cache.0.borrow_mut() = Some(canvas_el);
            *context_clone.borrow_mut() = Some(ssaa_canvas);
        }
        if let Some(ssaa_canvas) = context_clone.borrow().as_ref() {
            render_balls_with_ssaa(ssaa_canvas, &balls.borrow());
        }
        frame_clone.set(frame_clone.get() + 1);
        fps_clone.set(fps_clone.get() + frame_time);
        if fps_clone.get() >= 1.0 {
            let fps: f64 = f64::from(frame_clone.get()) / fps_clone.get();
            state.get_fps().set(fps);
            frame_clone.set(0);
            fps_clone.set(0.0);
        }
        let next_id: i32 = window_value
            .request_animation_frame(
                cell_clone
                    .try_get()
                    .expect("raf closure should exist")
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap_or(0);
        raf_clone.set(Some(next_id));
    }));
    let _: Result<(), _> = closure_cell.try_set(raf_closure);
    let start_timeout_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let start_timeout_clone: Rc<Cell<Option<i32>>> = start_timeout_id.clone();
    let raf_for_start: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_start: RafClosureCell = closure_cell.clone();
    let state_for_start: UseGame2D = state;
    let start_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        state_for_start.get_loaded().set(true);
        let start_window: Window = window().expect("no global window exists");
        let start_id: i32 = start_window
            .request_animation_frame(
                cell_for_start
                    .try_get()
                    .expect("raf closure should exist")
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap_or(0);
        raf_for_start.set(Some(start_id));
    }));
    let start_callback: Function = start_closure.as_ref().unchecked_ref::<Function>().clone();
    start_closure.forget();
    let start_window: Window = window().expect("no global window exists");
    let timeout_id: i32 = start_window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            &start_callback,
            GAME_2D_LOOP_START_DELAY_MILLIS,
        )
        .unwrap_or(0);
    start_timeout_clone.set(Some(timeout_id));
    let loading_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        draw_game_2d_loading();
    }));
    let loading_callback: Function = loading_closure.as_ref().unchecked_ref::<Function>().clone();
    loading_closure.forget();
    let _ =
        start_window.set_timeout_with_callback_and_timeout_and_arguments_0(&loading_callback, 0);
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
                GAME_2D_RESIZE_DEBOUNCE_MILLIS,
            )
            .unwrap_or_default();
        timer_for_event.set(Some(new_timer));
    });
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
        let _: Option<_> = closure_cell.try_take();
    });
}

/// Creates the reactive state signals for the 2D WebGPU demo.
///
/// Allocates hook slots in this fixed order:
/// 1. fps
/// 2. loaded
/// 3. active
/// 4. loop_started
pub(crate) fn use_game_2d_webgpu_state() -> UseGame2DWebGpu {
    UseGame2DWebGpu {
        fps: App::use_signal(|| 0.0),
        loaded: App::use_signal(|| false),
        active: App::use_signal(|| false),
        loop_started: App::use_signal(|| false),
        init_error_code: App::use_signal(|| ""),
        pointer_text: App::use_signal(|| GAME_2D_POINTER_EMPTY_TEXT.to_string()),
    }
}

/// Creates the reactive state signals for the 2D WebGL demo tab.
///
/// Allocates hook slots in this fixed order:
/// 1. fps
/// 2. loaded
/// 3. active
/// 4. loop_started
/// 5. init_error_code
/// 6. pointer_text
///
/// # Returns
///
/// - `UseGame2DWebGl` - The WebGL demo state.
pub(crate) fn use_game_2d_webgl_state() -> UseGame2DWebGl {
    UseGame2DWebGl {
        fps: App::use_signal(|| 0.0),
        loaded: App::use_signal(|| false),
        active: App::use_signal(|| false),
        loop_started: App::use_signal(|| false),
        init_error_code: App::use_signal(|| ""),
        pointer_text: App::use_signal(|| GAME_2D_POINTER_EMPTY_TEXT.to_string()),
    }
}

/// Attaches the engine's input event listeners to a canvas.
///
/// Builds a throwaway `EngineHandle` purely for its `register_input`
/// wiring; the render config's backend is irrelevant to input handling,
/// so a WebGL config is used as the neutral default. The returned cell is
/// updated by DOM event listeners (`mousemove`, `mousedown`, touch ...)
/// and is polled each frame from the render loop. Listeners are never
/// detached (engine convention), so calling this twice on the same
/// selector would double-report — each demo tab calls it exactly once.
///
/// # Arguments
///
/// - `&str` - The CSS selector of the canvas element to listen on.
///
/// # Returns
///
/// - `Option<InputStateCell>` - The shared input state cell, or `None` if
///   the canvas element was not found in the DOM.
pub(crate) fn attach_game_2d_input(canvas_selector: &str) -> Option<InputStateCell> {
    let config: EngineConfig = EngineConfig::create(RenderConfig::webgl(
        canvas_selector,
        GAME_2D_CANVAS_WIDTH,
        GAME_2D_CANVAS_HEIGHT,
    ));
    let mut handle: EngineHandle = Engine::new_handle(config);
    handle.register_input()
}

/// Queries a canvas element by CSS selector.
///
/// # Arguments
///
/// - `&str` - The CSS selector of the canvas element.
///
/// # Returns
///
/// - `Option<HtmlCanvasElement>` - The canvas element, if present.
pub(crate) fn game_2d_canvas_element(canvas_selector: &str) -> Option<HtmlCanvasElement> {
    let window_value: Window = window().expect("no global window exists");
    let document_value: Document = window_value.document().expect("should have a document");
    let element: Element = document_value
        .query_selector(canvas_selector)
        .ok()
        .flatten()?;
    Some(element.unchecked_into())
}

/// Polls the latest pointer position in client coordinates.
///
/// Mouse and touch are unified: an active primary touch wins over the
/// mouse position (touch devices never fire `mousemove`). Until the
/// pointer first enters the canvas the function returns `None` so the
/// demo keeps the triangle centered and the readout empty; once seen,
/// `has_pointer` latches and the last known position is kept even when
/// the pointer leaves (matching how a cursor stays put).
///
/// # Arguments
///
/// - `&InputStateCell` - The shared input state cell.
/// - `&Rc<Cell<bool>>` - Latch set to `true` on the first pointer sighting.
///
/// # Returns
///
/// - `Option<(f64, f64)>` - The client-space `(x, y)` position.
pub(crate) fn game_2d_pointer_client_position(
    input: &InputStateCell,
    has_pointer: &Rc<Cell<bool>>,
) -> Option<(f64, f64)> {
    let state: &InputState = input.get();
    if let Some(position) = state.primary_touch_position() {
        has_pointer.set(true);
        return Some((position.get_x(), position.get_y()));
    }
    if state.get_mouse_moved() {
        has_pointer.set(true);
    }
    if !has_pointer.get() {
        return None;
    }
    let position: Vector2D = state.get_mouse_position();
    Some((position.get_x(), position.get_y()))
}

/// Maps a client-space pointer position to GPU clip space.
///
/// Clip space x runs -1 (left) to 1 (right) and y runs -1 (bottom) to 1
/// (top), so the client y axis is flipped. Positions outside the canvas
/// are NOT clamped: the triangle follows the pointer offscreen, which
/// keeps the interaction continuous at the edges.
///
/// # Arguments
///
/// - `&HtmlCanvasElement` - The canvas whose bounding rect defines the mapping.
/// - `f64` - The client x coordinate.
/// - `f64` - The client y coordinate.
///
/// # Returns
///
/// - `(f32, f32)` - The clip-space `(x, y)` offset.
pub(crate) fn game_2d_client_to_clip(
    canvas: &HtmlCanvasElement,
    client_x: f64,
    client_y: f64,
) -> (f32, f32) {
    let rect: DomRect = canvas.get_bounding_client_rect();
    let width: f64 = rect.width().max(1.0);
    let height: f64 = rect.height().max(1.0);
    let x: f32 = ((client_x - rect.left()) / width * 2.0 - 1.0) as f32;
    let y: f32 = (1.0 - (client_y - rect.top()) / height * 2.0) as f32;
    (x, y)
}

/// Creates a click event handler that selects a tab on the 2D game page.
///
/// # Arguments
///
/// - `Signal<Game2DTab>` - The tab signal to update.
/// - `Game2DTab` - The tab variant to set.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets the active tab.
pub(crate) fn game_2d_on_tab_select(
    tab: Signal<Game2DTab>,
    value: Game2DTab,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        tab.set(value);
    }))
}

/// Starts the 2D WebGPU render loop driven by `requestAnimationFrame`.
///
/// Asynchronously initializes a `WebGpuRenderer`, creates a render pipeline
/// from a WGSL shader, and runs a `requestAnimationFrame` loop that renders
/// an RGB triangle following the pointer (via a uniform buffer) with an
/// animated clear color each frame.
///
/// # Arguments
///
/// - `UseGame2DWebGpu` - The WebGPU demo state for signal updates.
pub(crate) fn start_game_2d_webgpu_loop(state: UseGame2DWebGpu) {
    let init_state: UseGame2DWebGpu = state;
    let loop_state: UseGame2DWebGpu = state;
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
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
                GAME_2D_RESIZE_DEBOUNCE_MILLIS,
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
        let _: Option<_> = cell_for_cleanup.try_take();
        // Release GPU resources before dropping the renderer so the
        // device and swap chain are freed eagerly. Without this the
        // old GPU device can linger until GC, causing a fresh
        // WebGpuRenderer::init() either to reuse the dead device
        // (silent black canvas) or to fail to acquire a new one.
        if let Some(renderer) = renderer_for_cleanup.borrow_mut().take() {
            renderer.dispose();
        }
    });
    let cancelled_for_init: Rc<Cell<bool>> = cancelled.clone();
    spawn_local(async move {
        let config: RenderConfig = RenderConfig::webgpu(
            GAME_2D_WEBGPU_CANVAS_SELECTOR,
            GAME_2D_CANVAS_WIDTH,
            GAME_2D_CANVAS_HEIGHT,
        );
        let renderer: Result<WebGpuRenderer, WebGpuInitError> =
            Engine::webgpu_renderer(&config).await;
        if cancelled_for_init.get() {
            return;
        }
        let renderer: WebGpuRenderer = match renderer {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!("[euv-engine][game_2d] webgpu init failed: {error}"));
                init_state.get_init_error_code().set(error.code());
                init_state.get_loaded().set(true);
                return;
            }
        };
        let pipeline: JsValue = renderer.create_render_pipeline(GAME_2D_WEBGPU_SHADER);
        let uniform_buffer: JsValue = renderer.create_uniform_buffer(&[0.0, 0.0]);
        let bind_group: JsValue = renderer.create_uniform_bind_group(&pipeline, &uniform_buffer);
        let input: Option<InputStateCell> = attach_game_2d_input(GAME_2D_WEBGPU_CANVAS_SELECTOR);
        let pointer_canvas: Option<HtmlCanvasElement> =
            game_2d_canvas_element(GAME_2D_WEBGPU_CANVAS_SELECTOR);
        let has_pointer: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let last_pointer: Rc<Cell<(f64, f64)>> = Rc::new(Cell::new((0.0, 0.0)));
        init_state.get_active().set(true);
        init_state.get_loaded().set(true);
        *renderer_rc.borrow_mut() = Some(renderer);
        let pipeline_rc: Rc<JsValue> = Rc::new(pipeline);
        let buffer_rc: Rc<JsValue> = Rc::new(uniform_buffer);
        let bind_group_rc: Rc<JsValue> = Rc::new(bind_group);
        let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
        let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
        let renderer_for_loop: Rc<RefCell<Option<WebGpuRenderer>>> = renderer_rc.clone();
        let pipeline_for_loop: Rc<JsValue> = pipeline_rc.clone();
        let buffer_for_loop: Rc<JsValue> = buffer_rc.clone();
        let bind_group_for_loop: Rc<JsValue> = bind_group_rc.clone();
        let input_for_loop: Option<InputStateCell> = input.clone();
        let canvas_for_loop: Option<HtmlCanvasElement> = pointer_canvas.clone();
        let has_pointer_for_loop: Rc<Cell<bool>> = has_pointer.clone();
        let last_pointer_for_loop: Rc<Cell<(f64, f64)>> = last_pointer.clone();
        let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_clone: RafClosureCell = closure_cell.clone();
        let last_clone: Rc<Cell<f64>> = last_time.clone();
        let frame_clone: Rc<Cell<u32>> = frame_count.clone();
        let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
        let resize_dirty_for_loop: Rc<Cell<bool>> = resize_dirty.clone();
        let cancelled_for_loop: Rc<Cell<bool>> = cancelled.clone();
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            if cancelled_for_loop.get() {
                return;
            }
            let window_value: Window = window().expect("no global window exists");
            let performance: Performance = window_value
                .performance()
                .expect("performance should exist");
            let current_time: f64 = performance.now() / 1000.0;
            let prev: f64 = last_clone.get();
            let frame_time: f64 = if prev < 0.0 {
                GAME_2D_FIXED_TIMESTEP
            } else {
                (current_time - prev).min(0.25)
            };
            last_clone.set(current_time);
            // The resize-debounce path only clears the flag and computes
            // the new dimensions. The actual `renderer.resize(...)` call
            // is folded into the render block below so we hold
            // `renderer_for_loop.borrow_mut()` exactly once per frame.
            // Otherwise we previously panicked with `RefCell already
            // borrowed` when both blocks tried to borrow the same cell.
            let resize_dirty: bool = if resize_dirty_for_loop.get() {
                resize_dirty_for_loop.set(false);
                true
            } else {
                false
            };
            let window_for_dpr: Window = window().expect("no global window exists");
            let dpr: f64 = Reflect::get(
                window_for_dpr.as_ref(),
                &JsValue::from_str("devicePixelRatio"),
            )
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .filter(|value: &f64| value.is_finite() && *value >= 1.0)
            .unwrap_or(1.0);
            let new_physical_width: u32 = (GAME_2D_CANVAS_WIDTH * dpr).round() as u32;
            let new_physical_height: u32 = (GAME_2D_CANVAS_HEIGHT * dpr).round() as u32;
            // Borrow the renderer exactly once for the entire frame. We
            // use `borrow_mut().as_mut()` (NOT `borrow_mut().take()`) so
            // we do not have to write back - the RefMut guard releases
            // automatically when this block exits, avoiding a second
            // `borrow_mut()` call that previously panicked with
            // `RefCell already borrowed`.
            if let Some(renderer) = renderer_for_loop.borrow_mut().as_mut() {
                // Only re-size the backing store when the debounced
                // resize event fired. We deliberately do NOT call
                // sync_to_current_canvas() on every frame because
                // `HTMLCanvasElement.clientWidth` in Chrome tracks
                // `canvas.width` (the backing-store size), NOT the
                // CSS layout box, so a sync loop would read its own
                // writes and grow the texture exponentially each
                // frame until WebGPU caps at maxTextureDimension2D
                // and reports `Texture size exceeded`. The init-time
                // backing store already accounts for `dpr`, so a
                // non-resize frame needs no work here.
                if resize_dirty {
                    let _ = renderer.resize(new_physical_width, new_physical_height);
                }
                let t: f64 = current_time;
                let r: f64 = (t * 0.5).sin() * 0.3 + 0.1;
                let g: f64 = (t * 0.3 + 2.0).sin() * 0.3 + 0.1;
                let b: f64 = (t * 0.7 + 4.0).sin() * 0.3 + 0.1;
                // Poll the input state (updated by DOM listeners on the
                // canvas) and push the pointer offset into the uniform
                // buffer so the triangle follows the cursor/touch.
                if let (Some(input), Some(canvas)) = (&input_for_loop, &canvas_for_loop)
                    && let Some((client_x, client_y)) =
                        game_2d_pointer_client_position(input, &has_pointer_for_loop)
                {
                    last_pointer_for_loop.set((client_x, client_y));
                    let (offset_x, offset_y) = game_2d_client_to_clip(canvas, client_x, client_y);
                    renderer.update_uniform_buffer(&buffer_for_loop, &[offset_x, offset_y]);
                }
                renderer.render_frame_with_bind_group(
                    &pipeline_for_loop,
                    &bind_group_for_loop,
                    (r, g, b, 1.0),
                    3,
                );
            }
            frame_clone.set(frame_clone.get() + 1);
            fps_clone.set(fps_clone.get() + frame_time);
            if fps_clone.get() >= 1.0 {
                let fps: f64 = f64::from(frame_clone.get()) / fps_clone.get();
                loop_state.get_fps().set(fps);
                // Refresh the pointer readout alongside the FPS counter so
                // the page re-renders at 1 Hz instead of every frame.
                let pointer_text: String = if has_pointer_for_loop.get() {
                    let (pointer_x, pointer_y) = last_pointer_for_loop.get();
                    format!("({pointer_x:.0}, {pointer_y:.0})")
                } else {
                    GAME_2D_POINTER_EMPTY_TEXT.to_string()
                };
                loop_state.get_pointer_text().set(pointer_text);
                frame_clone.set(0);
                fps_clone.set(0.0);
            }
            let next_id: i32 = window_value
                .request_animation_frame(
                    cell_clone
                        .try_get()
                        .expect("raf closure should exist")
                        .as_ref()
                        .unchecked_ref(),
                )
                .unwrap_or(0);
            if cancelled_for_loop.get() {
                raf_clone.set(None);
            } else {
                raf_clone.set(Some(next_id));
            }
        }));
        let _: Result<(), _> = closure_cell.try_set(raf_closure);
        let start_window: Window = window().expect("no global window exists");
        let start_id: i32 = start_window
            .request_animation_frame(
                closure_cell
                    .try_get()
                    .expect("raf closure should exist")
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap_or(0);
        raf_id.set(Some(start_id));
    });
}

/// Starts the 2D WebGL render loop driven by `requestAnimationFrame`.
///
/// Initializes a `WebGlRenderer`, compiles a GLSL ES 3.00 program, and runs
/// a `requestAnimationFrame` loop that renders an RGB triangle following
/// the pointer (via a `vec2` uniform) with an animated clear color each
/// frame. WebGL initialization is synchronous; the `spawn_local` wrapper
/// only defers execution past the current render pass so the canvas
/// element exists in the DOM.
///
/// # Arguments
///
/// - `UseGame2DWebGl` - The WebGL demo state for signal updates.
pub(crate) fn start_game_2d_webgl_loop(state: UseGame2DWebGl) {
    let init_state: UseGame2DWebGl = state;
    let loop_state: UseGame2DWebGl = state;
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(MaybeEngineCell::new());
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let resize_timer: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let renderer_rc: Rc<RefCell<Option<WebGlRenderer>>> = Rc::new(RefCell::new(None));
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
                GAME_2D_RESIZE_DEBOUNCE_MILLIS,
            )
            .unwrap_or_default();
        resize_timer_for_event.set(Some(new_timer));
    });
    let raf_for_cleanup: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_for_cleanup: RafClosureCell = closure_cell.clone();
    let renderer_for_cleanup: Rc<RefCell<Option<WebGlRenderer>>> = renderer_rc.clone();
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
        let _: Option<_> = cell_for_cleanup.try_take();
        // WebGL has no explicit `destroy()` on the context: dropping the
        // last JS reference lets the browser GC reclaim the GL context.
        let _: Option<WebGlRenderer> = renderer_for_cleanup.borrow_mut().take();
    });
    let cancelled_for_init: Rc<Cell<bool>> = cancelled.clone();
    spawn_local(async move {
        if cancelled_for_init.get() {
            return;
        }
        let config: RenderConfig = RenderConfig::webgl(
            GAME_2D_WEBGL_CANVAS_SELECTOR,
            GAME_2D_CANVAS_WIDTH,
            GAME_2D_CANVAS_HEIGHT,
        );
        let renderer: WebGlRenderer = match Engine::webgl_renderer(&config) {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!("[euv-engine][game_2d] webgl init failed: {error}"));
                init_state.get_init_error_code().set(error.code());
                init_state.get_loaded().set(true);
                return;
            }
        };
        let program: WebGlProgram = match renderer
            .create_program(GAME_2D_WEBGL_VERTEX_SHADER, GAME_2D_WEBGL_FRAGMENT_SHADER)
        {
            Ok(value) => value,
            Err(error) => {
                Console::error(format!(
                    "[euv-engine][game_2d] webgl program failed: {error}"
                ));
                init_state.get_init_error_code().set("WEBGL_PROGRAM_ERROR");
                init_state.get_loaded().set(true);
                return;
            }
        };
        let input: Option<InputStateCell> = attach_game_2d_input(GAME_2D_WEBGL_CANVAS_SELECTOR);
        let pointer_canvas: Option<HtmlCanvasElement> =
            game_2d_canvas_element(GAME_2D_WEBGL_CANVAS_SELECTOR);
        let has_pointer: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let last_pointer: Rc<Cell<(f64, f64)>> = Rc::new(Cell::new((0.0, 0.0)));
        init_state.get_active().set(true);
        init_state.get_loaded().set(true);
        *renderer_rc.borrow_mut() = Some(renderer);
        let program_rc: Rc<WebGlProgram> = Rc::new(program);
        let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
        let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
        let renderer_for_loop: Rc<RefCell<Option<WebGlRenderer>>> = renderer_rc.clone();
        let program_for_loop: Rc<WebGlProgram> = program_rc.clone();
        let input_for_loop: Option<InputStateCell> = input.clone();
        let canvas_for_loop: Option<HtmlCanvasElement> = pointer_canvas.clone();
        let has_pointer_for_loop: Rc<Cell<bool>> = has_pointer.clone();
        let last_pointer_for_loop: Rc<Cell<(f64, f64)>> = last_pointer.clone();
        let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
        let cell_clone: RafClosureCell = closure_cell.clone();
        let last_clone: Rc<Cell<f64>> = last_time.clone();
        let frame_clone: Rc<Cell<u32>> = frame_count.clone();
        let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
        let resize_dirty_for_loop: Rc<Cell<bool>> = resize_dirty.clone();
        let cancelled_for_loop: Rc<Cell<bool>> = cancelled.clone();
        let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            if cancelled_for_loop.get() {
                return;
            }
            let window_value: Window = window().expect("no global window exists");
            let performance: Performance = window_value
                .performance()
                .expect("performance should exist");
            let current_time: f64 = performance.now() / 1000.0;
            let prev: f64 = last_clone.get();
            let frame_time: f64 = if prev < 0.0 {
                GAME_2D_FIXED_TIMESTEP
            } else {
                (current_time - prev).min(0.25)
            };
            last_clone.set(current_time);
            let resize_dirty: bool = if resize_dirty_for_loop.get() {
                resize_dirty_for_loop.set(false);
                true
            } else {
                false
            };
            let window_for_dpr: Window = window().expect("no global window exists");
            let dpr: f64 = Reflect::get(
                window_for_dpr.as_ref(),
                &JsValue::from_str("devicePixelRatio"),
            )
            .ok()
            .and_then(|value: JsValue| value.as_f64())
            .filter(|value: &f64| value.is_finite() && *value >= 1.0)
            .unwrap_or(1.0);
            let new_physical_width: u32 = (GAME_2D_CANVAS_WIDTH * dpr).round() as u32;
            let new_physical_height: u32 = (GAME_2D_CANVAS_HEIGHT * dpr).round() as u32;
            if let Some(renderer) = renderer_for_loop.borrow_mut().as_mut() {
                if resize_dirty {
                    renderer.resize(new_physical_width, new_physical_height);
                }
                let t: f64 = current_time;
                let r: f64 = (t * 0.5).sin() * 0.3 + 0.1;
                let g: f64 = (t * 0.3 + 2.0).sin() * 0.3 + 0.1;
                let b: f64 = (t * 0.7 + 4.0).sin() * 0.3 + 0.1;
                // Poll the input state (updated by DOM listeners on the
                // canvas) and push the pointer offset into the `vec2`
                // uniform so the triangle follows the cursor/touch.
                if let (Some(input), Some(canvas)) = (&input_for_loop, &canvas_for_loop)
                    && let Some((client_x, client_y)) =
                        game_2d_pointer_client_position(input, &has_pointer_for_loop)
                {
                    last_pointer_for_loop.set((client_x, client_y));
                    let (offset_x, offset_y) = game_2d_client_to_clip(canvas, client_x, client_y);
                    renderer.set_uniform_2f(
                        &program_for_loop,
                        "u_pointer_offset",
                        offset_x,
                        offset_y,
                    );
                }
                renderer.render_frame(&program_for_loop, (r, g, b, 1.0), 3);
            }
            frame_clone.set(frame_clone.get() + 1);
            fps_clone.set(fps_clone.get() + frame_time);
            if fps_clone.get() >= 1.0 {
                let fps: f64 = f64::from(frame_clone.get()) / fps_clone.get();
                loop_state.get_fps().set(fps);
                let pointer_text: String = if has_pointer_for_loop.get() {
                    let (pointer_x, pointer_y) = last_pointer_for_loop.get();
                    format!("({pointer_x:.0}, {pointer_y:.0})")
                } else {
                    GAME_2D_POINTER_EMPTY_TEXT.to_string()
                };
                loop_state.get_pointer_text().set(pointer_text);
                frame_clone.set(0);
                fps_clone.set(0.0);
            }
            let next_id: i32 = window_value
                .request_animation_frame(
                    cell_clone
                        .try_get()
                        .expect("raf closure should exist")
                        .as_ref()
                        .unchecked_ref(),
                )
                .unwrap_or(0);
            if cancelled_for_loop.get() {
                raf_clone.set(None);
            } else {
                raf_clone.set(Some(next_id));
            }
        }));
        let _: Result<(), _> = closure_cell.try_set(raf_closure);
        let start_window: Window = window().expect("no global window exists");
        let start_id: i32 = start_window
            .request_animation_frame(
                closure_cell
                    .try_get()
                    .expect("raf closure should exist")
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap_or(0);
        raf_id.set(Some(start_id));
    });
}
