use crate::*;

/// Props for the `euv_vconsole_panel` component.
///
/// Defines the strongly-typed interface for the vConsole debug panel.
#[derive(Clone, Debug, Default)]
pub struct EuvVconsolePanelProps {
    /// The reactive signal controlling panel visibility.
    pub panel_open: Signal<bool>,
}

/// Props for the `euv_vconsole_fab` component.
///
/// Defines the strongly-typed interface for the vConsole floating action button.
#[derive(Clone, Debug, Default)]
pub struct EuvVconsoleFabProps {
    /// The reactive signal controlling panel visibility.
    pub panel_open: Signal<bool>,
    /// The reactive signal holding the console log entries.
    pub console_signal: Signal<Vec<ConsoleEntry>>,
}

/// Props for the `euv_vconsole_drawer` component.
///
/// Defines the strongly-typed interface for the vConsole drawer panel.
#[derive(Clone, Debug, Default)]
pub struct EuvVconsoleDrawerProps {
    /// The reactive signal holding the console log entries.
    pub console_signal: Signal<Vec<ConsoleEntry>>,
    /// The reactive signal controlling panel visibility.
    pub panel_open: Signal<bool>,
}
