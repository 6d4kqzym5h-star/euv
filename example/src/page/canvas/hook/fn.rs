use crate::*;

/// Creates a click event handler that enters fullscreen drawing mode.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click event handler.
pub(crate) fn canvas_on_draw(state: UseCanvas) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        enter_fullscreen(state);
    }))
}

/// Creates the canvas drawing board reactive state signals wrapped
/// in a `UseCanvas` struct.
///
/// Initializes with default stroke color, line width, and an empty
/// snapshot data URL.
///
/// # Returns
///
/// - `UseCanvas` - The canvas drawing board state.
pub(crate) fn use_canvas_state() -> UseCanvas {
    let initial_stroke_color: String = load_stroke_color();
    let initial_line_width: f64 = load_line_width();
    UseCanvas::new(
        use_signal(|| false),
        use_signal(move || initial_stroke_color.clone()),
        use_signal(move || initial_line_width),
        use_signal(|| false),
        use_signal(String::new),
    )
}

/// Captures the current canvas content as a data URL and stores it
/// in the `snapshot_data_url` signal.
///
/// Must be called while the canvas element is still in the DOM.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
pub(crate) fn update_snapshot(state: UseCanvas) {
    let window_value: Window = window().expect("no global window exists");
    let document_value: Document = window_value.document().expect("should have a document");
    let Some(element) = document_value
        .query_selector(CANVAS_DRAWING_SELECTOR)
        .ok()
        .flatten()
    else {
        return;
    };
    let canvas_element: HtmlCanvasElement = element.unchecked_into();
    let data_url: String = canvas_element.to_data_url().unwrap_or_default();
    state.get_snapshot_data_url().set(data_url);
}

/// Begins a new drawing stroke on the canvas at the specified coordinates.
///
/// Sets the `drawing` signal to `true`, retrieves the 2D rendering context,
/// and starts a new path at the given point. Applies the current stroke
/// color and line width from the state signals. The line width is clamped
/// to a minimum of `CANVAS_MIN_LINE_WIDTH` to prevent rendering artifacts.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
/// - `f64` - The x coordinate of the starting point.
/// - `f64` - The y coordinate of the starting point.
pub(crate) fn start_drawing(state: UseCanvas, offset_x: f64, offset_y: f64) {
    state.get_drawing().set(true);
    let window_value: Window = window().expect("no global window exists");
    let document_value: Document = window_value.document().expect("should have a document");
    let Some(element) = document_value
        .query_selector(CANVAS_DRAWING_SELECTOR)
        .ok()
        .flatten()
    else {
        return;
    };
    let canvas_element: HtmlCanvasElement = element.unchecked_into();
    let Some(context_object) = canvas_element.get_context("2d").ok().flatten() else {
        return;
    };
    let context_2d: CanvasRenderingContext2d = context_object.unchecked_into();
    context_2d.begin_path();
    let _ = Reflect::set(
        &context_2d,
        &JsValue::from_str("strokeStyle"),
        &JsValue::from_str(&state.get_stroke_color().get()),
    );
    let line_width: f64 = state.get_line_width().get().max(CANVAS_MIN_LINE_WIDTH);
    context_2d.set_line_width(line_width);
    context_2d.set_line_cap("round");
    context_2d.set_line_join("round");
    context_2d.move_to(offset_x, offset_y);
}

/// Continues the current drawing stroke to the specified coordinates.
///
/// If the `drawing` signal is `true`, draws a line segment from the
/// current path position to the given point and strokes it.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
/// - `f64` - The x coordinate of the destination point.
/// - `f64` - The y coordinate of the destination point.
pub(crate) fn continue_drawing(state: UseCanvas, offset_x: f64, offset_y: f64) {
    if !state.get_drawing().get() {
        return;
    }
    let window_value: Window = window().expect("no global window exists");
    let document_value: Document = window_value.document().expect("should have a document");
    let Some(element) = document_value
        .query_selector(CANVAS_DRAWING_SELECTOR)
        .ok()
        .flatten()
    else {
        return;
    };
    let canvas_element: HtmlCanvasElement = element.unchecked_into();
    let Some(context_object) = canvas_element.get_context("2d").ok().flatten() else {
        return;
    };
    let context_2d: CanvasRenderingContext2d = context_object.unchecked_into();
    context_2d.line_to(offset_x, offset_y);
    context_2d.stroke();
}

