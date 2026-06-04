use crate::*;

/// Reads the current hash-based route from the browser URL.
///
/// # Returns
///
/// - `String` - The hash fragment without the leading `#`, or `"/"` if empty.
pub(crate) fn current_route() -> String {
    let window: Window = window().expect("no global window exists");
    let hash: String = window.location().hash().unwrap_or_default();
    let route: String = hash.strip_prefix('#').unwrap_or(&hash).to_string();
    if route.is_empty() {
        "/".to_string()
    } else {
        route
    }
}

/// Navigates to a new hash-based route.
///
/// # Arguments
///
/// - `&str` - The target route path.
pub(crate) fn navigate(route: &str) {
    let window: Window = window().expect("no global window exists");
    let location: Location = window.location();
    let new_hash: String = format!("#{}", route);
    let _ = location.set_hash(&new_hash);
}

/// Creates a link click handler that navigates to the given route.
///
/// # Arguments
///
/// - `String` - The target route path.
///
/// # Returns
///
/// - `NativeEventHandler` - An event handler for click events.
pub(crate) fn link_handler(route: String) -> NativeEventHandler {
    NativeEventHandler::create("click", move |_event: Event| {
        navigate(&route);
    })
}

/// Checks whether the current viewport width qualifies as a mobile device.
///
/// Uses `MOBILE_BREAKPOINT` (768px) as the threshold.
///
/// # Returns
///
/// - `bool` - `true` if the viewport width is less than the mobile breakpoint.
pub(crate) fn is_mobile() -> bool {
    let window: Window = window().expect("no global window exists");
    let width: f64 = window
        .inner_width()
        .ok()
        .map(|value: JsValue| Number::from(value).value_of())
        .unwrap_or(0.0);
    width < MOBILE_BREAKPOINT as f64
}

/// Resolves the current route to the corresponding page virtual DOM tree.
///
/// Matches the route string against all registered page paths and returns
/// the appropriate page component. Falls back to a 404 page for unknown routes.
///
/// # Arguments
///
/// - `PageRouterProps` - The typed props containing the route signal.
///
/// # Returns
///
/// - `VirtualNode` - The virtual DOM tree of the matched page.
#[component]
pub(crate) fn page_router(node: VirtualNode<PageRouterProps>) -> VirtualNode {
    let PageRouterProps { route_signal }: PageRouterProps =
        node.try_get_props().unwrap_or_default();
    html! {
        match { route_signal.get().as_str() } {
            "/" => {
                page_home {}
            }
            "/signals" => {
                page_signals {}
            }
            "/event" => {
                page_event {}
            }
            "/list" => {
                page_list {}
            }
            "/observer" => {
                page_observer {}
            }
            "/conditional" => {
                page_conditional {}
            }
            "/modal" => {
                page_modal {}
            }
            "/select" => {
                page_select {}
            }
            "/async" => {
                page_async_demo {}
            }
            "/form" => {
                page_form {}
            }
            "/file-upload" => {
                page_file_upload {}
            }
            "/timer" => {
                page_timer {}
            }
            "/animation" => {
                page_animation {}
            }
            "/browser" => {
                page_browser {}
            }
            "/lifecycle" => {
                page_lifecycle {}
            }
            "/keep-alive" => {
                page_keep_alive {}
            }
            "/component-binding" => {
                page_component_binding {}
            }
            "/custom-attrs" => {
                page_custom_attrs {}
            }
            "/dynamic-component" => {
                page_dynamic_component {}
            }
            "/virtual-list" => {
                page_virtual_list {}
            }
            _ => {
                page_not_found {}
            }
        }
    }
}
