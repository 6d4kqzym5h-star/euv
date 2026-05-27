use crate::*;

/// Props for the `primary_button` component.
///
/// Defines the strongly-typed interface for the primary button.
#[derive(Data, Debug, Default, New)]
pub(crate) struct PrimaryButtonProps {
    /// The button label text, used as fallback when no children are provided.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) label: String,
    /// Optional click event handler.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) onclick: Option<NativeEventHandler>,
    /// Whether the button is disabled.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) disabled: bool,
}
