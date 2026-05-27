/// The prefix for internal watch signal identifier names.
pub(crate) const WATCH_SIGNAL_PREFIX: &str = "__euv_watch_signal_";

/// Error message when signal count does not match closure parameter count.
pub(crate) const ERR_SIGNAL_PARAM_MISMATCH: &str =
    "the number of signal expressions must match the number of closure parameters";
