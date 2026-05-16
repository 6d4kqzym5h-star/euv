use crate::*;

/// An event handling demo page showcasing all supported browser event types.
///
/// # Returns
///
/// - `VirtualNode`: The event demo page virtual DOM tree.
pub fn page_event() -> VirtualNode {
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
    let form_input_value: Signal<String> = use_signal(|| "".to_string());
    let form_change_value: Signal<String> = use_signal(|| "None".to_string());
    let form_checkbox: Signal<bool> = use_signal(|| false);
    let form_select_value: Signal<String> = use_signal(|| "None".to_string());
    let submit_count: Signal<i32> = use_signal(|| 0);
    let media_status: Signal<String> = use_signal(|| "Not started".to_string());
    let media_event_log: Signal<String> = use_signal(|| "None".to_string());
    html! {
        div {
            class: c_page_container()
            div {
                class: c_page_header()
                h1 {
                    class: c_page_title()
                    "Event Handling"
                }
                p {
                    class: c_page_subtitle()
                    "Complete browser event demo: keyboard, mouse, focus, drag, wheel, clipboard, touch, form, and media events."
                }
            }
            my_card {
                title: "Keyboard Events"
                input {
                    r#type: "text"
                    placeholder: "Type here to capture key events..."
                    class: c_form_input()
                    onkey_down: move |event: NativeEvent| {
                        if let NativeEvent::Keyboard(keyboard_event) = event {
                            let key_name: String = keyboard_event.get_key().clone();
                            last_key.set(key_name);
                            let code_name: String = keyboard_event.get_code().clone();
                            last_key_code.set(code_name);
                            let is_repeat: bool = *keyboard_event.get_repeat();
                            key_repeat.set(is_repeat);
                            let mut modifier: String = String::new();
                            if *keyboard_event.get_ctrl_key() {
                                modifier.push_str("Ctrl+");
                            }
                            if *keyboard_event.get_shift_key() {
                                modifier.push_str("Shift+");
                            }
                            if *keyboard_event.get_alt_key() {
                                modifier.push_str("Alt+");
                            }
                            if *keyboard_event.get_meta_key() {
                                modifier.push_str("Meta+");
                            }
                            if modifier.is_empty() {
                                modifier = "None".to_string();
                            }
                            mouse_modifier.set(modifier);
                            Console::log(&format!("KeyDown: {} (code: {})", last_key.get(), last_key_code.get()));
                        }
                    }
                    onkey_up: move |event: NativeEvent| {
                        if let NativeEvent::Keyboard(keyboard_event) = event {
                            let key_name: String = keyboard_event.get_key().clone();
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
                    onclick: move |event: NativeEvent| {
                        if let NativeEvent::Mouse(mouse_event) = event {
                            let pos: String = format!("({}, {})", *mouse_event.get_client_x(), *mouse_event.get_client_y());
                            mouse_pos.set(pos);
                            let screen: String = format!("({}, {})", *mouse_event.get_screen_x(), *mouse_event.get_screen_y());
                            mouse_screen_pos.set(screen);
                            let current: i32 = click_count.get();
                            click_count.set(current + 1);
                            Console::log(&format!("Click: {} at ({}, {})", current + 1, *mouse_event.get_client_x(), *mouse_event.get_client_y()));
                        }
                    }
                    ondbl_click: move |_event: NativeEvent| {
                        let current: i32 = double_click_count.get();
                        double_click_count.set(current + 1);
                        Console::log(&format!("DblClick: #{}", current + 1));
                    }
                    onmouse_down: move |event: NativeEvent| {
                        if let NativeEvent::Mouse(mouse_event) = event {
                            let button_name: String = match *mouse_event.get_button() {
                                0 => "Left".to_string(),
                                1 => "Middle".to_string(),
                                2 => "Right".to_string(),
                                _ => format!("Button {}", *mouse_event.get_button()),
                            };
                            mouse_button.set(button_name);
                            let current: i32 = mouse_down_count.get();
                            mouse_down_count.set(current + 1);
                        }
                    }
                    onmouse_up: move |_event: NativeEvent| {
                        let current: i32 = mouse_up_count.get();
                        mouse_up_count.set(current + 1);
                    }
                    onmouse_move: move |event: NativeEvent| {
                        if let NativeEvent::Mouse(mouse_event) = event {
                            let pos: String = format!("({}, {})", *mouse_event.get_client_x(), *mouse_event.get_client_y());
                            mouse_pos.set(pos);
                            let buttons_mask: String = format!("{}", *mouse_event.get_buttons());
                            mouse_buttons.set(buttons_mask);
                        }
                    }
                    onmouse_enter: move |_event: NativeEvent| {
                        let current: i32 = mouse_enter_count.get();
                        mouse_enter_count.set(current + 1);
                    }
                    onmouse_leave: move |_event: NativeEvent| {
                        let current: i32 = mouse_leave_count.get();
                        mouse_leave_count.set(current + 1);
                    }
                    oncontext_menu: move |event: NativeEvent| {
                        if let NativeEvent::Mouse(_) = event {
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
                    style: {display: "flex"; gap: "16px"; flex-wrap: "wrap";}
                    div {
                        class: c_event_drag_zone()
                        onmouse_over: move |_event: NativeEvent| {
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
                        onmouse_out: move |_event: NativeEvent| {
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
                    r#type: "text"
                    placeholder: "Click to focus, click outside to blur..."
                    class: c_form_input()
                    onfocus: move |_event: NativeEvent| {
                        focus_status.set("Focused".to_string());
                        let current: i32 = focus_in_count.get();
                        focus_in_count.set(current + 1);
                        Console::log("Focus: input gained focus");
                    }
                    onblur: move |_event: NativeEvent| {
                        focus_status.set("Not focused".to_string());
                        let current: i32 = focus_out_count.get();
                        focus_out_count.set(current + 1);
                        Console::log("Blur: input lost focus");
                    }
                    onfocus_in: move |_event: NativeEvent| {
                        Console::log("FocusIn: focus entered");
                    }
                    onfocus_out: move |_event: NativeEvent| {
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
                    ondrag_start: move |_event: NativeEvent| {
                        drag_status.set("Dragging".to_string());
                        Console::log("DragStart: drag started");
                    }
                    ondrag: move |event: NativeEvent| {
                        if let NativeEvent::Drag(drag_event) = event {
                            let pos: String = format!("({}, {})", *drag_event.get_client_x(), *drag_event.get_client_y());
                            drag_pos.set(pos);
                        }
                    }
                    ondrag_end: move |_event: NativeEvent| {
                        drag_status.set("Ended".to_string());
                        Console::log("DragEnd: drag ended");
                    }
                    ondrag_over: move |_event: NativeEvent| {
                    }
                    ondrag_enter: move |_event: NativeEvent| {
                        Console::log("DragEnter: entered drop zone");
                    }
                    ondrag_leave: move |_event: NativeEvent| {
                        Console::log("DragLeave: left drop zone");
                    }
                    ondrop: move |event: NativeEvent| {
                        if let NativeEvent::Drag(drag_event) = event {
                            let types_str: String = drag_event.get_types().join(", ");
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
                    onwheel: move |event: NativeEvent| {
                        if let NativeEvent::Wheel(wheel_event) = event {
                            let delta: String = format!("({:.1}, {:.1})", *wheel_event.get_delta_x(), *wheel_event.get_delta_y());
                            wheel_delta.set(delta);
                            let current: f64 = wheel_total.get();
                            wheel_total.set(current + *wheel_event.get_delta_y());
                            let mode_name: String = match *wheel_event.get_delta_mode() {
                                0 => "pixel".to_string(),
                                1 => "line".to_string(),
                                2 => "page".to_string(),
                                _ => "unknown".to_string(),
                            };
                            Console::log(&format!("Wheel: dx={:.1}, dy={:.1}, mode={}", *wheel_event.get_delta_x(), *wheel_event.get_delta_y(), mode_name));
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
                        r#type: "text"
                        placeholder: "Try copy, cut, or paste here..."
                        class: c_form_input()
                        value: "Sample text for clipboard"
                        oncopy: move |event: NativeEvent| {
                            clipboard_event_type.set("Copy".to_string());
                            if let NativeEvent::Clipboard(clipboard_event) = event {
                                let data: String = clipboard_event.try_get_data().as_ref().cloned().unwrap_or_else(|| "No data".to_string());
                                clipboard_data.set(data);
                            }
                            Console::log("Copy: text copied");
                        }
                        oncut: move |event: NativeEvent| {
                            clipboard_event_type.set("Cut".to_string());
                            if let NativeEvent::Clipboard(clipboard_event) = event {
                                let data: String = clipboard_event.try_get_data().as_ref().cloned().unwrap_or_else(|| "No data".to_string());
                                clipboard_data.set(data);
                            }
                            Console::log("Cut: text cut");
                        }
                        onpaste: move |event: NativeEvent| {
                            clipboard_event_type.set("Paste".to_string());
                            if let NativeEvent::Clipboard(clipboard_event) = event {
                                let data: String = clipboard_event.try_get_data().as_ref().cloned().unwrap_or_else(|| "No data".to_string());
                                clipboard_data.set(data);
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
                    ontouch_start: move |event: NativeEvent| {
                        if let NativeEvent::Touch(touch_event) = event {
                            let info: String = format!("Start: {} touches at ({}, {})", *touch_event.get_touches_count(), *touch_event.get_client_x(), *touch_event.get_client_y());
                            touch_info.set(info);
                            Console::log(&format!("TouchStart: {} touches", *touch_event.get_touches_count()));
                        }
                    }
                    ontouch_move: move |event: NativeEvent| {
                        if let NativeEvent::Touch(touch_event) = event {
                            let info: String = format!("Move: {} touches at ({}, {})", *touch_event.get_touches_count(), *touch_event.get_client_x(), *touch_event.get_client_y());
                            touch_info.set(info);
                        }
                    }
                    ontouch_end: move |event: NativeEvent| {
                        if let NativeEvent::Touch(touch_event) = event {
                            let info: String = format!("End: {} touches remaining", *touch_event.get_touches_count());
                            touch_info.set(info);
                            Console::log("TouchEnd: touch ended");
                        }
                    }
                    ontouch_cancel: move |_event: NativeEvent| {
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
                        onsubmit: move |event: NativeEvent| {
                            let current: i32 = submit_count.get();
                            submit_count.set(current + 1);
                            Console::log(&format!("Event: {:?}", event));
                            Console::log(&format!("Form submitted #{}", current + 1));
                        }
                        div {
                            class: c_form_input_wrapper()
                            label {
                                class: c_form_label()
                                "Input (oninput & onchange)"
                            }
                            input {
                                r#type: "text"
                                placeholder: "Type to trigger input/change events..."
                                class: c_form_input()
                                oninput: move |event: NativeEvent| {
                                    if let NativeEvent::Input(input_event) = event {
                                        form_input_value.set(input_event.get_value().clone());
                                    }
                                }
                                onchange: move |event: NativeEvent| {
                                    if let NativeEvent::Change(change_event) = event {
                                        form_change_value.set(change_event.get_value().clone());
                                    }
                                }
                            }
                        }
                        div {
                            class: c_form_checkbox_row()
                            input {
                                r#type: "checkbox"
                                class: c_form_checkbox()
                                onchange: move |event: NativeEvent| {
                                    if let NativeEvent::Change(change_event) = event {
                                        form_checkbox.set(*change_event.get_checked());
                                    }
                                }
                            }
                            label {
                                class: c_form_checkbox_label()
                                "Checkbox (onchange)"
                            }
                        }
                        div {
                            class: c_form_input_wrapper()
                            label {
                                class: c_form_label()
                                "Select (onchange)"
                            }
                            select {
                                class: c_select_input()
                                onchange: move |event: NativeEvent| {
                                    if let NativeEvent::Change(change_event) = event {
                                        form_select_value.set(change_event.get_value().clone());
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
                        controls: "true"
                        src: "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3"
                        onplay: move |_event: NativeEvent| {
                            media_status.set("Playing".to_string());
                            media_event_log.set("Play".to_string());
                            Console::log("Play: audio started");
                        }
                        onpause: move |_event: NativeEvent| {
                            media_status.set("Paused".to_string());
                            media_event_log.set("Pause".to_string());
                            Console::log("Pause: audio paused");
                        }
                        onended: move |_event: NativeEvent| {
                            media_status.set("Ended".to_string());
                            media_event_log.set("Ended".to_string());
                            Console::log("Ended: audio ended");
                        }
                        onloaded_data: move |_event: NativeEvent| {
                            media_status.set("Loaded".to_string());
                            media_event_log.set("LoadedData".to_string());
                        }
                        oncan_play: move |_event: NativeEvent| {
                            media_event_log.set("CanPlay".to_string());
                        }
                        onvolume_change: move |_event: NativeEvent| {
                            media_event_log.set("VolumeChange".to_string());
                            Console::log("VolumeChange: volume changed");
                        }
                        ontime_update: move |_event: NativeEvent| {
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
