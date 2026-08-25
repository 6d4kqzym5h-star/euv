use super::*;

/// Represents the available tabs in the keep-alive demo page.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum KeepAliveTab {
    /// The counter tab demonstrating persistent counter state.
    #[default]
    Counter,
    /// The form tab demonstrating persistent form input state.
    Form,
    /// The timer tab demonstrating persistent timer state.
    Timer,
}
