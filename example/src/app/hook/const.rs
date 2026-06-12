/// The window event fired when any element within the document gains focus.
///
/// Unlike `focus`, the `focusin` event bubbles, so a single listener on
/// `window` is sufficient to observe focus changes on every input element.
pub(crate) const FOCUS_IN_EVENT: &str = "focusin";

/// The `resize` event used as a fallback signal for the on-screen keyboard
/// appearing or disappearing on platforms (such as Android) that resize the
/// layout viewport instead of only the visual viewport.
pub(crate) const KEYBOARD_RESIZE_EVENT: &str = "resize";

/// The `resize` event emitted by `window.visualViewport` whenever the visual
/// viewport size changes, which is the most reliable signal for the on-screen
/// keyboard showing or hiding on iOS Safari.
pub(crate) const VISUAL_VIEWPORT_RESIZE_EVENT: &str = "resize";

/// The `scroll` event emitted by `window.visualViewport` whenever the visual
/// viewport scrolls.
///
/// On mobile devices the on-screen keyboard may close without blurring the
/// focused element. If the user then scrolls the page and re-taps the field,
/// the keyboard reappears but the field may no longer be within the visible
/// viewport. Listening for this event ensures the focused field is
/// repositioned into the visible area after every viewport scroll.
pub(crate) const VISUAL_VIEWPORT_SCROLL_EVENT: &str = "scroll";

/// CSS selector matching every focusable text-entry element whose visibility
/// must be guaranteed when the on-screen keyboard appears.
///
/// Covers single-line inputs (excluding non-text controls such as checkboxes,
/// radios, buttons, and hidden fields), multi-line textareas, native selects,
/// and rich `contenteditable` regions.
pub(crate) const EDITABLE_ELEMENT_SELECTOR: &str = "input:not([type=checkbox]):not([type=radio]):not([type=button]):not([type=submit]):not([type=reset]):not([type=file]):not([type=hidden]), textarea, select, [contenteditable=true], [contenteditable='']";

/// The delay in milliseconds before scrolling a focused field into view.
///
/// Gives the browser enough time to animate the on-screen keyboard in and to
/// settle the visual viewport so the occlusion math reads the final geometry
/// rather than an intermediate frame.
pub(crate) const KEYBOARD_SCROLL_DELAY_MILLIS: i32 = 300;

/// The minimum visual-viewport height shrink (in CSS pixels) that is treated as
/// the on-screen keyboard being open, filtering out minor browser-chrome
/// fluctuations such as collapsing address bars.
pub(crate) const KEYBOARD_OPEN_THRESHOLD_PX: f64 = 120.0;

/// The vertical breathing room (in CSS pixels) kept between a focused field and
/// the top edge of the keyboard (or viewport) after scrolling.
pub(crate) const KEYBOARD_VISIBLE_MARGIN_PX: f64 = 16.0;
