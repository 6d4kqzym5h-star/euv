use crate::*;

/// Creates a click event handler that sets the tag name signal.
///
/// # Arguments
///
/// - `Signal<String>` - The tag name signal to update.
/// - `&str` - The tag name value to set.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets the tag name.
pub(crate) fn tag_on_select(
    try_get_tag_name: Signal<String>,
    value: &str,
) -> Option<Rc<dyn Fn(Event)>> {
    let value_owned: String = value.to_string();
    Some(Rc::new(move |_: Event| {
        try_get_tag_name.set(value_owned.clone());
    }))
}

/// Creates an input event handler that updates the content text signal.
///
/// # Arguments
///
/// - `Signal<String>` - The content text signal to update.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler that updates the content text.
pub(crate) fn content_on_input(content: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let value: String = event
            .target()
            .and_then(|target: EventTarget| target.dyn_into::<HtmlInputElement>().ok())
            .map(|input: HtmlInputElement| input.value())
            .unwrap_or_default();
        content.set(value);
    }))
}
