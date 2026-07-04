/// The visual variant of the `euv_button` component.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EuvButtonVariant {
    /// A solid button with accent background and contrasting text.
    #[default]
    Primary,
    /// An outline button with transparent background and accent border.
    Outline,
}
