use crate::*;

/// Registers global event listeners that force the browser to recalculate
/// `env(safe-area-inset-*)` values after exiting any type of fullscreen.
///
/// Listens for the native Fullscreen API exit events (`fullscreenchange`,
/// `webkitfullscreenchange`) as well as `resize` which fires after orientation
/// or viewport changes that may invalidate inset values.
///
/// On detecting a fullscreen exit, forces a layout recalculation by toggling
/// the root element's `display` property momentarily. This ensures iOS Safari
/// and other WebKit browsers re-evaluate the `env()` CSS functions.
///
/// This hook should be called once during app initialization and covers:
/// - Native video fullscreen → exit
/// - CSS simulated fullscreen → exit (canvas drawing mode)
/// - Any future fullscreen scenarios
pub(crate) fn use_safe_area_fix() {
    use_window_event("fullscreenchange", || {
        force_safe_area_recalc();
    });
    use_window_event("webkitfullscreenchange", || {
        force_safe_area_recalc();
    });
}

/// Forces the browser to recalculate `env(safe-area-inset-*)` values by
/// triggering a synchronous layout reflow on the `#app` root element.
///
/// The technique toggles `display: none` on the root element, reads a
/// layout property to force a synchronous reflow, then restores the
/// original display value. A `requestAnimationFrame` is used to also
/// trigger a scroll-to-current-position which helps iOS Safari refresh
/// the viewport insets after fullscreen transitions.
pub(crate) fn force_safe_area_recalc() {
    let window_value: Window = window().expect("no global window exists");
    let document_value: Document = window_value.document().expect("should have a document");
    let Some(root_element) = document_value.query_selector("#app").ok().flatten() else {
        return;
    };
    let html_element: HtmlElement = root_element.unchecked_into();
    // Toggle a harmless property to force reflow of env() values
    let _ = html_element
        .style()
        .set_property("padding-bottom", "env(safe-area-inset-bottom, 0px)");
    // Force synchronous layout reflow by reading offsetHeight
    let _ = html_element.offset_height();
    let _ = html_element.style().remove_property("padding-bottom");
    // Additional iOS workaround: scroll to current position after a frame
    let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let win: Window = window().expect("no global window exists");
        let scroll_x: f64 = win.scroll_x().unwrap_or(0.0);
        let scroll_y: f64 = win.scroll_y().unwrap_or(0.0);
        win.scroll_to_with_x_and_y(scroll_x, scroll_y);
    }));
    let _ = window_value.request_animation_frame(raf_closure.as_ref().unchecked_ref());
    raf_closure.forget();
}

/// Creates a click event handler that toggles a boolean signal.
///
/// Produces a `NativeEventHandler` that flips the value of the given
/// boolean signal on each click. Useful for toggle buttons, visibility
/// switches, and drawer open/close patterns.
///
/// # Arguments
///
/// - `Signal<bool>` - The boolean signal to toggle.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click event handler that toggles the signal.
pub(crate) fn use_toggle(signal: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = signal.get();
        signal.set(!current);
    }))
}

/// Creates a click event handler that toggles the mobile nav drawer signal
/// with proper browser history management.
///
/// When toggling from open to closed, calls `overlay_back` to remove the
/// extra history entry that was pushed when the drawer opened. When toggling
/// from closed to open, the `use_overlay_history` hook handles the
/// `pushState` call automatically.
///
/// # Arguments
///
/// - `Signal<bool>` - The boolean signal controlling the drawer visibility.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click event handler that toggles the drawer.
pub(crate) fn use_drawer_toggle(drawer_open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let is_open: bool = drawer_open.get();
        if is_open {
            overlay_back(None);
        }
        drawer_open.set(!is_open);
    }))
}

/// Creates an input event handler that updates a string signal.
///
/// # Arguments
///
/// - `Signal<String>` - The signal to update with the input value.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler.
pub(crate) fn on_input_value(signal: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let value: Option<String> = event.target().and_then(|target: EventTarget| {
            if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                return Some(input.value());
            }
            if let Ok(textarea) = target.clone().dyn_into::<HtmlTextAreaElement>() {
                return Some(textarea.value());
            }
            if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
                return Some(select.value());
            }
            None
        });
        if let Some(value) = value {
            signal.set(value);
        }
    }))
}

