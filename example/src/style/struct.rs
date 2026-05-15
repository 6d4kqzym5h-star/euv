use crate::*;

/// Holds the reactive state for the application theme.
///
/// Provides the current theme signal and the derived CSS variables signal
/// for injecting into the root element's style attribute.
pub struct ThemeState {
    /// The current theme name signal ("light" or "dark").
    pub theme: Signal<String>,
    /// The derived CSS custom properties string signal for the current theme.
    pub style: Signal<String>,
}
