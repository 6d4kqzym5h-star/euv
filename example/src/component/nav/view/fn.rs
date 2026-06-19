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
