use crate::*;

/// Props for the `my_modal` component.
///
/// Defines the strongly-typed interface for the modal dialog.
pub struct MyModalProps {
    /// The modal title text.
    pub title: String,
    /// Optional close handler triggered by overlay or close button click.
    pub on_close: Option<NativeEventHandler>,
}
