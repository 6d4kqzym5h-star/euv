use crate::*;

/// Reactive state for the keyboard events demo.
#[derive(Clone, Copy, Data)]
pub(crate) struct UseKeyboardEvent {
    /// The last key pressed down.
    #[get(type(copy))]
    pub(crate) last_key: Signal<String>,
    /// The last key code pressed down.
    #[get(type(copy))]
    pub(crate) last_key_code: Signal<String>,
    /// The last key released up.
    #[get(type(copy))]
    pub(crate) last_key_up: Signal<String>,
    /// Whether the key press is a repeat.
    #[get(type(copy))]
    pub(crate) key_repeat: Signal<bool>,
    /// The active modifier keys string.
    #[get(type(copy))]
    pub(crate) modifier: Signal<String>,
}

/// Reactive state for the mouse events demo.
#[derive(Clone, Copy, Data)]
pub(crate) struct UseMouseEvent {
    /// The total click count.
    #[get(type(copy))]
    pub(crate) click_count: Signal<i32>,
    /// The total double-click count.
    #[get(type(copy))]
    pub(crate) double_click_count: Signal<i32>,
    /// The client position string.
    #[get(type(copy))]
    pub(crate) mouse_pos: Signal<String>,
    /// The screen position string.
    #[get(type(copy))]
    pub(crate) mouse_screen_pos: Signal<String>,
    /// The mouse button name.
    #[get(type(copy))]
    pub(crate) mouse_button: Signal<String>,
    /// The mouse buttons bitmask string.
    #[get(type(copy))]
    pub(crate) mouse_buttons: Signal<String>,
    /// The mouse enter count.
    #[get(type(copy))]
    pub(crate) mouse_enter_count: Signal<i32>,
    /// The mouse leave count.
    #[get(type(copy))]
    pub(crate) mouse_leave_count: Signal<i32>,
    /// The mouse over count.
    #[get(type(copy))]
    pub(crate) mouse_over_count: Signal<i32>,
    /// The mouse out count.
    #[get(type(copy))]
    pub(crate) mouse_out_count: Signal<i32>,
    /// The mouse down count.
    #[get(type(copy))]
    pub(crate) mouse_down_count: Signal<i32>,
    /// The mouse up count.
    #[get(type(copy))]
    pub(crate) mouse_up_count: Signal<i32>,
}

/// Reactive state for the focus events demo.
#[derive(Clone, Copy, Data)]
pub(crate) struct UseFocusEvent {
    /// The current focus status string.
    #[get(type(copy))]
    pub(crate) focus_status: Signal<String>,
    /// The focus-in count.
    #[get(type(copy))]
    pub(crate) focus_in_count: Signal<i32>,
    /// The focus-out count.
    #[get(type(copy))]
    pub(crate) focus_out_count: Signal<i32>,
}

/// Reactive state for the drag events demo.
#[derive(Clone, Copy, Data)]
pub(crate) struct UseDragEvent {
    /// The current drag status string.
    #[get(type(copy))]
    pub(crate) drag_status: Signal<String>,
    /// The drag position string.
    #[get(type(copy))]
    pub(crate) drag_pos: Signal<String>,
    /// The drag data types string.
    #[get(type(copy))]
    pub(crate) drag_types: Signal<String>,
    /// The counter used to track whether the cursor is inside the drop zone.
    ///
    /// Incremented on `dragenter`, decremented on `dragleave`.
    /// Non-zero means the cursor is inside the zone, avoiding false
    /// triggers when moving between child elements.
    #[get(type(copy))]
    pub(crate) drag_enter_counter: Signal<i32>,
    /// The pending drag position to be flushed on the next animation frame.
    #[get(type(copy))]
    pub(crate) drag_pending_pos: Signal<String>,
    /// The request-animation-frame ID used for throttling drag position updates.
    #[get(type(copy))]
    pub(crate) drag_raf_id: Signal<i32>,
}

/// Reactive state for the wheel event demo.
#[derive(Clone, Copy, Data)]
pub(crate) struct UseWheelEvent {
    /// The wheel delta string.
    #[get(type(copy))]
    pub(crate) wheel_delta: Signal<String>,
    /// The accumulated vertical scroll total.
    #[get(type(copy))]
    pub(crate) wheel_total: Signal<f64>,
}

/// Reactive state for the clipboard events demo.
#[derive(Clone, Copy, Data)]
pub(crate) struct UseClipboardEvent {
    /// The clipboard data content.
    #[get(type(copy))]
    pub(crate) clipboard_data: Signal<String>,
    /// The clipboard event type name.
    #[get(type(copy))]
    pub(crate) clipboard_event_type: Signal<String>,
}

/// Reactive state for the touch events demo.
#[derive(Clone, Copy, Data)]
pub(crate) struct UseTouchEvent {
    /// The touch information string.
    #[get(type(copy))]
    pub(crate) touch_info: Signal<String>,
}

/// Reactive state for the form events demo.
#[derive(Clone, Copy, Data)]
pub(crate) struct UseFormEvent {
    /// The input value from oninput.
    #[get(type(copy))]
    pub(crate) euv_input_value: Signal<String>,
    /// The change value from onchange.
    #[get(type(copy))]
    pub(crate) form_change_value: Signal<String>,
    /// The checkbox checked state.
    #[get(type(copy))]
    pub(crate) form_checkbox: Signal<bool>,
    /// The select value from onchange.
    #[get(type(copy))]
    pub(crate) form_select_value: Signal<String>,
    /// The form submit count.
    #[get(type(copy))]
    pub(crate) submit_count: Signal<i32>,
}

/// Reactive state for the audio media events demo.
#[derive(Clone, Copy, Data)]
pub(crate) struct UseMediaEvent {
    /// The media playback status.
    #[get(type(copy))]
    pub(crate) media_status: Signal<String>,
    /// The last media event log.
    #[get(type(copy))]
    pub(crate) media_event_log: Signal<String>,
}

/// Reactive state for the video events demo.
#[derive(Clone, Copy, Data)]
pub(crate) struct UseVideoEvent {
    /// The video playback status.
    #[get(type(copy))]
    pub(crate) video_status: Signal<String>,
    /// The last video event log.
    #[get(type(copy))]
    pub(crate) video_event_log: Signal<String>,
    /// The current playback time string.
    #[get(type(copy))]
    pub(crate) video_current_time: Signal<String>,
    /// The video duration string.
    #[get(type(copy))]
    pub(crate) video_duration: Signal<String>,
    /// The buffered percentage string.
    #[get(type(copy))]
    pub(crate) video_buffered: Signal<String>,
    /// The playback rate string.
    #[get(type(copy))]
    pub(crate) video_playback_rate: Signal<String>,
}

/// Reactive state for the image events demo.
#[derive(Clone, Copy, Data)]
pub(crate) struct UseImageEvent {
    /// The image load status.
    #[get(type(copy))]
    pub(crate) image_status: Signal<String>,
    /// The last image event log.
    #[get(type(copy))]
    pub(crate) image_event_log: Signal<String>,
    /// The image natural size string.
    #[get(type(copy))]
    pub(crate) image_natural_size: Signal<String>,
}
