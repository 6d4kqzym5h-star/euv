use crate::*;

/// Holds the reactive state for the application theme.
///
/// Provides the current theme signal and the derived root element class name
/// signal that combines the app root class with the active theme class.
#[derive(Clone, Copy, Data, New)]
pub struct ThemeState {
    /// The current theme name signal ("light" or "dark").
    #[get(type(copy))]
    pub theme: Signal<String>,
    /// The derived root element class name signal combining the app root
    /// class and the active theme class.
    #[get(type(copy))]
    pub root_class: Signal<String>,
}
