use crate::*;

/// Props for the `form_input` component.
///
/// Defines the strongly-typed interface for the labeled input field.
#[derive(Data, Debug, Default, New)]
pub(crate) struct FormInputProps {
    /// The unique identifier for the input element.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: String,
    /// The label text displayed above the input.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) label: String,
    /// The placeholder text shown when the input is empty.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) placeholder: String,
    /// The current input value.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) value: String,
    /// The autocomplete hint for the browser.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) autocomplete: String,
}
