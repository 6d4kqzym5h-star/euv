use super::*;

/// Renders a navigation item link with active state styling.
///
/// # Arguments
///
/// - `VirtualNode<EuvNavItemProps>` - The props node containing route signal, icon, label, and target.
///
/// # Returns
///
/// - `VirtualNode` - The navigation item virtual DOM tree.
#[component]
pub fn euv_nav_item(node: VirtualNode<EuvNavItemProps>) -> VirtualNode {
    let EuvNavItemProps {
        route_signal,
        icon,
        label,
        target,
        on_click,
        class,
    }: EuvNavItemProps = node.try_get_props().unwrap_or_default();
    let target_string: String = target.to_string();
    let current_route_value: String = route_signal.get();
    let is_active: bool = current_route_value == target;
    let click_handler: NativeEventHandler = match on_click {
        Some(handler) => NativeEventHandler::create("click", move |event: Event| {
            event.prevent_default();
            handler(event);
        }),
        None => Router::link_handler(target_string.clone()),
    };
    let base_class: &Css = if is_active {
        c_nav_item_active()
    } else {
        c_nav_item_inactive()
    };
    let effective_class: String = match class {
        Some(custom_class) => format!("{} {}", base_class.get_name(), custom_class.get_name()),
        None => base_class.get_name().to_string(),
    };
    html! {
        a {
            href: format!("#{target_string}")
            target: BLANK_BROWSER_TARGET
            class: effective_class
            onclick: click_handler
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
/// - `VirtualNode<EuvMobileNavItemProps>` - The props node containing route signal, drawer open signal, icon, label, and target.
///
/// # Returns
///
/// - `VirtualNode` - The mobile navigation item virtual DOM tree.
#[component]
pub fn euv_mobile_nav_item(node: VirtualNode<EuvMobileNavItemProps>) -> VirtualNode {
    let EuvMobileNavItemProps {
        route_signal,
        drawer_open: _,
        icon,
        label,
        target,
        on_navigate,
    }: EuvMobileNavItemProps = node.try_get_props().unwrap_or_default();
    let target_string: String = target.to_string();
    let current_route_value: String = route_signal.get();
    let is_active: bool = current_route_value == target;
    let nav_target: String = target_string.clone();
    let on_mobile_nav_click = move |event: Event| {
        event.prevent_default();
        Router::overlay_back(Some(nav_target.clone()));
        if let Some(ref callback) = on_navigate {
            callback();
        }
    };
    html! {
        a {
            href: format!("#{target_string}")
            target: BLANK_BROWSER_TARGET
            class: if is_active {
                c_nav_item_active()
            } else {
                c_nav_item_inactive()
            }
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

/// Renders a configurable navigation items list.
///
/// Automatically renders desktop or mobile nav items based on whether
/// `drawer_open` signal is provided.
///
/// # Arguments
///
/// - `VirtualNode<EuvNavItemsProps>` - The props node containing items configuration.
///
/// # Returns
///
/// - `VirtualNode` - The scrollable nav items container virtual DOM tree.
#[component]
pub fn euv_nav_items(node: VirtualNode<EuvNavItemsProps>) -> VirtualNode {
    let EuvNavItemsProps {
        route_signal,
        items,
        drawer_open,
        on_item_click,
    }: EuvNavItemsProps = node.try_get_props().unwrap_or_default();
    let children: Vec<VirtualNode> =
        items
            .into_iter()
            .map(|item: EuvNavItemConfig| {
                let EuvNavItemConfig {
                    icon,
                    label,
                    target,
                } = item;
                if let Some(drawer_open_signal) = drawer_open {
                    let on_navigate: Option<NavEventCallback> = on_item_click.as_ref().map(
                        |cb: &NavItemClickCallback| -> NavEventCallback {
                            let callback: NavItemClickCallback = cb.clone();
                            let target_str: String = target.to_string();
                            Rc::new(move || callback(&target_str))
                        },
                    );
                    html! {
                        euv_mobile_nav_item {
                            route_signal: route_signal
                            drawer_open: drawer_open_signal
                            icon: icon
                            label: label
                            target: target
                            on_navigate: on_navigate
                        }
                    }
                } else {
                    let on_click: Option<ClickEventHandler> = on_item_click.as_ref().map(
                        |cb: &NavItemClickCallback| -> ClickEventHandler {
                            let callback: NavItemClickCallback = cb.clone();
                            let target_str: String = target.to_string();
                            Rc::new(move |_: Event| callback(&target_str))
                        },
                    );
                    html! {
                        euv_nav_item {
                            route_signal: route_signal
                            icon: icon
                            label: label
                            target: target
                            on_click: on_click
                        }
                    }
                }
            })
            .collect();
    html! {
        div {
            class: c_nav_items_scroll()
            children
        }
    }
}
