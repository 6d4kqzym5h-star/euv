use crate::*;

/// Props for the `my_modal` component.
///
/// Defines the strongly-typed interface for the modal dialog.
pub(crate) struct MyModalProps {
    /// The modal title text.
    pub(crate) title: String,
    /// Optional close handler triggered by overlay or close button click.
    pub(crate) on_close: Option<NativeEventHandler>,
}
