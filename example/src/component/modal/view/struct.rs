use crate::*;

/// Props for the `my_modal` component.
///
/// Defines the strongly-typed interface for the modal dialog.
#[derive(Data, New)]
pub(crate) struct MyModalProps {
    /// The modal title text.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) title: String,
    /// Optional close handler triggered by overlay or close button click.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) on_close: Option<NativeEventHandler>,
}
