use crate::*;

/// Implements static event extraction methods on the `Input` namespace struct.
impl Input {
    /// Extracts the key code string from a keyboard event.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The keyboard event.
    ///
    /// # Returns
    ///
    /// - `String` - The key code string (e.g., `"KeyA"`, `"Space"`, `"ArrowLeft"`).
    pub fn extract_key_code(event: &Event) -> String {
        Reflect::get(event.as_ref(), &JsValue::from_str(INPUT_KEY_CODE_PROPERTY))
            .ok()
            .and_then(|value: JsValue| value.as_string())
            .unwrap_or_default()
    }

    /// Extracts the mouse button enum from a mouse event.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The mouse event.
    ///
    /// # Returns
    ///
    /// - `MouseButton` - The mouse button that was pressed or released.
    pub fn extract_mouse_button(event: &Event) -> MouseButton {
        let button_value: i32 = Reflect::get(
            event.as_ref(),
            &JsValue::from_str(INPUT_MOUSE_BUTTON_PROPERTY),
        )
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .map(|float: f64| float as i32)
        .unwrap_or(0);
        match button_value {
            0 => MouseButton::Left,
            1 => MouseButton::Middle,
            2 => MouseButton::Right,
            3 => MouseButton::Button4,
            4 => MouseButton::Button5,
            _ => MouseButton::Left,
        }
    }

    /// Extracts the client (viewport) coordinates from a mouse event.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The mouse event.
    ///
    /// # Returns
    ///
    /// - `Vector2D` - The `(x, y)` client coordinates.
    pub fn extract_mouse_position(event: &Event) -> Vector2D {
        let client_x: f64 =
            Reflect::get(event.as_ref(), &JsValue::from_str(INPUT_CLIENT_X_PROPERTY))
                .ok()
                .and_then(|value: JsValue| value.as_f64())
                .unwrap_or(0.0);
        let client_y: f64 =
            Reflect::get(event.as_ref(), &JsValue::from_str(INPUT_CLIENT_Y_PROPERTY))
                .ok()
                .and_then(|value: JsValue| value.as_f64())
                .unwrap_or(0.0);
        Vector2D::new(client_x, client_y)
    }
}

/// Implements input state management for `InputState`.
impl InputState {
    /// Records a key press event, adding to `keys_pressed` and `keys_held`.
    ///
    /// # Arguments
    ///
    /// - `String` - The key code string (e.g., `"KeyA"`, `"Space"`).
    pub fn press_key(&mut self, key_code: String) {
        if !self.get_keys_held().contains(&key_code) {
            self.get_mut_keys_pressed().insert(key_code.clone());
        }
        self.get_mut_keys_held().insert(key_code);
    }

    /// Records a key release event, moving from `keys_held` to `keys_released`.
    ///
    /// # Arguments
    ///
    /// - `String` - The key code string.
    pub fn release_key(&mut self, key_code: String) {
        self.get_mut_keys_held().remove(&key_code);
        self.get_mut_keys_released().insert(key_code);
    }

    /// Tests whether a key was pressed during this frame.
    ///
    /// # Arguments
    ///
    /// - `&str` - The key code string.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the key was pressed this frame.
    pub fn is_key_pressed(&self, key_code: &str) -> bool {
        self.get_keys_pressed().contains(key_code)
    }

    /// Tests whether a key is currently held down.
    ///
    /// # Arguments
    ///
    /// - `&str` - The key code string.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the key is held.
    pub fn is_key_held(&self, key_code: &str) -> bool {
        self.get_keys_held().contains(key_code)
    }

    /// Tests whether a key was released during this frame.
    ///
    /// # Arguments
    ///
    /// - `&str` - The key code string.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the key was released this frame.
    pub fn is_key_released(&self, key_code: &str) -> bool {
        self.get_keys_released().contains(key_code)
    }

    /// Records a mouse button press at the given position.
    ///
    /// # Arguments
    ///
    /// - `MouseButton` - The button that was pressed.
    /// - `Vector2D` - The mouse position.
    pub fn press_mouse_button(&mut self, button: MouseButton, position: Vector2D) {
        if !self.get_mouse_buttons_held().contains(&button) {
            self.get_mut_mouse_buttons_pressed().insert(button);
        }
        self.get_mut_mouse_buttons_held().insert(button);
        self.set_mouse_position(position);
    }

    /// Records a mouse button release.
    ///
    /// # Arguments
    ///
    /// - `MouseButton` - The button that was released.
    pub fn release_mouse_button(&mut self, button: MouseButton) {
        self.get_mut_mouse_buttons_held().remove(&button);
        self.get_mut_mouse_buttons_released().insert(button);
    }

    /// Updates the mouse position and computes the delta from the previous position.
    ///
    /// # Arguments
    ///
    /// - `Vector2D` - The new mouse position.
    pub fn update_mouse_position(&mut self, position: Vector2D) {
        self.set_mouse_delta(position - self.get_mouse_position());
        self.set_mouse_position(position);
        self.set_mouse_moved(true);
    }

    /// Tests whether a mouse button was pressed during this frame.
    ///
    /// # Arguments
    ///
    /// - `MouseButton` - The button to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the button was pressed this frame.
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.get_mouse_buttons_pressed().contains(&button)
    }

    /// Tests whether a mouse button is currently held down.
    ///
    /// # Arguments
    ///
    /// - `MouseButton` - The button to check.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the button is held.
    pub fn is_mouse_button_held(&self, button: MouseButton) -> bool {
        self.get_mouse_buttons_held().contains(&button)
    }

    /// Adds or updates a touch point.
    ///
    /// # Arguments
    ///
    /// - `i32` - The touch identifier.
    /// - `Vector2D` - The touch position.
    pub fn update_touch(&mut self, identifier: i32, position: Vector2D) {
        self.get_mut_touch_points().insert(identifier, position);
    }

    /// Records a new touch point that started this frame.
    ///
    /// # Arguments
    ///
    /// - `i32` - The touch identifier.
    /// - `Vector2D` - The touch position.
    pub fn start_touch(&mut self, identifier: i32, position: Vector2D) {
        self.get_mut_touch_points().insert(identifier, position);
        self.get_mut_touch_started().insert(identifier);
    }

    /// Removes a touch point and marks it as ended this frame.
    ///
    /// # Arguments
    ///
    /// - `i32` - The touch identifier.
    pub fn end_touch(&mut self, identifier: i32) {
        self.get_mut_touch_points().remove(&identifier);
        self.get_mut_touch_ended().insert(identifier);
    }

    /// Clears all per-frame input data (pressed, released, deltas).
    ///
    /// Should be called at the end of each game frame after all input has been processed.
    pub fn end_frame(&mut self) {
        self.get_mut_keys_pressed().clear();
        self.get_mut_keys_released().clear();
        self.get_mut_mouse_buttons_pressed().clear();
        self.get_mut_mouse_buttons_released().clear();
        self.set_mouse_delta(Vector2D::zero());
        self.set_mouse_moved(false);
        self.get_mut_touch_started().clear();
        self.get_mut_touch_ended().clear();
    }
}

/// Implements `Default` for `InputState` as a fresh empty state.
impl Default for InputState {
    fn default() -> InputState {
        InputState::new()
    }
}
