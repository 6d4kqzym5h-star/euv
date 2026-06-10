use crate::*;

/// Creates stopwatch state signals wrapped in a `UseStopwatch` struct.
///
/// Must be called at the top level of a component function (not inside
/// conditionals or loops) to maintain hook call order stability.
///
/// # Returns
///
/// - `UseStopwatch` - The stopwatch state containing seconds, running, and handle signals.
pub(crate) fn use_stopwatch() -> UseStopwatch {
    UseStopwatch::new(use_signal(|| 0), use_signal(|| false), use_signal(|| None))
}

/// Creates countdown state signals wrapped in a `UseCountdown` struct.
///
/// Must be called at the top level of a component function (not inside
/// conditionals or loops) to maintain hook call order stability.
///
/// # Returns
///
/// - `UseCountdown` - The countdown state containing total, remaining, running, handle, and input signals.
pub(crate) fn use_countdown() -> UseCountdown {
    UseCountdown::new(
        use_signal(|| 60),
        use_signal(|| 60),
        use_signal(|| false),
        use_signal(|| None),
        use_signal(|| "60".to_string()),
    )
}

/// Creates a click event handler that starts the stopwatch.
///
/// # Arguments
///
/// - `UseStopwatch` - The stopwatch state (only `Copy` signals are read from this).
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to start the stopwatch.
pub(crate) fn stopwatch_on_start(state: UseStopwatch) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let was_running: bool = state.get_running().get();
        if was_running {
            return;
        }
        state.get_running().set(true);
        let handle_opt: Option<IntervalHandle> = state.get_handle().get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        let seconds_signal: Signal<i32> = state.get_seconds();
        let handle_signal: Signal<Option<IntervalHandle>> = state.get_handle();
        let new_handle: IntervalHandle = use_interval(1000, move || {
            let current: i32 = seconds_signal.get();
            seconds_signal.set(current + 1);
        });
        handle_signal.set(Some(new_handle));
    }))
}

/// Creates a click event handler that pauses the stopwatch.
///
/// Clears the interval first to stop ticking immediately, then updates state.
///
/// # Arguments
///
/// - `UseStopwatch` - The stopwatch state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to pause the stopwatch.
pub(crate) fn stopwatch_on_pause(state: UseStopwatch) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let handle_opt: Option<IntervalHandle> = state.get_handle().get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        state.get_handle().set(None);
        state.get_running().set(false);
    }))
}

/// Creates a click event handler that resets the stopwatch.
///
/// Immediately clears the interval, resets running state and seconds counter.
///
/// # Arguments
///
/// - `UseStopwatch` - The stopwatch state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to reset the stopwatch.
pub(crate) fn stopwatch_on_reset(state: UseStopwatch) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let handle_opt: Option<IntervalHandle> = state.get_handle().get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        state.get_handle().set(None);
        state.get_running().set(false);
        state.get_seconds().set(0);
    }))
}

/// Creates a click event handler that starts the countdown.
///
/// If the countdown was paused (handle is None but remaining > 0 and remaining < total),
/// resumes from the current remaining value. Otherwise starts fresh from the input value.
///
/// # Arguments
///
/// - `UseCountdown` - The countdown state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to start the countdown.
pub(crate) fn countdown_on_start(state: UseCountdown) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let was_running: bool = state.get_running().get();
        if was_running {
            return;
        }
        let current_remaining: i32 = state.get_remaining().get();
        let current_total: i32 = state.get_total().get();
        let has_paused_state: bool = current_remaining > 0 && current_remaining < current_total;
        if !has_paused_state {
            let input_text: String = state.get_input().get();
            let parsed: i32 = input_text.parse::<i32>().unwrap_or(60);
            let safe_total: i32 = if parsed > 0 { parsed } else { 60 };
            state.get_total().set(safe_total);
            state.get_remaining().set(safe_total);
        }
        state.get_running().set(true);
        let handle_opt: Option<IntervalHandle> = state.get_handle().get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        let remaining_signal: Signal<i32> = state.get_remaining();
        let running_signal: Signal<bool> = state.get_running();
        let handle_signal: Signal<Option<IntervalHandle>> = state.get_handle();
        let new_handle: IntervalHandle = use_interval(1000, move || {
            if running_signal.get() {
                let current: i32 = remaining_signal.get();
                if current > 0 {
                    remaining_signal.set(current - 1);
                } else {
                    running_signal.set(false);
                    handle_signal.set(None);
                }
            }
        });
        handle_signal.set(Some(new_handle));
    }))
}

/// Creates a click event handler that pauses the countdown.
///
/// Stops the interval and sets running to false, preserving the current
/// remaining value so the countdown can be resumed from where it was paused.
///
/// # Arguments
///
/// - `UseCountdown` - The countdown state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to pause the countdown.
pub(crate) fn countdown_on_pause(state: UseCountdown) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let handle_opt: Option<IntervalHandle> = state.get_handle().get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        state.get_handle().set(None);
        state.get_running().set(false);
    }))
}

/// Creates a click event handler that resets the countdown.
///
/// Immediately clears the interval, resets running state, and restores
/// remaining to the total value.
///
/// # Arguments
///
/// - `UseCountdown` - The countdown state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to reset the countdown.
pub(crate) fn countdown_on_reset(state: UseCountdown) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let handle_opt: Option<IntervalHandle> = state.get_handle().get();
        if let Some(existing_handle) = handle_opt {
            existing_handle.clear();
        }
        state.get_handle().set(None);
        state.get_running().set(false);
        let current_total: i32 = state.get_total().get();
        state.get_remaining().set(current_total);
    }))
}

/// Creates an input event handler that updates the countdown input signal.
///
/// # Arguments
///
/// - `UseCountdown` - The countdown state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - An input handler for the countdown input field.
pub(crate) fn countdown_on_input(state: UseCountdown) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            state.get_input().set(input.value());
        }
    }))
}
