use crate::*;

/// Props for the `euv_field` component.
///
/// Defines the strongly-typed interface for a form field with label, input, and error display.
#[derive(Clone, CustomDebug, Data, Default)]
pub struct EuvFieldProps {
    /// The unique identifier for the input element.
    #[get(type(copy))]
    pub id: &'static str,
    /// The HTML name attribute for the input element.
    #[get(type(copy))]
    pub name: &'static str,
    /// The label text displayed above the input.
    #[get(type(copy))]
    pub label: &'static str,
    /// The HTML input type (e.g. "text", "email", "password", "number").
    #[get(type(copy))]
    pub input_type: &'static str,
    /// The placeholder text shown when the input is empty.
    #[get(type(copy))]
    pub placeholder: &'static str,
    /// The autocomplete hint for the browser.
    #[get(type(copy))]
    pub autocomplete: &'static str,
    /// The current input value.
    #[get(type(copy))]
    pub value: Signal<String>,
    /// The error message signal; when non-empty, the input shows error styling. When None, no error display.
    pub error: Option<Signal<String>>,
    /// Optional input event handler; defaults to `on_input_value(value)` if None.
    #[debug(skip)]
    pub oninput: Option<Rc<dyn Fn(Event)>>,
}
