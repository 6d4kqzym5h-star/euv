use crate::*;

/// Reactive state for a progress bar feature.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseProgress {
    /// Whether the progress bar animation is currently running.
    #[get(type(copy))]
    pub(crate) running: Signal<bool>,
}
