use crate::*;

/// Renders a navigation item link with active state styling.
///
/// # Arguments
///
/// - `Signal<String>`: The reactive signal holding the current route.
/// - `&str`: The display label for the navigation item.
/// - `&str`: The target route path.
///
/// # Returns
///
/// - `VirtualNode`: The navigation item virtual DOM tree.
pub fn nav_item(route_signal: Signal<String>, label: &str, target: &str) -> VirtualNode {
    let target_string: String = target.to_string();
    let current_route_value: String = route_signal.get();
    let is_active: bool = current_route_value == target;
    html! {
        a {
            href: format!("#{}", target_string)
            class: if is_active { c_nav_item_active() } else { c_nav_item_inactive() }
            onclick: link_handler(target_string)
            label
        }
    }
}

/// Renders the application shell with navigation and route-based page content.
///
/// # Returns
///
/// - `VirtualNode`: The root application virtual DOM tree.
pub fn app() -> VirtualNode {
    init_console();
    let route_signal: Signal<String> = use_signal(current_route);
    let panel_open: Signal<bool> = use_signal(|| false);
    let theme_state: ThemeState = use_theme();
    let theme_signal: Signal<String> = theme_state.theme;
    let theme_style_signal: Signal<String> = theme_state.style;
    let window: Window = window().expect("no global window exists");
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let new_route: String = current_route();
        route_signal.set(new_route);
    }));
    window
        .add_event_listener_with_callback(
            &NativeEventName::HashChange.to_string(),
            closure.as_ref().unchecked_ref(),
        )
        .unwrap();
    closure.forget();
    html! {
        div {
            class: c_app_root()
            style: theme_style_signal
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
                {nav_item(route_signal, "Signals", "/signals")}
                {nav_item(route_signal, "Event", "/event")}
                {nav_item(route_signal, "List", "/list")}
                {nav_item(route_signal, "Conditional", "/conditional")}
                {nav_item(route_signal, "Modal", "/modal")}
                {nav_item(route_signal, "Select", "/select")}
                {nav_item(route_signal, "Async", "/async")}
                {nav_item(route_signal, "Form", "/form")}
                {nav_item(route_signal, "Timer", "/timer")}
                {nav_item(route_signal, "Animation", "/animation")}
                {nav_item(route_signal, "Browser", "/browser")}
                {nav_item(route_signal, "Lifecycle", "/lifecycle")}
                {nav_item(route_signal, "Custom Attrs", "/custom-attrs")}
                div {
                    class: c_nav_theme_toggle()
                    button {
                        class: c_nav_theme_button()
                        onclick: move |_event: NativeEvent| {
                            let current: String = theme_signal.get();
                            if current == "light" {
                                theme_signal.set("dark".to_string());
                            } else {
                                theme_signal.set("light".to_string());
                            }
                        }
                        if {theme_signal.get() == "dark"} {
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
                match {route_signal.get().as_str()} {
                    "/" | "/signals" => {
                        page_signals()
                    }
                    "/event" => {
                        page_event()
                    }
                    "/list" => {
                        page_list()
                    }
                    "/conditional" => {
                        page_conditional()
                    }
                    "/modal" => {
                        page_modal()
                    }
                    "/select" => {
                        page_select()
                    }
                    "/async" => {
                        page_async_demo()
                    }
                    "/form" => {
                        page_form()
                    }
                    "/timer" => {
                        page_timer()
                    }
                    "/animation" => {
                        page_animation()
                    }
                    "/browser" => {
                        page_browser()
                    }
                    "/lifecycle" => {
                        page_lifecycle()
                    }
                    "/custom-attrs" => {
                        page_custom_attrs()
                    }
                    _ => {
                        page_not_found()
                    }
                }
            }
            {vconsole_panel(panel_open)}
        }
    }
}
