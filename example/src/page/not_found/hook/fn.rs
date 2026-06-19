use crate::*;

/// Creates a click event handler that navigates to the home page.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler that navigates to "/".
pub(crate) fn not_found_on_go_home() -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        navigate("/");
    }))
}
