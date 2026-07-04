use crate::*;

/// Props for the `euv_input` component.
///
/// Defines the strongly-typed interface for the labeled input field.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvInputProps {
    /// The unique identifier for the input element.
    pub id: &'static str,
    /// The HTML name attribute for the input element.
    pub name: &'static str,
    /// The label text displayed above the input. Empty string means no label.
    pub label: &'static str,
    /// The HTML input type (e.g. "text", "email", "password", "number").
    pub input_type: &'static str,
    /// The placeholder text shown when the input is empty.
    pub placeholder: &'static str,
    /// The current input value (reactive Signal).
    pub value: Signal<String>,
    /// The autocomplete hint for the browser.
    pub autocomplete: &'static str,
    /// Optional input event handler. Defaults to `on_input_value(value)` if None.
    #[debug(skip)]
    pub oninput: Option<Rc<dyn Fn(Event)>>,
    /// Optional custom CSS class. Defaults to `c_euv_input()` if empty.
    pub class: Css,
}
