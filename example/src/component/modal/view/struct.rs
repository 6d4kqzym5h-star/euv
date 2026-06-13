use crate::*;

/// Props for the `my_modal` component.
///
/// Defines the strongly-typed interface for the modal dialog.
#[derive(Clone, Default)]
pub(crate) struct MyModalProps {
    /// The modal title text.
    pub(crate) title: &'static str,
    /// Optional close handler triggered by overlay or close button click.
    pub(crate) onclick: Option<Rc<dyn Fn(Event)>>,
    /// Whether the modal is playing its exit animation.
    pub(crate) closing: bool,
}
