use crate::*;

/// Maps each `NativeEventName` variant to its corresponding DOM event string.
impl NativeEventName {
    /// Returns the string representation of this event name for DOM binding.
    pub fn as_str(&self) -> String {
        match self {
            NativeEventName::Click => "click".to_string(),
            NativeEventName::DblClick => "dblclick".to_string(),
            NativeEventName::MouseDown => "mousedown".to_string(),
            NativeEventName::MouseUp => "mouseup".to_string(),
            NativeEventName::MouseMove => "mousemove".to_string(),
            NativeEventName::MouseEnter => "mouseenter".to_string(),
            NativeEventName::MouseLeave => "mouseleave".to_string(),
            NativeEventName::MouseOver => "mouseover".to_string(),
            NativeEventName::MouseOut => "mouseout".to_string(),
            NativeEventName::ContextMenu => "contextmenu".to_string(),
            NativeEventName::Input => "input".to_string(),
            NativeEventName::KeyDown => "keydown".to_string(),
            NativeEventName::KeyUp => "keyup".to_string(),
            NativeEventName::KeyPress => "keypress".to_string(),
            NativeEventName::Focus => "focus".to_string(),
            NativeEventName::Blur => "blur".to_string(),
            NativeEventName::FocusIn => "focusin".to_string(),
            NativeEventName::FocusOut => "focusout".to_string(),
            NativeEventName::Submit => "submit".to_string(),
            NativeEventName::Change => "change".to_string(),
            NativeEventName::Drag => "drag".to_string(),
            NativeEventName::DragStart => "dragstart".to_string(),
            NativeEventName::DragEnd => "dragend".to_string(),
            NativeEventName::DragOver => "dragover".to_string(),
            NativeEventName::DragEnter => "dragenter".to_string(),
            NativeEventName::DragLeave => "dragleave".to_string(),
            NativeEventName::Drop => "drop".to_string(),
            NativeEventName::TouchStart => "touchstart".to_string(),
            NativeEventName::TouchEnd => "touchend".to_string(),
            NativeEventName::TouchMove => "touchmove".to_string(),
            NativeEventName::TouchCancel => "touchcancel".to_string(),
            NativeEventName::Wheel => "wheel".to_string(),
            NativeEventName::Copy => "copy".to_string(),
            NativeEventName::Cut => "cut".to_string(),
            NativeEventName::Paste => "paste".to_string(),
            NativeEventName::Play => "play".to_string(),
            NativeEventName::Pause => "pause".to_string(),
            NativeEventName::Ended => "ended".to_string(),
            NativeEventName::LoadedData => "loadeddata".to_string(),
            NativeEventName::CanPlay => "canplay".to_string(),
            NativeEventName::VolumeChange => "volumechange".to_string(),
            NativeEventName::TimeUpdate => "timeupdate".to_string(),
            NativeEventName::HashChange => "hashchange".to_string(),
            NativeEventName::EuvSignalUpdate => "__euv_signal_update__".to_string(),
            NativeEventName::Other(name) => name.clone(),
        }
    }
}

/// Implements `Display` for `NativeEventName` by delegating to `as_str`.
///
/// This automatically provides the `ToString` trait via blanket implementation.
impl std::fmt::Display for NativeEventName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Implementation of event handler construction, cloning, and invocation.
impl NativeEventHandler {
    /// Creates a new event handler from an `NativeEventName` enum and callback.
    pub fn new<F>(event_name: NativeEventName, callback: F) -> Self
    where
        F: FnMut(NativeEvent) + 'static,
    {
        NativeEventHandler {
            event_name: event_name.as_str(),
            callback: Rc::new(RefCell::new(callback)),
        }
    }

    /// Invokes the underlying callback with the given event.
    pub fn handle(&self, event: NativeEvent) {
        let mut cb = self.get_callback().borrow_mut();
        cb(event);
    }
}

