use crate::*;

/// Reactive state for a lifecycle demo feature.
///
/// # Fields
///
/// - `Signal<i32>`: The render count.
/// - `Signal<Vec<String>>`: The event log entries.
#[derive(Clone, Copy)]
pub struct UseLifecycle {
    pub render_count: Signal<i32>,
    pub logs: Signal<Vec<String>>,
}

/// Creates lifecycle demo state signals wrapped in a `UseLifecycle` struct.
///
/// # Returns
///
/// - `UseLifecycle`: The lifecycle state.
pub fn use_lifecycle() -> UseLifecycle {
    UseLifecycle {
        render_count: use_signal(|| 1),
        logs: use_signal(|| vec!["Component mounted".to_string()]),
    }
}

/// Creates a click event handler that increments the render count and logs the update.
///
/// # Arguments
///
/// - `UseLifecycle`: The lifecycle state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to trigger an update.
pub fn lifecycle_on_trigger(state: UseLifecycle) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let current: i32 = state.render_count.get();
        state.render_count.set(current + 1);
        let mut current_logs: Vec<String> = state.logs.get();
        current_logs.push(format!("Updated: render count = {}", current + 1));
        state.logs.set(current_logs);
    })
}
