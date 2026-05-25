use crate::*;

/// Props for the `primary_button` component.
///
/// Defines the strongly-typed interface for the primary button.
#[derive(Data, Debug, Default)]
pub(crate) struct PrimaryButtonProps {
    /// The button label text, used as fallback when no children are provided.
    pub(crate) label: String,
    /// Optional click event handler.
    pub(crate) onclick: Option<NativeEventHandler>,
    /// Whether the button is disabled.
    pub(crate) disabled: bool,
}