/// Ends the current drawing stroke.
///
/// Sets the `drawing` signal to `false` and closes the current path.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
pub(crate) fn stop_drawing(state: UseCanvas) {
    if !state.get_drawing().get() {
        return;
    }
    state.get_drawing().set(false);
    let window_value: Window = window().expect("no global window exists");
    let document_value: Document = window_value.document().expect("should have a document");
    let Some(element) = document_value
        .query_selector(CANVAS_DRAWING_SELECTOR)
        .ok()
        .flatten()
    else {
        return;
    };
    let canvas_element: HtmlCanvasElement = element.unchecked_into();
    let Some(context_object) = canvas_element.get_context("2d").ok().flatten() else {
        return;
    };
    let context_2d: CanvasRenderingContext2d = context_object.unchecked_into();
    context_2d.close_path();
}

/// Clears the entire canvas and refills it with the white background color.
///
/// Retrieves the canvas element, clears the entire drawing area, and
/// applies the background fill. The canvas dimensions are preserved.
///
/// # Arguments
///
/// - `&str` - The CSS selector of the `<canvas>` element to clear.
pub(crate) fn clear_canvas(canvas_selector: &str) {
    let window_value: Window = window().expect("no global window exists");
    let document_value: Document = window_value.document().expect("should have a document");
    let Some(element) = document_value
        .query_selector(canvas_selector)
        .ok()
        .flatten()
    else {
        return;
    };
    let canvas_element: HtmlCanvasElement = element.unchecked_into();
    let width: f64 = canvas_element.width() as f64;
    let height: f64 = canvas_element.height() as f64;
    let Some(context_object) = canvas_element.get_context("2d").ok().flatten() else {
        return;
    };
    let context_2d: CanvasRenderingContext2d = context_object.unchecked_into();
    context_2d.clear_rect(0.0, 0.0, width, height);
    let _ = Reflect::set(
        &context_2d,
        &JsValue::from_str("fillStyle"),
        &JsValue::from_str(CANVAS_BACKGROUND_COLOR),
    );
    context_2d.fill_rect(0.0, 0.0, width, height);
}

/// Extracts the pointer offset coordinates relative to the canvas element
/// from a mouse or touch event.
///
/// Reads `offsetX` and `offsetY` properties from the event via JavaScript
/// reflection. These properties provide coordinates relative to the target
/// element, which is ideal for canvas drawing.
///
/// # Arguments
///
/// - `&Event` - The mouse or touch event.
///
/// # Returns
///
/// - `(f64, f64)` - A tuple containing the `(offset_x, offset_y)` coordinates.
pub(crate) fn get_pointer_offset(event: &Event) -> (f64, f64) {
    let target: JsValue = event
        .target()
        .map_or(JsValue::NULL, |event_target: EventTarget| {
            event_target.into()
        });
    let offset_x: f64 = Reflect::get(&target, &JsValue::from_str("offsetX"))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or(0.0);
    let offset_y: f64 = Reflect::get(&target, &JsValue::from_str("offsetY"))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or(0.0);
    (offset_x, offset_y)
}

/// Extracts the client coordinates from a mouse event.
///
/// Reads `clientX` and `clientY` properties from the event, which
/// represent the coordinates within the viewport. These are used
/// in fullscreen mode where CSS transforms make `offsetX`/`offsetY`
/// unreliable.
///
/// # Arguments
///
/// - `&Event` - The mouse event.
///
/// # Returns
///
/// - `(f64, f64)` - A tuple containing the `(client_x, client_y)` coordinates.
pub(crate) fn get_mouse_client(event: &Event) -> (f64, f64) {
    let client_x: f64 = Reflect::get(event.as_ref(), &JsValue::from_str("clientX"))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or(0.0);
    let client_y: f64 = Reflect::get(event.as_ref(), &JsValue::from_str("clientY"))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or(0.0);
    (client_x, client_y)
}

