use crate::*;

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

/// Creates a focus event handler that scrolls the focused input into view.
///
/// On mobile devices, focusing an input can open the virtual keyboard and obscure
/// the field. This handler waits briefly for the keyboard to appear, then checks
/// whether the input's bottom edge is below the visible viewport (using
/// `VisualViewport` when available). If so, it scrolls the page just enough to
/// leave a small gap between the input and the keyboard.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A focus handler.
pub(crate) fn on_focus_scroll_into_view() -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Some(target) = event.target() else {
            return;
        };
        let Ok(element) = target.dyn_into::<HtmlElement>() else {
            return;
        };
        let window: Window = window().expect("no global window exists");
        let element_clone: HtmlElement = element.clone();
        let window_clone: Window = window.clone();
        let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let rect: DomRect = element_clone.get_bounding_client_rect();
            let input_bottom: f64 = rect.bottom();
            let viewport_height: f64 = window_clone
                .visual_viewport()
                .map(|viewport: VisualViewport| viewport.height())
                .unwrap_or_else(|| {
                    window_clone
                        .inner_height()
                        .map(|height: JsValue| height.as_f64().unwrap_or(0.0))
                        .unwrap_or(0.0)
                });
            let visible_bottom: f64 = viewport_height - KEYBOARD_FOCUS_GAP;
            if input_bottom > visible_bottom {
                let scroll_amount: f64 = input_bottom - visible_bottom;
                window_clone.scroll_by_with_x_and_y(0.0, scroll_amount);
            }
        }));
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref::<Function>(),
            FOCUS_SCROLL_DELAY_MILLIS,
        );
        closure.forget();
    }))
}
