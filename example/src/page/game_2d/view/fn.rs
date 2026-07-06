use crate::*;

/// A 2D bouncing balls physics game demo powered by the euv-engine.
///
/// Click on the canvas to spawn balls. Each ball is affected by gravity,
/// bounces off walls with restitution, and collides with other balls
/// using impulse-based physics. The game loop runs at a fixed 60 Hz
/// timestep with interpolation via `requestAnimationFrame`.
///
/// # Returns
///
/// - `VirtualNode` - The 2D game demo page virtual DOM tree.
#[component]
pub(crate) fn page_game_2d(node: VirtualNode<PageGame2DProps>) -> VirtualNode {
    let PageGame2DProps = node.try_get_props().unwrap_or_default();
    let state: UseGame2D = use_game_2d_state();
    let balls_store: Signal<BallStore> = App::use_signal(|| {
        let balls: Rc<RefCell<Vec<Ball>>> = Rc::new(RefCell::new(Vec::new()));
        balls
            .borrow_mut()
            .push(create_ball(Vector2D::new(GAME_2D_CANVAS_WIDTH * 0.5, 50.0)));
        balls.borrow_mut().push(create_ball(Vector2D::new(
            GAME_2D_CANVAS_WIDTH * 0.3,
            100.0,
        )));
        balls
            .borrow_mut()
            .push(create_ball(Vector2D::new(GAME_2D_CANVAS_WIDTH * 0.7, 80.0)));
        BallStore(balls)
    });
    let balls: Rc<RefCell<Vec<Ball>>> = balls_store.get().0;
    let canvas_cache: CanvasCache =
        App::use_signal(|| CanvasCache(Rc::new(RefCell::new(None)))).get();
    let loop_started: Signal<bool> = App::use_signal(|| false);
    if !loop_started.get() {
        loop_started.set(true);
        state.get_ball_count().set(balls.borrow().len());
        state.get_total_spawned().set(balls.borrow().len());
        start_game_2d_loop(state, balls.clone(), canvas_cache.clone());
    }
    let on_canvas_click: Option<Rc<dyn Fn(Event)>> =
        game_2d_on_spawn_ball(state, balls.clone(), canvas_cache.clone());
    let on_canvas_touch: Option<Rc<dyn Fn(Event)>> =
        game_2d_on_touch_spawn_ball(state, balls.clone(), canvas_cache.clone());
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = game_2d_on_toggle_pause(state);
    let on_clear: Option<Rc<dyn Fn(Event)>> = game_2d_on_clear(state, balls.clone());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let ball_count: usize = state.get_ball_count().get();
    let total: usize = state.get_total_spawned().get();
    let pause_label: &str = if state.get_running().get() {
        "Pause"
    } else {
        "Resume"
    };
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🎮"
                title: "2D Game Engine"
                subtitle: "A bouncing balls physics demo powered by euv-engine. Click on the canvas to spawn balls. Each ball has gravity, wall bouncing with restitution, and impulse-based ball-to-ball collision."
            }
            euv_card {
                title: "Bouncing Balls"
                div {
                    class: c_game_stats_bar()
                    span {
                        class: c_game_stats_label()
                        "FPS: "
                        span {
                            class: c_game_stats_fps_value()
                            fps_display
                        }
                    }
                    span {
                        class: c_game_stats_label()
                        "Balls: "
                        span {
                            class: c_game_stats_count_value()
                            ball_count
                        }
                    }
                    span {
                        class: c_game_stats_label()
                        "Total: "
                        span {
                            class: c_game_stats_total_value()
                            total
                        }
                    }
                }
                div {
                    class: c_game_canvas_wrapper()
                    canvas {
                        id: GAME_2D_CANVAS_ID
                        class: c_game_2d_canvas(&format!("{} / {}", GAME_2D_CANVAS_WIDTH as i32, GAME_2D_CANVAS_HEIGHT as i32), GAME_2D_BACKGROUND_COLOR)
                        onclick: on_canvas_click
                        ontouchstart: on_canvas_touch
                    }
                    if { !state.get_loaded().get() } {
                        euv_loading {
                            title: "Loading..."
                            overlay: true
                            background: GAME_2D_BACKGROUND_COLOR
                        }
                    }
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: pause_label
                        onclick: on_toggle_pause
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Clear"
                        onclick: on_clear
                    }
                }
            }
            euv_card {
                title: "2D Engine Features"
                p {
                    class: c_game_description()
                    "This demo uses euv-engine's Vector2D for position/velocity math, impulse-based collision resolution with mass proportional to radius squared, wall reflection with configurable restitution, and a fixed-timestep game loop with accumulator pattern for deterministic physics at 60 Hz."
                }
            }
        }
    }
}
