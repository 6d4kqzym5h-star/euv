use crate::*;

/// Props for the `form_input` component.
///
/// Defines the strongly-typed interface for the labeled input field.
#[derive(Data, Debug, Default)]
pub(crate) struct FormInputProps {
    /// The unique identifier for the input element.
    pub(crate) id: String,
    /// The label text displayed above the input.
    pub(crate) label: String,
    /// The placeholder text shown when the input is empty.
    pub(crate) placeholder: String,
    /// The current input value.
    pub(crate) value: String,
    /// The autocomplete hint for the browser.
    pub(crate) autocomplete: String,
}
