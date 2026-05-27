use crate::*;

/// Props for the `my_badge` component.
///
/// Defines the strongly-typed interface for the badge status indicator.
#[derive(Data, New)]
pub(crate) struct MyBadgeProps {
    /// The badge background/border color.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) color: String,
    /// The badge text content.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) text: String,
    /// Whether to render in outline style instead of solid fill.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) outline: bool,
    /// Optional click event handler.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) on_click: Option<NativeEventHandler>,
}
