use crate::*;

/// Creates an input event handler that updates the dynamic attribute key signal.
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the dynamic attribute key.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler for the attribute key.
pub(crate) fn attrs_on_input_key(attr_key: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            attr_key.set(input.value());
        }
    }))
}

/// Creates an input event handler that updates the dynamic attribute value signal.
///
/// # Arguments
///
/// - `Signal<String>` - The signal holding the dynamic attribute value.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler for the attribute value.
pub(crate) fn attrs_on_input_value(attr_value: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            attr_value.set(input.value());
        }
    }))
}
