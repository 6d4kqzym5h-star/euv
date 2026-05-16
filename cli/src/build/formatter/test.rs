use super::*;

#[test]
fn test_format_source_preserves_simple_html() {
    let input: &str = r##"fn foo() -> VirtualNode {
    html! {
        div {
            class: c_container()
            "Hello"
        }
    }
}"##;
    let result: String = format_source(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_preserves_multiline_structure() {
    let input: &str = r##"{
        div {
            class: root_class_signal
            nav {
                class: c_app_nav()
                h2 {
                    class: c_nav_header()
                    span {
                        class: c_nav_logo()
                        "E"
                    }
                    span {
                        class: c_inline()
                        "euv example"
                    }
                }
            }
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_preserves_if_else() {
    let input: &str = r##"{
        a {
            href: format!("#{}", target_string)
            class: if is_active { c_nav_item_active() } else { c_nav_item_inactive() }
            onclick: link_handler(target_string)
            label
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_preserves_match() {
    let input: &str = r##"{
        main {
            class: c_app_main()
            match { route_signal.get().as_str() } {
                "/" | "/signals" => {
                    page_signals()
                }
                "/event" => {
                    page_event()
                }
                _ => {
                    page_not_found()
                }
            }
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_preserves_closure_in_onclick() {
    let input: &str = r##"{
        primary_button {
            label: "Toggle"
            onclick: move |_event: NativeEvent| {
                let current: bool = box_visible.get();
                box_visible.set(!current);
            }
            "Toggle Visibility"
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_preserves_for_loop() {
    let input: &str = r##"{
        div {
            class: c_log_container()
            for (index, log) in { logs.get().iter().enumerate() } {
                div {
                    key: index.to_string()
                    class: c_log_item()
                    log.clone()
                }
            }
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_preserves_style_with_expressions() {
    let input: &str = r##"{
        div {
            class: c_progress_bar()
            style: { width: format!("{}%", progress_value.get()); transition: "width 0.1s ease"; background: if progress_value.get() >= 100 { "#059669".to_string() } else { "#4f46e5".to_string() }; height: "100%"; border-radius: "999px"; transition: "all 0.3s ease"; }
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_preserves_inline_if_else_class() {
    let input: &str = r##"{
        div {
            class: if spin_active.get() { c_anim_spin() } else { c_anim_spin_stopped() }
            "⟳"
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_preserves_if_else_block_children() {
    let input: &str = r##"{
        button {
            class: c_nav_theme_button()
            onclick: toggle_theme(theme_signal)
            if { theme_signal.get() == "dark" } {
                "☀"
            } else {
                "🌙"
            }
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_preserves_closure_with_nested_closure() {
    let input: &str = r##"{
        primary_button {
            label: "Start"
            onclick: move |_event: NativeEvent| {
                progress_value.set(0);
                let progress: Signal<i32> = progress_value;
                let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                    let current: i32 = progress.get();
                    if current < 100 {
                        progress.set(current + 1);
                    }
                }));
                let window: Window = window().expect("no global window exists");
                closure.forget();
            }
            "Start"
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_preserves_interpolation_block() {
    let input: &str = r##"{
        div {
            class: c_app_nav()
            { nav_item(route_signal, "Signals", "/signals") }
            { nav_item(route_signal, "Event", "/event") }
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_preserves_style_with_multiple_attrs() {
    let input: &str = r##"{
        div {
            class: c_anim_color_box()
            style: { background: get_anim_color(color_index.get()); transition: "background 0.5s ease, transform 0.3s ease"; transform: if scale_active.get() { "scale(1.1)" } else { "scale(1)" }; }
            onclick: move |_event: NativeEvent| {
                let current: bool = scale_active.get();
                scale_active.set(!current);
            }
            "Click me to scale!"
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_normalizes_colon_spacing() {
    let input: &str = "{div {class :  c_foo()  href : format!(\"#{}\", x) label}}";
    let expected: &str = "{ div { class: c_foo()  href: format!(\"#{}\", x) label } }";
    let result: String = format_brace_block(input);
    assert_eq!(result, expected);
}

#[test]
fn test_format_brace_block_normalizes_brace_spacing_single_line() {
    let input: &str = "{div{class: c_foo()\"hello\"}}";
    let expected: &str = "{ div { class: c_foo()\"hello\" } }";
    let result: String = format_brace_block(input);
    assert_eq!(result, expected);
}

#[test]
fn test_format_brace_block_preserves_empty_braces() {
    let input: &str = "{  }";
    let expected: &str = "{ }";
    let result: String = format_brace_block(input);
    assert_eq!(result, expected);
}

#[test]
fn test_format_source_preserves_non_macro_code() {
    let input: &str = r##"fn foo() -> i32 {
    let x: i32 = 42;
    x + 1
}"##;
    let result: String = format_source(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_source_finds_html_macro() {
    let input: &str = r##"fn foo() -> VirtualNode {
    let x: i32 = 42;
    html! {
        div {
            class: c_container()
            "Hello"
        }
    }
}"##;
    let result: String = format_source(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_source_handles_string_with_macro_name() {
    let input: &str = r##"fn foo() -> &'static str {
    "html! is a macro name inside a string"
}"##;
    let result: String = format_source(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_preserves_string_with_braces() {
    let input: &str = r##"{
        div {
            class: c_container()
            format!("{} world", "hello")
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_preserves_line_comment() {
    let input: &str = r##"{
        div {
            // This is a comment
            class: c_container()
            "Hello"
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}

#[test]
fn test_find_macro_spans_html() {
    let source: &str = r##"html! { div { "hi" } }"##;
    let spans: Vec<MacroSpan> = find_macro_spans(source);
    assert_eq!(spans.len(), 1);
    let extracted: &str = &source[spans[0].start..spans[0].end];
    assert!(extracted.starts_with('{'));
    assert!(extracted.ends_with('}'));
}

#[test]
fn test_find_macro_spans_class() {
    let source: &str = r##"class! { c_foo { color: "red" } }"##;
    let spans: Vec<MacroSpan> = find_macro_spans(source);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_find_macro_spans_css_vars() {
    let source: &str = r##"css_vars! { --primary: "#4f46e5" }"##;
    let spans: Vec<MacroSpan> = find_macro_spans(source);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_find_macro_spans_var() {
    let source: &str = "var!(--primary)";
    let spans: Vec<MacroSpan> = find_macro_spans(source);
    assert_eq!(spans.len(), 0);
}

#[test]
fn test_find_macro_spans_multiple() {
    let source: &str = r##"let a = html! { "hi" }; let b = class! { c_x { color: "red" } };"##;
    let spans: Vec<MacroSpan> = find_macro_spans(source);
    assert_eq!(spans.len(), 2);
}

#[test]
fn test_format_brace_block_preserves_full_app_example() {
    let input: &str = r##"{
        div {
            class: root_class_signal
            nav {
                class: c_app_nav()
                h2 {
                    class: c_nav_header()
                    span {
                        class: c_nav_logo()
                        "E"
                    }
                    span {
                        class: c_inline()
                        "euv example"
                    }
                }
                p {
                    class: c_nav_section_label()
                    "Pages"
                }
                { nav_item(route_signal, "Signals", "/signals") }
                { nav_item(route_signal, "Event", "/event") }
                { nav_item(route_signal, "List", "/list") }
                div {
                    class: c_nav_theme_toggle()
                    button {
                        class: c_nav_theme_button()
                        onclick: toggle_theme(theme_signal)
                        if { theme_signal.get() == "dark" } {
                            "☀"
                        } else {
                            "🌙"
                        }
                    }
                }
                p {
                    class: c_nav_footer()
                    "Built with euv & WASM"
                }
            }
            main {
                class: c_app_main()
                match { route_signal.get().as_str() } {
                    "/" | "/signals" => {
                        page_signals()
                    }
                    "/event" => {
                        page_event()
                    }
                    _ => {
                        page_not_found()
                    }
                }
            }
            { vconsole_panel(panel_open) }
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}

#[test]
fn test_format_brace_block_preserves_animation_page_example() {
    let input: &str = r##"{
        div {
            class: c_page_container()
            my_card {
                title: "Fade In / Out"
                primary_button {
                    label: "Toggle"
                    onclick: move |_event: NativeEvent| {
                        let current: bool = box_visible.get();
                        box_visible.set(!current);
                    }
                    "Toggle Visibility"
                }
                if { box_visible.get() } {
                    div {
                        class: c_anim_fade_in()
                        "This element fades in and out with a smooth transition."
                    }
                } else {
                    ""
                }
            }
            my_card {
                title: "Spinning Element"
                div {
                    class: c_anim_spin_container()
                    div {
                        class: if spin_active.get() { c_anim_spin() } else { c_anim_spin_stopped() }
                        "⟳"
                    }
                }
            }
        }
    }"##;
    let result: String = format_brace_block(input);
    assert_eq!(result, input);
}
