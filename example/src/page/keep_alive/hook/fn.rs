use crate::*;

/// Creates a click event handler that sets the active tab signal for the keep-alive demo.
///
/// # Arguments
///
/// - `Signal<KeepAliveTab>` - The tab signal to update.
/// - `KeepAliveTab` - The tab variant to set.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that sets the active tab.
pub(crate) fn keep_alive_tab_on_select(
    tab: Signal<KeepAliveTab>,
    value: KeepAliveTab,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        tab.set(value);
    }))
}
