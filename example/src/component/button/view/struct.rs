use crate::*;

/// Props for the `euv_button` component.
///
/// Defines the strongly-typed interface for a versatile button.
#[derive(Clone, Default)]
pub(crate) struct EuvButtonProps {
    /// The visual variant determining colour scheme.
    pub(crate) variant: EuvButtonVariant,
    /// The button label text, used as fallback when no children are provided.
    pub(crate) label: &'static str,
    /// Optional click event handler.
    pub(crate) onclick: Option<Rc<dyn Fn(Event)>>,
    /// Whether the button is disabled.
    pub(crate) disabled: Signal<bool>,
}
