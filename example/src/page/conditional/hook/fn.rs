use crate::*;

/// Creates a click event handler that sets the user type signal.
///
/// # Arguments
///
/// - `Signal<String>` - The user type signal to update.
/// - `&str` - The user type value to set.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets the user type.
pub(crate) fn user_type_on_select(
    user_type: Signal<String>,
    value: &str,
) -> Option<Rc<dyn Fn(Event)>> {
    let value_owned: String = value.to_string();
    Some(Rc::new(move |_event: Event| {
        user_type.set(value_owned.clone());
    }))
}

/// Creates a click event handler that sets the active tab signal.
///
/// # Arguments
///
/// - `Signal<String>` - The tab signal to update.
/// - `&str` - The tab value to set.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets the active tab.
pub(crate) fn tab_on_select(tab: Signal<String>, value: &str) -> Option<Rc<dyn Fn(Event)>> {
    let value_owned: String = value.to_string();
    Some(Rc::new(move |_event: Event| {
        tab.set(value_owned.clone());
    }))
}
