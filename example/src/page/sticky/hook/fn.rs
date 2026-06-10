use crate::*;

/// Creates sticky demo state signals wrapped in a `UseSticky` struct.
///
/// # Returns
///
/// - `UseSticky` - The sticky demo state.
pub(crate) fn use_sticky() -> UseSticky {
    UseSticky::new(
        use_signal(|| true),
        use_signal(|| true),
        use_signal(|| true),
        use_signal(|| false),
        use_signal(|| false),
    )
}

/// Creates a click event handler that toggles the sticky header visibility.
///
/// # Arguments
///
/// - `UseSticky` - The sticky demo state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to toggle the sticky header.
pub(crate) fn sticky_on_toggle_sticky(state: UseSticky) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_sticky_enabled().get();
        state.get_sticky_enabled().set(!current);
    }))
}

/// Creates a click event handler that toggles the glassmorphism effect.
///
/// # Arguments
///
/// - `UseSticky` - The sticky demo state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to toggle the glassmorphism effect.
pub(crate) fn sticky_on_toggle_glass(state: UseSticky) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_glass_enabled().get();
        state.get_glass_enabled().set(!current);
    }))
}

/// Creates an input event handler that updates the backdrop blur via
/// `requestAnimationFrame` throttling to avoid redundant GPU repaints.
///
/// Instead of applying `backdrop-filter` on every `oninput` event (which
/// can fire dozens of times per frame), stores the pending value and
/// schedules a single `requestAnimationFrame` callback. The callback
/// reads the latest pending value and applies it exactly once per paint
/// frame, keeping the slider smooth even though `backdrop-filter` is
/// an expensive GPU operation.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler for the blur slider.
pub(crate) fn sticky_on_blur_change() -> Option<Rc<dyn Fn(Event)>> {
    let pending_value: Rc<Cell<i32>> = Rc::new(Cell::new(STICKY_BLUR_INITIAL));
    let raf_id: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            let value: i32 = input.value().parse::<i32>().unwrap_or(0);
            pending_value.set(value);
            if raf_id.get().is_some() {
                return;
            }
            let pending_for_raf: Rc<Cell<i32>> = pending_value.clone();
            let raf_id_clone: Rc<Cell<Option<i32>>> = raf_id.clone();
            let window_value: Window = window().expect("no global window exists");
            let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                raf_id_clone.set(None);
                let current_value: i32 = pending_for_raf.get();
                let doc: Window = window().expect("no global window exists");
                let document_value: Document = doc.document().expect("should have a document");
                if let Some(overlay_element) = document_value
                    .query_selector(STICKY_BLUR_OVERLAY_SELECTOR)
                    .ok()
                    .flatten()
                {
                    let html_element: HtmlElement = overlay_element.unchecked_into();
                    let blur_style: String = format!("blur({}px)", current_value);
                    let _ = html_element
                        .style()
                        .set_property("backdrop-filter", &blur_style);
                    let _ = html_element
                        .style()
                        .set_property("-webkit-backdrop-filter", &blur_style);
                }
                if let Some(label_element) = document_value
                    .query_selector(STICKY_BLUR_LABEL_SELECTOR)
                    .ok()
                    .flatten()
                {
                    let label_html: HtmlElement = label_element.unchecked_into();
                    label_html.set_text_content(Some(&format!(
                        "{}: {}px",
                        STICKY_BLUR_LABEL, current_value
                    )));
                }
            }));
            let id: i32 = window_value
                .request_animation_frame(raf_closure.as_ref().unchecked_ref())
                .unwrap_or(0);
            raf_id.set(Some(id));
            raf_closure.forget();
        }
    }))
}

/// Creates an input event handler that rotates the gradient wheel using
/// CSS `transform: rotate()` instead of recalculating the entire
/// conic-gradient on every slider input.
///
/// `transform: rotate()` is a pure GPU compositing operation that does
/// not trigger paint or layout, making it effectively free compared to
/// changing `background` which forces the browser to recompute the
/// gradient on every frame. The label text is updated directly via DOM.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler for the angle slider.
pub(crate) fn sticky_on_angle_change() -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            let value: i32 = input.value().parse::<i32>().unwrap_or(0);
            let window_value: Window = window().expect("no global window exists");
            let document_value: Document = window_value.document().expect("should have a document");
            if let Some(wheel_element) = document_value
                .query_selector(STICKY_GRADIENT_WHEEL_SELECTOR)
                .ok()
                .flatten()
            {
                let html_element: HtmlElement = wheel_element.unchecked_into();
                let _ = html_element
                    .style()
                    .set_property("transform", &format!("rotate({}deg)", value));
            }
            if let Some(label_element) = document_value
                .query_selector(STICKY_ANGLE_LABEL_SELECTOR)
                .ok()
                .flatten()
            {
                let label_html: HtmlElement = label_element.unchecked_into();
                label_html.set_text_content(Some(&format!("{}: {}°", STICKY_ANGLE_LABEL, value)));
            }
        }
    }))
}

/// Creates a click event handler that toggles the text shadow effect.
///
/// # Arguments
///
/// - `UseSticky` - The sticky demo state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to toggle text shadow.
pub(crate) fn sticky_on_toggle_text_shadow(state: UseSticky) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_text_shadow_enabled().get();
        state.get_text_shadow_enabled().set(!current);
    }))
}

/// Creates a click event handler that toggles the clip path effect.
///
/// # Arguments
///
/// - `UseSticky` - The sticky demo state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to toggle clip path.
pub(crate) fn sticky_on_toggle_clip_path(state: UseSticky) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_clip_path_enabled().get();
        state.get_clip_path_enabled().set(!current);
    }))
}

/// Creates a click event handler that toggles the scroll shadow visibility.
///
/// # Arguments
///
/// - `UseSticky` - The sticky demo state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to toggle scroll shadow.
pub(crate) fn sticky_on_toggle_scroll_shadow(state: UseSticky) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = state.get_scroll_shadow_visible().get();
        state.get_scroll_shadow_visible().set(!current);
    }))
}
