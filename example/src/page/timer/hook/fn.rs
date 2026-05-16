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
/// - `FnMut() + 'static`: The closure to invoke on each interval tick.
///
/// # Returns
///
/// - `IntervalHandle`: A handle that can be used to cancel the interval.
///
/// # Panics
///
/// Panics if `window()` is unavailable on the current platform.
pub(crate) fn use_interval<F>(millis: i32, callback: F) -> IntervalHandle
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

/// Creates stopwatch state signals wrapped in a `UseStopwatch` struct.
///
/// Must be called at the top level of a component function (not inside
/// conditionals or loops) to maintain hook call order stability.
///
/// # Returns
///
/// - `UseStopwatch`: The stopwatch state containing seconds, running, and handle signals.
pub fn use_stopwatch() -> UseStopwatch {
    let seconds: Signal<i32> = use_signal(|| 0);
    let running: Signal<bool> = use_signal(|| false);
    let handle: Signal<Option<IntervalHandle>> = use_signal(|| None);
    UseStopwatch {
        seconds,
        running,
        handle,
    }
}

/// Creates countdown state signals wrapped in a `UseCountdown` struct.
///
/// Must be called at the top level of a component function (not inside
/// conditionals or loops) to maintain hook call order stability.
///
/// # Returns
///
/// - `UseCountdown`: The countdown state containing total, remaining, running, handle, and input signals.
pub fn use_countdown() -> UseCountdown {
    let total: Signal<i32> = use_signal(|| 60);
    let remaining: Signal<i32> = use_signal(|| 60);
    let running: Signal<bool> = use_signal(|| false);
    let handle: Signal<Option<IntervalHandle>> = use_signal(|| None);
    let input: Signal<String> = use_signal(|| "60".to_string());
    UseCountdown {
        total,
        remaining,
        running,
        handle,
        input,
    }
}

/// Creates a click event handler that starts the stopwatch.
///
/// # Arguments
///
/// - `UseStopwatch`: The stopwatch state (only `Copy` signals are read from this).
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to start the stopwatch.
pub fn stopwatch_on_start(state: UseStopwatch) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let was_running: bool = state.running.get();
        if !was_running {
            state.running.set(true);
            let handle_opt: Option<IntervalHandle> = state.handle.get();
            if let Some(existing_handle) = handle_opt {
                existing_handle.clear();
            }
            let seconds_signal: Signal<i32> = state.seconds;
            let running_signal: Signal<bool> = state.running;
            let handle_signal: Signal<Option<IntervalHandle>> = state.handle;
            let new_handle: IntervalHandle = use_interval(1000, move || {
                if running_signal.get() {
                    let current: i32 = seconds_signal.get();
                    seconds_signal.set(current + 1);
                }
            });
            handle_signal.set(Some(new_handle));
        }
    })
}

/// Creates a click event handler that pauses the stopwatch.
///
/// # Arguments
///
/// - `UseStopwatch`: The stopwatch state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to pause the stopwatch.
pub fn stopwatch_on_pause(state: UseStopwatch) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        state.running.set(false);
        let handle_opt: Option<IntervalHandle> = state.handle.get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        state.handle.set(None);
    })
}

/// Creates a click event handler that resets the stopwatch.
///
/// # Arguments
///
/// - `UseStopwatch`: The stopwatch state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to reset the stopwatch.
pub fn stopwatch_on_reset(state: UseStopwatch) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        state.running.set(false);
        let handle_opt: Option<IntervalHandle> = state.handle.get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        state.handle.set(None);
        state.seconds.set(0);
    })
}

/// Creates a click event handler that starts the countdown.
///
/// # Arguments
///
/// - `UseCountdown`: The countdown state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to start the countdown.
pub fn countdown_on_start(state: UseCountdown) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let input_text: String = state.input.get();
        let parsed: i32 = input_text.parse::<i32>().unwrap_or(60);
        let safe_total: i32 = if parsed > 0 { parsed } else { 60 };
        state.total.set(safe_total);
        state.remaining.set(safe_total);
        state.running.set(true);
        let handle_opt: Option<IntervalHandle> = state.handle.get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        let remaining_signal: Signal<i32> = state.remaining;
        let running_signal: Signal<bool> = state.running;
        let handle_signal: Signal<Option<IntervalHandle>> = state.handle;
        let new_handle: IntervalHandle = use_interval(1000, move || {
            if running_signal.get() {
                let current: i32 = remaining_signal.get();
                if current > 0 {
                    remaining_signal.set(current - 1);
                } else {
                    running_signal.set(false);
                }
            }
        });
        handle_signal.set(Some(new_handle));
    })
}

/// Creates a click event handler that pauses the countdown.
///
/// # Arguments
///
/// - `UseCountdown`: The countdown state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to pause the countdown.
pub fn countdown_on_pause(state: UseCountdown) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        state.running.set(false);
        let handle_opt: Option<IntervalHandle> = state.handle.get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        state.handle.set(None);
    })
}

/// Creates a click event handler that resets the countdown.
///
/// # Arguments
///
/// - `UseCountdown`: The countdown state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to reset the countdown.
pub fn countdown_on_reset(state: UseCountdown) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        state.running.set(false);
        let handle_opt: Option<IntervalHandle> = state.handle.get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        state.handle.set(None);
        let current_total: i32 = state.total.get();
        state.remaining.set(current_total);
    })
}

/// Creates an input event handler that updates the countdown input signal.
///
/// # Arguments
///
/// - `UseCountdown`: The countdown state.
///
/// # Returns
///
/// - `NativeEventHandler`: An input handler for the countdown input field.
pub fn countdown_on_input(state: UseCountdown) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Input, move |event: NativeEvent| {
        if let NativeEvent::Input(input_event) = event {
            state.input.set(input_event.get_value().clone());
        }
    })
}
