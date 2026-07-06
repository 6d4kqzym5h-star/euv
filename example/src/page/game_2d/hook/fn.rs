use crate::*;

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
    (mouse_event.client_x() as f64, mouse_event.client_y() as f64)
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
    (touch.client_x() as f64, touch.client_y() as f64)
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
/// Applies gravity, integrates velocity and position, handles wall collisions
/// with restitution, and resolves ball-to-ball collisions using impulse-based
/// response.
///
/// # Arguments
///
/// - `&mut [Ball]` - The mutable ball slice.
/// - `f64` - The delta time in seconds.
pub(crate) fn update_balls(balls: &mut [Ball], delta_time: f64) {
    let gravity: Vector2D = Vector2D::new(0.0, GAME_2D_GRAVITY);
    let damping: f64 = (1.0 - GAME_2D_LINEAR_DAMPING * delta_time).max(0.0);
    for ball in balls.iter_mut() {
        ball.velocity += gravity.scaled(delta_time);
        ball.velocity = ball.velocity.scaled(damping);
        ball.position += ball.velocity.scaled(delta_time);
    }
    for ball in balls.iter_mut() {
        resolve_wall_collision(ball);
    }
    let count: usize = balls.len();
    for i in 0..count {
        for j in (i + 1)..count {
            let (left, right) = balls.split_at_mut(j);
            resolve_ball_collision(&mut left[i], &mut right[0]);
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

/// Renders all balls onto the canvas using a pre-cached rendering context.
///
/// Clears the canvas with the background color, then draws each ball as a
/// filled circle with its assigned color.
///
/// # Arguments
///
/// - `&CanvasRenderingContext2d` - The cached canvas 2D rendering context.
/// - `&[Ball]` - The ball list to render.
pub(crate) fn render_balls_with_context(context: &CanvasRenderingContext2d, balls: &[Ball]) {
    let fill_style_key: JsValue = JsValue::from_str(GAME_2D_PROPERTY_FILL_STYLE);
    let _ = Reflect::set(
        context,
        &fill_style_key,
        &JsValue::from_str(GAME_2D_BACKGROUND_COLOR),
    );
    context.fill_rect(0.0, 0.0, GAME_2D_CANVAS_WIDTH, GAME_2D_CANVAS_HEIGHT);
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
}

/// Queries the 2D game canvas element and caches its 2D rendering context.
///
/// # Returns
///
/// - `Option<(HtmlCanvasElement, CanvasRenderingContext2d)>` - The canvas element and cached context, or `None` if unavailable.
pub(crate) fn acquire_game_2d_canvas() -> Option<(HtmlCanvasElement, CanvasRenderingContext2d)> {
    let window_value: Window = window().expect("no global window exists");
    let document_value: Document = window_value.document().expect("should have a document");
    let element: Element = document_value
        .query_selector(GAME_2D_CANVAS_SELECTOR)
        .ok()
        .flatten()?;
    let canvas: HtmlCanvasElement = element.unchecked_into();
    canvas.set_width(GAME_2D_CANVAS_WIDTH as u32);
    canvas.set_height(GAME_2D_CANVAS_HEIGHT as u32);
    let context_object: Object = canvas.get_context(GAME_2D_CONTEXT_TYPE_2D).ok().flatten()?;
    let context: CanvasRenderingContext2d = context_object.unchecked_into();
    CanvasRenderer::enable_context_anti_aliasing(&context);
    Some((canvas, context))
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
    let canvas_context: Rc<RefCell<Option<CanvasRenderingContext2d>>> = Rc::new(RefCell::new(None));
    let resize_dirty: Rc<Cell<bool>> = Rc::new(Cell::new(false));
    let accumulator: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let last_time: Rc<Cell<f64>> = Rc::new(Cell::new(-1.0));
    let frame_count: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let fps_timer: Rc<Cell<f64>> = Rc::new(Cell::new(0.0));
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    let closure_cell: RafClosureCell = Rc::new(RefCell::new(None));
    let state_clone: UseGame2D = state;
    let balls_clone: Rc<RefCell<Vec<Ball>>> = balls;
    let cache_clone: CanvasCache = canvas_cache;
    let acc_clone: Rc<Cell<f64>> = accumulator.clone();
    let last_clone: Rc<Cell<f64>> = last_time.clone();
    let frame_clone: Rc<Cell<u32>> = frame_count.clone();
    let fps_clone: Rc<Cell<f64>> = fps_timer.clone();
    let raf_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
    let cell_clone: RafClosureCell = closure_cell.clone();
    let context_clone: Rc<RefCell<Option<CanvasRenderingContext2d>>> = canvas_context.clone();
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
        if state_clone.get_running().get() {
            while acc_clone.get() >= GAME_2D_FIXED_TIMESTEP {
                update_balls(&mut balls_clone.borrow_mut(), GAME_2D_FIXED_TIMESTEP);
                acc_clone.set(acc_clone.get() - GAME_2D_FIXED_TIMESTEP);
            }
        }
        if dirty_clone.get() {
            *context_clone.borrow_mut() = None;
            *cache_clone.0.borrow_mut() = None;
            dirty_clone.set(false);
        }
        if context_clone.borrow().is_none()
            && let Some((canvas_el, ctx)) = acquire_game_2d_canvas()
        {
            *cache_clone.0.borrow_mut() = Some(canvas_el);
            *context_clone.borrow_mut() = Some(ctx);
        }
        if let Some(context) = context_clone.borrow().as_ref() {
            render_balls_with_context(context, &balls_clone.borrow());
        }
        frame_clone.set(frame_clone.get() + 1);
        fps_clone.set(fps_clone.get() + frame_time);
        if fps_clone.get() >= 1.0 {
            let fps: f64 = frame_clone.get() as f64 / fps_clone.get();
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
    }) as Box<dyn FnMut()>);
    let window_value: Window = window().expect("no global window exists");
    let id: i32 = window_value
        .request_animation_frame(raf_closure.as_ref().unchecked_ref())
        .unwrap_or(0);
    raf_id.set(Some(id));
    *closure_cell.borrow_mut() = Some(raf_closure);
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
        if let Some(timer_id) = debounce_timer.get() {
            let window_value: Window = window().expect("no global window exists");
            window_value.clear_timeout_with_handle(timer_id);
        }
        closure_cell.borrow_mut().take();
    });
}
