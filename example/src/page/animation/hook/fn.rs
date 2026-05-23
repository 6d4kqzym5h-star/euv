use crate::*;

/// Creates progress bar state signals wrapped in a `UseProgress` struct.
///
/// # Returns
///
/// - `UseProgress` - The progress state containing value, running, and handle signals.
pub fn use_progress() -> UseProgress {
    UseProgress::new(use_signal(|| 0), use_signal(|| false), use_signal(|| None))
}

/// Creates a click event handler that starts the progress bar animation.
///
/// Resets the progress value to 0, sets running to true, and starts a
/// 30ms interval that increments the progress until it reaches 100.
///
/// # Arguments
///
/// - `UseProgress` - The progress state.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler to start the progress bar.
pub fn progress_on_start(state: UseProgress) -> NativeEventHandler {
    NativeEventHandler::create(NativeEventName::Click, move |_event: Event| {
        state.get_value().set(0);
        state.get_running().set(true);
        let handle_opt: Option<IntervalHandle> = state.get_handle().get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        let value_signal: Signal<i32> = state.get_value();
        let running_signal: Signal<bool> = state.get_running();
        let handle_signal: Signal<Option<IntervalHandle>> = state.get_handle();
        let new_handle: IntervalHandle = use_interval(30, move || {
            if running_signal.get() {
                let current: i32 = value_signal.get();
                if current < 100 {
                    value_signal.set(current + 1);
                } else {
                    running_signal.set(false);
                }
            }
        });
        handle_signal.set(Some(new_handle));
    })
}

/// Creates a click event handler that resets the progress bar.
///
/// # Arguments
///
/// - `UseProgress` - The progress state.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler to reset the progress bar.
pub fn progress_on_reset(state: UseProgress) -> NativeEventHandler {
    NativeEventHandler::create(NativeEventName::Click, move |_event: Event| {
        state.get_running().set(false);
        let handle_opt: Option<IntervalHandle> = state.get_handle().get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        state.get_handle().set(None);
        state.get_value().set(0);
    })
}

/// Creates a click event handler that cycles to the next animation color.
///
/// # Arguments
///
/// - `Signal<i32>` - The color index signal (0-4).
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler to advance the color index.
pub fn color_cycle_on_next(color_index: Signal<i32>) -> NativeEventHandler {
    NativeEventHandler::create(NativeEventName::Click, move |_event: Event| {
        let current: i32 = color_index.get();
        color_index.set((current + 1) % 5);
    })
}
