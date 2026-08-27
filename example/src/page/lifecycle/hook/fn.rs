use super::*;

/// Builds the human-readable log line for a render-count snapshot.
/// Helper body of the `create_log_data` free function.
///
/// # Arguments
///
/// - `i32` - A 32-bit signed integer (`i32`).
///
/// # Returns
///
/// - `String` - A `String` value.
fn create_log_data(times: i32) -> String {
    format!("Updated: render count = {times}")
}

/// Creates lifecycle demo state signals wrapped in a `UseLifecycle` struct.
///
/// # Returns
///
/// - `UseLifecycle` - The lifecycle state.
pub(crate) fn use_lifecycle() -> UseLifecycle {
    let times: i32 = 1;
    UseLifecycle::new(
        App::use_signal(|| times),
        App::use_signal(|| vec![create_log_data(times)]),
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
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to trigger an update.
pub(crate) fn lifecycle_on_trigger(state: UseLifecycle) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: i32 = state.get_render_count().get();
        let next: i32 = current + 1;
        state.get_render_count().set(next);
        let mut current_logs: Vec<String> = state.get_logs().get();
        current_logs.push(create_log_data(next));
        state.get_logs().set(current_logs);
    }))
}
