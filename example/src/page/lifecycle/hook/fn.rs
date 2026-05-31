use crate::*;

/// Creates lifecycle demo state signals wrapped in a `UseLifecycle` struct.
///
/// # Returns
///
/// - `UseLifecycle` - The lifecycle state.
pub(crate) fn use_lifecycle() -> UseLifecycle {
    UseLifecycle::new(use_signal(|| 0), use_signal(std::vec::Vec::new))
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
    Some(Rc::new(move |_event: Event| {
        let current: i32 = state.get_render_count().get();
        let next: i32 = current + 1;
        state.get_render_count().set(next);
        let mut current_logs: Vec<String> = state.get_logs().get();
        current_logs.push(format!("Updated: render count = {}", next));
        state.get_logs().set(current_logs);
    }))
}
