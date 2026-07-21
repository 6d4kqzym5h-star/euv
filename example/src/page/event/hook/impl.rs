use super::*;

/// Provides a default empty keyboard event state with placeholder signals.
impl Default for UseKeyboardEvent {
    /// Creates a default `UseKeyboardEvent` with empty signals.
    fn default() -> Self {
        Self {
            last_key: Signal::create(String::new()),
            last_key_code: Signal::create(String::new()),
            last_key_up: Signal::create(String::new()),
            key_repeat: Signal::create(false),
            modifier: Signal::create(String::new()),
        }
    }
}

/// Provides a default empty mouse event state with placeholder signals.
impl Default for UseMouseEvent {
    /// Creates a default `UseMouseEvent` with empty signals.
    fn default() -> Self {
        Self {
            click_count: Signal::create(0),
            double_click_count: Signal::create(0),
            mouse_pos: Signal::create(String::new()),
            mouse_screen_pos: Signal::create(String::new()),
            mouse_button: Signal::create(String::new()),
            mouse_buttons: Signal::create(String::new()),
            mouse_enter_count: Signal::create(0),
            mouse_leave_count: Signal::create(0),
            mouse_over_count: Signal::create(0),
            mouse_out_count: Signal::create(0),
            mouse_down_count: Signal::create(0),
            mouse_up_count: Signal::create(0),
        }
    }
}

/// Provides a default empty focus event state with placeholder signals.
impl Default for UseFocusEvent {
    /// Creates a default `UseFocusEvent` with empty signals.
    fn default() -> Self {
        Self {
            focus_status: Signal::create(String::new()),
            focus_in_count: Signal::create(0),
            focus_out_count: Signal::create(0),
        }
    }
}

/// Provides a default empty drag event state with placeholder signals.
impl Default for UseDragEvent {
    /// Creates a default `UseDragEvent` with empty signals.
    fn default() -> Self {
        Self {
            drag_status: Signal::create(String::new()),
            drag_pos: Signal::create(String::new()),
            drag_types: Signal::create(String::new()),
            drag_enter_counter: Signal::create(0),
            drag_pending_pos: Signal::create(String::new()),
            drag_raf_id: Signal::create(-1),
        }
    }
}

/// Provides a default empty wheel event state with placeholder signals.
impl Default for UseWheelEvent {
    /// Creates a default `UseWheelEvent` with empty signals.
    fn default() -> Self {
        Self {
            wheel_delta: Signal::create(String::new()),
            wheel_total: Signal::create(0.0),
        }
    }
}

/// Provides a default empty clipboard event state with placeholder signals.
impl Default for UseClipboardEvent {
    /// Creates a default `UseClipboardEvent` with empty signals.
    fn default() -> Self {
        Self {
            clipboard_data: Signal::create(String::new()),
            clipboard_event_type: Signal::create(String::new()),
        }
    }
}

/// Provides a default empty touch event state with placeholder signals.
impl Default for UseTouchEvent {
    /// Creates a default `UseTouchEvent` with empty signals.
    fn default() -> Self {
        Self {
            touch_info: Signal::create(String::new()),
        }
    }
}

/// Provides a default empty form event state with placeholder signals.
impl Default for UseFormEvent {
    /// Creates a default `UseFormEvent` with empty signals.
    fn default() -> Self {
        Self {
            euv_input_value: Signal::create(String::new()),
            form_change_value: Signal::create(String::new()),
            form_checkbox: Signal::create(false),
            form_select_value: Signal::create(String::new()),
            submit_count: Signal::create(0),
        }
    }
}

/// Provides a default empty media event state with placeholder signals.
impl Default for UseMediaEvent {
    /// Creates a default `UseMediaEvent` with empty signals.
    fn default() -> Self {
        Self {
            media_status: Signal::create(String::new()),
            media_event_log: Signal::create(String::new()),
        }
    }
}

/// Provides a default empty video event state with placeholder signals.
impl Default for UseVideoEvent {
    /// Creates a default `UseVideoEvent` with empty signals.
    fn default() -> Self {
        Self {
            video_status: Signal::create(String::new()),
            video_event_log: Signal::create(String::new()),
            video_current_time: Signal::create(String::new()),
            video_duration: Signal::create(String::new()),
            video_buffered: Signal::create(String::new()),
            video_playback_rate: Signal::create(String::new()),
        }
    }
}

/// Provides a default empty image event state with placeholder signals.
impl Default for UseImageEvent {
    /// Creates a default `UseImageEvent` with empty signals.
    fn default() -> Self {
        Self {
            image_status: Signal::create(String::new()),
            image_event_log: Signal::create(String::new()),
            image_natural_size: Signal::create(String::new()),
        }
    }
}
