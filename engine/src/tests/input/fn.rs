use super::*;

// =====================================================================
// input tests
// =====================================================================



/// Verifies that a fresh `EngineHandle` starts without a registered input
/// cell and that `register_input`'s accessor has the expected shape.
#[test]
fn engine_handle_starts_without_input_cell() {
    let config: EngineConfig = EngineConfig::default();
    let handle: EngineHandle = EngineHandle::new(config, None, None, None, None);
    let cell: &Option<InputStateCell> = handle.try_get_input_cell();
    assert!(cell.is_none());
}

/// Verifies the `InputState` mutation and readout accessor shapes used by
/// the DOM event listeners and per-frame consumers.
#[test]
fn input_state_frame_lifecycle() {
    let mut state: InputState = InputState::new();
    state.press_key("KeyW".to_string());
    state.press_mouse_button(MouseButton::Left, Vector2D::new(10.0, 20.0));
    state.update_mouse_position(Vector2D::new(12.0, 24.0));
    state.start_touch(1, Vector2D::new(5.0, 6.0));
    assert!(state.get_keys_pressed().contains("KeyW"));
    assert!(state.get_keys_held().contains("KeyW"));
    assert!(
        state
            .get_mouse_buttons_pressed()
            .contains(&MouseButton::Left)
    );
    assert!(state.get_mouse_buttons_held().contains(&MouseButton::Left));
    assert_eq!(state.get_mouse_position().get_x(), 12.0);
    assert_eq!(state.get_mouse_position().get_y(), 24.0);
    assert!(state.get_touch_points().contains_key(&1));
    assert!(state.get_touch_started().contains(&1));
    state.end_frame();
    assert!(state.get_keys_pressed().is_empty());
    assert!(state.get_keys_held().contains("KeyW"));
    assert!(state.get_mouse_buttons_pressed().is_empty());
    assert!(state.get_mouse_buttons_held().contains(&MouseButton::Left));
    assert!(!state.get_mouse_moved());
    assert!(state.get_touch_started().is_empty());
    assert!(state.get_touch_points().contains_key(&1));
    state.release_key("KeyW".to_string());
    state.release_mouse_button(MouseButton::Left);
    state.end_touch(1);
    assert!(state.get_keys_released().contains("KeyW"));
    assert!(!state.get_keys_held().contains("KeyW"));
    assert!(!state.get_mouse_buttons_held().contains(&MouseButton::Left));
    assert!(!state.get_touch_points().contains_key(&1));
    assert!(state.get_touch_ended().contains(&1));
}
