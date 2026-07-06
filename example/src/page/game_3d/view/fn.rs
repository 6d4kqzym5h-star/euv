use crate::*;

/// A 3D rotating cubes demo powered by the euv-engine 3D math library.
///
/// Displays multiple 3D cubes rendered on a 2D canvas using perspective
/// projection. The camera orbits around the scene and can be dragged
/// with the mouse or touch. Each cube rotates independently using
/// quaternion-based angular velocity integration. Features back-face
/// culling and painter's algorithm depth sorting.
///
/// # Returns
///
/// - `VirtualNode` - The 3D game demo page virtual DOM tree.
#[component]
pub(crate) fn page_game_3d(node: VirtualNode<PageGame3DProps>) -> VirtualNode {
    let _page_game_3d_props: PageGame3DProps = node.try_get_props().unwrap_or_default();
    let state: UseGame3D = use_game_3d_state();
    let cubes_store: Signal<CubeStore> = App::use_signal(|| {
        let cubes: Rc<RefCell<Vec<Cube3D>>> = Rc::new(RefCell::new(create_initial_cubes()));
        CubeStore(cubes)
    });
    let cubes: Rc<RefCell<Vec<Cube3D>>> = cubes_store.get().0;
    let angles_store: Signal<CameraAngles> = App::use_signal(CameraAngles::default);
    let angles: CameraAngles = angles_store.get();
    let loop_started: Signal<bool> = App::use_signal(|| false);
    let last_pointer: PointerPositionSignal = App::use_signal(|| Rc::new(Cell::new(None)));
    if !loop_started.get() {
        loop_started.set(true);
        state.get_cube_count().set(cubes.borrow().len());
        start_game_3d_loop(state, cubes.clone(), angles.clone());
    }
    let on_toggle_pause: Option<Rc<dyn Fn(Event)>> = game_3d_on_toggle_pause(state);
    let on_toggle_auto_rotate: Option<Rc<dyn Fn(Event)>> = game_3d_on_toggle_auto_rotate(state);
    let on_reset_camera: Option<Rc<dyn Fn(Event)>> = game_3d_on_reset_camera(angles.clone());
    let pointer_cell: Rc<Cell<Option<(f64, f64)>>> = last_pointer.get();
    let on_pointer_down: Option<Rc<dyn Fn(Event)>> = game_3d_on_pointer_down(pointer_cell.clone());
    let on_pointer_move: Option<Rc<dyn Fn(Event)>> =
        game_3d_on_pointer_move(angles.clone(), pointer_cell.clone());
    let on_pointer_up: Option<Rc<dyn Fn(Event)>> = game_3d_on_pointer_up(pointer_cell.clone());
    let on_touch_start: Option<Rc<dyn Fn(Event)>> = game_3d_on_touch_start(pointer_cell.clone());
    let on_touch_move: Option<Rc<dyn Fn(Event)>> =
        game_3d_on_touch_move(angles.clone(), pointer_cell.clone());
    let on_touch_end: Option<Rc<dyn Fn(Event)>> = game_3d_on_touch_end(pointer_cell.clone());
    let fps_display: String = format!("{:.1}", state.get_fps().get());
    let cube_count: usize = state.get_cube_count().get();
    let pause_label: &str = if state.get_running().get() {
        "Pause"
    } else {
        "Resume"
    };
    let auto_rotate_label: &str = if state.get_auto_rotate().get() {
        "Auto: On"
    } else {
        "Auto: Off"
    };
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🎲"
                title: "3D Game Engine"
                subtitle: "A rotating cubes 3D demo powered by euv-engine's Vector3D, Quaternion, Matrix4x4, and Camera3D. Drag to orbit the camera."
            }
            euv_card {
                title: "3D Rotating Cubes"
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
                        "Cubes: "
                        span {
                            class: c_game_stats_count_value()
                            cube_count
                        }
                    }
                }
                canvas {
                    id: GAME_3D_CANVAS_ID
                    class: c_game_3d_canvas(&format!("{} / {}", GAME_3D_CANVAS_WIDTH as i32, GAME_3D_CANVAS_HEIGHT as i32), GAME_3D_BACKGROUND_COLOR)
                    onmousedown: on_pointer_down.clone()
                    onmousemove: on_pointer_move.clone()
                    onmouseup: on_pointer_up.clone()
                    onmouseleave: on_pointer_up.clone()
                    ontouchstart: on_touch_start.clone()
                    ontouchmove: on_touch_move.clone()
                    ontouchend: on_touch_end.clone()
                    ontouchcancel: on_touch_end.clone()
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
                        label: auto_rotate_label
                        onclick: on_toggle_auto_rotate
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: "Reset Camera"
                        onclick: on_reset_camera
                    }
                }
            }
            euv_card {
                title: "3D Engine Features"
                p {
                    class: c_game_description()
                    "This demo uses euv-engine's 3D math: Vector3D for positions, Quaternion for rotation, Matrix4x4 for view/projection transforms, Camera3D for orbit camera with perspective projection, and Transform3D for cube transforms. Features include back-face culling, painter's algorithm depth sorting, and quaternion-based angular velocity integration."
                }
            }
        }
    }
}
