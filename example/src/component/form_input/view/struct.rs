
/// Props for the `form_input` component.
///
/// Defines the strongly-typed interface for the labeled input field.
#[derive(Default)]
pub(crate) struct FormInputProps {
    /// The unique identifier for the input element.
    pub(crate) id: &'static str,
    /// The label text displayed above the input.
    pub(crate) label: &'static str,
    /// The placeholder text shown when the input is empty.
    pub(crate) placeholder: &'static str,
    /// The current input value.
    pub(crate) value: &'static str,
    /// The autocomplete hint for the browser.
    pub(crate) autocomplete: &'static str,
}