/// Extracts the first touch point coordinates relative to the canvas element
/// from a touch event.
///
/// Reads the first item from `touches`, then computes `offsetX` and
/// `offsetY` by subtracting the canvas bounding rect from the touch
/// client coordinates.
///
/// # Arguments
///
/// - `&Event` - The touch event.
///
/// # Returns
///
/// - `(f64, f64)` - A tuple containing the `(offset_x, offset_y)` coordinates.
pub(crate) fn get_touch_offset(event: &Event) -> (f64, f64) {
    let target: JsValue = event
        .target()
        .map_or(JsValue::NULL, |event_target: EventTarget| {
            event_target.into()
        });
    let element: Element = target.unchecked_into();
    let rect: DomRect = element.get_bounding_client_rect();
    let touches_value: JsValue = Reflect::get(event.as_ref(), &JsValue::from_str("touches"))
        .ok()
        .unwrap_or(JsValue::NULL);
    let touches: Array = touches_value.unchecked_into();
    if touches.length() == 0 {
        return (0.0, 0.0);
    }
    let first_touch: JsValue = touches.get(0);
    let client_x: f64 = Reflect::get(&first_touch, &JsValue::from_str("clientX"))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or(0.0);
    let client_y: f64 = Reflect::get(&first_touch, &JsValue::from_str("clientY"))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or(0.0);
    let offset_x: f64 = client_x - rect.left();
    let offset_y: f64 = client_y - rect.top();
    (offset_x, offset_y)
}

/// Extracts the first touch point client coordinates from a touch event.
///
/// Reads `clientX` and `clientY` from the first touch, which are
/// used in fullscreen mode for accurate coordinate mapping after
/// CSS transforms.
///
/// # Arguments
///
/// - `&Event` - The touch event.
///
/// # Returns
///
/// - `(f64, f64)` - A tuple containing the `(client_x, client_y)` coordinates.
pub(crate) fn get_touch_client(event: &Event) -> (f64, f64) {
    let touches_value: JsValue = Reflect::get(event.as_ref(), &JsValue::from_str("touches"))
        .ok()
        .unwrap_or(JsValue::NULL);
    let touches: Array = touches_value.unchecked_into();
    if touches.length() == 0 {
        return (0.0, 0.0);
    }
    let first_touch: JsValue = touches.get(0);
    let client_x: f64 = Reflect::get(&first_touch, &JsValue::from_str("clientX"))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or(0.0);
    let client_y: f64 = Reflect::get(&first_touch, &JsValue::from_str("clientY"))
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or(0.0);
    (client_x, client_y)
}

/// Prevents the default browser behavior for the given event.
///
/// Used to prevent scrolling and other default touch behaviors
/// during canvas drawing.
///
/// # Arguments
///
/// - `&Event` - The event to prevent default on.
pub(crate) fn prevent_event_default(event: &Event) {
    event.prevent_default();
}

/// Maps screen coordinates to canvas-internal coordinates.
///
/// In fullscreen mode, calculates the position relative to the canvas
/// bounding rectangle using `clientX`/`clientY`. In normal mode,
/// returns `offsetX`/`offsetY` directly.
///
/// # Arguments
///
/// - `f64` - The `offsetX` value from the event (used in normal mode).
/// - `f64` - The `offsetY` value from the event (used in normal mode).
/// - `f64` - The `clientX` value from the event (used in fullscreen mode).
/// - `f64` - The `clientY` value from the event (used in fullscreen mode).
/// - `bool` - Whether the canvas is in fullscreen mode.
///
/// # Returns
///
/// - `(f64, f64)` - The mapped `(x, y)` coordinates in canvas space.
pub(crate) fn map_rotated_offset(
    offset_x: f64,
    offset_y: f64,
    client_x: f64,
    client_y: f64,
    is_fullscreen: bool,
) -> (f64, f64) {
    if !is_fullscreen {
        return (offset_x, offset_y);
    }
    let window_value: Window = window().expect("no global window exists");
    let document_value: Document = window_value.document().expect("should have a document");
    let Some(element) = document_value
        .query_selector(CANVAS_DRAWING_SELECTOR)
        .ok()
        .flatten()
    else {
        return (offset_x, offset_y);
    };
    let canvas_element: HtmlCanvasElement = element.unchecked_into();
    let rect: DomRect = canvas_element.get_bounding_client_rect();
    let canvas_x: f64 = client_x - rect.left();
    let canvas_y: f64 = client_y - rect.top();
    (canvas_x, canvas_y)
}

