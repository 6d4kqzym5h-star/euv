/// A zero-sized application context struct providing access to core euv framework APIs.
///
/// This struct serves as a namespace for framework functions, allowing them to be
/// accessed in a more object-oriented style. It contains no fields and all methods
/// are effectively static.
///
/// The `App` struct provides unified access to:
/// - Signal management (`use_signal`)
/// - Batched updates (`batch`)
/// - DOM mounting (`mount`)
/// - Scheduled updates (`schedule_update`)
/// - Cleanup registration (`use_cleanup`)
/// - Interval handling (`use_interval`)
/// - Window event handling (`use_window_event`)
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct App;
