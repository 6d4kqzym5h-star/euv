use crate::*;

/// A wrapper around an event callback.
///
/// Stores the event name and a reference-counted mutable closure.
#[derive(CustomDebug, Data)]
pub struct NativeEventHandler {
    /// The name of the event (e.g., "click", "input").
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) event_name: String,
    /// The callback function to invoke when the event fires.
    #[debug(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) callback: Rc<RefCell<dyn FnMut(NativeEvent)>>,
}

/// Data associated with a mouse event.
///
/// Captures coordinates, buttons, and modifier key states.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct NativeMouseEvent {
    /// The X coordinate relative to the viewport.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) client_x: i32,
    /// The Y coordinate relative to the viewport.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) client_y: i32,
    /// The X coordinate relative to the screen.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) screen_x: i32,
    /// The Y coordinate relative to the screen.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) screen_y: i32,
    /// Which mouse button was pressed.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) button: i16,
    /// Bitmask of pressed buttons.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) buttons: u16,
    /// Whether the ctrl key was pressed.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) ctrl_key: bool,
    /// Whether the shift key was pressed.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) shift_key: bool,
    /// Whether the alt key was pressed.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) alt_key: bool,
    /// Whether the meta key was pressed.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) meta_key: bool,
}

/// Data associated with an input event.
///
/// Contains the current value and the type of input change.
#[derive(Clone, Data, Debug, Default, Eq, New, PartialEq)]
pub struct NativeInputEvent {
    /// The current value of the input element.
    #[get(pub)]
    #[set(pub)]
    value: String,
    /// The type of input (e.g., "insertText", "deleteContentBackward").
    #[get(pub)]
    #[set(pub)]
    input_type: String,
}

/// Data associated with a keyboard event.
///
/// Captures the pressed key, physical code, location, and modifier states.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct NativeKeyboardEvent {
    /// The key that was pressed.
    #[get(pub)]
    #[set(pub)]
    pub(crate) key: String,
    /// The numeric code of the key.
    #[get(pub)]
    #[set(pub)]
    pub(crate) code: String,
    /// The physical key location.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) location: u32,
    /// Whether the ctrl key was pressed.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) ctrl_key: bool,
    /// Whether the shift key was pressed.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) shift_key: bool,
    /// Whether the alt key was pressed.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) alt_key: bool,
    /// Whether the meta key was pressed.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) meta_key: bool,
    /// Whether the key is being held down.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) repeat: bool,
}

/// Data associated with a focus event.
///
/// Indicates whether the element is gaining or losing focus.
#[derive(Clone, Data, Debug, Default, Eq, New, PartialEq)]
pub struct NativeFocusEvent {
    /// Whether the element is receiving focus.
    #[get(pub, type(copy))]
    #[set(pub)]
    is_focus: bool,
    /// Whether the element is losing focus.
    #[get(pub, type(copy))]
    #[set(pub)]
    is_blur: bool,
}

/// Data associated with a form submit event.
///
/// Identifies the element that triggered the submission.
#[derive(Clone, Data, Debug, Default, Eq, New, PartialEq)]
pub struct NativeSubmitEvent {
    /// The submitter element identifier.
    #[get(pub)]
    #[set(pub)]
    submitter: Option<String>,
}

/// Data associated with a change event.
///
/// Contains the new value and checked state for form controls.
#[derive(Clone, Data, Debug, Default, Eq, New, PartialEq)]
pub struct NativeChangeEvent {
    /// The new value after the change.
    #[get(pub)]
    #[set(pub)]
    value: String,
    /// Whether the element is checked (for checkboxes/radios).
    #[get(pub, type(copy))]
    #[set(pub)]
    checked: bool,
}

/// Data associated with a drag event.
///
/// Captures the drag position and available data transfer types.
#[derive(Clone, Data, Debug, Default, Eq, New, PartialEq)]
pub struct NativeDragEvent {
    /// The X coordinate of the drag.
    #[get(pub, type(copy))]
    #[set(pub)]
    client_x: i32,
    /// The Y coordinate of the drag.
    #[get(pub, type(copy))]
    #[set(pub)]
    client_y: i32,
    /// The data transfer types available.
    #[get(pub)]
    #[set(pub)]
    types: Vec<String>,
}

/// Data associated with a touch event.
///
/// Captures the number of touch points and the first touch coordinates.
#[derive(Clone, Data, Debug, Default, Eq, New, PartialEq)]
pub struct NativeTouchEvent {
    /// The number of touch points.
    #[get(pub, type(copy))]
    #[set(pub)]
    touches_count: u32,
    /// The X coordinate of the first touch.
    #[get(pub, type(copy))]
    #[set(pub)]
    client_x: i32,
    /// The Y coordinate of the first touch.
    #[get(pub, type(copy))]
    #[set(pub)]
    client_y: i32,
}

/// Data associated with a wheel event.
///
/// Captures scroll deltas and the delta mode.
#[derive(Clone, Data, Debug, Default, New, PartialEq)]
pub struct NativeWheelEvent {
    /// Horizontal scroll delta.
    #[get(pub, type(copy))]
    #[set(pub)]
    delta_x: f64,
    /// Vertical scroll delta.
    #[get(pub, type(copy))]
    #[set(pub)]
    delta_y: f64,
    /// Scroll delta mode.
    #[get(pub, type(copy))]
    #[set(pub)]
    delta_mode: u32,
}

/// Data associated with a clipboard event.
///
/// Contains the clipboard text data if available.
#[derive(Clone, Data, Debug, Default, Eq, New, PartialEq)]
pub struct NativeClipboardEvent {
    /// The clipboard data if available.
    #[get(pub)]
    #[set(pub)]
    data: Option<String>,
}

/// Data associated with a media event.
///
/// Identifies the type of media event that occurred.
#[derive(Clone, Data, Debug, Default, Eq, New, PartialEq)]
pub struct NativeMediaEvent {
    /// The type of media event (e.g., "play", "pause", "ended").
    #[get(pub)]
    #[set(pub)]
    event_type: String,
}
