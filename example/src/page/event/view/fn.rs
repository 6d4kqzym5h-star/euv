use crate::*;

/// An event handling demo page showcasing all supported browser event types.
///
/// # Returns
///
/// - `VirtualNode` - The event demo page virtual DOM tree.
#[component]
pub(crate) fn page_event(node: VirtualNode<PageEventProps>) -> VirtualNode {
    let PageEventProps = node.try_get_props().unwrap_or_default();
    let keyboard: UseKeyboardEvent = use_keyboard_event();
    let mouse: UseMouseEvent = use_mouse_event();
    let focus: UseFocusEvent = use_focus_event();
    let drag: UseDragEvent = use_drag_event();
    let wheel: UseWheelEvent = use_wheel_event();
    let clipboard: UseClipboardEvent = use_clipboard_event();
    let touch: UseTouchEvent = use_touch_event();
    let form: UseFormEvent = use_form_event();
    let media: UseMediaEvent = use_media_event();
    let video: UseVideoEvent = use_video_event();
    let image: UseImageEvent = use_image_event();
    let current_url: String = current_url_without_params();
    let qr_code_data_url: String = generate_qr_code_data_url(&current_url);
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Event Handling"
                subtitle: "Complete browser event demo: keyboard, mouse, focus, drag, wheel, clipboard, touch, form, media, video, and image events."
            }
            my_card {
                title: "Keyboard Events"
                input {
                    id: EVENT_KEYBOARD_ID
                    name: EVENT_KEYBOARD_NAME
                    r#type: EVENT_TEXT_TYPE
                    autocomplete: EVENT_AUTOCOMPLETE_OFF
                    placeholder: EVENT_KEYBOARD_PLACEHOLDER
                    class: c_form_input()
                    onkeydown: move |event: Event| {
                        if let Some(keyboard_event) = event.dyn_ref::<KeyboardEvent>() {
                            let key_name: String = keyboard_event.key();
                            keyboard.get_last_key().set(key_name);
                            let code_name: String = keyboard_event.code();
                            keyboard.get_last_key_code().set(code_name);
                            let is_repeat: bool = keyboard_event.repeat();
                            keyboard.get_key_repeat().set(is_repeat);
                            let mut modifier: String = String::new();
                            if keyboard_event.ctrl_key() {
                                modifier.push_str("Ctrl+");
                            }
                            if keyboard_event.shift_key() {
                                modifier.push_str("Shift+");
                            }
                            if keyboard_event.alt_key() {
                                modifier.push_str("Alt+");
                            }
                            if keyboard_event.meta_key() {
                                modifier.push_str("Meta+");
                            }
                            if modifier.is_empty() {
                                modifier = "None".to_string();
                            }
                            keyboard.get_modifier().set(modifier);
                            Console::log(&format!("KeyDown: {} (code: {})", keyboard.get_last_key().get(), keyboard.get_last_key_code().get()));
                        }
                    }
                    onkeyup: move |event: Event| {
                        if let Some(keyboard_event) = event.dyn_ref::<KeyboardEvent>() {
                            let key_name: String = keyboard_event.key();
                            keyboard.get_last_key_up().set(key_name.clone());
                            Console::log(&format!("KeyUp: {}", key_name));
                        }
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "KeyDown:"
                        }
                        span {
                            class: c_event_info_value()
                            keyboard.get_last_key()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "KeyCode:"
                        }
                        span {
                            class: c_event_info_value()
                            keyboard.get_last_key_code()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "KeyUp:"
                        }
                        span {
                            class: c_event_info_value()
                            keyboard.get_last_key_up()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Repeat:"
                        }
                        span {
                            class: c_event_info_value()
                            keyboard.get_key_repeat()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Modifiers:"
                        }
                        span {
                            class: c_event_info_value()
                            keyboard.get_modifier()
                        }
                    }
                }
            }
            my_card {
                title: "Mouse Events"
                div {
                    class: c_event_mouse_area()
                    onclick: move |event: Event| {
                        if let Some(mouse_event) = event.dyn_ref::<MouseEvent>() {
                            let pos: String = format!("({}, {})", mouse_event.client_x(), mouse_event.client_y());
                            mouse.get_mouse_pos().set(pos);
                            let screen: String = format!("({}, {})", mouse_event.screen_x(), mouse_event.screen_y());
                            mouse.get_mouse_screen_pos().set(screen);
                            let current: i32 = mouse.get_click_count().get();
                            mouse.get_click_count().set(current + 1);
                            Console::log(&format!("Click: {} at ({}, {})", current + 1, mouse_event.client_x(), mouse_event.client_y()));
                        }
                    }
                    ondblclick: move |_event: Event| {
                        let current: i32 = mouse.get_double_click_count().get();
                        mouse.get_double_click_count().set(current + 1);
                        Console::log(&format!("DblClick: #{}", current + 1));
                    }
                    onmousedown: move |event: Event| {
                        if let Some(mouse_event) = event.dyn_ref::<MouseEvent>() {
                            let button_name: String = match mouse_event.button() {
                                0 => "Left".to_string(),
                                1 => "Middle".to_string(),
                                2 => "Right".to_string(),
                                _ => format!("Button {}", mouse_event.button()),
                            };
                            mouse.get_mouse_button().set(button_name);
                            let current: i32 = mouse.get_mouse_down_count().get();
                            mouse.get_mouse_down_count().set(current + 1);
                        }
                    }
                    onmouseup: move |_event: Event| {
                        let current: i32 = mouse.get_mouse_up_count().get();
                        mouse.get_mouse_up_count().set(current + 1);
                    }
                    onmousemove: move |event: Event| {
                        if let Some(mouse_event) = event.dyn_ref::<MouseEvent>() {
                            let pos: String = format!("({}, {})", mouse_event.client_x(), mouse_event.client_y());
                            mouse.get_mouse_pos().set(pos);
                            let buttons_mask: String = format!("{}", mouse_event.buttons());
                            mouse.get_mouse_buttons().set(buttons_mask);
                        }
                    }
                    onmouseenter: move |_event: Event| {
                        let current: i32 = mouse.get_mouse_enter_count().get();
                        mouse.get_mouse_enter_count().set(current + 1);
                    }
                    onmouseleave: move |_event: Event| {
                        let current: i32 = mouse.get_mouse_leave_count().get();
                        mouse.get_mouse_leave_count().set(current + 1);
                    }
                    oncontextmenu: move |event: Event| {
                        if event.dyn_ref::<MouseEvent>().is_some() {
                            Console::log("ContextMenu: right-click detected");
                        }
                    }
                    p {
                        class: c_demo_text()
                        "Click, double-click, right-click, or move mouse here"
                    }
                    p {
                        class: c_demo_text_muted()
                        "Tracks click, dblclick, mousedown, mouseup, mousemove, mouseenter, mouseleave, contextmenu"
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Clicks:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_click_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "DblClicks:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_double_click_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "MouseDown:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_down_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "MouseUp:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_up_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Client:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_pos()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Screen:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_screen_pos()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Button:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_button()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Buttons:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_buttons()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Enter:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_enter_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Leave:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_leave_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Over:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_over_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Out:"
                        }
                        span {
                            class: c_event_info_value()
                            mouse.get_mouse_out_count()
                        }
                    }
                }
            }
            my_card {
                title: "Mouse Over/Out Events"
                div {
                    class: c_switcher()
                    div {
                        class: c_event_drag_zone()
                        class: c_switcher_col()
                        onmouseover: move |_event: Event| {
                            let current: i32 = mouse.get_mouse_over_count().get();
                            mouse.get_mouse_over_count().set(current + 1);
                        }
                        p {
                            class: c_demo_text()
                            "Mouse Over zone"
                        }
                        p {
                            class: c_demo_text_muted()
                            "Move mouse over this area"
                        }
                    }
                    div {
                        class: c_event_drag_zone_active()
                        class: c_switcher_col()
                        onmouseout: move |_event: Event| {
                            let current: i32 = mouse.get_mouse_out_count().get();
                            mouse.get_mouse_out_count().set(current + 1);
                        }
                        p {
                            class: c_demo_text()
                            "Mouse Out zone"
                        }
                        p {
                            class: c_demo_text_muted()
                            "Move mouse out of this area"
                        }
                    }
                }
            }
            my_card {
                title: "Focus Events"
                input {
                    id: EVENT_FOCUS_ID
                    name: EVENT_FOCUS_NAME
                    r#type: EVENT_TEXT_TYPE
                    autocomplete: EVENT_AUTOCOMPLETE_OFF
                    placeholder: EVENT_FOCUS_PLACEHOLDER
                    class: c_form_input()
                    onfocus: move |_event: Event| {
                        focus.get_focus_status().set("Focused".to_string());
                        let current: i32 = focus.get_focus_in_count().get();
                        focus.get_focus_in_count().set(current + 1);
                        Console::log("Focus: input gained focus");
                    }
                    onblur: move |_event: Event| {
                        focus.get_focus_status().set("Not focused".to_string());
                        let current: i32 = focus.get_focus_out_count().get();
                        focus.get_focus_out_count().set(current + 1);
                        Console::log("Blur: input lost focus");
                    }
                    onfocusin: move |_event: Event| {
                        Console::log("FocusIn: focus entered");
                    }
                    onfocusout: move |_event: Event| {
                        Console::log("FocusOut: focus left");
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Status:"
                        }
                        span {
                            class: c_event_info_value()
                            focus.get_focus_status()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "FocusIn:"
                        }
                        span {
                            class: c_event_info_value()
                            focus.get_focus_in_count()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "FocusOut:"
                        }
                        span {
                            class: c_event_info_value()
                            focus.get_focus_out_count()
                        }
                    }
                }
            }
            my_card {
                title: "Drag Events"
                div {
                    class: c_event_drag_zone()
                    ondragstart: move |_event: Event| {
                        drag.get_drag_status().set("Dragging".to_string());
                        drag.get_drag_enter_counter().set(1);
                        Console::log("DragStart: drag started");
                    }
                    ondrag: move |event: Event| {
                        if drag.get_drag_enter_counter().get() > 0
                            && let Some(drag_event) = event.dyn_ref::<DragEvent>()
                        {
                            let pos: String = format!("({}, {})", drag_event.client_x(), drag_event.client_y());
                            drag.get_drag_pending_pos().set(pos);
                            if drag.get_drag_raf_id().get() == -1 {
                                let pos_signal: Signal<String> = drag.get_drag_pos();
                                let pending_signal: Signal<String> = drag.get_drag_pending_pos();
                                let raf_id_signal: Signal<i32> = drag.get_drag_raf_id();
                                let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                                    pos_signal.set(pending_signal.get());
                                    raf_id_signal.set(-1);
                                }));
                                let window: Window = window().expect("no global window exists");
                                let id: i32 = window.request_animation_frame(closure.as_ref().unchecked_ref()).unwrap_or(-1);
                                drag.get_drag_raf_id().set(id);
                                closure.forget();
                            }
                        }
                    }
                    ondragend: move |_event: Event| {
                        let raf_id: i32 = drag.get_drag_raf_id().get();
                        if raf_id != -1 {
                            let window: Window = window().expect("no global window exists");
                            let _ = window.cancel_animation_frame(raf_id);
                            drag.get_drag_raf_id().set(-1);
                        }
                        if !drag.get_drag_pending_pos().get().is_empty() {
                            drag.get_drag_pos().set(drag.get_drag_pending_pos().get());
                        }
                        drag.get_drag_status().set("Ended".to_string());
                        drag.get_drag_enter_counter().set(0);
                        Console::log("DragEnd: drag ended");
                    }
                    ondragover: move |event: Event| {
                        event.prevent_default();
                    }
                    ondragenter: move |_event: Event| {
                        let counter: i32 = drag.get_drag_enter_counter().get();
                        drag.get_drag_enter_counter().set(counter + 1);
                        if counter == 0 {
                            drag.get_drag_status().set("Dragging".to_string());
                            Console::log("DragEnter: entered drop zone");
                        }
                    }
                    ondragleave: move |_event: Event| {
                        let counter: i32 = drag.get_drag_enter_counter().get();
                        drag.get_drag_enter_counter().set(counter - 1);
                        if counter <= 1 {
                            drag.get_drag_status().set("Outside".to_string());
                            Console::log("DragLeave: left drop zone");
                        }
                    }
                    ondrop: move |event: Event| {
                        if let Some(drag_event) = event.dyn_ref::<DragEvent>() {
                            let types_str: String = drag_event
                                .data_transfer()
                                .map(|data_transfer: DataTransfer| {
                                    let type_count: u32 = data_transfer.types().length();
                                    (0..type_count)
                                        .filter_map(|index: u32| data_transfer.types().get(index).as_string())
                                        .collect::<Vec<String>>()
                                        .join(", ")
                                })
                                .unwrap_or_default();
                            if types_str.is_empty() {
                                drag.get_drag_types().set("None".to_string());
                            } else {
                                drag.get_drag_types().set(types_str);
                            }
                        }
                        drag.get_drag_status().set("Dropped".to_string());
                        Console::log("Drop: item dropped");
                    }
                    div {
                        class: c_event_drag_item()
                        draggable: EVENT_DRAGGABLE_TRUE
                        "Drag Me"
                    }
                    p {
                        class: c_demo_text_muted()
                        "dragstart, drag, dragend, dragover, dragenter, dragleave, drop"
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Status:"
                        }
                        span {
                            class: c_event_info_value()
                            drag.get_drag_status()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Position:"
                        }
                        span {
                            class: c_event_info_value()
                            drag.get_drag_pos()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Types:"
                        }
                        span {
                            class: c_event_info_value()
                            drag.get_drag_types()
                        }
                    }
                }
            }
            my_card {
                title: "Wheel Event"
                div {
                    class: c_event_wheel_zone()
                    onwheel: move |event: Event| {
                        if let Some(wheel_event) = event.dyn_ref::<WheelEvent>() {
                            let delta: String = format!("({:.1}, {:.1})", wheel_event.delta_x(), wheel_event.delta_y());
                            wheel.get_wheel_delta().set(delta);
                            let current: f64 = wheel.get_wheel_total().get();
                            wheel.get_wheel_total().set(current + wheel_event.delta_y());
                            let mode_name: String = match wheel_event.delta_mode() {
                                0 => "pixel".to_string(),
                                1 => "line".to_string(),
                                2 => "page".to_string(),
                                _ => "unknown".to_string(),
                            };
                            Console::log(&format!("Wheel: dx={:.1}, dy={:.1}, mode={}", wheel_event.delta_x(), wheel_event.delta_y(), mode_name));
                        }
                    }
                    p {
                        class: c_demo_text()
                        "Scroll mouse wheel here"
                    }
                    p {
                        class: c_demo_text_muted()
                        "Tracks wheel delta and scroll mode"
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Delta:"
                        }
                        span {
                            class: c_event_info_value()
                            wheel.get_wheel_delta()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Total Y:"
                        }
                        span {
                            class: c_event_info_value()
                            wheel.get_wheel_total()
                        }
                    }
                }
            }
            my_card {
                title: "Clipboard Events"
                div {
                    class: c_event_clipboard_area()
                        input {
                            id: EVENT_CLIPBOARD_ID
                            name: EVENT_CLIPBOARD_NAME
                            r#type: EVENT_TEXT_TYPE
                            autocomplete: EVENT_AUTOCOMPLETE_OFF
                            placeholder: EVENT_CLIPBOARD_PLACEHOLDER
                        class: c_form_input()
                        value: "Sample text for clipboard"
                        oncopy: move |event: Event| {
                            clipboard.get_clipboard_event_type().set("Copy".to_string());
                            if let Some(clipboard_event) = event.dyn_ref::<ClipboardEvent>() {
                                let data: Option<String> = clipboard_event
                                    .clipboard_data()
                                    .and_then(|cd: DataTransfer| cd.get_data("text").ok());
                                clipboard.get_clipboard_data().set(data.unwrap_or_else(|| "No data".to_string()));
                            }
                            Console::log("Copy: text copied");
                        }
                        oncut: move |event: Event| {
                            clipboard.get_clipboard_event_type().set("Cut".to_string());
                            if let Some(clipboard_event) = event.dyn_ref::<ClipboardEvent>() {
                                let data: Option<String> = clipboard_event
                                    .clipboard_data()
                                    .and_then(|cd: DataTransfer| cd.get_data("text").ok());
                                clipboard.get_clipboard_data().set(data.unwrap_or_else(|| "No data".to_string()));
                            }
                            Console::log("Cut: text cut");
                        }
                        onpaste: move |event: Event| {
                            clipboard.get_clipboard_event_type().set("Paste".to_string());
                            if let Some(clipboard_event) = event.dyn_ref::<ClipboardEvent>() {
                                let data: Option<String> = clipboard_event
                                    .clipboard_data()
                                    .and_then(|cd: DataTransfer| cd.get_data("text").ok());
                                clipboard.get_clipboard_data().set(data.unwrap_or_else(|| "No data".to_string()));
                            }
                            Console::log("Paste: text pasted");
                        }
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Event:"
                        }
                        span {
                            class: c_event_info_value()
                            clipboard.get_clipboard_event_type()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Data:"
                        }
                        span {
                            class: c_event_info_value()
                            clipboard.get_clipboard_data()
                        }
                    }
                }
            }
            my_card {
                title: "Touch Events"
                div {
                    class: c_event_touch_zone()
                    ontouchstart: move |event: Event| {
                        if let Some(touch_event) = event.dyn_ref::<TouchEvent>() {
                            let touches: TouchList = touch_event.touches();
                            let first: Option<Touch> = touches.get(0);
                            let info: String = format!("Start: {} touches at ({}, {})", touches.length(), first.as_ref().map(|touch: &Touch| touch.client_x()).unwrap_or_default(), first.as_ref().map(|touch: &Touch| touch.client_y()).unwrap_or_default());
                            touch.get_touch_info().set(info);
                            Console::log(&format!("TouchStart: {} touches", touches.length()));
                        }
                    }
                    ontouchmove: move |event: Event| {
                        if let Some(touch_event) = event.dyn_ref::<TouchEvent>() {
                            let touches: TouchList = touch_event.touches();
                            let first: Option<Touch> = touches.get(0);
                            let info: String = format!("Move: {} touches at ({}, {})", touches.length(), first.as_ref().map(|touch: &Touch| touch.client_x()).unwrap_or_default(), first.as_ref().map(|touch: &Touch| touch.client_y()).unwrap_or_default());
                            touch.get_touch_info().set(info);
                        }
                    }
                    ontouchend: move |event: Event| {
                        if let Some(touch_event) = event.dyn_ref::<TouchEvent>() {
                            let touches: TouchList = touch_event.touches();
                            let info: String = format!("End: {} touches remaining", touches.length());
                            touch.get_touch_info().set(info);
                            Console::log("TouchEnd: touch ended");
                        }
                    }
                    ontouchcancel: move |_event: Event| {
                        touch.get_touch_info().set("Cancelled".to_string());
                        Console::log("TouchCancel: touch cancelled");
                    }
                    p {
                        class: c_demo_text()
                        "Touch this area (mobile/touchscreen)"
                    }
                    p {
                        class: c_demo_text_muted()
                        "touchstart, touchmove, touchend, touchcancel"
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Touch:"
                        }
                        span {
                            class: c_event_info_value()
                            touch.get_touch_info()
                        }
                    }
                }
            }
            my_card {
                title: "Form Events"
                div {
                    class: c_event_form_area()
                    form {
                        onsubmit: move |event: Event| {
                            event.prevent_default();
                            let current: i32 = form.get_submit_count().get();
                            form.get_submit_count().set(current + 1);
                            Console::log(&format!("Form submitted #{}", current + 1));
                        }
                        div {
                            class: c_form_input_wrapper()
                            label {
                                r#for: EVENT_FORM_INPUT_ID
                                class: c_form_label()
                                "Input (oninput & onchange)"
                            }
                            input {
                                r#type: EVENT_TEXT_TYPE
                                id: EVENT_FORM_INPUT_ID
                                name: EVENT_FORM_INPUT_NAME
                                autocomplete: EVENT_AUTOCOMPLETE_OFF
                                placeholder: EVENT_FORM_INPUT_PLACEHOLDER
                                class: c_form_input()
                                oninput: move |event: Event| {
                                    if let Some(target) = event.target()
                                        && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                                            form.get_form_input_value().set(input.value());
                                        }
                                }
                                onchange: move |event: Event| {
                                    if let Some(target) = event.target()
                                        && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                                            form.get_form_change_value().set(input.value());
                                        }
                                }
                            }
                        }
                        div {
                            class: c_form_checkbox_row()
                            input {
                                id: EVENT_FORM_CHECKBOX_ID
                                name: EVENT_FORM_CHECKBOX_NAME
                                r#type: EVENT_CHECKBOX_TYPE
                                autocomplete: EVENT_AUTOCOMPLETE_OFF
                                class: c_form_checkbox()
                                onchange: move |event: Event| {
                                    if let Some(target) = event.target()
                                        && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                                            form.get_form_checkbox().set(input.checked());
                                        }
                                }
                            }
                            label {
                                r#for: EVENT_FORM_CHECKBOX_ID
                                class: c_form_checkbox_label()
                                "Checkbox (onchange)"
                            }
                        }
                        div {
                            class: c_form_input_wrapper()
                            label {
                                r#for: EVENT_FORM_SELECT_ID
                                class: c_form_label()
                                "Select (onchange)"
                            }
                            select {
                                id: EVENT_FORM_SELECT_ID
                                name: EVENT_FORM_SELECT_NAME
                                autocomplete: EVENT_AUTOCOMPLETE_OFF
                                class: c_select_input()
                                onchange: move |event: Event| {
                                    if let Some(target) = event.target()
                                        && let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
                                            form.get_form_select_value().set(select.value());
                                        }
                                }
                                option {
                                    value: ""
                                    "-- Choose --"
                                }
                                option {
                                    value: "alpha"
                                    "Alpha"
                                }
                                option {
                                    value: "beta"
                                    "Beta"
                                }
                                option {
                                    value: "gamma"
                                    "Gamma"
                                }
                            }
                        }
                        primary_button {
                            label: "Submit"
                            "Submit"
                        }
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Input:"
                        }
                        span {
                            class: c_event_info_value()
                            form.get_form_input_value()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Change:"
                        }
                        span {
                            class: c_event_info_value()
                            form.get_form_change_value()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Checked:"
                        }
                        span {
                            class: c_event_info_value()
                            form.get_form_checkbox()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Select:"
                        }
                        span {
                            class: c_event_info_value()
                            form.get_form_select_value()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Submits:"
                        }
                        span {
                            class: c_event_info_value()
                            form.get_submit_count()
                        }
                    }
                }
            }
            my_card {
                title: "Audio Media Events"
                div {
                    class: c_event_media_area()
                    audio {
                        class: c_event_audio()
                        controls: EVENT_CONTROLS_TRUE
                        src: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3"
                        onplay: move |_event: Event| {
                            media.get_media_status().set("Playing".to_string());
                            media.get_media_event_log().set("Play".to_string());
                            Console::log("Play: audio started");
                        }
                        onpause: move |_event: Event| {
                            media.get_media_status().set("Paused".to_string());
                            media.get_media_event_log().set("Pause".to_string());
                            Console::log("Pause: audio paused");
                        }
                        onended: move |_event: Event| {
                            media.get_media_status().set("Ended".to_string());
                            media.get_media_event_log().set("Ended".to_string());
                            Console::log("Ended: audio ended");
                        }
                        onloadeddata: move |_event: Event| {
                            media.get_media_status().set("Loaded".to_string());
                            media.get_media_event_log().set("LoadedData".to_string());
                        }
                        oncanplay: move |_event: Event| {
                            media.get_media_event_log().set("CanPlay".to_string());
                        }
                        onvolumechange: move |_event: Event| {
                            media.get_media_event_log().set("VolumeChange".to_string());
                            Console::log("VolumeChange: volume changed");
                        }
                        ontimeupdate: move |_event: Event| {
                            media.get_media_event_log().set("TimeUpdate".to_string());
                        }
                        p {
                            class: c_demo_text_muted()
                            "Audio player with play, pause, ended, loadeddata, canplay, volumechange, timeupdate events"
                        }
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Status:"
                        }
                        span {
                            class: c_event_info_value()
                            media.get_media_status()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Last Event:"
                        }
                        span {
                            class: c_event_info_value()
                            media.get_media_event_log()
                        }
                    }
                }
            }
            my_card {
                title: "Video Events"
                div {
                    class: c_event_video_area()
                    video {
                        id: EVENT_VIDEO_ID
                        class: c_event_video()
                        controls: EVENT_CONTROLS_TRUE
                        preload: EVENT_PRELOAD_METADATA
                        src: EVENT_VIDEO_SRC
                        onplay: move |_event: Event| {
                            video.get_video_status().set("Playing".to_string());
                            video.get_video_event_log().set("Play".to_string());
                            Console::log("Video Play: video started");
                        }
                        onpause: move |_event: Event| {
                            video.get_video_status().set("Paused".to_string());
                            video.get_video_event_log().set("Pause".to_string());
                            Console::log("Video Pause: video paused");
                        }
                        onended: move |_event: Event| {
                            video.get_video_status().set("Ended".to_string());
                            video.get_video_event_log().set("Ended".to_string());
                            Console::log("Video Ended: video ended");
                        }
                        onloadeddata: move |_event: Event| {
                            video.get_video_status().set("Data Loaded".to_string());
                            video.get_video_event_log().set("LoadedData".to_string());
                            Console::log("Video LoadedData: data loaded");
                        }
                        onloadedmetadata: move |_event: Event| {
                            video.get_video_status().set("Meta Loaded".to_string());
                            video.get_video_event_log().set("LoadedMetadata".to_string());
                            Console::log("Video LoadedMetadata: metadata loaded");
                        }
                        oncanplay: move |_event: Event| {
                            video.get_video_event_log().set("CanPlay".to_string());
                            Console::log("Video CanPlay: can play");
                        }
                        oncanplaythrough: move |_event: Event| {
                            video.get_video_event_log().set("CanPlayThrough".to_string());
                            Console::log("Video CanPlayThrough: can play through");
                        }
                        onwaiting: move |_event: Event| {
                            video.get_video_status().set("Waiting".to_string());
                            video.get_video_event_log().set("Waiting".to_string());
                            Console::log("Video Waiting: buffering");
                        }
                        onplaying: move |_event: Event| {
                            video.get_video_status().set("Playing".to_string());
                            video.get_video_event_log().set("Playing".to_string());
                            Console::log("Video Playing: playback resumed");
                        }
                        ontimeupdate: move |event: Event| {
                            if let Some(target) = event.target()
                                && let Ok(video_el) = target.clone().dyn_into::<HtmlMediaElement>() {
                                    let current: String = format!("{:.2}", video_el.current_time());
                                    video.get_video_current_time().set(current);
                                }
                            video.get_video_event_log().set("TimeUpdate".to_string());
                        }
                        ondurationchange: move |event: Event| {
                            if let Some(target) = event.target()
                                && let Ok(video_el) = target.clone().dyn_into::<HtmlMediaElement>() {
                                    let dur: String = format!("{:.2}", video_el.duration());
                                    video.get_video_duration().set(dur);
                                }
                            video.get_video_event_log().set("DurationChange".to_string());
                            Console::log("Video DurationChange: duration changed");
                        }
                        onprogress: move |_event: Event| {
                            video.get_video_event_log().set("Progress".to_string());
                        }
                        onseeking: move |_event: Event| {
                            video.get_video_event_log().set("Seeking".to_string());
                            Console::log("Video Seeking: seeking started");
                        }
                        onseeked: move |_event: Event| {
                            video.get_video_event_log().set("Seeked".to_string());
                            Console::log("Video Seeked: seek completed");
                        }
                        onvolumechange: move |_event: Event| {
                            video.get_video_event_log().set("VolumeChange".to_string());
                            Console::log("Video VolumeChange: volume changed");
                        }
                        onratechange: move |event: Event| {
                            if let Some(target) = event.target()
                                && let Ok(video_el) = target.clone().dyn_into::<HtmlMediaElement>() {
                                    let rate: String = format!("{}", video_el.playback_rate());
                                    video.get_video_playback_rate().set(rate);
                                }
                            video.get_video_event_log().set("RateChange".to_string());
                            Console::log("Video RateChange: playback rate changed");
                        }
                        onemptied: move |_event: Event| {
                            video.get_video_event_log().set("Emptied".to_string());
                            Console::log("Video Emptied: media emptied");
                        }
                        onstalled: move |_event: Event| {
                            video.get_video_event_log().set("Stalled".to_string());
                            Console::log("Video Stalled: data transfer stalled");
                        }
                        onsuspend: move |_event: Event| {
                            video.get_video_event_log().set("Suspend".to_string());
                            Console::log("Video Suspend: data transfer suspended");
                        }
                        onloadstart: move |_event: Event| {
                            video.get_video_event_log().set("LoadStart".to_string());
                            Console::log("Video LoadStart: loading started");
                        }
                        onerror: move |_event: Event| {
                            video.get_video_status().set("Error".to_string());
                            video.get_video_event_log().set("Error".to_string());
                            Console::log("Video Error: error occurred");
                        }
                        p {
                            class: c_demo_text_muted()
                            "Video player with play, pause, ended, loadeddata, loadedmetadata, canplay, canplaythrough, waiting, playing, timeupdate, durationchange, progress, seeking, seeked, volumechange, ratechange, emptied, stalled, suspend, loadstart, error events"
                        }
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Status:"
                        }
                        span {
                            class: c_event_info_value()
                            video.get_video_status()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Last Event:"
                        }
                        span {
                            class: c_event_info_value()
                            video.get_video_event_log()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Current Time:"
                        }
                        span {
                            class: c_event_info_value()
                            video.get_video_current_time()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Duration:"
                        }
                        span {
                            class: c_event_info_value()
                            video.get_video_duration()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Playback Rate:"
                        }
                        span {
                            class: c_event_info_value()
                            video.get_video_playback_rate()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Buffered:"
                        }
                        span {
                            class: c_event_info_value()
                            video.get_video_buffered()
                        }
                    }
                }
            }
            my_card {
                title: "Image Events"
                div {
                    class: c_event_image_area()
                    img {
                        id: EVENT_IMAGE_ID
                        class: c_event_image()
                        src: qr_code_data_url
                        alt: EVENT_IMAGE_ALT
                        onload: move |event: Event| {
                            if let Some(target) = event.target()
                                && let Ok(img_el) = target.clone().dyn_into::<HtmlImageElement>() {
                                    let size: String = format!("{}x{}", img_el.natural_width(), img_el.natural_height());
                                    image.get_image_natural_size().set(size);
                                }
                            image.get_image_status().set("Loaded".to_string());
                            image.get_image_event_log().set("Load".to_string());
                            Console::log("Image Load: image loaded successfully");
                        }
                        onerror: move |_event: Event| {
                            image.get_image_status().set("Error".to_string());
                            image.get_image_event_log().set("Error".to_string());
                            Console::log("Image Error: failed to load image");
                        }
                    }
                    p {
                        class: c_event_url_text()
                        current_url
                    }
                }
                div {
                    class: c_event_info_grid()
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Status:"
                        }
                        span {
                            class: c_event_info_value()
                            image.get_image_status()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Last Event:"
                        }
                        span {
                            class: c_event_info_value()
                            image.get_image_event_log()
                        }
                    }
                    div {
                        class: c_event_info_row()
                        span {
                            class: c_event_info_label()
                            "Natural Size:"
                        }
                        span {
                            class: c_event_info_value()
                            image.get_image_natural_size()
                        }
                    }
                }
            }
        }
    }
}
