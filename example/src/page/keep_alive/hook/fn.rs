use crate::*;

/// Creates a click event handler that sets the active tab signal for the keep-alive demo.
///
/// # Arguments
///
/// - `Signal<String>` - The tab signal to update.
/// - `&str` - The tab value to set.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler that sets the active tab.
pub(crate) fn keep_alive_tab_on_select(
    tab: Signal<String>,
    value: &str,
) -> Option<Rc<dyn Fn(Event)>> {
    let value_owned: String = value.to_string();
    Some(Rc::new(move |_event: Event| {
        tab.set(value_owned.clone());
    }))
}
