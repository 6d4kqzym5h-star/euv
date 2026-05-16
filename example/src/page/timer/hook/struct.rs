use crate::*;

/// A handle to a browser interval timer created by `use_interval`.
///
/// Stores the numeric interval ID returned by `window.setInterval` so the
/// timer can be cancelled later via `clear_interval`.
///
/// # Fields
///
/// - `i32`: The interval ID assigned by the browser.
pub struct IntervalHandle {
    pub(crate) interval_id: i32,
}

/// Reactive state for a stopwatch feature.
///
/// Contains only `Copy` signal fields so the struct can be freely
/// captured inside `html!` closures without causing `FnOnce` issues.
///
/// # Fields
///
/// - `Signal<i32>`: The elapsed seconds counter.
/// - `Signal<bool>`: Whether the stopwatch is currently running.
/// - `Signal<Option<IntervalHandle>>`: The active interval handle, if any.
#[derive(Clone, Copy)]
pub struct UseStopwatch {
    pub seconds: Signal<i32>,
    pub running: Signal<bool>,
    pub handle: Signal<Option<IntervalHandle>>,
}

/// Reactive state for a countdown timer feature.
///
/// Contains only `Copy` signal fields so the struct can be freely
/// captured inside `html!` closures without causing `FnOnce` issues.
///
/// # Fields
///
/// - `Signal<i32>`: The total countdown seconds.
/// - `Signal<i32>`: The remaining countdown seconds.
/// - `Signal<bool>`: Whether the countdown is currently running.
/// - `Signal<Option<IntervalHandle>>`: The active interval handle, if any.
/// - `Signal<String>`: The user input string for setting countdown seconds.
#[derive(Clone, Copy)]
pub struct UseCountdown {
    pub total: Signal<i32>,
    pub remaining: Signal<i32>,
    pub running: Signal<bool>,
    pub handle: Signal<Option<IntervalHandle>>,
    pub input: Signal<String>,
}
