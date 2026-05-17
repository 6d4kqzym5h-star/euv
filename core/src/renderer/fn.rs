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
///
/// # Arguments
///
/// - `Event` - The event containing the target element.
///
/// # Returns
///
/// - `String` - The current value of the input, textarea, or select element.
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
///
/// # Arguments
///
/// - `&Event` - The change event containing the target element.
///
/// # Returns
///
/// - `(String, bool)` - A tuple of the element value and its checked state.
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

/// Mounts the given virtual DOM tree to the document body.
///
/// # Arguments
///
/// - `FnOnce() -> VirtualNode + 'static` - A closure that returns the virtual DOM tree to render.
///
/// # Panics
///
/// Panics if the document body cannot be found.
pub fn mount_body<F>(render_fn: F)
where
    F: FnOnce() -> VirtualNode,
{
    mount("body", render_fn);
}

/// Mounts the given virtual DOM tree to a specific element matched by a CSS selector.
///
/// Supported selector syntax:
/// - `"#id"` — select by element ID
/// - `".class"` — select by class name (uses the first match)
/// - `"tag"` — select by tag name (uses the first match)
///
/// # Arguments
///
/// - `&str` - A CSS selector string to locate the target element.
/// - `FnOnce() -> VirtualNode + 'static` - A closure that returns the virtual DOM tree to render.
///
/// # Panics
///
/// Panics if no global `window` or `document` exists, or if the selector does not match any element.
pub fn mount<F>(selector: &str, render_fn: F)
where
    F: FnOnce() -> VirtualNode,
{
    let window: Window = web_sys::window().expect("no global window exists");
    let document: Document = window.document().expect("should have a document");
    let target: Element = if selector == "body" {
        document.body().expect("document should have a body").into()
    } else if let Some(id) = selector.strip_prefix('#') {
        document
            .get_element_by_id(id)
            .unwrap_or_else(|| panic!("no element found with id '{}'", id))
    } else if let Some(class) = selector.strip_prefix('.') {
        document
            .get_elements_by_class_name(class)
            .item(0)
            .unwrap_or_else(|| panic!("no element found with class '{}'", class))
    } else {
        document
            .get_elements_by_tag_name(selector)
            .item(0)
            .unwrap_or_else(|| panic!("no element found with tag '{}'", selector))
    };
    let mut renderer: Renderer = Renderer::new(target);
    let vnode: VirtualNode = render_fn();
    renderer.render(vnode);
}

/// Returns a mutable reference to the global handler registry.
///
/// Lazily initializes the registry on first access via `Box::leak`.
/// The allocated memory lives for the remainder of the program.
///
/// # Returns
///
/// - `&'static mut HashMap<(usize, String), HandlerEntry>` - A mutable reference to the global handler registry.
///
/// # Panics
///
/// Panics if the registry pointer is invalid after lazy initialization.
pub(crate) fn get_handler_registry() -> &'static mut HashMap<(usize, String), HandlerEntry> {
    unsafe {
        if HANDLER_REGISTRY.is_null() {
            let registry: Box<HashMap<(usize, String), HandlerEntry>> = Box::default();
            HANDLER_REGISTRY = Box::leak(registry) as *mut HashMap<(usize, String), HandlerEntry>;
        }
        &mut *HANDLER_REGISTRY
    }
}

/// Returns a mutable reference to the global DynamicNode listener registry.
///
/// Lazily initializes the registry on first access via `Box::leak`.
/// Maps `data-euv-dynamic-id` values to the `JsValue` reference of the
/// corresponding `__euv_signal_update__` event listener closure.
///
/// # Returns
///
/// - `&'static mut HashMap<usize, JsValue>` - A mutable reference to the global DynamicNode listener registry.
#[cfg(target_arch = "wasm32")]
pub(crate) fn get_dynamic_listener_registry() -> &'static mut HashMap<usize, JsValue> {
    unsafe {
        if DYNAMIC_LISTENER_REGISTRY.is_null() {
            let registry: Box<HashMap<usize, JsValue>> = Box::default();
            DYNAMIC_LISTENER_REGISTRY = Box::leak(registry) as *mut HashMap<usize, JsValue>;
        }
        &mut *DYNAMIC_LISTENER_REGISTRY
    }
}