/// Enters CSS fullscreen mode by setting the `fullscreen` signal to
/// `true` and pushing a browser history entry via `history.pushState`
/// so that the system back button will exit fullscreen instead of
/// navigating to the previous page.
///
/// After the DOM updates, the canvas is resized and initialized with
/// the content from `snapshot_data_url` (if any).
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
pub(crate) fn enter_fullscreen(state: UseCanvas) {
    state.get_fullscreen().set(true);
    let window_value: Window = window().expect("no global window exists");
    let history: History = window_value.history().expect("no history object exists");
    let _ = history.push_state(&JsValue::NULL, "");
    let snapshot_data_url: String = state.get_snapshot_data_url().get();
    let resize_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        resize_fullscreen_canvas(&snapshot_data_url);
    }));
    let _ = window_value.request_animation_frame(resize_closure.as_ref().unchecked_ref());
    resize_closure.forget();
}

/// Calculates and applies the optimal pixel dimensions for the canvas
/// in fullscreen mode, then draws the snapshot content directly.
///
/// The canvas pixel buffer and CSS display size are both set to 9:16
/// (portrait) so that the canvas visually fills the portrait drawing
/// area without any CSS rotation. Drawing coordinates are in portrait
/// space. Since the snapshot shares the same portrait orientation,
/// it is drawn directly without any rotation transform.
///
/// # Arguments
///
/// - `&str` - The snapshot data URL to draw onto the canvas.
pub(crate) fn resize_fullscreen_canvas(snapshot_data_url: &str) {
    let window_value: Window = window().expect("no global window exists");
    let document_value: Document = window_value.document().expect("should have a document");
    let Some(wrapper_element) = document_value
        .query_selector(CANVAS_FULLSCREEN_WRAPPER_SELECTOR)
        .ok()
        .flatten()
    else {
        return;
    };
    let Some(canvas_element_obj) = document_value
        .query_selector(CANVAS_DRAWING_SELECTOR)
        .ok()
        .flatten()
    else {
        return;
    };
    let canvas_element: HtmlCanvasElement = canvas_element_obj.unchecked_into();
    let wrapper_width: i32 = wrapper_element.client_width();
    let wrapper_height: i32 = wrapper_element.client_height();
    let canvas_width: f64 = if (wrapper_height as f64) < (wrapper_width as f64) * 16.0 / 9.0 {
        wrapper_height as f64 * 9.0 / 16.0
    } else {
        wrapper_width as f64
    };
    let canvas_height: f64 = canvas_width * 16.0 / 9.0;
    canvas_element
        .style()
        .set_property("width", &format!("{}px", canvas_width as i32))
        .unwrap_or(());
    canvas_element
        .style()
        .set_property("height", &format!("{}px", canvas_height as i32))
        .unwrap_or(());
    canvas_element.set_width(canvas_width as u32);
    canvas_element.set_height(canvas_height as u32);
    let Some(context_object) = canvas_element.get_context("2d").ok().flatten() else {
        return;
    };
    let context_2d: CanvasRenderingContext2d = context_object.unchecked_into();
    let _ = Reflect::set(
        &context_2d,
        &JsValue::from_str("fillStyle"),
        &JsValue::from_str(CANVAS_BACKGROUND_COLOR),
    );
    context_2d.fill_rect(0.0, 0.0, canvas_width, canvas_height);
    if snapshot_data_url.is_empty() {
        return;
    }
    let image: HtmlImageElement = HtmlImageElement::new().expect("should create image element");
    image.set_src(snapshot_data_url);
    let draw_image: HtmlImageElement = image.clone();
    let draw_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let _ = context_2d.draw_image_with_html_image_element_and_dw_and_dh(
            &draw_image,
            0.0,
            0.0,
            canvas_width,
            canvas_height,
        );
    }));
    image.set_onload(Some(draw_closure.as_ref().unchecked_ref()));
    draw_closure.forget();
}

