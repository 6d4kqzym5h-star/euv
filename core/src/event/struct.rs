use crate::*;

/// A wrapper around an event callback.
///
/// Stores the event name and a reference-counted mutable closure.
#[derive(Data)]
pub struct NativeEventHandler {
    /// The name of the event (e.g., "click", "input").
    #[set(pub(crate))]
    pub(crate) event_name: String,
    /// The callback function to invoke when the event fires.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) callback: Rc<RefCell<dyn FnMut(NativeEvent)>>,
}

/// Data associated with a mouse event.
///
/// Captures coordinates, buttons, and modifier key states.
#[derive(Data, Default)]
pub struct NativeMouseEvent {
    /// The X coordinate relative to the viewport.
    pub(crate) client_x: i32,
    /// The Y coordinate relative to the viewport.
    pub(crate) client_y: i32,
    /// The X coordinate relative to the screen.
    pub(crate) screen_x: i32,
    /// The Y coordinate relative to the screen.
    pub(crate) screen_y: i32,
    /// Which mouse button was pressed.
    pub(crate) button: i16,
    /// Bitmask of pressed buttons.
    pub(crate) buttons: u16,
    /// Whether the ctrl key was pressed.
    pub(crate) ctrl_key: bool,
    /// Whether the shift key was pressed.
    pub(crate) shift_key: bool,
    /// Whether the alt key was pressed.
    pub(crate) alt_key: bool,
    /// Whether the meta key was pressed.
    pub(crate) meta_key: bool,
}

/// Data associated with an input event.
///
/// Contains the current value and the type of input change.
#[derive(Data, Default)]
pub struct NativeInputEvent {
    /// The current value of the input element.
    value: String,
    /// The type of input (e.g., "insertText", "deleteContentBackward").
    input_type: String,
}

/// Data associated with a keyboard event.
///
/// Captures the pressed key, physical code, location, and modifier states.
#[derive(Data, Default)]
pub struct NativeKeyboardEvent {
    /// The key that was pressed.
    pub(crate) key: String,
    /// The numeric code of the key.
    pub(crate) code: String,
    /// The physical key location.
    pub(crate) location: u32,
    /// Whether the ctrl key was pressed.
    pub(crate) ctrl_key: bool,
    /// Whether the shift key was pressed.
    pub(crate) shift_key: bool,
    /// Whether the alt key was pressed.
    pub(crate) alt_key: bool,
    /// Whether the meta key was pressed.
    pub(crate) meta_key: bool,
    /// Whether the key is being held down.
    pub(crate) repeat: bool,
}

/// Data associated with a focus event.
///
/// Indicates whether the element is gaining or losing focus.
#[derive(Data, Default)]
pub struct NativeFocusEvent {
    /// Whether the element is receiving focus.
    is_focus: bool,
    /// Whether the element is losing focus.
    is_blur: bool,
}

/// Data associated with a form submit event.
///
/// Identifies the element that triggered the submission.
#[derive(Data, Default)]
pub struct NativeSubmitEvent {
    /// The submitter element identifier.
    submitter: Option<String>,
}

/// Data associated with a change event.
///
/// Contains the new value and checked state for form controls.
#[derive(Data, Default)]
pub struct NativeChangeEvent {
    /// The new value after the change.
    value: String,
    /// Whether the element is checked (for checkboxes/radios).
    checked: bool,
}

/// Data associated with a drag event.
///
/// Captures the drag position and available data transfer types.
#[derive(Data, Default)]
pub struct NativeDragEvent {
    /// The X coordinate of the drag.
    client_x: i32,
    /// The Y coordinate of the drag.
    client_y: i32,
    /// The data transfer types available.
    types: Vec<String>,
}

/// Data associated with a touch event.
///
/// Captures the number of touch points and the first touch coordinates.
#[derive(Data, Default)]
pub struct NativeTouchEvent {
    /// The number of touch points.
    touches_count: u32,
    /// The X coordinate of the first touch.
    client_x: i32,
    /// The Y coordinate of the first touch.
    client_y: i32,
}

/// Data associated with a wheel event.
///
/// Captures scroll deltas and the delta mode.
#[derive(Data, Default)]
pub struct NativeWheelEvent {
    /// Horizontal scroll delta.
    delta_x: f64,
    /// Vertical scroll delta.
    delta_y: f64,
    /// Scroll delta mode.
    delta_mode: u32,
}

/// Data associated with a clipboard event.
///
/// Contains the clipboard text data if available.
#[derive(Data, Default)]
pub struct NativeClipboardEvent {
    /// The clipboard data if available.
    data: Option<String>,
}

/// Data associated with a media event.
///
/// Identifies the type of media event that occurred.
#[derive(Data, Default)]
pub struct NativeMediaEvent {
    /// The type of media event (e.g., "play", "pause", "ended").
    event_type: String,
}
