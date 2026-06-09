use crate::*;

/// A canvas drawing board page component that shows a preview of the
/// drawing and a button to enter fullscreen drawing mode.
///
/// In normal mode, displays a card with the current drawing preview
/// (or a placeholder if no drawing exists) and a "Draw" button.
/// Clicking the button enters fullscreen mode with a portrait-oriented
/// canvas for drawing.
///
/// # Returns
///
/// - `VirtualNode` - The canvas drawing board page virtual DOM tree.
#[component]
pub(crate) fn page_canvas(node: VirtualNode<PageCanvasProps>) -> VirtualNode {
    let _page_canvas_props: PageCanvasProps = node.try_get_props().unwrap_or_default();
    let state: UseCanvas = use_canvas_state();
    use_fullscreen_popstate(state);
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Canvas"
                subtitle: "A freehand drawing board. Tap Draw to start creating."
            }
            my_card {
                title: "Drawing Board"
                primary_button {
                    label: CANVAS_DRAW_LABEL
                    onclick: canvas_on_draw(state)
                    CANVAS_DRAW_LABEL
                }
                div {
                    class: c_canvas_preview_container()
                    if { state.get_snapshot_data_url().get().is_empty() } {
                        div {
                            class: c_canvas_placeholder()
                            "No drawing yet. Tap Draw to start."
                        }
                    } else {
                        img {
                            class: c_canvas_preview_image()
                            src: state.get_snapshot_data_url().get()
                        }
                    }
                }
            }
        }
        if { state.get_fullscreen().get() } {
            div {
                id: CANVAS_CONTAINER_ID
                class: c_canvas_container_fullscreen()
                div {
                    class: c_canvas_fullscreen_toolbar()
                    button {
                        class: c_canvas_fullscreen_button()
                        onclick: move |_event: Event| {
                            clear_canvas(CANVAS_DRAWING_SELECTOR);
                        }
                        "Clear"
                    }
                    div {
                        class: c_canvas_fullscreen_toolbar_center()
                        input {
                            r#type: "color"
                            class: c_canvas_color_input()
                            value: state.get_stroke_color().get()
                            oninput: move |event: Event| {
                                let new_color: String = Reflect::get(event.as_ref(), &JsValue::from_str("target"))
                                    .ok()
                                    .and_then(|target: JsValue| Reflect::get(&target, &JsValue::from_str("value")).ok())
                                    .and_then(|value: JsValue| value.as_string())
                                    .unwrap_or_default();
                                state.get_stroke_color().set(new_color);
                            }
                        }
                        input {
                            r#type: "range"
                            class: c_canvas_fullscreen_range_input()
                            min: CANVAS_MIN_LINE_WIDTH
                            max: CANVAS_MAX_LINE_WIDTH
                            step: CANVAS_LINE_WIDTH_STEP
                            value: state.get_line_width().get()
                            oninput: move |event: Event| {
                                let new_width: f64 = Reflect::get(event.as_ref(), &JsValue::from_str("target"))
                                    .ok()
                                    .and_then(|target: JsValue| Reflect::get(&target, &JsValue::from_str("value")).ok())
                                    .and_then(|value: JsValue| value.as_string())
                                    .and_then(|string: String| string.parse::<f64>().ok())
                                    .unwrap_or(CANVAS_DEFAULT_LINE_WIDTH);
                                state.get_line_width().set(new_width);
                            }
                        }
                    }
                    button {
                        class: c_canvas_fullscreen_button()
                        onclick: move |_event: Event| {
                            exit_fullscreen(state);
                        }
                        CANVAS_FULLSCREEN_EXIT_LABEL
                    }
                }
                div {
                    id: CANVAS_FULLSCREEN_WRAPPER_ID
                    class: c_canvas_drawing_fullscreen_wrapper()
                    canvas {
                        id: CANVAS_DRAWING_ID
                        class: c_canvas_drawing_fullscreen()
                        onmousedown: move |event: Event| {
                            let (offset_x, offset_y): (f64, f64) = get_pointer_offset(&event);
                            let (client_x, client_y): (f64, f64) = get_mouse_client(&event);
                            let (mapped_x, mapped_y): (f64, f64) = map_rotated_offset(offset_x, offset_y, client_x, client_y, true);
                            start_drawing(state, mapped_x, mapped_y);
                        }
                        onmousemove: move |event: Event| {
                            let (offset_x, offset_y): (f64, f64) = get_pointer_offset(&event);
                            let (client_x, client_y): (f64, f64) = get_mouse_client(&event);
                            let (mapped_x, mapped_y): (f64, f64) = map_rotated_offset(offset_x, offset_y, client_x, client_y, true);
                            continue_drawing(state, mapped_x, mapped_y);
                        }
                        onmouseup: move |_event: Event| {
                            stop_drawing(state);
                        }
                        onmouseleave: move |_event: Event| {
                            stop_drawing(state);
                        }
                        ontouchstart: move |event: Event| {
                            prevent_event_default(&event);
                            let (offset_x, offset_y): (f64, f64) = get_touch_offset(&event);
                            let (client_x, client_y): (f64, f64) = get_touch_client(&event);
                            let (mapped_x, mapped_y): (f64, f64) = map_rotated_offset(offset_x, offset_y, client_x, client_y, true);
                            start_drawing(state, mapped_x, mapped_y);
                        }
                        ontouchmove: move |event: Event| {
                            prevent_event_default(&event);
                            let (offset_x, offset_y): (f64, f64) = get_touch_offset(&event);
                            let (client_x, client_y): (f64, f64) = get_touch_client(&event);
                            let (mapped_x, mapped_y): (f64, f64) = map_rotated_offset(offset_x, offset_y, client_x, client_y, true);
                            continue_drawing(state, mapped_x, mapped_y);
                        }
                        ontouchend: move |_event: Event| {
                            stop_drawing(state);
                        }
                    }
                }
            }
        }
    }
}
