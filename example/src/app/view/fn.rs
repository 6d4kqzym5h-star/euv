use crate::*;

/// Renders a navigation item link with active state styling.
///
/// # Arguments
///
/// - `NavItemProps` - The typed props containing route signal, label, and target.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The navigation item virtual DOM tree.
#[component]
pub(crate) fn nav_item(node: VirtualNode<NavItemProps>) -> VirtualNode {
    let NavItemProps {
        route_signal,
        label,
        target,
    } = node.try_get_props().unwrap_or_default();
    let target_string: String = target.to_string();
    let current_route_value: String = route_signal.get();
    let is_active: bool = current_route_value == target;
    html! {
        a {
            href: format!("#{target_string}")
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
/// - `MobileNavItemProps` - The typed props containing route signal, drawer open signal, label, and target.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The mobile navigation item virtual DOM tree.
#[component]
pub(crate) fn mobile_nav_item(node: VirtualNode<MobileNavItemProps>) -> VirtualNode {
    let MobileNavItemProps {
        route_signal,
        drawer_open,
        label,
        target,
    } = node.try_get_props().unwrap_or_default();
    let target_string: String = target.to_string();
    let current_route_value: String = route_signal.get();
    let is_active: bool = current_route_value == target;
    let nav_target: String = target_string.clone();
    html! {
        a {
            href: format!("#{target_string}")
            class: if { is_active } { c_nav_item_active() } else { c_nav_item_inactive() }
            onclick: move |_event: Event| {
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
/// - `BuildDesktopNavItemsProps` - The typed props containing the route signal.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The scrollable nav items container virtual DOM tree.
#[component]
pub(crate) fn build_desktop_nav_items(node: VirtualNode<BuildDesktopNavItemsProps>) -> VirtualNode {
    let BuildDesktopNavItemsProps { route_signal } = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_nav_items_scroll()
            nav_item {
                route_signal: route_signal
                label: "Signals"
                target: "/signals"
            }
            nav_item {
                route_signal: route_signal
                label: "Event"
                target: "/event"
            }
            nav_item {
                route_signal: route_signal
                label: "List"
                target: "/list"
            }
            nav_item {
                route_signal: route_signal
                label: "Observer"
                target: "/observer"
            }
            nav_item {
                route_signal: route_signal
                label: "Conditional"
                target: "/conditional"
            }
            nav_item {
                route_signal: route_signal
                label: "Modal"
                target: "/modal"
            }
            nav_item {
                route_signal: route_signal
                label: "Select"
                target: "/select"
            }
            nav_item {
                route_signal: route_signal
                label: "Async"
                target: "/async"
            }
            nav_item {
                route_signal: route_signal
                label: "Form"
                target: "/form"
            }
            nav_item {
                route_signal: route_signal
                label: "File Upload"
                target: "/file-upload"
            }
            nav_item {
                route_signal: route_signal
                label: "Timer"
                target: "/timer"
            }
            nav_item {
                route_signal: route_signal
                label: "Animation"
                target: "/animation"
            }
            nav_item {
                route_signal: route_signal
                label: "Browser"
                target: "/browser"
            }
            nav_item {
                route_signal: route_signal
                label: "Lifecycle"
                target: "/lifecycle"
            }
            nav_item {
                route_signal: route_signal
                label: "Keep-Alive"
                target: "/keep-alive"
            }
            nav_item {
                route_signal: route_signal
                label: "Component Binding"
                target: "/component-binding"
            }
            nav_item {
                route_signal: route_signal
                label: "Custom Attrs"
                target: "/custom-attrs"
            }
            nav_item {
                route_signal: route_signal
                label: "Dynamic Component"
                target: "/dynamic-component"
            }
            nav_item {
                route_signal: route_signal
                label: "Virtual List"
                target: "/virtual-list"
            }
        }
    }
}

/// Builds the navigation items list for the mobile drawer.
///
/// Navigation items close the drawer on click.
///
/// # Arguments
///
/// - `BuildMobileNavItemsProps` - The typed props containing route signal and drawer open signal.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The scrollable nav items container virtual DOM tree.
#[component]
pub(crate) fn build_mobile_nav_items(node: VirtualNode<BuildMobileNavItemsProps>) -> VirtualNode {
    let BuildMobileNavItemsProps {
        route_signal,
        drawer_open,
    } = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_nav_items_scroll()
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Signals"
                target: "/signals"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Event"
                target: "/event"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "List"
                target: "/list"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Observer"
                target: "/observer"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Conditional"
                target: "/conditional"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Modal"
                target: "/modal"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Select"
                target: "/select"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Async"
                target: "/async"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Form"
                target: "/form"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "File Upload"
                target: "/file-upload"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Timer"
                target: "/timer"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Animation"
                target: "/animation"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Browser"
                target: "/browser"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Lifecycle"
                target: "/lifecycle"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Keep-Alive"
                target: "/keep-alive"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Component Binding"
                target: "/component-binding"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Custom Attrs"
                target: "/custom-attrs"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Dynamic Component"
                target: "/dynamic-component"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                label: "Virtual List"
                target: "/virtual-list"
            }
        }
    }
}

/// Renders the desktop layout with a persistent left sidebar navigation.
///
/// # Arguments
///
/// - `DesktopLayoutProps` - The typed props containing route, theme, root class, and panel signals.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The desktop application shell virtual DOM tree.
#[component]
pub(crate) fn desktop_layout(node: VirtualNode<DesktopLayoutProps>) -> VirtualNode {
    let DesktopLayoutProps {
        route_signal,
        theme_signal,
        root_class_signal,
        panel_open,
    } = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: root_class_signal
            nav {
                class: c_app_nav()
                a {
                    href: "https://github.com/euv-dev/euv"
                    target: "_blank"
                    class: c_nav_header()
                    logo_button {
                        variant: LogoButtonVariant::Nav
                    }
                    span {
                        class: c_nav_brand_title()
                        "Euv"
                    }
                }
                p {
                    class: c_nav_section_label()
                    "Pages"
                }
                build_desktop_nav_items {
                    route_signal: route_signal
                }
                div {
                    class: c_nav_theme_toggle()
                    button {
                        class: c_nav_theme_button()
                        onclick: toggle_theme(theme_signal)
                        if { theme_signal.get() == THEME_DARK } {
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
                page_router {
                    route_signal
                }
            }
            vconsole_panel {
                panel_open
            }
        }
    }
}

/// Renders the mobile layout with a top header bar and a slide-out navigation drawer.
///
/// # Arguments
///
/// - `MobileLayoutProps` - The typed props containing route, theme, root class, panel, and drawer signals.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The mobile application shell virtual DOM tree.
#[component]
pub(crate) fn mobile_layout(node: VirtualNode<MobileLayoutProps>) -> VirtualNode {
    let MobileLayoutProps {
        route_signal,
        theme_signal,
        root_class_signal,
        panel_open,
        drawer_open,
    } = node.try_get_props().unwrap_or_default();
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
                    logo_button {
                        variant: LogoButtonVariant::Nav
                    }
                    span {
                        class: c_nav_brand_title()
                        "Euv"
                    }
                }
                button {
                    class: c_mobile_menu_button()
                    onclick: toggle_theme(theme_signal)
                    if { theme_signal.get() == THEME_DARK } {
                        "☀"
                    } else {
                        "🌙"
                    }
                }
            }
            main {
                class: c_mobile_main()
                page_router {
                    route_signal
                }
            }
            vconsole_panel {
                panel_open
            }
            if { drawer_open.get() } {
                div {
                    class: c_mobile_overlay()
                    onclick: move |_event: Event| {
                        drawer_open.set(false);
                    }
                }
                nav {
                    class: c_mobile_nav_drawer()
                    div {
                        class: c_mobile_nav_drawer_header()
                        div {
                            class: c_mobile_header_left()
                            button {
                                class: c_mobile_menu_button_active()
                                onclick: use_toggle(drawer_open)
                                "☰"
                            }
                            logo_button {
                                variant: LogoButtonVariant::Nav
                            }
                            span {
                                class: c_nav_brand_title()
                                "Euv"
                            }
                        }
                        button {
                            class: c_mobile_menu_button()
                            onclick: move |_event: Event| {
                                drawer_open.set(false);
                            }
                            "✕"
                        }
                    }
                    p {
                        class: c_nav_section_label()
                        "Pages"
                    }
                    build_mobile_nav_items {
                        route_signal: route_signal
                        drawer_open: drawer_open
                    }
                    div {
                        class: c_nav_theme_toggle()
                        button {
                            class: c_nav_theme_button()
                            onclick: toggle_theme(theme_signal)
                            if { theme_signal.get() == THEME_DARK } {
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
pub(crate) fn app() -> VirtualNode {
    init_console();
    let route_signal: Signal<String> = use_signal(current_route);
    let panel_open: Signal<bool> = use_signal(|| false);
    let drawer_open: Signal<bool> = use_signal(|| false);
    let mobile_signal: Signal<bool> = use_resize();
    let theme_state: ThemeState = use_theme(mobile_signal);
    let theme_signal: Signal<String> = theme_state.get_theme();
    let root_class_signal: Signal<String> = theme_state.get_root_class();
    use_hash_change(route_signal);
    use_scroll_to_top(route_signal);
    use_pop_state(panel_open);
    use_scroll_drawer_to_active(drawer_open);
    html! {
        if { mobile_signal.get() } {
            mobile_layout {
                route_signal
                theme_signal
                root_class_signal
                panel_open
                drawer_open
            }
        } else {
            desktop_layout {
                route_signal
                theme_signal
                root_class_signal
                panel_open
            }
        }
    }
}
