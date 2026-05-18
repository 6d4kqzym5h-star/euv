use crate::*;

/// Renders a navigation item link with active state styling.
///
/// # Arguments
///
/// - `Signal<String>` - The reactive signal holding the current route.
/// - `&str` - The display label for the navigation item.
/// - `&str` - The target route path.
///
/// # Returns
///
/// - `VirtualNode` - The navigation item virtual DOM tree.
pub fn nav_item(route_signal: Signal<String>, label: &str, target: &str) -> VirtualNode {
    let target_string: String = target.to_string();
    let current_route_value: String = route_signal.get();
    let is_active: bool = current_route_value == target;
    html! {
        a {
            href: format!("#{}", target_string)
            class: if { is_active } { c_nav_item_active() } else { c_nav_item_inactive() }
            onclick: link_handler(target_string)
            label
        }
    }
}

/// Renders a mobile navigation item link that closes the drawer on navigation.
///
/// # Arguments
///
/// - `Signal<String>` - The reactive signal holding the current route.
/// - `Signal<bool>` - The reactive signal controlling the mobile nav drawer visibility.
/// - `&str` - The display label for the navigation item.
/// - `&str` - The target route path.
///
/// # Returns
///
/// - `VirtualNode` - The mobile navigation item virtual DOM tree.
fn mobile_nav_item(
    route_signal: Signal<String>,
    drawer_open: Signal<bool>,
    label: &str,
    target: &str,
) -> VirtualNode {
    let target_string: String = target.to_string();
    let current_route_value: String = route_signal.get();
    let is_active: bool = current_route_value == target;
    let nav_target: String = target_string.clone();
    html! {
        a {
            href: format!("#{}", target_string)
            class: if { is_active } { c_nav_item_active() } else { c_nav_item_inactive() }
            onclick: move |_event: NativeEvent| {
                navigate(&nav_target);
                drawer_open.set(false);
            }
            label
        }
    }
}

/// Builds the navigation items list for the desktop sidebar.
///
/// # Arguments
///
/// - `Signal<String>` - The reactive signal holding the current route.
///
/// # Returns
///
/// - `VirtualNode` - The scrollable nav items container virtual DOM tree.
fn build_desktop_nav_items(route_signal: Signal<String>) -> VirtualNode {
    html! {
        div {
            class: c_nav_items_scroll()
            { nav_item(route_signal, "Signals", "/signals") }
            { nav_item(route_signal, "Event", "/event") }
            { nav_item(route_signal, "List", "/list") }
            { nav_item(route_signal, "Conditional", "/conditional") }
            { nav_item(route_signal, "Modal", "/modal") }
            { nav_item(route_signal, "Select", "/select") }
            { nav_item(route_signal, "Async", "/async") }
            { nav_item(route_signal, "Form", "/form") }
            { nav_item(route_signal, "File Upload", "/file-upload") }
            { nav_item(route_signal, "Timer", "/timer") }
            { nav_item(route_signal, "Animation", "/animation") }
            { nav_item(route_signal, "Browser", "/browser") }
            { nav_item(route_signal, "Lifecycle", "/lifecycle") }
            { nav_item(route_signal, "Keep-Alive", "/keep-alive") }
            { nav_item(route_signal, "Component Binding", "/component-binding") }
            { nav_item(route_signal, "Custom Attrs", "/custom-attrs") }
        }
    }
}

