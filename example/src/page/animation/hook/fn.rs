use crate::*;

/// Starts a recurring interval that invokes the given closure at the specified period.
///
/// Internally creates a `Closure<dyn FnMut()>` and registers it with
/// `window.setInterval`. The closure is leaked via `Closure::forget` so it
/// persists for the application lifetime.
///
/// Returns an `IntervalHandle` that can be used to cancel the timer via
/// `IntervalHandle::clear`.
///
/// # Arguments
///
/// - `i32`: The interval period in milliseconds.
/// - `F`: The closure to invoke on each interval tick.
///
/// # Returns
///
/// - `IntervalHandle`: A handle that can be used to cancel the interval.
///
/// # Panics
///
/// Panics if `window()` is unavailable on the current platform.
pub fn use_interval<F>(millis: i32, callback: F) -> IntervalHandle
where
    F: FnMut() + 'static,
{
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(callback));
    let window: Window = window().expect("no global window exists");
    let interval_id: i32 = window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            millis,
        )
        .expect("failed to set interval");
    closure.forget();
    IntervalHandle { interval_id }
}
