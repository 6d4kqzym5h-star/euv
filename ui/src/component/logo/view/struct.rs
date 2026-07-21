use super::*;

/// Props for the `euv_logo` component.
///
/// Defines the strongly-typed interface for a branded logo button
/// that renders the "E" letter with a gradient background.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvLogoProps {
    /// The display variant controlling size and positioning.
    #[get(type(copy))]
    pub variant: LogoButtonVariant,
    /// Optional click event handler.
    #[debug(skip)]
    pub on_click: Option<Rc<dyn Fn(Event)>>,
}
