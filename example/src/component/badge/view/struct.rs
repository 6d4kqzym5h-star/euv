use crate::*;

/// Props for the `my_badge` component.
///
/// Defines the strongly-typed interface for the badge status indicator.
pub struct MyBadgeProps {
    /// The badge background/border color.
    pub color: String,
    /// The badge text content.
    pub text: String,
    /// Whether to render in outline style instead of solid fill.
    pub outline: bool,
    /// Optional click event handler.
    pub on_click: Option<NativeEventHandler>,
}
