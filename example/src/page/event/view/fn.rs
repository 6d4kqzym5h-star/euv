use crate::*;

/// An event handling demo page showcasing all supported browser event types.
///
/// # Returns
///
/// - `VirtualNode` - The event demo page virtual DOM tree.
pub(crate) fn page_event() -> VirtualNode {
    let last_key: Signal<String> = use_signal(|| "None".to_string());
    let last_key_code: Signal<String> = use_signal(|| "None".to_string());
    let last_key_up: Signal<String> = use_signal(|| "None".to_string());
    let key_repeat: Signal<bool> = use_signal(|| false);
    let click_count: Signal<i32> = use_signal(|| 0);
    let double_click_count: Signal<i32> = use_signal(|| 0);
    let mouse_pos: Signal<String> = use_signal(|| "(0, 0)".to_string());
    let mouse_screen_pos: Signal<String> = use_signal(|| "(0, 0)".to_string());
    let mouse_button: Signal<String> = use_signal(|| "None".to_string());
    let mouse_buttons: Signal<String> = use_signal(|| "0".to_string());
    let mouse_modifier: Signal<String> = use_signal(|| "None".to_string());
    let mouse_enter_count: Signal<i32> = use_signal(|| 0);
    let mouse_leave_count: Signal<i32> = use_signal(|| 0);
    let mouse_over_count: Signal<i32> = use_signal(|| 0);
    let mouse_out_count: Signal<i32> = use_signal(|| 0);
    let mouse_down_count: Signal<i32> = use_signal(|| 0);
    let mouse_up_count: Signal<i32> = use_signal(|| 0);
    let focus_status: Signal<String> = use_signal(|| "Not focused".to_string());
    let focus_in_count: Signal<i32> = use_signal(|| 0);
    let focus_out_count: Signal<i32> = use_signal(|| 0);
    let drag_status: Signal<String> = use_signal(|| "Idle".to_string());
    let drag_pos: Signal<String> = use_signal(|| "(-, -)".to_string());
    let drag_types: Signal<String> = use_signal(|| "None".to_string());
    let wheel_delta: Signal<String> = use_signal(|| "(0, 0)".to_string());
    let wheel_total: Signal<f64> = use_signal(|| 0.0);
    let clipboard_data: Signal<String> = use_signal(|| "None".to_string());
    let clipboard_event_type: Signal<String> = use_signal(|| "None".to_string());
    let touch_info: Signal<String> = use_signal(|| "No touch".to_string());
    let form_input_value: Signal<String> = use_signal(String::new);
    let form_change_value: Signal<String> = use_signal(|| "None".to_string());
    let form_checkbox: Signal<bool> = use_signal(|| false);
    let form_select_value: Signal<String> = use_signal(|| "None".to_string());
    let submit_count: Signal<i32> = use_signal(|| 0);
    let media_status: Signal<String> = use_signal(|| "Not started".to_string());
    let media_event_log: Signal<String> = use_signal(|| "None".to_string());
    html! {
        div {
            class: c_page_container()
            page_header("Event Handling", "Complete browser event demo: keyboard, mouse, focus, drag, wheel, clipboard, touch, form, and media events.")
            my_card {
                title: "Keyboard Events"
                input {
                    id: "event-keyboard"
                    name: "keyboard"
                    r#type: "text"
                    autocomplete: "off"
                    placeholder: "Type here to capture key events..."
                    class: c_form_input()
                    onkey_down: move |event: Event| {
                        if let Some(keyboard_event) = event.dyn_ref::<KeyboardEvent>() {
                            let key_name: String = keyboard_event.key();
                            last_key.set(key_name);
                            let code_name: String = keyboard_event.code();
                            last_key_code.set(code_name);
                            let is_repeat: bool = keyboard_event.repeat();
                            key_repeat.set(is_repeat);
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
                            mouse_modifier.set(modifier);
                            Console::log(&format!("KeyDown: {} (code: {})", last_key.get(), last_key_code.get()));
                        }
                    }
                    onkey_up: move |event: Event| {
                        if let Some(keyboard_event) = event.dyn_ref::<KeyboardEvent>() {
                            let key_name: String = keyboard_event.key();
                            last_key_up.set(key_name.clone());
                            Console::log(&format!("KeyUp: {}", key_name));
                        }
                    }
                }
                div {
                    class: c_event_section_row()
                    div {
                        class: c_event_section_col()
                        p {
                            class: c_event_result()
                            "KeyDown: "
                            span {
                                class: c_event_highlight()
                                last_key
                            }
                        }
                        p {
                            class: c_event_result()
                            "KeyCode: "
                            span {
                                class: c_event_highlight()
                                last_key_code
                            }
                        }
                    }
                    div {
                        class: c_event_section_col()
                        p {
                            class: c_event_result()
                            "KeyUp: "
                            span {
                                class: c_event_highlight()
                                last_key_up
                            }
                        }
                        p {
                            class: c_event_result()
                            "Repeat: "
                            span {
                                class: c_event_highlight()
                                key_repeat
                            }
                        }
                    }
                }
                p {
                    class: c_event_result()
                    "Modifiers: "
                    span {
                        class: c_event_highlight()
                        mouse_modifier
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
                            mouse_pos.set(pos);
                            let screen: String = format!("({}, {})", mouse_event.screen_x(), mouse_event.screen_y());
                            mouse_screen_pos.set(screen);
                            let current: i32 = click_count.get();
                            click_count.set(current + 1);
                            Console::log(&format!("Click: {} at ({}, {})", current + 1, mouse_event.client_x(), mouse_event.client_y()));
                        }
                    }
                    ondbl_click: move |_event: Event| {
                        let current: i32 = double_click_count.get();
                        double_click_count.set(current + 1);
                        Console::log(&format!("DblClick: #{}", current + 1));
                    }
                    onmouse_down: move |event: Event| {
                        if let Some(mouse_event) = event.dyn_ref::<MouseEvent>() {
                            let button_name: String = match mouse_event.button() {
                                0 => "Left".to_string(),
                                1 => "Middle".to_string(),
                                2 => "Right".to_string(),
                                _ => format!("Button {}", mouse_event.button()),
                            };
                            mouse_button.set(button_name);
                            let current: i32 = mouse_down_count.get();
                            mouse_down_count.set(current + 1);
                        }
                    }
                    onmouse_up: move |_event: Event| {
                        let current: i32 = mouse_up_count.get();
                        mouse_up_count.set(current + 1);
                    }
                    onmouse_move: move |event: Event| {
                        if let Some(mouse_event) = event.dyn_ref::<MouseEvent>() {
                            let pos: String = format!("({}, {})", mouse_event.client_x(), mouse_event.client_y());
                            mouse_pos.set(pos);
                            let buttons_mask: String = format!("{}", mouse_event.buttons());
                            mouse_buttons.set(buttons_mask);
                        }
                    }
                    onmouse_enter: move |_event: Event| {
                        let current: i32 = mouse_enter_count.get();
                        mouse_enter_count.set(current + 1);
                    }
                    onmouse_leave: move |_event: Event| {
                        let current: i32 = mouse_leave_count.get();
                        mouse_leave_count.set(current + 1);
                    }
                    oncontext_menu: move |event: Event| {
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
                    class: c_event_section_row()
                    div {
                        class: c_event_section_col()
                        p {
                            class: c_event_result()
                            "Clicks: "
                            span {
                                class: c_event_highlight()
                                click_count
                            }
                        }
                        p {
                            class: c_event_result()
                            "DblClicks: "
                            span {
                                class: c_event_highlight()
                                double_click_count
                            }
                        }
                        p {
                            class: c_event_result()
                            "MouseDown: "
                            span {
                                class: c_event_highlight()
                                mouse_down_count
                            }
                        }
                        p {
                            class: c_event_result()
                            "MouseUp: "
                            span {
                                class: c_event_highlight()
                                mouse_up_count
                            }
                        }
                    }
                    div {
                        class: c_event_section_col()
                        p {
                            class: c_event_result()
                            "Client: "
                            span {
                                class: c_event_highlight()
                                mouse_pos
                            }
                        }
                        p {
                            class: c_event_result()
                            "Screen: "
                            span {
                                class: c_event_highlight()
                                mouse_screen_pos
                            }
                        }
                        p {
                            class: c_event_result()
                            "Button: "
                            span {
                                class: c_event_highlight()
                                mouse_button
                            }
                        }
                        p {
                            class: c_event_result()
                            "Buttons: "
                            span {
                                class: c_event_highlight()
                                mouse_buttons
                            }
                        }
                    }
                    div {
                        class: c_event_section_col()
                        p {
                            class: c_event_result()
                            "Enter: "
                            span {
                                class: c_event_highlight()
                                mouse_enter_count
                            }
                        }
                        p {
                            class: c_event_result()
                            "Leave: "
                            span {
                                class: c_event_highlight()
                                mouse_leave_count
                            }
                        }
                        p {
                            class: c_event_result()
                            "Over: "
                            span {
                                class: c_event_highlight()
                                mouse_over_count
                            }
                        }
                        p {
                            class: c_event_result()
                            "Out: "
                            span {
                                class: c_event_highlight()
                                mouse_out_count
                            }
                        }
                    }
                }
            }
            my_card {
                title: "Mouse Over/Out Events"
                div {
                    style: { display: "flex"; gap: "16px"; flex-wrap: "wrap"; }
                    div {
                        class: c_event_drag_zone()
                        onmouse_over: move |_event: Event| {
                            let current: i32 = mouse_over_count.get();
                            mouse_over_count.set(current + 1);
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
                        onmouse_out: move |_event: Event| {
                            let current: i32 = mouse_out_count.get();
                            mouse_out_count.set(current + 1);
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
                    id: "event-focus"
                    name: "focus"
                    r#type: "text"
                    autocomplete: "off"
                    placeholder: "Click to focus, click outside to blur..."
                    class: c_form_input()
                    onfocus: move |_event: Event| {
                        focus_status.set("Focused".to_string());
                        let current: i32 = focus_in_count.get();
                        focus_in_count.set(current + 1);
                        Console::log("Focus: input gained focus");
                    }
                    onblur: move |_event: Event| {
                        focus_status.set("Not focused".to_string());
                        let current: i32 = focus_out_count.get();
                        focus_out_count.set(current + 1);
                        Console::log("Blur: input lost focus");
                    }
                    onfocus_in: move |_event: Event| {
                        Console::log("FocusIn: focus entered");
                    }
                    onfocus_out: move |_event: Event| {
                        Console::log("FocusOut: focus left");
                    }
                }
                div {
                    class: c_event_section_row()
                    p {
                        class: c_event_result()
                        "Status: "
                        span {
                            class: c_event_highlight()
                            focus_status
                        }
                    }
                    p {
                        class: c_event_result()
                        "FocusIn: "
                        span {
                            class: c_event_highlight()
                            focus_in_count
                        }
                    }
                    p {
                        class: c_event_result()
                        "FocusOut: "
                        span {
                            class: c_event_highlight()
                            focus_out_count
                        }
                    }
                }
            }
            my_card {
                title: "Drag Events"
                div {
                    class: c_event_drag_zone()
                    ondrag_start: move |_event: Event| {
                        drag_status.set("Dragging".to_string());
                        Console::log("DragStart: drag started");
                    }
                    ondrag: move |event: Event| {
                        if let Some(drag_event) = event.dyn_ref::<DragEvent>() {
                            let pos: String = format!("({}, {})", drag_event.client_x(), drag_event.client_y());
                            drag_pos.set(pos);
                        }
                    }
                    ondrag_end: move |_event: Event| {
                        drag_status.set("Ended".to_string());
                        Console::log("DragEnd: drag ended");
                    }
                    ondrag_over: move |_event: Event| {
                    }
                    ondrag_enter: move |_event: Event| {
                        Console::log("DragEnter: entered drop zone");
                    }
                    ondrag_leave: move |_event: Event| {
                        Console::log("DragLeave: left drop zone");
                    }
                    ondrop: move |event: Event| {
                        if let Some(drag_event) = event.dyn_ref::<DragEvent>() {
                            let types_str: String = drag_event
                                .data_transfer()
                                .map(|dt: DataTransfer| {
                                    let len: u32 = dt.types().length();
                                    (0..len)
                                        .filter_map(|i: u32| dt.types().get(i).as_string())
                                        .collect::<Vec<String>>()
                                        .join(", ")
                                })
                                .unwrap_or_default();
                            if types_str.is_empty() {
                                drag_types.set("None".to_string());
                            } else {
                                drag_types.set(types_str);
                            }
                        }
                        drag_status.set("Dropped".to_string());
                        Console::log("Drop: item dropped");
                    }
                    div {
                        class: c_event_drag_item()
                        draggable: "true"
                        "Drag Me"
                    }
                    p {
                        class: c_demo_text_muted()
                        "dragstart, drag, dragend, dragover, dragenter, dragleave, drop"
                    }
                }
                div {
                    class: c_event_section_row()
                    p {
                        class: c_event_result()
                        "Status: "
                        span {
                            class: c_event_highlight()
                            drag_status
                        }
                    }
                    p {
                        class: c_event_result()
                        "Position: "
                        span {
                            class: c_event_highlight()
                            drag_pos
                        }
                    }
                    p {
                        class: c_event_result()
                        "Types: "
                        span {
                            class: c_event_highlight()
                            drag_types
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
                            wheel_delta.set(delta);
                            let current: f64 = wheel_total.get();
                            wheel_total.set(current + wheel_event.delta_y());
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
                    class: c_event_section_row()
                    p {
                        class: c_event_result()
                        "Delta: "
                        span {
                            class: c_event_highlight()
                            wheel_delta
                        }
                    }
                    p {
                        class: c_event_result()
                        "Total Y: "
                        span {
                            class: c_event_highlight()
                            wheel_total
                        }
                    }
                }
            }
            my_card {
                title: "Clipboard Events"
                div {
                    class: c_event_clipboard_area()
                        input {
                            id: "event-clipboard"
                            name: "clipboard"
                            r#type: "text"
                            autocomplete: "off"
                            placeholder: "Try copy, cut, or paste here..."
                        class: c_form_input()
                        value: "Sample text for clipboard"
                        oncopy: move |event: Event| {
                            clipboard_event_type.set("Copy".to_string());
                            if let Some(clipboard_event) = event.dyn_ref::<ClipboardEvent>() {
                                let data: Option<String> = clipboard_event
                                    .clipboard_data()
                                    .and_then(|cd| cd.get_data("text").ok());
                                clipboard_data.set(data.unwrap_or_else(|| "No data".to_string()));
                            }
                            Console::log("Copy: text copied");
                        }
                        oncut: move |event: Event| {
                            clipboard_event_type.set("Cut".to_string());
                            if let Some(clipboard_event) = event.dyn_ref::<ClipboardEvent>() {
                                let data: Option<String> = clipboard_event
                                    .clipboard_data()
                                    .and_then(|cd| cd.get_data("text").ok());
                                clipboard_data.set(data.unwrap_or_else(|| "No data".to_string()));
                            }
                            Console::log("Cut: text cut");
                        }
                        onpaste: move |event: Event| {
                            clipboard_event_type.set("Paste".to_string());
                            if let Some(clipboard_event) = event.dyn_ref::<ClipboardEvent>() {
                                let data: Option<String> = clipboard_event
                                    .clipboard_data()
                                    .and_then(|cd| cd.get_data("text").ok());
                                clipboard_data.set(data.unwrap_or_else(|| "No data".to_string()));
                            }
                            Console::log("Paste: text pasted");
                        }
                    }
                }
                div {
                    class: c_event_section_row()
                    p {
                        class: c_event_result()
                        "Event: "
                        span {
                            class: c_event_highlight()
                            clipboard_event_type
                        }
                    }
                    p {
                        class: c_event_result()
                        "Data: "
                        span {
                            class: c_event_highlight()
                            clipboard_data
                        }
                    }
                }
            }
            my_card {
                title: "Touch Events"
                div {
                    class: c_event_touch_zone()
                    ontouch_start: move |event: Event| {
                        if let Some(touch_event) = event.dyn_ref::<TouchEvent>() {
                            let touches: TouchList = touch_event.touches();
                            let first: Option<Touch> = touches.get(0);
                            let info: String = format!("Start: {} touches at ({}, {})", touches.length(), first.as_ref().map(|t: &Touch| t.client_x()).unwrap_or(0), first.as_ref().map(|t: &Touch| t.client_y()).unwrap_or(0));
                            touch_info.set(info);
                            Console::log(&format!("TouchStart: {} touches", touches.length()));
                        }
                    }
                    ontouch_move: move |event: Event| {
                        if let Some(touch_event) = event.dyn_ref::<TouchEvent>() {
                            let touches: TouchList = touch_event.touches();
                            let first: Option<Touch> = touches.get(0);
                            let info: String = format!("Move: {} touches at ({}, {})", touches.length(), first.as_ref().map(|t: &Touch| t.client_x()).unwrap_or(0), first.as_ref().map(|t: &Touch| t.client_y()).unwrap_or(0));
                            touch_info.set(info);
                        }
                    }
                    ontouch_end: move |event: Event| {
                        if let Some(touch_event) = event.dyn_ref::<TouchEvent>() {
                            let touches: TouchList = touch_event.touches();
                            let info: String = format!("End: {} touches remaining", touches.length());
                            touch_info.set(info);
                            Console::log("TouchEnd: touch ended");
                        }
                    }
                    ontouch_cancel: move |_event: Event| {
                        touch_info.set("Cancelled".to_string());
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
                p {
                    class: c_event_result()
                    "Touch: "
                    span {
                        class: c_event_highlight()
                        touch_info
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
                            let current: i32 = submit_count.get();
                            submit_count.set(current + 1);
                            Console::log(&format!("Event: {:?}", event));
                            Console::log(&format!("Form submitted #{}", current + 1));
                        }
                        div {
                            class: c_form_input_wrapper()
                            label {
                                r#for: "event-form-input"
                                class: c_form_label()
                                "Input (oninput & onchange)"
                            }
                            input {
                            r#type: "text"
                            id: "event-form-input"
                            name: "form_input"
                            autocomplete: "off"
                            placeholder: "Type to trigger input/change events..."
                                class: c_form_input()
                                oninput: move |event: Event| {
                                    if let Some(target) = event.target()
                                        && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                                            form_input_value.set(input.value());
                                        }
                                }
                                onchange: move |event: Event| {
                                    if let Some(target) = event.target()
                                        && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                                            form_change_value.set(input.value());
                                        }
                                }
                            }
                        }
                        div {
                            class: c_form_checkbox_row()
                            input {
                                id: "event-form-checkbox"
                                name: "form_checkbox"
                                r#type: "checkbox"
                                autocomplete: "off"
                                class: c_form_checkbox()
                                onchange: move |event: Event| {
                                    if let Some(target) = event.target()
                                        && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                                            form_checkbox.set(input.checked());
                                        }
                                }
                            }
                            label {
                                r#for: "event-form-checkbox"
                                class: c_form_checkbox_label()
                                "Checkbox (onchange)"
                            }
                        }
                        div {
                            class: c_form_input_wrapper()
                            label {
                                r#for: "event-form-select"
                                class: c_form_label()
                                "Select (onchange)"
                            }
                            select {
                                id: "event-form-select"
                                name: "form_select"
                                autocomplete: "off"
                                class: c_select_input()
                                onchange: move |event: Event| {
                                    if let Some(target) = event.target()
                                        && let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
                                            form_select_value.set(select.value());
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
                    class: c_event_section_row()
                    p {
                        class: c_event_result()
                        "Input: "
                        span {
                            class: c_event_highlight()
                            form_input_value
                        }
                    }
                    p {
                        class: c_event_result()
                        "Change: "
                        span {
                            class: c_event_highlight()
                            form_change_value
                        }
                    }
                    p {
                        class: c_event_result()
                        "Checked: "
                        span {
                            class: c_event_highlight()
                            form_checkbox
                        }
                    }
                    p {
                        class: c_event_result()
                        "Select: "
                        span {
                            class: c_event_highlight()
                            form_select_value
                        }
                    }
                    p {
                        class: c_event_result()
                        "Submits: "
                        span {
                            class: c_event_highlight()
                            submit_count
                        }
                    }
                }
            }
            my_card {
                title: "Media Events"
                div {
                    class: c_event_media_area()
                    audio {
                        class: c_event_audio()
                        controls: "true"
                        src: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3"
                        onplay: move |_event: Event| {
                            media_status.set("Playing".to_string());
                            media_event_log.set("Play".to_string());
                            Console::log("Play: audio started");
                        }
                        onpause: move |_event: Event| {
                            media_status.set("Paused".to_string());
                            media_event_log.set("Pause".to_string());
                            Console::log("Pause: audio paused");
                        }
                        onended: move |_event: Event| {
                            media_status.set("Ended".to_string());
                            media_event_log.set("Ended".to_string());
                            Console::log("Ended: audio ended");
                        }
                        onloaded_data: move |_event: Event| {
                            media_status.set("Loaded".to_string());
                            media_event_log.set("LoadedData".to_string());
                        }
                        oncan_play: move |_event: Event| {
                            media_event_log.set("CanPlay".to_string());
                        }
                        onvolume_change: move |_event: Event| {
                            media_event_log.set("VolumeChange".to_string());
                            Console::log("VolumeChange: volume changed");
                        }
                        ontime_update: move |_event: Event| {
                            media_event_log.set("TimeUpdate".to_string());
                        }
                        p {
                            class: c_demo_text_muted()
                            "Audio player with play, pause, ended, loadeddata, canplay, volumechange, timeupdate events"
                        }
                    }
                }
                div {
                    class: c_event_section_row()
                    p {
                        class: c_event_result()
                        "Status: "
                        span {
                            class: c_event_highlight()
                            media_status
                        }
                    }
                    p {
                        class: c_event_result()
                        "Last Event: "
                        span {
                            class: c_event_highlight()
                            media_event_log
                        }
                    }
                }
            }
        }
    }
}
