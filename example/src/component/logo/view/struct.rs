use crate::*;

/// Props for the `euv_logo` component.
///
/// Defines the strongly-typed interface for a branded logo button
/// that renders the "E" letter with a gradient background.
#[derive(Clone, Default)]
pub(crate) struct EuvLogoProps {
    /// The display variant controlling size and positioning.
    pub(crate) variant: LogoButtonVariant,
    /// Optional click event handler.
    pub(crate) on_click: Option<Rc<dyn Fn(Event)>>,
}
