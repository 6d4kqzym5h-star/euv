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
/// Always defers the actual `location.set_hash()` call to `queueMicrotask`
/// to prevent synchronous `hashchange` dispatch while any caller frame is still
/// on the stack. This avoids wasm_bindgen's `"closure invoked recursively
/// or after being dropped"` error which occurs when `set_hash()` fires
/// `hashchange` synchronously and the handler (or the reactive update chain
/// it triggers) calls `navigate()` again before the original dispatch finishes.
///
/// On mobile, this pattern is especially common because `close_drawer_and_navigate`
/// calls `history.back()` which synchronously dispatches `popstate`, whose
/// handler calls `navigate()` while the `popstate` proxy Closure is still active.
///
/// Multiple rapid `navigate()` calls before the microtask fires are coalesced:
/// only the **last** target route wins, as earlier routes were superseded by
/// a more recent navigation intent.
///
/// # Arguments
///
/// - `&str` - The target route path.
pub(crate) fn navigate(route: &str) {
    DEFERRED_NAVIGATION.with(|cell: &Cell<Option<String>>| cell.set(Some(route.to_string())));
    let deferred_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let target_route: Option<String> =
            DEFERRED_NAVIGATION.with(|cell: &Cell<Option<String>>| cell.take());
        if let Some(route_value) = target_route {
            let nav_window: Window = web_sys::window().expect("no global window exists");
            let nav_location: Location = nav_window.location();
            let nav_new_hash: String = format!("#{}", route_value);
            let _ = nav_location.set_hash(&nav_new_hash);
        }
    }));
    let window: Window = window().expect("no global window exists");
    window.queue_microtask(deferred_closure.as_ref().unchecked_ref::<Function>());
    deferred_closure.forget();
}

/// Creates a link click handler that navigates to the given route.
///
/// Calls `event.prevent_default()` to prevent the `<a>` element's
/// default hash navigation, then programmatically navigates via
/// `navigate()`. Without `preventDefault`, both the `<a href>` default
/// behavior and `navigate()` would fire, potentially creating duplicate
/// history entries and causing incorrect browser back/forward behavior.
///
/// # Arguments
///
/// - `String` - The target route path.
///
/// # Returns
///
/// - `NativeEventHandler` - An event handler for click events.
pub(crate) fn link_handler(route: String) -> NativeEventHandler {
    NativeEventHandler::create("click", move |event: Event| {
        event.prevent_default();
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
            "/counter" => {
                page_counter {}
            }
            "/badge" => {
                page_badge {}
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
            "/camera" => {
                page_camera {}
            }
            "/canvas" => {
                page_canvas {}
            }
            "/tags" => {
                page_tags {}
            }
            "/sse" => {
                page_sse {}
            }
            "/sticky" => {
                page_sticky {}
            }
            "/websocket" => {
                page_websocket {}
            }
            _ => {
                page_not_found {}
            }
        }
    }
}
