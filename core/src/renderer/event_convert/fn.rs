use crate::*;

/// Converts a web_sys event into a euv event.
///
/// # Arguments
///
/// - `&Event` - The raw browser event.
/// - `&str` - The name of the event for dispatching to the correct variant.
///
/// # Returns
///
/// - `NativeEvent` - The corresponding euv event variant.
pub(crate) fn convert_web_event(event: &Event, event_name: &str) -> NativeEvent {
    match event_name {
        "click" | "mousedown" | "mouseup" | "mousemove" | "mouseenter" | "mouseleave"
        | "mouseover" | "mouseout" | "dblclick" | "contextmenu" => {
            if let Some(mouse_event) = event.dyn_ref::<MouseEvent>() {
                NativeEvent::Mouse(NativeMouseEvent {
                    client_x: mouse_event.client_x(),
                    client_y: mouse_event.client_y(),
                    screen_x: mouse_event.screen_x(),
                    screen_y: mouse_event.screen_y(),
                    button: mouse_event.button(),
                    buttons: mouse_event.buttons(),
                    ctrl_key: mouse_event.ctrl_key(),
                    shift_key: mouse_event.shift_key(),
                    alt_key: mouse_event.alt_key(),
                    meta_key: mouse_event.meta_key(),
                })
            } else {
                NativeEvent::Generic
            }
        }
        "input" => {
            if let Some(input_event) = event.dyn_ref::<InputEvent>() {
                let value: String = get_input_value(event);
                NativeEvent::Input(NativeInputEvent::new(value, input_event.input_type()))
            } else {
                NativeEvent::Input(NativeInputEvent::new(get_input_value(event), String::new()))
            }
        }
        "keydown" | "keyup" | "keypress" => {
            if let Some(key_event) = event.dyn_ref::<KeyboardEvent>() {
                NativeEvent::Keyboard(NativeKeyboardEvent {
                    key: key_event.key(),
                    code: key_event.code(),
                    location: key_event.location(),
                    ctrl_key: key_event.ctrl_key(),
                    shift_key: key_event.shift_key(),
                    alt_key: key_event.alt_key(),
                    meta_key: key_event.meta_key(),
                    repeat: key_event.repeat(),
                })
            } else {
                NativeEvent::Generic
            }
        }
        "focus" | "blur" | "focusin" | "focusout" => {
            let is_focus: bool = event_name == "focus" || event_name == "focusin";
            NativeEvent::Focus(NativeFocusEvent::new(is_focus, !is_focus))
        }
        "submit" => {
            if let Some(submit_event) = event.dyn_ref::<SubmitEvent>() {
                let submitter: Option<String> = submit_event
                    .submitter()
                    .and_then(|s| s.dyn_into::<HtmlElement>().ok())
                    .map(|el| el.id());
                NativeEvent::Submit(NativeSubmitEvent::new(submitter))
            } else {
                NativeEvent::Generic
            }
        }
        "change" => {
            let (value, checked) = get_change_value(event);
            NativeEvent::Change(NativeChangeEvent::new(value, checked))
        }
        "drag" | "dragstart" | "dragend" | "dragover" | "dragenter" | "dragleave" | "drop" => {
            if let Some(drag_event) = event.dyn_ref::<DragEvent>() {
                let types: Vec<String> = drag_event
                    .data_transfer()
                    .map(|dt| {
                        let len: u32 = dt.types().length();
                        (0..len)
                            .filter_map(|i: u32| dt.types().get(i).as_string())
                            .collect()
                    })
                    .unwrap_or_default();
                NativeEvent::Drag(NativeDragEvent::new(
                    drag_event.client_x(),
                    drag_event.client_y(),
                    types,
                ))
            } else {
                NativeEvent::Generic
            }
        }
        "touchstart" | "touchend" | "touchmove" | "touchcancel" => {
            if let Some(touch_event) = event.dyn_ref::<TouchEvent>() {
                let touches: TouchList = touch_event.touches();
                let first: Option<Touch> = touches.get(0);
                NativeEvent::Touch(NativeTouchEvent::new(
                    touches.length(),
                    first.as_ref().map(|t| t.client_x()).unwrap_or(0),
                    first.as_ref().map(|t| t.client_y()).unwrap_or(0),
                ))
            } else {
                NativeEvent::Generic
            }
        }
        "wheel" => {
            if let Some(wheel_event) = event.dyn_ref::<WheelEvent>() {
                NativeEvent::Wheel(NativeWheelEvent::new(
                    wheel_event.delta_x(),
                    wheel_event.delta_y(),
                    wheel_event.delta_mode(),
                ))
            } else {
                NativeEvent::Generic
            }
        }
        "copy" | "cut" | "paste" => {
            if let Some(clipboard_event) = event.dyn_ref::<ClipboardEvent>() {
                let data: Option<String> = clipboard_event
                    .clipboard_data()
                    .and_then(|cd| cd.get_data("text").ok());
                NativeEvent::Clipboard(NativeClipboardEvent::new(data))
            } else {
                NativeEvent::Generic
            }
        }
        "play" | "pause" | "ended" | "loadeddata" | "canplay" | "volumechange" | "timeupdate" => {
            NativeEvent::Media(NativeMediaEvent::new(event_name.to_string()))
        }
        _ => NativeEvent::Generic,
    }
}

/// Extracts the value from an input-like event target.
fn get_input_value(event: &Event) -> String {
    if let Some(target) = event.target() {
        if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
            return input.value();
        }
        if let Ok(textarea) = target.clone().dyn_into::<HtmlTextAreaElement>() {
            return textarea.value();
        }
        if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
            return select.value();
        }
    }
    String::new()
}

/// Extracts value and checked state from a change event target.
fn get_change_value(event: &Event) -> (String, bool) {
    if let Some(target) = event.target() {
        if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
            return (input.value(), input.checked());
        }
        if let Ok(textarea) = target.clone().dyn_into::<HtmlTextAreaElement>() {
            return (textarea.value(), false);
        }
        if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
            return (select.value(), false);
        }
    }
    (String::new(), false)
}
