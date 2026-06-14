use crate::*;

/// Renders a navigation item link with active state styling.
///
/// # Arguments
///
/// - `NavItemProps` - The typed props containing route signal, icon, label, and target.
/// - `VirtualNode` - The children nodes.
///
/// # Returns
///
/// - `VirtualNode` - The navigation item virtual DOM tree.
#[component]
pub(crate) fn nav_item(node: VirtualNode<NavItemProps>) -> VirtualNode {
    let NavItemProps {
        route_signal,
        icon,
        label,
        target,
    }: NavItemProps = node.try_get_props().unwrap_or_default();
    let target_string: String = target.to_string();
    let current_route_value: String = route_signal.get();
    let is_active: bool = current_route_value == target;
    let on_nav_click: NativeEventHandler = link_handler(target_string.clone());
    html! {
        a {
            href: format!("#{target_string}")
            target: "_blank"
            class: if { is_active } { c_nav_item_active() } else { c_nav_item_inactive() }
            onclick: on_nav_click
            span {
                class: c_nav_item_icon()
                icon
            }
            span {
                class: c_nav_item_label()
                label
            }
        }
    }
}

/// Renders a mobile navigation item link that closes the drawer on navigation.
///
/// # Arguments
///
/// - `MobileNavItemProps` - The typed props containing route signal, drawer open signal, icon, label, and target.
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
        icon,
        label,
        target,
    }: MobileNavItemProps = node.try_get_props().unwrap_or_default();
    let target_string: String = target.to_string();
    let current_route_value: String = route_signal.get();
    let is_active: bool = current_route_value == target;
    let nav_target: String = target_string.clone();
    let on_mobile_nav_click = move |event: Event| {
        event.prevent_default();
        close_drawer_and_navigate(drawer_open, nav_target.clone());
    };
    html! {
        a {
            href: format!("#{target_string}")
            target: "_blank"
            class: if { is_active } { c_nav_item_active() } else { c_nav_item_inactive() }
            onclick: on_mobile_nav_click
            span {
                class: c_nav_item_icon()
                icon
            }
            span {
                class: c_nav_item_label()
                label
            }
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
    let BuildDesktopNavItemsProps { route_signal }: BuildDesktopNavItemsProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_nav_items_scroll()
            nav_item {
                route_signal: route_signal
                icon: "🏠"
                label: "Home"
                target: "/"
            }
            nav_item {
                route_signal: route_signal
                icon: "🔢"
                label: "Counter"
                target: "/counter"
            }
            nav_item {
                route_signal: route_signal
                icon: "🏷️"
                label: "Badge"
                target: "/badge"
            }
            nav_item {
                route_signal: route_signal
                icon: "🎯"
                label: "Event"
                target: "/event"
            }
            nav_item {
                route_signal: route_signal
                icon: "📝"
                label: "List"
                target: "/list"
            }
            nav_item {
                route_signal: route_signal
                icon: "👁️"
                label: "Observer"
                target: "/observer"
            }
            nav_item {
                route_signal: route_signal
                icon: "🔀"
                label: "Condition"
                target: "/conditional"
            }
            nav_item {
                route_signal: route_signal
                icon: "💬"
                label: "Modal"
                target: "/modal"
            }
            nav_item {
                route_signal: route_signal
                icon: "📋"
                label: "Select"
                target: "/select"
            }
            nav_item {
                route_signal: route_signal
                icon: "⏳"
                label: "Async"
                target: "/async"
            }
            nav_item {
                route_signal: route_signal
                icon: "📄"
                label: "Form"
                target: "/form"
            }
            nav_item {
                route_signal: route_signal
                icon: "📁"
                label: "Upload"
                target: "/file-upload"
            }
            nav_item {
                route_signal: route_signal
                icon: "⏱️"
                label: "Timer"
                target: "/timer"
            }
            nav_item {
                route_signal: route_signal
                icon: "🎬"
                label: "Animation"
                target: "/animation"
            }
            nav_item {
                route_signal: route_signal
                icon: "🌐"
                label: "Browser"
                target: "/browser"
            }
            nav_item {
                route_signal: route_signal
                icon: "♻️"
                label: "Lifecycle"
                target: "/lifecycle"
            }
            nav_item {
                route_signal: route_signal
                icon: "💚"
                label: "KeepAlive"
                target: "/keep-alive"
            }
            nav_item {
                route_signal: route_signal
                icon: "🔗"
                label: "Binding"
                target: "/component-binding"
            }
            nav_item {
                route_signal: route_signal
                icon: "⚙️"
                label: "Attrs"
                target: "/custom-attrs"
            }
            nav_item {
                route_signal: route_signal
                icon: "🏷️"
                label: "DynTag"
                target: "/dynamic-component"
            }
            nav_item {
                route_signal: route_signal
                icon: "📊"
                label: "VList"
                target: "/virtual-list"
            }
            nav_item {
                route_signal: route_signal
                icon: "📷"
                label: "Camera"
                target: "/camera"
            }
            nav_item {
                route_signal: route_signal
                icon: "🎨"
                label: "Canvas"
                target: "/canvas"
            }
            nav_item {
                route_signal: route_signal
                icon: "🏷️"
                label: "Tags"
                target: "/tags"
            }
            nav_item {
                route_signal: route_signal
                icon: "📡"
                label: "SSE"
                target: "/sse"
            }
            nav_item {
                route_signal: route_signal
                icon: "📌"
                label: "Sticky"
                target: "/sticky"
            }
            nav_item {
                route_signal: route_signal
                icon: "🔌"
                label: "WebSocket"
                target: "/websocket"
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
    }: BuildMobileNavItemsProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_nav_items_scroll()
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "🏠"
                label: "Home"
                target: "/"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "🔢"
                label: "Counter"
                target: "/counter"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "🏷️"
                label: "Badge"
                target: "/badge"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "🎯"
                label: "Event"
                target: "/event"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "📝"
                label: "List"
                target: "/list"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "👁️"
                label: "Observer"
                target: "/observer"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "🔀"
                label: "Condition"
                target: "/conditional"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "💬"
                label: "Modal"
                target: "/modal"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "📋"
                label: "Select"
                target: "/select"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "⏳"
                label: "Async"
                target: "/async"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "📄"
                label: "Form"
                target: "/form"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "📁"
                label: "Upload"
                target: "/file-upload"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "⏱️"
                label: "Timer"
                target: "/timer"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "🎬"
                label: "Animation"
                target: "/animation"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "🌐"
                label: "Browser"
                target: "/browser"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "♻️"
                label: "Lifecycle"
                target: "/lifecycle"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "💚"
                label: "KeepAlive"
                target: "/keep-alive"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "🔗"
                label: "Binding"
                target: "/component-binding"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "⚙️"
                label: "Attrs"
                target: "/custom-attrs"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "🏷️"
                label: "DynTag"
                target: "/dynamic-component"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "📊"
                label: "VList"
                target: "/virtual-list"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "📷"
                label: "Camera"
                target: "/camera"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "🎨"
                label: "Canvas"
                target: "/canvas"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "🏷️"
                label: "Tags"
                target: "/tags"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "📡"
                label: "SSE"
                target: "/sse"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "📌"
                label: "Sticky"
                target: "/sticky"
            }
            mobile_nav_item {
                route_signal: route_signal
                drawer_open: drawer_open
                icon: "🔌"
                label: "WebSocket"
                target: "/websocket"
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
        theme_signal: _,
        root_class_signal,
        panel_open,
    }: DesktopLayoutProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: root_class_signal
            nav {
                class: c_app_nav()
                a {
                    href: GITHUB_URL
                    target: "_blank"
                    onclick: external_link_handler(GITHUB_URL.to_string())
                    class: c_nav_header()
                    logo_button {
                        variant: LogoButtonVariant::Nav
                    }
                    span {
                        class: c_nav_brand_title()
                        BRAND_NAME
                    }
                }
                p {
                    class: c_nav_section_label()
                    "Pages"
                }
                build_desktop_nav_items {
                    route_signal: route_signal
                }
                a {
                    href: GITHUB_URL
                    target: "_blank"
                    onclick: external_link_handler(GITHUB_URL.to_string())
                    class: c_nav_footer()
                    div {
                        class: c_nav_footer_divider()
                    }
                    span {
                        class: c_nav_footer_text()
                        "Built with"
                    }
                    span {
                        class: c_nav_footer_brand()
                        BRAND_NAME
                    }
                    span {
                        class: c_nav_footer_badge()
                        "WASM"
                    }
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
    }: MobileLayoutProps = node.try_get_props().unwrap_or_default();
    let on_overlay_click = move |_: Event| {
        overlay_back(None);
        drawer_open.set(false);
    };
    let on_drawer_close_click = move |_: Event| {
        overlay_back(None);
        drawer_open.set(false);
    };
    html! {
        div {
            class: root_class_signal
            header {
                class: c_mobile_header()
                    div {
                        class: c_mobile_header_left()
                        button {
                            class: if { drawer_open.get() } { c_mobile_menu_button_active() } else { c_mobile_menu_button() }
                            onclick: use_drawer_toggle(drawer_open)
                            "☰"
                        }
                        a {
                            href: GITHUB_URL
                            target: "_blank"
                            onclick: external_link_handler(GITHUB_URL.to_string())
                            class: c_mobile_header_logo()
                            logo_button {
                                variant: LogoButtonVariant::Nav
                            }
                            span {
                                class: c_nav_brand_title()
                                BRAND_NAME
                            }
                        }
                    }
                button {
                    class: c_mobile_theme_button()
                    onclick: toggle_theme(theme_signal)
                    div {
                        class: if { theme_signal.get() == THEME_DARK } { c_theme_icon_sun() } else { c_theme_icon_moon() }
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
            div {
                class: if { drawer_open.get() } { c_mobile_overlay().to_string() } else { format!("{} {}", c_mobile_overlay().get_name(), c_mobile_overlay_hidden().get_name()) }
                onclick: on_overlay_click
            }
            nav {
                class: if { drawer_open.get() } { c_mobile_nav_drawer().to_string() } else { format!("{} {}", c_mobile_nav_drawer().get_name(), c_mobile_nav_drawer_closed().get_name()) }
                div {
                    class: c_mobile_nav_drawer_header()
                    div {
                        class: c_mobile_header_left()
                        button {
                            class: if { drawer_open.get() } { c_mobile_menu_button_active() } else { c_mobile_menu_button() }
                            onclick: use_drawer_toggle(drawer_open)
                            "☰"
                        }
                        a {
                            href: GITHUB_URL
                            target: "_blank"
                            onclick: external_link_handler(GITHUB_URL.to_string())
                            class: c_mobile_header_logo()
                            logo_button {
                                variant: LogoButtonVariant::Nav
                            }
                            span {
                                class: c_nav_brand_title()
                                BRAND_NAME
                            }
                        }
                    }
                    button {
                        class: c_mobile_drawer_close_button()
                        onclick: on_drawer_close_click
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
                a {
                    href: GITHUB_URL
                    target: "_blank"
                    onclick: external_link_handler(GITHUB_URL.to_string())
                    class: c_nav_footer()
                    div {
                        class: c_nav_footer_divider()
                    }
                    span {
                        class: c_nav_footer_text()
                        "Built with"
                    }
                    span {
                        class: c_nav_footer_brand()
                        BRAND_NAME
                    }
                    span {
                        class: c_nav_footer_badge()
                        "WASM"
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
    use_overlay_history(panel_open, drawer_open, mobile_signal);
    use_scroll_drawer_to_active(drawer_open);
    use_safe_area_fix();
    use_keyboard_inset_fix();
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
