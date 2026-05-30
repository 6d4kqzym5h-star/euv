use crate::*;

/// Props for the `logo_button` component.
///
/// Defines the strongly-typed interface for a branded logo button
/// that renders the "E" letter with a gradient background.
#[derive(Default)]
pub(crate) struct LogoButtonProps {
    /// The display variant controlling size and positioning.
    pub(crate) variant: LogoButtonVariant,
    /// Optional click event handler.
    pub(crate) on_click: Option<Rc<dyn Fn(Event)>>,
    /// The child content rendered inside the button (e.g. badge overlay).
    pub(crate) children: VirtualNode,
}
