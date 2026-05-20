/// The currently active RenderEffect being tracked, if any.
/// When `Signal::get()` is called and this is `Some`, the signal
/// registers itself as a dependency of the active effect.
pub(crate) static mut CURRENT_EFFECT: Option<usize> = None;

/// Global map from signal address to dependent effect addresses.
///
/// Each entry maps a signal's inner pointer address to a list of
/// `RenderEffect` inner pointer addresses that depend on that signal.
/// When the signal changes, `notify_effect_subscribers` iterates the
/// list and calls `effect.run_once()` for each dependent effect.
pub(crate) static mut EFFECT_SUBSCRIBERS: Option<usize> = None;

/// Queue of effect addresses pending execution.
///
/// When `Signal::set()` triggers `notify_effect_subscribers`, effect
/// addresses are added here instead of being executed synchronously.
/// A single `queueMicrotask` callback drains the queue at the end of
/// the current microtask, batching all pending effects together.
pub(crate) static mut PENDING_EFFECTS: Option<usize> = None;

/// Whether a microtask to drain `PENDING_EFFECTS` has already been scheduled.
pub(crate) static mut EFFECT_FLUSH_SCHEDULED: bool = false;

/// Whether RenderEffect notifications are temporarily paused.
///
/// When `true`, `notify_effect_subscribers` collects signal addresses into
/// `PAUSED_EFFECT_SIGNALS` but does not schedule effect execution. When
/// unpaused, all collected signals are flushed at once. This prevents
/// event handlers from triggering full-component re-renders via
/// RenderEffect, while still allowing fine-grained `replace_subscribe`
/// listeners to update individual DOM nodes.
pub(crate) static mut EFFECT_NOTIFICATIONS_PAUSED: bool = false;

/// Signal addresses whose effect notifications were suppressed while
/// `EFFECT_NOTIFICATIONS_PAUSED` was `true`. When notifications are
/// resumed, these signals' dependent effects are scheduled for execution.
pub(crate) static mut PAUSED_EFFECT_SIGNALS: Option<usize> = None;

/// Cached JS closure wrapping `flush_pending_effects`. Created and forgotten
/// exactly once to avoid repeated `Closure::forget()` memory leaks.
#[cfg(target_arch = "wasm32")]
pub(crate) static mut FLUSH_CLOSURE_FN: Option<usize> = None;