/// Builds the navigation items list for the mobile drawer.
///
/// Navigation items close the drawer on click.
///
/// # Arguments
///
/// - `Signal<String>` - The reactive signal holding the current route.
/// - `Signal<bool>` - The reactive signal controlling the mobile nav drawer visibility.
///
/// # Returns
///
/// - `VirtualNode` - The scrollable nav items container virtual DOM tree.
fn build_mobile_nav_items(route_signal: Signal<String>, drawer_open: Signal<bool>) -> VirtualNode {
    html! {
        div {
            class: c_nav_items_scroll()
            { mobile_nav_item(route_signal, drawer_open, "Signals", "/signals") }
            { mobile_nav_item(route_signal, drawer_open, "Event", "/event") }
            { mobile_nav_item(route_signal, drawer_open, "List", "/list") }
            { mobile_nav_item(route_signal, drawer_open, "Conditional", "/conditional") }
            { mobile_nav_item(route_signal, drawer_open, "Modal", "/modal") }
            { mobile_nav_item(route_signal, drawer_open, "Select", "/select") }
            { mobile_nav_item(route_signal, drawer_open, "Async", "/async") }
            { mobile_nav_item(route_signal, drawer_open, "Form", "/form") }
            { mobile_nav_item(route_signal, drawer_open, "File Upload", "/file-upload") }
            { mobile_nav_item(route_signal, drawer_open, "Timer", "/timer") }
            { mobile_nav_item(route_signal, drawer_open, "Animation", "/animation") }
            { mobile_nav_item(route_signal, drawer_open, "Browser", "/browser") }
            { mobile_nav_item(route_signal, drawer_open, "Lifecycle", "/lifecycle") }
            { mobile_nav_item(route_signal, drawer_open, "Keep-Alive", "/keep-alive") }
            { mobile_nav_item(route_signal, drawer_open, "Component Binding", "/component-binding") }
            { mobile_nav_item(route_signal, drawer_open, "Custom Attrs", "/custom-attrs") }
        }
    }
}

/// Renders the desktop layout with a persistent left sidebar navigation.
///
/// # Arguments
///
/// - `Signal<String>` - The reactive signal holding the current route.
/// - `Signal<String>` - The reactive signal holding the current theme.
/// - `Signal<String>` - The reactive signal holding the root class name.
/// - `Signal<bool>` - The reactive signal controlling vconsole panel visibility.
///
/// # Returns
///
/// - `VirtualNode` - The desktop application shell virtual DOM tree.
fn desktop_layout(
    route_signal: Signal<String>,
    theme_signal: Signal<String>,
    root_class_signal: Signal<String>,
    panel_open: Signal<bool>,
) -> VirtualNode {
    html! {
        div {
            class: root_class_signal
            nav {
                class: c_app_nav()
                a {
                    href: "https://github.com/euv-dev/euv"
                    target: "_blank"
                    class: c_nav_header()
                    span {
                        class: c_nav_logo()
                        "E"
                    }
                    span {
                        class: c_inline()
                        "Euv"
                    }
                }
                p {
                    class: c_nav_section_label()
                    "Pages"
                }
                { build_desktop_nav_items(route_signal) }
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
                a {
                    href: "https://github.com/euv-dev/euv"
                    target: "_blank"
                    class: c_nav_footer()
                    "Built with Euv & WASM"
                }
            }
            main {
                class: c_app_main()
                match { route_signal.get().as_str() } {
                    "/" | "/signals" => page_signals(),
                    "/event" => page_event(),
                    "/list" => page_list(),
                    "/conditional" => page_conditional(),
                    "/modal" => page_modal(),
                    "/select" => page_select(),
                    "/async" => page_async_demo(),
                    "/form" => page_form(),
                    "/file-upload" => page_file_upload(),
                    "/timer" => page_timer(),
                    "/animation" => page_animation(),
                    "/browser" => page_browser(),
                    "/lifecycle" => page_lifecycle(),
                    "/keep-alive" => page_keep_alive(),
                    "/component-binding" => page_component_binding(),
                    "/custom-attrs" => page_custom_attrs(),
                    _ => page_not_found()
                }
            }
            { vconsole_panel(panel_open) }
        }
    }
}

