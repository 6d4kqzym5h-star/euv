use crate::*;

/// Props for the `primary_button` component.
///
/// Defines the strongly-typed interface for the primary button.
#[derive(Default)]
pub(crate) struct PrimaryButtonProps {
    /// The button label text, used as fallback when no children are provided.
    pub(crate) label: &'static str,
    /// Optional click event handler.
    pub(crate) onclick: Option<Rc<dyn Fn(Event)>>,
    /// Whether the button is disabled.
    pub(crate) disabled: bool,
    /// The child content rendered inside the button.
    pub(crate) children: VirtualNode,
}
