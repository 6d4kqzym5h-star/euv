use crate::*;

/// Reactive state for a lifecycle demo feature.
#[derive(Clone, Copy, Data, New)]
pub struct UseLifecycle {
    /// The render count.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub render_count: Signal<i32>,
    /// The event log entries.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub logs: Signal<Vec<String>>,
}
