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
        resize();
    });
    use_window_event("webkitfullscreenchange", || {
        resize();
    });
    use_window_event("resize", || {
        resize();
    });
}

/// Resizes the window.
pub(crate) fn resize() {
    let deferred_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let win: Window = window().expect("no global window exists");
        let scroll_x: f64 = win.scroll_x().unwrap_or(0.0);
        let scroll_y: f64 = win.scroll_y().unwrap_or(0.0);
        win.scroll_to_with_x_and_y(scroll_x, scroll_y);
    }));
    let _ = window()
        .expect("no global window exists")
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            deferred_closure.as_ref().unchecked_ref::<Function>(),
            360,
        );
    deferred_closure.forget();
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
/// visible inside the viewport whenever the on-screen keyboard appears or the
/// user scrolls the page while a field remains focused.
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
/// - Listening for `visualViewport` `scroll` so the correction is re-applied
///   when the user scrolls the page while a text-entry field still has focus.
///   On mobile devices the keyboard may close without blurring the field; if
///   the user then scrolls and re-taps the field, the keyboard reappears but
///   the field may have been scrolled out of view. The `scroll` listener
///   ensures the focused field is repositioned into the visible area.
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
    use_window_event(FOCUS_OUT_EVENT, || {
        release_scroll_room_if_unfocused();
    });
    use_window_event(KEYBOARD_RESIZE_EVENT, || {
        ensure_focused_field_visible();
    });
    if let Some(viewport) = window().and_then(|window_value: Window| window_value.visual_viewport())
    {
        let resize_listener: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {
            ensure_focused_field_visible();
        }));
        let _ = viewport.add_event_listener_with_callback(
            VISUAL_VIEWPORT_RESIZE_EVENT,
            resize_listener.as_ref().unchecked_ref(),
        );
        resize_listener.forget();
        let scroll_listener: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {
            ensure_focused_field_visible();
        }));
        let _ = viewport.add_event_listener_with_callback(
            VISUAL_VIEWPORT_SCROLL_EVENT,
            scroll_listener.as_ref().unchecked_ref(),
        );
        scroll_listener.forget();
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

