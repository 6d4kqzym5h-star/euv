use crate::*;

/// Reactive state for a lifecycle demo feature.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseLifecycle {
    /// The render count.
    #[get(type(copy))]
    pub(crate) render_count: Signal<i32>,
    /// The event log entries.
    #[get(type(copy))]
    pub(crate) logs: Signal<Vec<String>>,
}
