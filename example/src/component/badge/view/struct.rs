use crate::*;

/// Props for the `my_badge` component.
///
/// Defines the strongly-typed interface for the badge status indicator.
pub(crate) struct MyBadgeProps {
    /// The badge background/border color.
    pub(crate) color: String,
    /// The badge text content.
    pub(crate) text: String,
    /// Whether to render in outline style instead of solid fill.
    pub(crate) outline: bool,
    /// Optional click event handler.
    pub(crate) on_click: Option<NativeEventHandler>,
}