/// Clones the event handler, sharing the underlying callback reference.
impl Clone for NativeEventHandler {
    fn clone(&self) -> Self {
        NativeEventHandler {
            event_name: self.get_event_name().clone(),
            callback: Rc::clone(self.get_callback()),
        }
    }
}

/// Converts an owned event handler into an attribute value.
impl IntoEventAttribute for NativeEventHandler {
    fn into_event_attribute(self) -> AttributeValue {
        AttributeValue::Event(self)
    }
}

/// Converts an optional event handler into an attribute value.
impl IntoEventAttribute for Option<NativeEventHandler> {
    fn into_event_attribute(self) -> AttributeValue {
        match self {
            Some(handler) => AttributeValue::Event(handler),
            None => AttributeValue::Text(String::new()),
        }
    }
}

/// Implementation of NativeInputEvent construction.
impl NativeInputEvent {
    /// Creates a new input event with the given value and input type.
    pub fn new(value: String, input_type: String) -> Self {
        let mut event: NativeInputEvent = NativeInputEvent::default();
        event.set_value(value);
        event.set_input_type(input_type);
        event
    }
}

/// Implementation of NativeFocusEvent construction.
impl NativeFocusEvent {
    /// Creates a new focus event.
    pub fn new(is_focus: bool, is_blur: bool) -> Self {
        let mut event: NativeFocusEvent = NativeFocusEvent::default();
        event.set_is_focus(is_focus);
        event.set_is_blur(is_blur);
        event
    }
}

/// Implementation of NativeSubmitEvent construction.
impl NativeSubmitEvent {
    /// Creates a new submit event with the given submitter.
    pub fn new(submitter: Option<String>) -> Self {
        let mut event: NativeSubmitEvent = NativeSubmitEvent::default();
        event.set_submitter(submitter);
        event
    }
}

/// Implementation of NativeChangeEvent construction.
impl NativeChangeEvent {
    /// Creates a new change event with the given value and checked state.
    pub fn new(value: String, checked: bool) -> Self {
        let mut event: NativeChangeEvent = NativeChangeEvent::default();
        event.set_value(value);
        event.set_checked(checked);
        event
    }
}

/// Implementation of NativeDragEvent construction.
impl NativeDragEvent {
    /// Creates a new drag event with the given parameters.
    pub fn new(client_x: i32, client_y: i32, types: Vec<String>) -> Self {
        let mut event: NativeDragEvent = NativeDragEvent::default();
        event.set_client_x(client_x);
        event.set_client_y(client_y);
        event.set_types(types);
        event
    }
}

/// Implementation of NativeTouchEvent construction.
impl NativeTouchEvent {
    /// Creates a new touch event with the given parameters.
    pub fn new(touches_count: u32, client_x: i32, client_y: i32) -> Self {
        let mut event: NativeTouchEvent = NativeTouchEvent::default();
        event.set_touches_count(touches_count);
        event.set_client_x(client_x);
        event.set_client_y(client_y);
        event
    }
}

/// Implementation of NativeWheelEvent construction.
impl NativeWheelEvent {
    /// Creates a new wheel event with the given parameters.
    pub fn new(delta_x: f64, delta_y: f64, delta_mode: u32) -> Self {
        let mut event: NativeWheelEvent = NativeWheelEvent::default();
        event.set_delta_x(delta_x);
        event.set_delta_y(delta_y);
        event.set_delta_mode(delta_mode);
        event
    }
}

/// Implementation of NativeClipboardEvent construction.
impl NativeClipboardEvent {
    /// Creates a new clipboard event with the given data.
    pub fn new(data: Option<String>) -> Self {
        let mut event: NativeClipboardEvent = NativeClipboardEvent::default();
        event.set_data(data);
        event
    }
}

/// Implementation of NativeMediaEvent construction.
impl NativeMediaEvent {
    /// Creates a new media event with the given event type.
    pub fn new(event_type: String) -> Self {
        let mut event: NativeMediaEvent = NativeMediaEvent::default();
        event.set_event_type(event_type);
        event
    }
}