/// Scrolls the given text-entry field so it sits fully inside the visible
/// viewport with a comfortable gap kept between the field and the on-screen
/// keyboard (or the viewport bottom).
///
/// Rather than relying on the browser's `scrollIntoView`, which cannot account
/// for the on-screen keyboard region and does nothing useful when the app shell
/// pins the document to `overflow: hidden`, this function computes the exact
/// scroll delta needed to place the field inside a usable "band" and applies it
/// to the nearest scroll container directly. The field is repositioned on
/// *every* focus — even when it is already partially visible — so each tap
/// reliably brings the field into view with the required keyboard gap.
///
/// The band is bounded by:
///
/// - `band_bottom` = `effective_bottom - KEYBOARD_VISIBLE_MARGIN_PX`, the
///   lowest position the field's bottom edge may occupy while keeping a gap
///   above the keyboard.
/// - `band_top` = `visible_top + KEYBOARD_VISIBLE_MARGIN_PX`, the highest
///   position the field's top edge may occupy while keeping a gap below any
///   sticky header or notch.
///
/// The visible bottom edge (`effective_bottom`) is determined as follows:
///
/// - When `visualViewport` reports a shrink larger than
///   `KEYBOARD_OPEN_THRESHOLD_PX` the keyboard is considered open and the
///   measured `offsetTop + height` is used directly.
/// - Otherwise the keyboard height cannot be measured (it has not appeared
///   yet, or the browser does not shrink the viewport). In this case the
///   lower `KEYBOARD_ESTIMATED_FRACTION` of the layout viewport is reserved
///   as an estimated keyboard region, so a field pinned to the bottom of the
///   page is still pushed up into the area that stays visible once the
///   keyboard slides in.
///
/// On an unscrolled page (content shorter than the viewport) the scroll
/// container has no spare scroll distance, so [`ensure_bottom_scroll_room`]
/// reserves temporary bottom padding before the scroll range is measured. If
/// the nearest container still cannot absorb the full upward shift the
/// remaining overflow is consumed from outer scroll contexts via
/// [`scroll_ancestor_chain_into_view`].
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
    let keyboard_open: bool = layout_height - visible_height > KEYBOARD_OPEN_THRESHOLD_PX;
    // The y-coordinate of the lowest pixel that stays visible above the
    // keyboard. When the real keyboard height is known (the visual viewport
    // has shrunk) anchor to the measured visible bottom; otherwise reserve an
    // estimated keyboard region at the bottom of the layout viewport so
    // bottom-pinned fields are still lifted into the area that remains visible
    // once the keyboard finishes animating in.
    let effective_bottom: f64 = if keyboard_open {
        visible_top + visible_height
    } else {
        layout_height - layout_height * KEYBOARD_ESTIMATED_FRACTION
    };
    // The usable band where a focused field should sit, with a comfortable
    // margin kept against both the keyboard (or viewport bottom) and the top
    // edge so the field never hugs an edge.
    let band_top: f64 = visible_top + KEYBOARD_VISIBLE_MARGIN_PX;
    let band_bottom: f64 = effective_bottom - KEYBOARD_VISIBLE_MARGIN_PX;
    let rect: DomRect = element.get_bounding_client_rect();
    // Determine how far (and in which direction) the field must move so that it
    // sits inside the usable band. A positive `delta` scrolls the content up
    // (revealing lower content); a negative `delta` scrolls it down. The field
    // is *always* repositioned on focus so that every tap brings it into the
    // visible band with the required keyboard gap, even when it is already
    // partially visible.
    let delta: f64 = if rect.bottom() > band_bottom {
        // The field (or its lower edge) is below the usable band: lift it so
        // its bottom rests on the band's lower boundary, keeping the keyboard
        // gap. If the field is taller than the band, prefer revealing its top.
        let lift_to_bottom: f64 = rect.bottom() - band_bottom;
        let lift_to_top: f64 = rect.top() - band_top;
        if rect.height() > band_bottom - band_top {
            lift_to_top.max(0.0)
        } else {
            lift_to_bottom
        }
    } else if rect.top() < band_top {
        // The field is above the usable band (scrolled too far up or partially
        // hidden behind a sticky header): push the content down so its top
        // aligns with the band's upper boundary.
        rect.top() - band_top
    } else {
        // The field already sits comfortably within the band; no correction is
        // required and we avoid a redundant scroll that would jitter the view.
        return;
    };
    // When the field needs to move up the scroll container must have enough
    // room below the field to lift it above the keyboard. On an unscrolled page
    // (content shorter than the viewport) the container has no spare scroll
    // distance, so reserve temporary bottom padding on the overflow container
    // before measuring its scroll range. Reading `scroll_height` afterwards
    // forces a synchronous reflow, so the freshly added padding is reflected in
    // the measurements below.
    let container: Option<HtmlElement> = if delta > 0.0 {
        ensure_bottom_scroll_room(element, delta)
    } else {
        nearest_scrollable_ancestor(element)
    };
    let Some(container) = container else {
        return;
    };
    let current_scroll_top: i32 = container.scroll_top();
    let max_scroll_top: i32 = (container.scroll_height() - container.client_height()).max(0);
    let target_scroll_top: i32 = (current_scroll_top as f64 + delta)
        .round()
        .clamp(0.0, max_scroll_top as f64) as i32;
    container.set_scroll_top(target_scroll_top);
    // If the nearest container could not absorb the full upward shift (it hit
    // its maximum scroll offset) walk the ancestor chain to consume the
    // remaining overflow from outer scroll contexts.
    if delta > 0.0 && target_scroll_top >= max_scroll_top {
        let remaining_rect: DomRect = element.get_bounding_client_rect();
        let remaining: f64 = remaining_rect.bottom() - band_bottom;
        if remaining > 0.0 {
            scroll_ancestor_chain_into_view(element, remaining, effective_bottom);
        }
    }
}

