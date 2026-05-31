use crate::*;

/// Props for the `my_badge` component.
///
/// Defines the strongly-typed interface for the badge status indicator.
#[derive(Clone, Default)]
pub(crate) struct MyBadgeProps {
    /// The badge background/border color.
    pub(crate) color: &'static str,
    /// The badge text content.
    pub(crate) text: &'static str,
    /// Whether to render in outline style instead of solid fill.
    pub(crate) outline: bool,
    /// Optional click event handler.
    pub(crate) on_click: Option<Rc<dyn Fn(Event)>>,
}
