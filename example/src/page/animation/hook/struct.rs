use crate::*;

/// Reactive state for a progress bar feature.
#[derive(Clone, Copy, Data, New)]
pub struct UseProgress {
    /// The progress value (0-100).
    #[get(pub, type(copy))]
    pub value: Signal<i32>,
    /// Whether the progress is currently running.
    #[get(pub, type(copy))]
    pub running: Signal<bool>,
    /// The active interval handle, if any.
    #[get(pub, type(copy))]
    pub handle: Signal<Option<IntervalHandle>>,
}
