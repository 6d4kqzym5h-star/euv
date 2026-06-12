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
    watch!(route_signal, |_: String| {
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
/// The listener is automatically removed when the hook context is cleared.
///
/// # Arguments
///
/// - `Signal<String>` - The reactive signal that holds the current route and will be updated on each hash change.
pub(crate) fn use_hash_change(route_signal: Signal<String>) {
    use_window_event("hashchange", move || {
        route_signal.set(current_route());
    });
}

/// Manages browser history for overlay panels (vconsole, drawer) so that
/// the back button closes the topmost overlay instead of navigating away.
///
/// Pushes a `pushState` entry when the drawer opens on mobile and
/// consumes that entry via `history.back()` when the overlay closes.
/// A `popstate` listener dispatches the back event to the correct
/// overlay signal.
///
/// # Arguments
///
/// - `Signal<bool>` - The reactive signal controlling vconsole panel visibility.
/// - `Signal<bool>` - The reactive signal controlling the nav drawer visibility.
/// - `Signal<bool>` - The reactive signal tracking whether the viewport is mobile-sized.
pub(crate) fn use_overlay_history(
    panel_open: Signal<bool>,
    drawer_open: Signal<bool>,
    mobile_signal: Signal<bool>,
) {
    let was_drawer_open: Signal<bool> = use_signal(|| false);
    watch!(drawer_open, |is_open: bool| {
        let previous: bool = was_drawer_open.get();
        if is_open && !previous && mobile_signal.get() {
            let window: Window = window().expect("no global window exists");
            let history: History = window.history().expect("no history object exists");
            let _ = history.push_state(&JsValue::NULL, "");
        }
        was_drawer_open.set(is_open);
    });
    use_window_event("popstate", move || {
        if BACK_PENDING.with(|flag: &Cell<bool>| flag.get()) {
            BACK_PENDING.with(|flag: &Cell<bool>| flag.set(false));
            let pending_route: Option<String> =
                NAVIGATE_AFTER_BACK.with(|cell: &Cell<Option<String>>| cell.take());
            if let Some(route) = pending_route {
                navigate(&route);
            }
            return;
        }
        if let Some(closer) = modal_pop_closer() {
            closer();
            return;
        }
        if panel_open.get() {
            panel_open.set(false);
            return;
        }
        if drawer_open.get() {
            drawer_open.set(false);
        }
    });
}

/// Pushes a browser history entry for an overlay that is about to open.
///
/// Call this when an overlay (vconsole panel) opens so that the browser
/// back button will close the overlay instead of navigating away.
pub(crate) fn overlay_push_state() {
    let window: Window = window().expect("no global window exists");
    let history: History = window.history().expect("no history object exists");
    let _ = history.push_state(&JsValue::NULL, "");
}

/// Performs a programmatic `history.back()` to consume the overlay's
/// history entry, optionally scheduling a navigation to run after the
/// `popstate` event fires.
///
/// # Arguments
///
/// - `Option<String>` - An optional route to navigate to after the back completes.
pub(crate) fn overlay_back(navigate_target: Option<String>) {
    BACK_PENDING.with(|flag: &Cell<bool>| flag.set(true));
    if let Some(ref route) = navigate_target {
        NAVIGATE_AFTER_BACK.with(|cell: &Cell<Option<String>>| cell.set(Some(route.clone())));
    }
    let window: Window = window().expect("no global window exists");
    let history: History = window.history().expect("no history object exists");
    let _ = history.back();
}

/// Registers an open modal by pushing it onto the global modal stack and
/// adding a browser history entry, enabling nested modals.
///
/// The stack is ordered with the most recently opened modal on top. When the
/// user triggers a system back gesture (or presses the browser back button),
/// the `popstate` handler in `use_overlay_history` pops the topmost entry and
/// invokes its close callback, so the most recently opened modal is dismissed
/// first instead of navigating to the previous page.
///
/// If the given visibility signal is already on the stack, this is a no-op so
/// that re-opening an already-open modal does not create duplicate stack or
/// history entries.
///
/// # Convention
///
/// This is the low-level stack primitive. Prefer the higher-level
/// `open_modal` / `dismiss_modal` helpers in the modal page module, which pair
/// this push with the matching `modal_close_via_ui` removal. Calling this
/// directly without a matching close (or toggling the visibility signal
/// directly) desynchronizes the modal stack from the browser history and
/// breaks the back-gesture behavior.
///
/// # Arguments
///
/// - `Signal<bool>` - The modal's visibility signal, used as a stable identity for later removal.
/// - `Rc<dyn Fn()>` - The callback that closes the modal (e.g., sets the visibility signal to `false`).
pub(crate) fn modal_push(visible: Signal<bool>, closer: Rc<dyn Fn()>) {
    let already_open: bool = MODAL_STACK.with(|stack: &ModalStack| {
        stack
            .borrow()
            .iter()
            .any(|(signal, _): &ModalStackEntry| *signal == visible)
    });
    if already_open {
        return;
    }
    MODAL_STACK.with(|stack: &ModalStack| stack.borrow_mut().push((visible, closer)));
    overlay_push_state();
}

/// Pops the most recently opened modal from the global modal stack and returns
/// its close callback, without invoking it.
///
/// Used by the `popstate` handler to obtain the closer for the topmost
/// (most recently opened) modal so it can dismiss that modal in response to a
/// system back gesture. This is what makes nested modals close one layer at a
/// time, newest first.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn()>>` - The topmost modal's close callback, or `None` if no modal is open.
pub(crate) fn modal_pop_closer() -> Option<Rc<dyn Fn()>> {
    MODAL_STACK.with(|stack: &ModalStack| {
        stack
            .borrow_mut()
            .pop()
            .map(|(_, closer): ModalStackEntry| closer)
    })
}

/// Closes a modal that was opened via [`modal_push`] when the user dismisses
/// it through the UI (close button, overlay click, confirm/cancel action)
/// rather than the system back gesture.
///
/// Removes the entry matching the given visibility signal from the global
/// stack (by identity, not necessarily the top, so nested modals stay
/// consistent) and consumes one matching browser history entry via
/// `history.back()`, keeping the history count in sync so a subsequent back
/// gesture behaves correctly.
///
/// # Arguments
///
/// - `Signal<bool>` - The visibility signal identifying the modal to remove.
pub(crate) fn modal_close_via_ui(visible: Signal<bool>) {
    let removed: bool = MODAL_STACK.with(|stack: &ModalStack| {
        let mut entries = stack.borrow_mut();
        if let Some(index) = entries
            .iter()
            .rposition(|(signal, _): &ModalStackEntry| *signal == visible)
        {
            entries.remove(index);
            true
        } else {
            false
        }
    });
    if removed {
        overlay_back(None);
    }
}

/// Closes the drawer and navigates to the given route, properly handling
/// browser history so that the `pushState` entry created on drawer open
/// is consumed before the new hash navigation occurs.
///
/// Schedules the navigation to execute after `history.back()` fires its
/// `popstate` event. This avoids the race condition where `history.back()`
/// would cancel a synchronous `navigate` call.
///
/// # Arguments
///
/// - `Signal<bool>` - The reactive signal controlling the mobile nav drawer visibility.
/// - `String` - The target route path to navigate to after the drawer closes.
pub(crate) fn close_drawer_and_navigate(drawer_open: Signal<bool>, route: String) {
    overlay_back(Some(route));
    drawer_open.set(false);
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
    watch!(drawer_open, |is_open: bool| {
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

/// Opens the given URL in the system default browser using `window.open`
/// with the `_system` target name.
///
/// In a Tauri WebView environment, the `_system` target instructs the
/// shell opener plugin to delegate the URL to the operating system's
/// default browser. In a regular browser, `window.open` falls back to
/// opening a new tab or window as usual.
///
/// # Arguments
///
/// - `&str` - The URL to open.
pub(crate) fn open_system_browser(url: &str) {
    let window_value: Window = window().expect("no global window exists");
    if let Ok(open_fn) = Reflect::get(&window_value, &JsValue::from_str("open"))
        .and_then(|value: JsValue| value.dyn_into::<Function>())
    {
        let _ = open_fn.call2(
            &window_value,
            &JsValue::from_str(url),
            &JsValue::from_str(SYSTEM_BROWSER_TARGET),
        );
    }
}

/// Creates a click event handler for external `<a>` links that opens
/// the URL in the system default browser.
///
/// Calls `event.prevent_default()` to suppress the `<a>` element's
/// default navigation (which would open inside the WebView), then
/// delegates to `open_system_browser` so the URL is handled by the
/// operating system's default browser.
///
/// # Arguments
///
/// - `String` - The external URL to open on click.
///
/// # Returns
///
/// - `NativeEventHandler` - An event handler for click events.
pub(crate) fn external_link_handler(url: String) -> NativeEventHandler {
    NativeEventHandler::create("click", move |event: Event| {
        event.prevent_default();
        open_system_browser(&url);
    })
}

/// Creates a reactive signal that tracks whether the viewport is in mobile mode
/// and subscribes to browser `resize` events to keep it updated.
///
/// The resize handler is debounced by `RESIZE_DEBOUNCE_MILLIS` (16ms) to avoid
/// excessive recomputation during continuous resize operations.
/// The listener is automatically removed when the hook context is cleared.
///
/// # Returns
///
/// - `Signal<bool>` - A reactive signal that is `true` when the viewport is mobile-sized.
pub(crate) fn use_resize() -> Signal<bool> {
    let mobile_signal: Signal<bool> = use_signal(is_mobile);
    let timer_signal: Signal<Option<i32>> = use_signal(|| None);
    let debounce_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let mobile: bool = is_mobile();
        mobile_signal.set(mobile);
    }));
    let debounce_callback: Function = debounce_closure
        .as_ref()
        .unchecked_ref::<Function>()
        .clone();
    debounce_closure.forget();
    let timeout_window: Window = window().expect("no global window exists");
    use_window_event("resize", move || {
        let old_timer: Option<i32> = timer_signal.get();
        if let Some(timer_id) = old_timer {
            timeout_window.clear_timeout_with_handle(timer_id);
        }
        let new_timer: i32 = timeout_window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &debounce_callback,
                RESIZE_DEBOUNCE_MILLIS,
            )
            .unwrap_or_default();
        timer_signal.set(Some(new_timer));
    });
    mobile_signal
}
