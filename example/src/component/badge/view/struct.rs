use crate::*;

/// Props for the `my_badge` component.
///
/// Defines the strongly-typed interface for the badge status indicator.
#[derive(Clone, Default)]
pub(crate) struct MyBadgeProps {
    /// Whether to render in outline style instead of solid fill.
    pub(crate) outline: bool,
    /// The badge text content.
    pub(crate) text: &'static str,
    /// Optional click event handler.
    pub(crate) on_click: Option<Rc<dyn Fn(Event)>>,
}
