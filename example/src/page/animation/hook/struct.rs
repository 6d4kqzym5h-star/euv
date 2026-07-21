use super::*;

/// Reactive state for a progress bar feature.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseProgress {
    /// Whether the progress bar animation is currently running.
    #[get(type(copy))]
    pub(crate) running: Signal<bool>,
}