/// Returns a mutable reference to the global attribute signal listener registry.
///
/// Lazily initializes the registry on first access via `Box::leak`.
/// Maps `Signal<String>` inner pointer addresses to the `JsValue` reference
/// of the corresponding `__euv_signal_update__` event listener closure.
///
/// # Returns
///
/// - `&'static mut HashMap<usize, JsValue>` - A mutable reference to the global attribute signal listener registry.
#[cfg(target_arch = "wasm32")]
pub(crate) fn get_attr_signal_listener_registry() -> &'static mut HashMap<usize, JsValue> {
    unsafe {
        if ATTR_SIGNAL_LISTENER_REGISTRY.is_null() {
            let registry: Box<HashMap<usize, JsValue>> = Box::default();
            ATTR_SIGNAL_LISTENER_REGISTRY = Box::leak(registry) as *mut HashMap<usize, JsValue>;
        }
        &mut *ATTR_SIGNAL_LISTENER_REGISTRY
    }
}

/// Registers a `__euv_signal_update__` event listener for a DynamicNode placeholder.
///
/// If a previous listener was registered for the same `dynamic_id`, it is removed
/// from the window before the new one is added. This prevents listener accumulation
/// when match arms switch during route changes.
///
/// # Arguments
///
/// - `usize` - The `data-euv-dynamic-id` value identifying the placeholder element.
/// - `Closure<dyn FnMut()>` - The re-render closure to register as the event listener.
#[cfg(target_arch = "wasm32")]
pub(crate) fn register_dynamic_listener(dynamic_id: usize, closure: Closure<dyn FnMut()>) {
    let event_name: String = NativeEventName::EuvSignalUpdate.to_string();
    let registry: &mut HashMap<usize, JsValue> = get_dynamic_listener_registry();
    if let Some(old_js_value) = registry.remove(&dynamic_id) {
        let window: Window = window().unwrap();
        let _ =
            window.remove_event_listener_with_callback(&event_name, old_js_value.unchecked_ref());
    }
    let js_value: JsValue = closure.as_ref().clone();
    let window: Window = window().unwrap();
    window
        .add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
    registry.insert(dynamic_id, js_value);
}

/// Registers a `__euv_signal_update__` event listener for an attribute signal.
///
/// If a previous listener was registered for the same signal pointer, it is removed
/// from the window before the new one is added. This prevents listener accumulation
/// when the same signal slot is reused across match arm switches.
///
/// # Arguments
///
/// - `usize` - The inner pointer address of the `Signal<String>`.
/// - `Closure<dyn FnMut()>` - The attribute recomputation closure to register.
#[cfg(target_arch = "wasm32")]
pub(crate) fn register_attr_signal_listener(signal_key: usize, closure: Closure<dyn FnMut()>) {
    let event_name: String = NativeEventName::EuvSignalUpdate.to_string();
    let registry: &mut HashMap<usize, JsValue> = get_attr_signal_listener_registry();
    if let Some(old_js_value) = registry.remove(&signal_key) {
        let window: Window = window().unwrap();
        let _ =
            window.remove_event_listener_with_callback(&event_name, old_js_value.unchecked_ref());
    }
    let js_value: JsValue = closure.as_ref().clone();
    let window: Window = window().unwrap();
    window
        .add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
    registry.insert(signal_key, js_value);
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn register_dynamic_listener(_dynamic_id: usize, _closure: Closure<dyn FnMut()>) {}

#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub(crate) fn register_attr_signal_listener(_signal_key: usize, _closure: Closure<dyn FnMut()>) {}
