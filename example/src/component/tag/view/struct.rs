use crate::*;

/// Props for the `euv_tag` component.
///
/// Defines the strongly-typed interface for a status tag indicator.
#[derive(Clone, Default)]
pub(crate) struct EuvTagProps {
    /// The semantic colour type.
    pub(crate) color: EuvTagColor,
    /// The visual variant (solid or outline).
    pub(crate) variant: EuvTagVariant,
    /// The tag text content.
    pub(crate) text: &'static str,
    /// Optional click event handler.
    pub(crate) on_click: Option<Rc<dyn Fn(Event)>>,
}
