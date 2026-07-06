/// The JavaScript `performance` object property name used to access the high-resolution timer.
pub(crate) const PERFORMANCE_OBJECT: &str = "performance";

/// The JavaScript `now` method name on the `performance` object.
pub(crate) const PERFORMANCE_NOW_METHOD: &str = "now";

/// The initial value for the `last_time` field, indicating the loop has not started yet.
pub(crate) const UNINITIALIZED_TIME: f64 = -1.0;
