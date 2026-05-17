use crate::*;

/// Props for the `form_input` component.
///
/// Defines the strongly-typed interface for the labeled input field.
#[derive(Data, Debug, Default)]
pub struct FormInputProps {
    /// The label text displayed above the input.
    pub label: String,
    /// The placeholder text shown when the input is empty.
    pub placeholder: String,
    /// The current input value.
    pub value: String,
}
