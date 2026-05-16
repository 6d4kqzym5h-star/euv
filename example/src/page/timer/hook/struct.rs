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