/// Creates a change event handler that updates a string signal.
///
/// # Arguments
///
/// - `Signal<String>` - The signal to update with the change value.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A change handler.
pub(crate) fn on_change_value(signal: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let value: Option<String> = event.target().and_then(|target: EventTarget| {
            if let Ok(input) = target.clone().dyn_into::<HtmlInputElement>() {
                return Some(input.value());
            }
            if let Ok(select) = target.clone().dyn_into::<HtmlSelectElement>() {
                return Some(select.value());
            }
            if let Ok(textarea) = target.clone().dyn_into::<HtmlTextAreaElement>() {
                return Some(textarea.value());
            }
            None
        });
        if let Some(value) = value {
            signal.set(value);
        }
    }))
}

/// Creates a change event handler that updates a boolean signal from checkbox.
///
/// # Arguments
///
/// - `Signal<bool>` - The signal to update with the checked state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A change handler.
pub(crate) fn on_change_checked(signal: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            signal.set(input.checked());
        }
    }))
}

/// Registers the global listeners that keep a focused text-entry field fully
/// visible inside the viewport whenever the on-screen keyboard appears.
///
/// Because the application shell pins `html, body, #app` to `overflow: hidden`
/// and performs all scrolling inside the inner `<main>` container, the browser
/// can no longer scroll the document body to reveal a focused input when the
/// software keyboard slides up. As a result, fields near the bottom of the
/// screen would otherwise sit underneath the keyboard. This hook restores the
/// expected behaviour by:
///
/// - Listening for the bubbling `focusin` event so a single window-level
///   listener observes focus on every input, textarea, select, and
///   `contenteditable` region.
/// - Listening for `window` `resize` and `visualViewport` `resize` so the
///   correction is re-applied when the keyboard finishes animating in or when
///   the viewport geometry changes (orientation change, keyboard resize).
///
/// Each trigger defers to `ensure_focused_field_visible`, which performs the
/// occlusion math against the visual viewport and scrolls the field into the
/// visible area.
///
/// This hook should be called once during application initialisation.
pub(crate) fn use_keyboard_inset_fix() {
    use_window_event(FOCUS_IN_EVENT, || {
        ensure_focused_field_visible();
    });
    use_window_event(KEYBOARD_RESIZE_EVENT, || {
        ensure_focused_field_visible();
    });
    if let Some(viewport) = window().and_then(|window_value: Window| window_value.visual_viewport())
    {
        let listener: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {
            ensure_focused_field_visible();
        }));
        let _ = viewport.add_event_listener_with_callback(
            VISUAL_VIEWPORT_RESIZE_EVENT,
            listener.as_ref().unchecked_ref(),
        );
        listener.forget();
    }
}

/// Resolves the currently focused element and, if it is a text-entry field,
/// schedules a deferred scroll so the field is not occluded by the keyboard.
///
/// The active element is matched against `EDITABLE_ELEMENT_SELECTOR` to avoid
/// reacting to focus on buttons, links, or other non-editable controls. The
/// actual scroll is deferred via `requestAnimationFrame` followed by a
/// `setTimeout` of `KEYBOARD_SCROLL_DELAY_MILLIS` so the keyboard animation and
/// visual-viewport resize have completed before the geometry is measured.
pub(crate) fn ensure_focused_field_visible() {
    let window_value: Window = window().expect("no global window exists");
    let document_value: Document = window_value.document().expect("should have a document");
    let Some(active_element) = document_value.active_element() else {
        return;
    };
    if !active_element
        .matches(EDITABLE_ELEMENT_SELECTOR)
        .unwrap_or(false)
    {
        return;
    }
    let focused_element: HtmlElement = active_element.unchecked_into();
    let raf_window: Window = window_value.clone();
    let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let timeout_window: Window = raf_window.clone();
        let scroll_element: HtmlElement = focused_element.clone();
        let scroll_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            scroll_field_into_visible_area(&scroll_element);
        }));
        let _ = timeout_window.set_timeout_with_callback_and_timeout_and_arguments_0(
            scroll_closure.as_ref().unchecked_ref::<Function>(),
            KEYBOARD_SCROLL_DELAY_MILLIS,
        );
        scroll_closure.forget();
    }));
    let _ = window_value.request_animation_frame(raf_closure.as_ref().unchecked_ref());
    raf_closure.forget();
}

