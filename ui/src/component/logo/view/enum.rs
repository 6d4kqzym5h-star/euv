/// Display variant for the `euv_logo` component.
///
/// Controls the size and positioning of the logo button:
/// - `Nav` - Inline 32×32 button for the navigation sidebar.
/// - `Fab` - Fixed-position 48×48 button for the floating action button.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LogoButtonVariant {
    /// Inline logo button for the navigation sidebar.
    #[default]
    Nav,
    /// Fixed-position floating action button.
    Fab,
}
