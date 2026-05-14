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
    let background: String = if is_active {
        "rgba(79, 70, 229, 0.08)".to_string()
    } else {
        "transparent".to_string()
    };
    let active_color: String = if is_active {
        "#4f46e5".to_string()
    } else {
        "#6b7280".to_string()
    };
    let active_weight: String = if is_active {
        "600".to_string()
    } else {
        "400".to_string()
    };
    let active_border: String = if is_active {
        "3px solid #4f46e5".to_string()
    } else {
        "3px solid transparent".to_string()
    };
    rsx! {
        a {
            href: format!("#{}", target_string)
            style: {display: "block"; padding: "11px 20px"; text_decoration: "none"; color: {active_color}; font_weight: {active_weight}; border_left: {active_border}; background: {background}; font_size: "14px"; transition: "all 0.15s ease";}
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
    let route_signal: Signal<String> = use_signal(current_route);
    let route_updater: Signal<String> = route_signal;
    let window: Window = window().expect("no global window exists");
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let new_route: String = current_route();
        route_updater.set(new_route);
    }));
    window
        .add_event_listener_with_callback(
            &NativeEventName::HashChange.to_string(),
            closure.as_ref().unchecked_ref(),
        )
        .unwrap();
    closure.forget();
    rsx! {
        div {
            class: c_app_root()
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
                        "euv playground"
                    }
                }
                p {
                    class: c_nav_section_label()
                    "Pages"
                }
                {nav_item(route_signal, "Home", "/")}
                {nav_item(route_signal, "List", "/list")}
                {nav_item(route_signal, "Conditional", "/conditional")}
                {nav_item(route_signal, "Async", "/async")}
                {nav_item(route_signal, "Form", "/form")}
                {nav_item(route_signal, "Lifecycle", "/lifecycle")}
                p {
                    class: c_nav_footer()
                    "Built with euv & WASM"
                }
            }
            main {
                class: c_app_main()
                match {route_signal.get().as_str()} {
                    "/" => {
                        page_home()
                    }
                    "/list" => {
                        page_list()
                    }
                    "/conditional" => {
                        page_conditional()
                    }
                    "/async" => {
                        page_async_demo()
                    }
                    "/form" => {
                        page_form()
                    }
                    "/lifecycle" => {
                        page_lifecycle()
                    }
                    _ => {
                        page_not_found()
                    }
                }
            }
        }
    }
}
