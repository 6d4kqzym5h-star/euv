/// The visual variant of the `euv_tag` component.
#[derive(Clone, Default)]
pub(crate) enum EuvTagVariant {
    /// A solid-filled tag with coloured background.
    #[default]
    Solid,
    /// An outline tag with transparent background and coloured border.
    Outline,
}

/// The semantic colour type of the `euv_tag` component.
#[derive(Clone, Default)]
pub(crate) enum EuvTagColor {
    /// Success / positive status (green).
    #[default]
    Success,
    /// Error / negative status (red).
    Error,
    /// Warning / caution status (amber).
    Warning,
    /// Informational status (blue).
    Info,
    /// Purple / special status.
    Purple,
}
