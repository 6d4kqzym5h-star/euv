use crate::*;

/// Creates a click event handler that toggles the expanded state of a tag section.
///
/// # Arguments
///
/// - `Signal<bool>` - The signal controlling the section expansion.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that toggles the section.
pub(crate) fn use_section_toggle(expanded: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: bool = expanded.get();
        expanded.set(!current);
    }))
}
