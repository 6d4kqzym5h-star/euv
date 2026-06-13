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
    /// Whether the modal is playing its closing (exit) animation.
    ///
    /// When `true`, the overlay and content swap to their `*_closing` classes,
    /// which run the `euv-fade-out` / `euv-scale-out-modal` keyframes so the
    /// modal animates out before being removed from the DOM. This mirrors the
    /// open animation for a consistent enter/exit motion.
    pub(crate) closing: Signal<bool>,
}
