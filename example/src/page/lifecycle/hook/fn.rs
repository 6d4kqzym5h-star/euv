use crate::*;

/// Creates lifecycle demo state signals wrapped in a `UseLifecycle` struct.
///
/// # Returns
///
/// - `UseLifecycle` - The lifecycle state.
pub fn use_lifecycle() -> UseLifecycle {
    UseLifecycle::new(
        use_signal(|| 1),
        use_signal(|| vec!["Component mounted".to_string()]),
    )
}

/// Creates a click event handler that increments the render count and logs the update.
///
/// # Arguments
///
/// - `UseLifecycle` - The lifecycle state.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler to trigger an update.
pub fn lifecycle_on_trigger(state: UseLifecycle) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: Event| {
        let current: i32 = state.get_render_count().get();
        state.get_render_count().set(current + 1);
        let mut current_logs: Vec<String> = state.get_logs().get();
        current_logs.push(format!("Updated: render count = {}", current + 1));
        state.get_logs().set(current_logs);
    })
}
