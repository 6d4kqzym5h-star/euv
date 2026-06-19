use crate::*;

/// Props for the `euv_field` component.
///
/// Defines the strongly-typed interface for a form field with label, input, and error display.
#[derive(Clone, Default)]
pub(crate) struct EuvFieldProps {
    /// The unique identifier for the input element.
    pub(crate) id: &'static str,
    /// The HTML name attribute for the input element.
    pub(crate) name: &'static str,
    /// The label text displayed above the input.
    pub(crate) label: &'static str,
    /// The HTML input type (e.g. "text", "email", "password", "number").
    pub(crate) input_type: &'static str,
    /// The placeholder text shown when the input is empty.
    pub(crate) placeholder: &'static str,
    /// The autocomplete hint for the browser.
    pub(crate) autocomplete: &'static str,
    /// The current input value.
    pub(crate) value: Signal<String>,
    /// The error message signal; when non-empty, the input shows error styling. When None, no error display.
    pub(crate) error: Option<Signal<String>>,
    /// Optional input event handler; defaults to `on_input_value(value)` if None.
    pub(crate) oninput: Option<Rc<dyn Fn(Event)>>,
}
