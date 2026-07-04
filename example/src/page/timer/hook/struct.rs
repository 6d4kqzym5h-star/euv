use crate::*;

/// Reactive state for a stopwatch feature.
///
/// Contains only `Copy` signal fields so the struct can be freely
/// captured inside `html!` closures without causing `FnOnce` issues.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseStopwatch {
    /// The elapsed seconds counter.
    #[get(type(copy))]
    pub(crate) seconds: Signal<i32>,
    /// Whether the stopwatch is currently running.
    #[get(type(copy))]
    pub(crate) running: Signal<bool>,
    /// The active interval handle, if any.
    #[get(type(copy))]
    pub(crate) handle: Signal<Option<IntervalHandle>>,
}

/// Reactive state for a countdown timer feature.
///
/// Contains only `Copy` signal fields so the struct can be freely
/// captured inside `html!` closures without causing `FnOnce` issues.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseCountdown {
    /// The total countdown seconds.
    #[get(type(copy))]
    pub(crate) total: Signal<i32>,
    /// The remaining countdown seconds.
    #[get(type(copy))]
    pub(crate) remaining: Signal<i32>,
    /// Whether the countdown is currently running.
    #[get(type(copy))]
    pub(crate) running: Signal<bool>,
    /// The active interval handle, if any.
    #[get(type(copy))]
    pub(crate) handle: Signal<Option<IntervalHandle>>,
    /// The user input string for setting countdown seconds.
    #[get(type(copy))]
    pub(crate) input: Signal<String>,
}
