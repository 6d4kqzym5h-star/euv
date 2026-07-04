/// The visual variant of the `euv_tag` component.
#[derive(Clone, Copy, Debug, Default)]
pub enum EuvTagVariant {
    /// A solid-filled tag with coloured background.
    #[default]
    Solid,
    /// An outline tag with transparent background and coloured border.
    Outline,
}

/// The semantic colour type of the `euv_tag` component.
#[derive(Clone, Copy, Debug, Default)]
pub enum EuvTagColor {
    /// Black tag with white text.
    #[default]
    Black,
    /// White tag with black text.
    White,
}