/// Guarantees that the overflow container enclosing a focused field has enough
/// scroll distance below the field to lift it above the on-screen keyboard.
///
/// On a page whose content is shorter than the viewport the scroll container
/// has no spare scroll distance, so a field pinned near the bottom can never
/// be scrolled up regardless of how the scroll offset is adjusted. This is the
/// root cause of the "bottom field stays hidden on an unscrolled page" case.
///
/// The function locates the nearest ancestor that establishes a vertical
/// scroll context (`overflow-y` of `auto`, `scroll`, or `overlay`) — even when
/// it is not currently scrollable — and applies a temporary `padding-bottom`
/// large enough to cover the requested overflow plus the keyboard margin. The
/// reserved amount is tagged with a data attribute so it can be removed on
/// blur by [`clear_bottom_scroll_room`]. The padding is only ever grown, never
/// shrunk, so repeated focus events do not cause the layout to jump.
///
/// # Arguments
///
/// - `&HtmlElement` - The focused text-entry element to reveal.
/// - `f64` - The bottom overflow in CSS pixels that must be made scrollable.
///
/// # Returns
///
/// - `Option<HtmlElement>` - The overflow container with reserved room, or the
///   nearest already-scrollable ancestor when no scroll context is found.
pub(crate) fn ensure_bottom_scroll_room(
    element: &HtmlElement,
    overflow_bottom: f64,
) -> Option<HtmlElement> {
    let Some(container) = nearest_overflow_container(element) else {
        return nearest_scrollable_ancestor(element);
    };
    let needed: f64 = (overflow_bottom + KEYBOARD_VISIBLE_MARGIN_PX).max(0.0);
    if needed <= 0.0 {
        return Some(container);
    }
    let existing: f64 = container
        .get_attribute(KEYBOARD_RESERVED_PADDING_ATTR)
        .and_then(|value: String| value.parse::<f64>().ok())
        .unwrap_or(0.0);
    if needed > existing {
        let current_padding: f64 = current_padding_bottom(&container);
        let base_padding: f64 = current_padding - existing;
        let new_padding: f64 = base_padding + needed;
        let _ = container
            .style()
            .set_property("padding-bottom", &format!("{new_padding}px"));
        let _ = container.set_attribute(KEYBOARD_RESERVED_PADDING_ATTR, &needed.to_string());
    }
    Some(container)
}

/// Releases the reserved bottom scroll room, but only once focus has truly
/// left every editable field.
///
/// The `focusout` event fires *before* the next element gains focus, so when
/// the user taps from one input straight to another the document's active
/// element is momentarily the `body`. Clearing the reserved padding
/// immediately would cause the layout to jump back and forth on every
/// field-to-field tap. To avoid this, the check is deferred to a macrotask via
/// `setTimeout(0)`; by the time it runs the new field (if any) has received
/// focus, so the padding is only released when the active element is no longer
/// an editable field.
pub(crate) fn release_scroll_room_if_unfocused() {
    let window_value: Window = window().expect("no global window exists");
    let deferred: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let document_value: Document = window()
            .and_then(|value: Window| value.document())
            .expect("should have a document");
        let still_editing: bool = document_value
            .active_element()
            .map(|active: Element| active.matches(EDITABLE_ELEMENT_SELECTOR).unwrap_or(false))
            .unwrap_or(false);
        if !still_editing {
            clear_bottom_scroll_room();
        }
    }));
    let _ = window_value.set_timeout_with_callback_and_timeout_and_arguments_0(
        deferred.as_ref().unchecked_ref::<Function>(),
        0,
    );
    deferred.forget();
}

/// Removes any temporary bottom padding previously reserved by
/// [`ensure_bottom_scroll_room`] when no editable field is focused.
///
/// Invoked from the `focusout` listener. The reserved padding is restored to
/// the element's original `padding-bottom` (the value before any reservation)
/// by subtracting the amount recorded in the data attribute, and the attribute
/// is cleared so a subsequent focus starts from a clean baseline.
pub(crate) fn clear_bottom_scroll_room() {
    let window_value: Window = window().expect("no global window exists");
    let document_value: Document = window_value.document().expect("should have a document");
    let Ok(nodes) =
        document_value.query_selector_all(&format!("[{KEYBOARD_RESERVED_PADDING_ATTR}]"))
    else {
        return;
    };
    for index in 0..nodes.length() {
        let Some(node) = nodes.item(index) else {
            continue;
        };
        let Ok(container) = node.dyn_into::<HtmlElement>() else {
            continue;
        };
        let reserved: f64 = container
            .get_attribute(KEYBOARD_RESERVED_PADDING_ATTR)
            .and_then(|value: String| value.parse::<f64>().ok())
            .unwrap_or(0.0);
        let restored: f64 = (current_padding_bottom(&container) - reserved).max(0.0);
        let _ = container
            .style()
            .set_property("padding-bottom", &format!("{restored}px"));
        let _ = container.remove_attribute(KEYBOARD_RESERVED_PADDING_ATTR);
    }
}

