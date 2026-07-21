use super::*;

/// Props for the `euv_modal` component.
///
/// Defines the strongly-typed interface for the modal dialog.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvModalProps {
    /// The modal title text.
    #[get(type(copy))]
    pub title: &'static str,
    /// Optional close handler triggered by overlay or close button click.
    #[debug(skip)]
    pub onclick: Option<Rc<dyn Fn(Event)>>,
}
