use crate::*;

/// A handle to a browser interval timer created by `use_interval`.
///
/// Stores the numeric interval ID returned by `window.setInterval` so the
/// timer can be cancelled later via `clear_interval`.
#[derive(Clone, Copy, Data, New)]
pub struct IntervalHandle {
    /// The interval ID assigned by the browser.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) interval_id: i32,
}

/// Reactive state for a stopwatch feature.
///
/// Contains only `Copy` signal fields so the struct can be freely
/// captured inside `html!` closures without causing `FnOnce` issues.
#[derive(Clone, Copy, Data, New)]
pub struct UseStopwatch {
    /// The elapsed seconds counter.
    #[get(pub, type(copy))]
    pub seconds: Signal<i32>,
    /// Whether the stopwatch is currently running.
    #[get(pub, type(copy))]
    pub running: Signal<bool>,
    /// The active interval handle, if any.
    #[get(pub, type(copy))]
    pub handle: Signal<Option<IntervalHandle>>,
}

/// Reactive state for a countdown timer feature.
///
/// Contains only `Copy` signal fields so the struct can be freely
/// captured inside `html!` closures without causing `FnOnce` issues.
#[derive(Clone, Copy, Data, New)]
pub struct UseCountdown {
    /// The total countdown seconds.
    #[get(pub, type(copy))]
    pub total: Signal<i32>,
    /// The remaining countdown seconds.
    #[get(pub, type(copy))]
    pub remaining: Signal<i32>,
    /// Whether the countdown is currently running.
    #[get(pub, type(copy))]
    pub running: Signal<bool>,
    /// The active interval handle, if any.
    #[get(pub, type(copy))]
    pub handle: Signal<Option<IntervalHandle>>,
    /// The user input string for setting countdown seconds.
    #[get(pub, type(copy))]
    pub input: Signal<String>,
}
