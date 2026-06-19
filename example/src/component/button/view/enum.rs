/// The visual variant of the `euv_button` component.
#[derive(Clone, Default)]
pub(crate) enum EuvButtonVariant {
    /// A primary button with accent background.
    #[default]
    Primary,
    /// A secondary button with transparent background and accent border.
    Secondary,
    /// An outline button with transparent background and subtle border.
    Outline,
    /// A danger button with error-themed styling.
    Danger,
}