/// Renders the mobile layout with a top header bar and a slide-out navigation drawer.
///
/// # Arguments
///
/// - `Signal<String>` - The reactive signal holding the current route.
/// - `Signal<String>` - The reactive signal holding the current theme.
/// - `Signal<String>` - The reactive signal holding the root class name.
/// - `Signal<bool>` - The reactive signal controlling vconsole panel visibility.
/// - `Signal<bool>` - The reactive signal controlling the mobile nav drawer visibility.
///
/// # Returns
///
/// - `VirtualNode` - The mobile application shell virtual DOM tree.
fn mobile_layout(
    route_signal: Signal<String>,
    theme_signal: Signal<String>,
    root_class_signal: Signal<String>,
    panel_open: Signal<bool>,
    drawer_open: Signal<bool>,
) -> VirtualNode {
    html! {
        div {
            class: root_class_signal
            header {
                class: c_mobile_header()
                div {
                    class: c_mobile_header_left()
                    button {
                        class: if { drawer_open.get() } { c_mobile_menu_button_active() } else { c_mobile_menu_button() }
                        onclick: use_toggle(drawer_open)
                        "☰"
                    }
                    span {
                        class: c_nav_logo()
                        "E"
                    }
                    span {
                        class: c_inline()
                        "euv"
                    }
                }
                button {
                    class: c_mobile_menu_button()
                    onclick: toggle_theme(theme_signal)
                    if { theme_signal.get() == "dark" } {
                        "☀"
                    } else {
                        "🌙"
                    }
                }
            }
            main {
                class: c_mobile_main()
                match { route_signal.get().as_str() } {
                    "/" | "/signals" => page_signals(),
                    "/event" => page_event(),
                    "/list" => page_list(),
                    "/conditional" => page_conditional(),
                    "/modal" => page_modal(),
                    "/select" => page_select(),
                    "/async" => page_async_demo(),
                    "/form" => page_form(),
                    "/file-upload" => page_file_upload(),
                    "/timer" => page_timer(),
                    "/animation" => page_animation(),
                    "/browser" => page_browser(),
                    "/lifecycle" => page_lifecycle(),
                    "/keep-alive" => page_keep_alive(),
                    "/component-binding" => page_component_binding(),
                    "/custom-attrs" => page_custom_attrs(),
                    _ => page_not_found()
                }
            }
            { vconsole_panel(panel_open) }
            if { drawer_open.get() } {
                div {
                    class: c_mobile_overlay()
                    onclick: move |_event: NativeEvent| {
                        drawer_open.set(false);
                    }
                }
                nav {
                    class: c_mobile_nav_drawer()
                    div {
                        class: c_mobile_nav_drawer_header()
                        a {
                            href: "https://github.com/euv-dev/euv"
                            target: "_blank"
                            class: c_nav_header()
                            span {
                                class: c_nav_logo()
                                "E"
                            }
                            span {
                                class: c_inline()
                                "Euv"
                            }
                        }
                        button {
                            class: c_mobile_nav_drawer_close()
                            onclick: move |_event: NativeEvent| {
                                drawer_open.set(false);
                            }
                            "✕"
                        }
                    }
                    p {
                        class: c_nav_section_label()
                        "Pages"
                    }
                    { build_mobile_nav_items(route_signal, drawer_open) }
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
                    a {
                        href: "https://github.com/euv-dev/euv"
                        target: "_blank"
                        class: c_nav_footer()
                        "Built with Euv & WASM"
                    }
                }
            } else {
                ""
            }
        }
    }
}

/// Renders the application shell with navigation and route-based page content.
///
/// Detects viewport size and switches between desktop sidebar layout and
/// mobile header + drawer layout accordingly.
///
/// # Returns
///
/// - `VirtualNode` - The root application virtual DOM tree.
pub fn app() -> VirtualNode {
    init_console();
    let route_signal: Signal<String> = use_signal(current_route);
    let panel_open: Signal<bool> = use_signal(|| false);
    let drawer_open: Signal<bool> = use_signal(|| false);
    let mobile_signal: Signal<bool> = use_resize();
    let theme_state: ThemeState = use_theme(mobile_signal);
    let theme_signal: Signal<String> = theme_state.get_theme();
    let root_class: Signal<String> = theme_state.get_root_class();
    use_hash_change(route_signal);
    html! {
        if { mobile_signal.get() } {
            mobile_layout(route_signal, theme_signal, root_class, panel_open, drawer_open)
        } else {
            desktop_layout(route_signal, theme_signal, root_class, panel_open)
        }
    }
}
