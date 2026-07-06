use crate::*;

/// Creates a click event handler that sets the user type signal.
///
/// # Arguments
///
/// - `Signal<ConditionalUserType>` - The user type signal to update.
/// - `ConditionalUserType` - The user type variant to set.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets the user type.
pub(crate) fn user_type_on_select(
    user_type: Signal<ConditionalUserType>,
    value: ConditionalUserType,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        user_type.set(value);
    }))
}

/// Creates a click event handler that sets the active tab signal.
///
/// # Arguments
///
/// - `Signal<ConditionalTab>` - The tab signal to update.
/// - `ConditionalTab` - The tab variant to set.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets the active tab.
pub(crate) fn tab_on_select(
    tab: Signal<ConditionalTab>,
    value: ConditionalTab,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        tab.set(value);
    }))
}
