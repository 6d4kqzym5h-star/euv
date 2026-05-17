use crate::*;

/// Props for the `primary_button` component.
///
/// Defines the strongly-typed interface for the primary button.
#[derive(Data, Debug, Default)]
pub struct PrimaryButtonProps {
    /// The button label text, used as fallback when no children are provided.
    pub label: String,
    /// Optional click event handler.
    pub onclick: Option<NativeEventHandler>,
}
