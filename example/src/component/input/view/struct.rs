use crate::*;

/// Props for the `euv_input` component.
///
/// Defines the strongly-typed interface for the labeled input field.
#[derive(Clone, Default)]
pub(crate) struct EuvInputProps {
    /// The unique identifier for the input element.
    pub(crate) id: &'static str,
    /// The HTML name attribute for the input element.
    pub(crate) name: &'static str,
    /// The label text displayed above the input. Empty string means no label.
    pub(crate) label: &'static str,
    /// The HTML input type (e.g. "text", "email", "password", "number").
    pub(crate) input_type: &'static str,
    /// The placeholder text shown when the input is empty.
    pub(crate) placeholder: &'static str,
    /// The current input value (reactive Signal).
    pub(crate) value: Signal<String>,
    /// The autocomplete hint for the browser.
    pub(crate) autocomplete: &'static str,
    /// Optional input event handler. Defaults to `on_input_value(value)` if None.
    pub(crate) oninput: Option<Rc<dyn Fn(Event)>>,
    /// Optional custom CSS class. Defaults to `c_euv_input()` if empty.
    pub(crate) class: Css,
}