/// Scrolls the given text-entry field so it sits fully within the area not
/// covered by the on-screen keyboard, keeping a small margin for readability.
///
/// The visible bottom edge is derived from `window.visualViewport` (its
/// `offsetTop + height`) which excludes the keyboard region on iOS Safari; on
/// platforms without a visual viewport the layout `innerHeight` is used. When
/// the keyboard is open (detected by comparing the visual viewport height to
/// the layout height against `KEYBOARD_OPEN_THRESHOLD_PX`), the field is first
/// centred via `scrollIntoView`, then any residual occlusion is corrected by
/// nudging the nearest scrollable ancestor so the field clears the keyboard.
///
/// # Arguments
///
/// - `&HtmlElement` - The focused text-entry element to reveal.
pub(crate) fn scroll_field_into_visible_area(element: &HtmlElement) {
    let window_value: Window = window().expect("no global window exists");
    let layout_height: f64 = window_value
        .inner_height()
        .ok()
        .and_then(|value: JsValue| value.as_f64())
        .unwrap_or(0.0);
    let viewport: Option<VisualViewport> = window_value.visual_viewport();
    let visible_top: f64 = viewport
        .as_ref()
        .map(|value: &VisualViewport| value.offset_top())
        .unwrap_or(0.0);
    let visible_height: f64 = viewport
        .as_ref()
        .map(|value: &VisualViewport| value.height())
        .unwrap_or(layout_height);
    let visible_bottom: f64 = visible_top + visible_height;
    let keyboard_open: bool = layout_height - visible_height > KEYBOARD_OPEN_THRESHOLD_PX;
    // Always centre the field first; this resolves occlusion in the common case
    // and gracefully degrades on platforms without a visual viewport.
    let options: ScrollIntoViewOptions = ScrollIntoViewOptions::new();
    options.set_behavior(ScrollBehavior::Smooth);
    options.set_block(ScrollLogicalPosition::Center);
    element.scroll_into_view_with_scroll_into_view_options(&options);
    if !keyboard_open {
        return;
    }
    // Re-measure and correct any residual occlusion that `scrollIntoView`
    // cannot resolve, since it is unaware of the visual-viewport keyboard inset.
    let rect: DomRect = element.get_bounding_client_rect();
    let overflow_bottom: f64 = rect.bottom() + KEYBOARD_VISIBLE_MARGIN_PX - visible_bottom;
    let overflow_top: f64 = visible_top + KEYBOARD_VISIBLE_MARGIN_PX - rect.top();
    let delta: f64 = if overflow_bottom > 0.0 {
        overflow_bottom
    } else if overflow_top > 0.0 {
        -overflow_top
    } else {
        return;
    };
    if let Some(container) = nearest_scrollable_ancestor(element) {
        let current_scroll_top: i32 = container.scroll_top();
        container.set_scroll_top((current_scroll_top as f64 + delta).max(0.0) as i32);
    }
}

/// Walks up the ancestor chain from the given element to find the nearest
/// vertically scrollable container.
///
/// An element is considered scrollable when its `scrollHeight` exceeds its
/// `clientHeight`, indicating it can actually scroll its content. The search
/// stops at the document root; if no scrollable ancestor is found, `None` is
/// returned and the caller should fall back to `scrollIntoView` alone.
///
/// # Arguments
///
/// - `&HtmlElement` - The element whose scrollable ancestor is sought.
///
/// # Returns
///
/// - `Option<HtmlElement>` - The nearest scrollable ancestor, if any.
pub(crate) fn nearest_scrollable_ancestor(element: &HtmlElement) -> Option<HtmlElement> {
    let mut current: Option<Element> = element.parent_element();
    while let Some(node) = current {
        if let Ok(html_node) = node.clone().dyn_into::<HtmlElement>()
            && html_node.scroll_height() > html_node.client_height()
        {
            return Some(html_node);
        }
        current = node.parent_element();
    }
    None
}
