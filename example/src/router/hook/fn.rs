use crate::*;

/// Watches the route signal and scrolls the `<main>` content container
/// back to the top whenever the route changes.
///
/// On each route change, queries the document for the first `<main>`
/// element and resets its `scrollTop` to zero. The sidebar scroll
/// position is preserved natively since the `<nav>` element is never
/// destroyed during route transitions.
///
/// # Arguments
///
/// - `Signal<String>` - The reactive signal holding the current route path.
pub(crate) fn use_scroll_to_top(route_signal: Signal<String>) {
    watch!(route_signal, |_route_value| {
        let window_value: Window = window().expect("no global window exists");
        let document_value: Document = window_value.document().expect("should have a document");
        if let Some(main_element) = document_value.query_selector("main").ok().flatten() {
            let html_element: HtmlElement = main_element.unchecked_into();
            html_element.set_scroll_top(0);
        }
    });
}

/// Subscribes to browser `hashchange` events and updates the given signal.
///
/// Registers a global event listener on `window` that reads the current
/// route on every hash change and writes it into the provided signal.
/// The closure is leaked via `Closure::forget` so it persists for the
/// entire application lifetime.
///
/// # Arguments
///
/// - `Signal<String>` - The reactive signal that holds the current route and will be updated on each hash change.
pub(crate) fn use_hash_change(route_signal: Signal<String>) {
    let window: Window = window().expect("no global window exists");
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let new_route: String = current_route();
        route_signal.set(new_route);
    }));
    let _ = window.add_event_listener_with_callback("hashchange", closure.as_ref().unchecked_ref());
    closure.forget();
}

/// Pushes a new entry onto the browser history stack.
///
/// Used when the vconsole panel opens to ensure there is a history entry
/// that can be intercepted on browser back navigation, preventing the
/// page from navigating away.
pub(crate) fn push_state_on_open() {
    let window: Window = window().expect("no global window exists");
    let history: History = window.history().expect("no history object exists");
    let _ = history.push_state(&JsValue::NULL, "");
}

/// Navigates back one step in the browser history.
///
/// Used when the vconsole panel closes to remove the extra history entry
/// that was pushed on open, so the user does not need an extra back press
/// to leave the page.
pub(crate) fn back_on_close() {
    let window: Window = window().expect("no global window exists");
    let history: History = window.history().expect("no history object exists");
    let _ = history.back();
}

/// Subscribes to browser `popstate` events and closes the vconsole panel
/// instead of navigating back when the panel is open.
///
/// Requires `push_state_on_open` to be called when the panel opens so that
/// a history entry exists for the browser back button to consume. When the
/// `popstate` event fires while the panel is open, the handler simply closes
/// the panel (the consumed history entry is already gone). If the panel is
/// closed, the event propagates normally.
///
/// The closure is leaked via `Closure::forget` so it persists for the
/// entire application lifetime.
///
/// # Arguments
///
/// - `Signal<bool>` - The reactive signal controlling vconsole panel visibility.
pub(crate) fn use_pop_state(panel_open: Signal<bool>) {
    let window: Window = window().expect("no global window exists");
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let is_open: bool = panel_open.get();
        if is_open {
            panel_open.set(false);
        }
    }));
    let _ = window.add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref());
    closure.forget();
}

/// Watches the drawer open signal and scrolls the mobile navigation drawer
/// to make the currently active navigation item visible when the drawer opens.
///
/// Uses nested `requestAnimationFrame` to defer the scroll until after the
/// framework has completed its DOM update cycle. The first `requestAnimationFrame`
/// fires after the framework's own `requestAnimationFrame`-based render pass,
/// and the second one fires after the browser has laid out the new DOM.
/// Locates the scrollable `c-nav-items-scroll` container and the active nav
/// item within the drawer, then sets `scrollTop` so the active item appears
/// near the vertical center of the container.
///
/// # Arguments
///
/// - `Signal<bool>` - The reactive signal controlling the mobile nav drawer visibility.
pub(crate) fn use_scroll_drawer_to_active(drawer_open: Signal<bool>) {
    watch!(drawer_open, |is_open| {
        if !is_open {
            return;
        }
        let outer_window: Window = window().expect("no global window exists");
        let outer_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let inner_window: Window = window().expect("no global window exists");
            let inner_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                let window_value: Window = window().expect("no global window exists");
                let document_value: Document =
                    window_value.document().expect("should have a document");
                let Some(drawer_nav) = document_value
                    .query_selector(DRAWER_NAV_SELECTOR)
                    .ok()
                    .flatten()
                else {
                    return;
                };
                let Some(active_element) = drawer_nav
                    .query_selector(ACTIVE_NAV_ITEM_SELECTOR)
                    .ok()
                    .flatten()
                else {
                    return;
                };
                let active_html_element: HtmlElement = active_element.unchecked_into();
                let Some(scroll_container) = drawer_nav
                    .query_selector(NAV_ITEMS_SCROLL_SELECTOR)
                    .ok()
                    .flatten()
                else {
                    return;
                };
                let scroll_html_element: HtmlElement = scroll_container.unchecked_into();
                let active_rect: DomRect = active_html_element.get_bounding_client_rect();
                let container_rect: DomRect = scroll_html_element.get_bounding_client_rect();
                let offset_from_container_top: f64 = active_rect.top() - container_rect.top();
                let current_scroll_top: i32 = scroll_html_element.scroll_top();
                let container_height: f64 = container_rect.height();
                let active_height: f64 = active_rect.height();
                let target_scroll_top: f64 = current_scroll_top as f64 + offset_from_container_top
                    - (container_height - active_height) / 2.0;
                scroll_html_element.set_scroll_top(target_scroll_top.max(0.0) as i32);
            }));
            let _ = inner_window.request_animation_frame(inner_closure.as_ref().unchecked_ref());
            inner_closure.forget();
        }));
        let _ = outer_window.request_animation_frame(outer_closure.as_ref().unchecked_ref());
        outer_closure.forget();
    });
}

/// Creates a reactive signal that tracks whether the viewport is in mobile mode
/// and subscribes to browser `resize` events to keep it updated.
///
/// The resize handler is debounced by `RESIZE_DEBOUNCE_MILLIS` (150ms) to avoid
/// excessive recomputation during continuous resize operations.
/// The closures are leaked via `Closure::forget` so they persist for the
/// entire application lifetime.
///
/// # Returns
///
/// - `Signal<bool>` - A reactive signal that is `true` when the viewport is mobile-sized.
pub(crate) fn use_resize() -> Signal<bool> {
    let mobile_signal: Signal<bool> = use_signal(is_mobile);
    let timer_signal: Signal<Option<i32>> = use_signal(|| None);
    let debounce_window: Window = window().expect("no global window exists");
    let timeout_window: Window = debounce_window.clone();
    let resize_window: Window = debounce_window.clone();
    let debounce_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let mobile: bool = is_mobile();
        mobile_signal.set(mobile);
    }));
    let debounce_callback: Function = debounce_closure
        .as_ref()
        .unchecked_ref::<Function>()
        .clone();
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let old_timer: Option<i32> = timer_signal.get();
        if let Some(timer_id) = old_timer {
            debounce_window.clear_timeout_with_handle(timer_id);
        }
        let new_timer: i32 = timeout_window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &debounce_callback,
                RESIZE_DEBOUNCE_MILLIS,
            )
            .unwrap_or_default();
        timer_signal.set(Some(new_timer));
    }));
    let _ =
        resize_window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
    closure.forget();
    debounce_closure.forget();
    mobile_signal
}