/// Exits CSS fullscreen mode by capturing the canvas content as a
/// snapshot data URL, then setting the `fullscreen` signal to `false`
/// and consuming the browser history entry via `history.back()`.
///
/// The canvas content is captured before the fullscreen signal changes
/// so the DOM element is still available for data extraction.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
pub(crate) fn exit_fullscreen(state: UseCanvas) {
    update_snapshot(state);
    state.get_fullscreen().set(false);
    let window_value: Window = window().expect("no global window exists");
    let history: History = window_value.history().expect("no history object exists");
    let _ = history.back();
}

/// Exits CSS fullscreen mode without consuming a browser history entry.
///
/// Used when the exit is triggered by a `popstate` event (system back
/// button), because the `popstate` itself has already consumed the
/// `pushState` entry that was created when entering fullscreen.
/// Calling `history.back()` again would incorrectly consume an extra
/// history entry.
///
/// The canvas content is captured before the fullscreen signal changes
/// so the DOM element is still available for data extraction.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
pub(crate) fn exit_fullscreen_from_popstate(state: UseCanvas) {
    update_snapshot(state);
    state.get_fullscreen().set(false);
}

/// Subscribes to browser `popstate` events to handle the system back
/// button while the canvas is in fullscreen mode.
///
/// When the user presses the system back button while in fullscreen,
/// this handler exits fullscreen instead of allowing the browser to
/// navigate to the previous page. The `popstate` event is triggered
/// by the `pushState` entry that was created when entering fullscreen.
/// Uses `exit_fullscreen_from_popstate` to avoid double-consuming
/// history entries.
///
/// # Arguments
///
/// - `UseCanvas` - The canvas drawing board state.
///   Loads the persisted stroke color from localStorage.
///
/// Returns the stored color string if available and non-empty,
/// otherwise returns the default stroke color.
///
/// # Returns
///
/// - `String` - The persisted or default stroke color.
pub(crate) fn load_stroke_color() -> String {
    local_storage_get(CANVAS_STORAGE_KEY_STROKE_COLOR)
        .filter(|color: &String| !color.is_empty())
        .unwrap_or_else(|| CANVAS_DEFAULT_STROKE_COLOR.to_string())
}

/// Loads the persisted line width from localStorage.
///
/// Returns the stored line width if available and parseable,
/// otherwise returns the default line width.
///
/// # Returns
///
/// - `f64` - The persisted or default line width.
pub(crate) fn load_line_width() -> f64 {
    local_storage_get(CANVAS_STORAGE_KEY_LINE_WIDTH)
        .and_then(|width: String| width.parse::<f64>().ok())
        .unwrap_or(CANVAS_DEFAULT_LINE_WIDTH)
}

/// Persists the current stroke color to localStorage.
///
/// # Arguments
///
/// - `&str` - The stroke color value to persist.
pub(crate) fn save_stroke_color(color: &str) {
    local_storage_set(CANVAS_STORAGE_KEY_STROKE_COLOR, color);
}

/// Persists the current line width to localStorage.
///
/// # Arguments
///
/// - `f64` - The line width value to persist.
pub(crate) fn save_line_width(width: f64) {
    local_storage_set(CANVAS_STORAGE_KEY_LINE_WIDTH, &width.to_string());
}

pub(crate) fn use_fullscreen_popstate(state: UseCanvas) {
    use_window_event("popstate", move || {
        if state.get_fullscreen().get() {
            exit_fullscreen_from_popstate(state);
        }
    });
}
