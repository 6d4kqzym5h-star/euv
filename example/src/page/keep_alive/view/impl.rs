use super::*;

/// Implements `Display` for `KeepAliveTab` to provide human-readable tab labels.
impl std::fmt::Display for KeepAliveTab {
    /// Formats the tab variant as its display label string.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter to write the display string into.
    ///
    /// # Returns
    ///
    /// - `Result` - Whether the formatting succeeded.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_text: &str = match self {
            KeepAliveTab::Counter => "Counter",
            KeepAliveTab::Form => "Form",
            KeepAliveTab::Timer => "Timer",
        };
        f.write_str(display_text)
    }
}
