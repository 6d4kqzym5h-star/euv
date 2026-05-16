use crate::*;

/// Holds the reactive state for the application theme.
///
/// Provides the current theme signal and the derived root element class name
/// signal that combines the app root class with the active theme class.
pub(crate) struct ThemeState {
    /// The current theme name signal ("light" or "dark").
    pub theme: Signal<String>,
    /// The derived root element class name signal combining the app root
    /// class and the active theme class.
    pub root_class: Signal<String>,
}