/// Reads the resolved `padding-bottom` of an element in CSS pixels.
///
/// Falls back to `0.0` when the computed style is unavailable or cannot be
/// parsed (for example on platforms without `getComputedStyle`).
///
/// # Arguments
///
/// - `&HtmlElement` - The element whose `padding-bottom` is read.
///
/// # Returns
///
/// - `f64` - The resolved bottom padding in CSS pixels.
pub(crate) fn current_padding_bottom(element: &HtmlElement) -> f64 {
    let window_value: Window = window().expect("no global window exists");
    window_value
        .get_computed_style(element)
        .ok()
        .flatten()
        .and_then(|style: CssStyleDeclaration| style.get_property_value("padding-bottom").ok())
        .map(|value: String| value.trim_end_matches("px").to_string())
        .and_then(|value: String| value.parse::<f64>().ok())
        .unwrap_or(0.0)
}

/// Walks up the ancestor chain to find the nearest element that establishes a
/// vertical scroll context, i.e. whose computed `overflow-y` is `auto`,
/// `scroll`, or `overlay`.
///
/// Unlike [`nearest_scrollable_ancestor`], this does not require the container
/// to currently have overflowing content; it identifies the element that
/// *would* scroll once enough content (or padding) is present. This is what
/// allows a bottom-pinned field on an unscrolled page to be revealed: the
/// container is found first, given temporary bottom padding, and then scrolled.
///
/// # Arguments
///
/// - `&HtmlElement` - The element whose scroll-context ancestor is sought.
///
/// # Returns
///
/// - `Option<HtmlElement>` - The nearest scroll-context ancestor, if any.
pub(crate) fn nearest_overflow_container(element: &HtmlElement) -> Option<HtmlElement> {
    let window_value: Window = window().expect("no global window exists");
    let mut current: Option<Element> = element.parent_element();
    while let Some(node) = current {
        if let Ok(html_node) = node.clone().dyn_into::<HtmlElement>() {
            let overflow_y: String = window_value
                .get_computed_style(&html_node)
                .ok()
                .flatten()
                .and_then(|style: CssStyleDeclaration| style.get_property_value("overflow-y").ok())
                .unwrap_or_default();
            if matches!(overflow_y.as_str(), "auto" | "scroll" | "overlay") {
                return Some(html_node);
            }
        }
        current = node.parent_element();
    }
    None
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

/// When the nearest scrollable ancestor has been scrolled to its maximum but
/// the element still overflows the visible bottom edge (e.g. the field is
/// near the bottom of the page and there is not enough scroll distance left),
/// walks up the ancestor chain and nudges each outer scrollable container
/// further to maximise the visible area of the element.
///
/// The algorithm starts from the element's parent and visits every scrollable
/// ancestor. For each one it computes how much additional scroll distance is
/// available (`maxScrollTop - currentScrollTop`) and applies the smaller of
/// that and the remaining `overflow` so the field is revealed as much as
/// possible without overshooting.
///
/// After scrolling an outer ancestor the element's bounding rect is
/// re-measured so the remaining overflow is accurate for the next iteration.
///
/// # Arguments
///
/// - `&HtmlElement` - The focused text-entry element that is still occluded.
/// - `f64` - The initial bottom overflow in CSS pixels (element bottom +
///   margin minus the visible bottom edge).
/// - `f64` - The y-coordinate of the visible bottom edge (layout height
///   when the keyboard is closed, or `offsetTop + height` of the visual
///   viewport when the keyboard is open).
pub(crate) fn scroll_ancestor_chain_into_view(
    element: &HtmlElement,
    mut overflow: f64,
    effective_bottom: f64,
) {
    let mut current: Option<Element> = element.parent_element();
    while let Some(node) = current {
        if let Ok(ancestor) = node.clone().dyn_into::<HtmlElement>()
            && ancestor.scroll_height() > ancestor.client_height()
        {
            let current_scroll: i32 = ancestor.scroll_top();
            let max_scroll: i32 = (ancestor.scroll_height() - ancestor.client_height()).max(0);
            let available: i32 = (max_scroll - current_scroll).max(0);
            let needed: i32 = overflow.round() as i32;
            let scroll_delta: i32 = available.min(needed);
            if scroll_delta > 0 {
                ancestor.set_scroll_top(current_scroll + scroll_delta);
                let updated_rect: DomRect = element.get_bounding_client_rect();
                overflow = updated_rect.bottom() + KEYBOARD_VISIBLE_MARGIN_PX - effective_bottom;
                if overflow <= 0.0 {
                    return;
                }
            }
        }
        current = node.parent_element();
    }
}
