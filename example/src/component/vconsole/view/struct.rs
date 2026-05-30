use crate::*;

/// Props for the `vconsole_panel` component.
///
/// Defines the strongly-typed interface for the vConsole debug panel.
#[derive(Default)]
pub(crate) struct VconsolePanelProps {
    /// The reactive signal controlling panel visibility.
    pub(crate) panel_open: Signal<bool>,
}

/// Props for the `vconsole_fab` component.
///
/// Defines the strongly-typed interface for the vConsole floating action button.
#[derive(Default)]
pub(crate) struct VconsoleFabProps {
    /// The reactive signal controlling panel visibility.
    pub(crate) panel_open: Signal<bool>,
    /// The current number of log entries.
    pub(crate) log_count: usize,
}

/// Props for the `vconsole_drawer` component.
///
/// Defines the strongly-typed interface for the vConsole drawer panel.
#[derive(Default)]
pub(crate) struct VconsoleDrawerProps {
    /// The reactive signal holding the console log entries.
    pub(crate) console_signal: Signal<Vec<ConsoleEntry>>,
    /// The reactive signal controlling panel visibility.
    pub(crate) panel_open: Signal<bool>,
    /// The current number of log entries.
    pub(crate) log_count: usize,
}
