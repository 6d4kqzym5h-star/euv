use crate::*;

/// Creates progress bar state with a running signal.
///
/// # Returns
///
/// - `UseProgress` - The progress state containing a running signal.
pub(crate) fn use_progress() -> UseProgress {
    UseProgress::new(use_signal(|| false))
}

/// Creates a click event handler that starts the progress bar animation.
///
/// Sets running to true to trigger the CSS animation.
///
/// # Arguments
///
/// - `UseProgress` - The progress state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to start the progress bar.
pub(crate) fn progress_on_start(state: UseProgress) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_running().set(true);
    }))
}

/// Creates a click event handler that resets the progress bar.
///
/// Sets running to false to remove the CSS animation and reset the bar.
///
/// # Arguments
///
/// - `UseProgress` - The progress state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to reset the progress bar.
pub(crate) fn progress_on_reset(state: UseProgress) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_running().set(false);
    }))
}

/// Creates a click event handler that cycles to the next animation color.
///
/// # Arguments
///
/// - `Signal<i32>` - The color index signal (0-4).
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to advance the color index.
pub(crate) fn color_cycle_on_next(color_index: Signal<i32>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let current: i32 = color_index.get();
        color_index.set((current + 1) % 5);
    }))
}
