/// The gap (in CSS pixels) to keep between the focused input and the on-screen keyboard.
pub(crate) const KEYBOARD_FOCUS_GAP: f64 = 16.0;

/// The delay (in milliseconds) before adjusting scroll after an input gains focus.
///
/// Gives mobile browsers time to bring up the virtual keyboard and update the
/// visual viewport before we measure element position.
pub(crate) const FOCUS_SCROLL_DELAY_MILLIS: i32 = 200;

/// The extra padding-bottom (in CSS pixels) injected into `<main>` while an input
/// is focused on mobile. This guarantees enough scrollable space so that the
/// virtual keyboard does not obscure the input field.
pub(crate) const KEYBOARD_EXTRA_PADDING: &str = "360px";
