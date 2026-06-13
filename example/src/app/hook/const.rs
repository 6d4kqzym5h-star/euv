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
/// the top edge of the viewport (or any sticky header) after scrolling.
pub(crate) const KEYBOARD_VISIBLE_MARGIN_PX: f64 = 16.0;

/// The vertical gap (in CSS pixels) kept between the bottom of a focused field
/// and the top edge of the on-screen keyboard (or the reserved keyboard region
/// when the keyboard height cannot be measured).
///
/// This is intentionally larger than `KEYBOARD_VISIBLE_MARGIN_PX` so the field
/// is lifted comfortably above the keyboard rather than hugging its top edge,
/// leaving room for the caret, any inline validation message, and a more
/// balanced composition.
pub(crate) const KEYBOARD_GAP_PX: f64 = 48.0;

/// The fraction of the layout viewport height that is reserved as an estimated
/// on-screen keyboard region when the real keyboard height cannot be measured.
///
/// On many mobile browsers the `visualViewport` does not shrink (or has not
/// finished animating) at the moment a field gains focus, so the keyboard
/// appears "closed" even though it is about to cover the lower portion of the
/// screen. Without a reserve, a field pinned to the bottom of the page is
/// considered fully visible and never scrolled, leaving it hidden behind the
/// keyboard. Reserving roughly the lower third of the viewport pushes such
/// fields up into a region that stays visible once the keyboard appears.
pub(crate) const KEYBOARD_ESTIMATED_FRACTION: f64 = 0.36;

/// The data attribute used to record how much temporary `padding-bottom` (in
/// CSS pixels) has been reserved on a scroll container to make a bottom-pinned
/// focused field scrollable above the keyboard.
///
/// The attribute lets the `focusout` handler precisely subtract the reserved
/// amount and restore the container's original padding once editing ends,
/// without having to remember per-container state in Rust.
pub(crate) const KEYBOARD_RESERVED_PADDING_ATTR: &str = "data-euv-kb-pad";

/// The window event fired when an element within the document loses focus.
///
/// Like `focusin`, the `focusout` event bubbles, so a single window-level
/// listener observes blur on every editable element. It is used to release the
/// temporary bottom padding reserved while a field was focused.
pub(crate) const FOCUS_OUT_EVENT: &str = "focusout";
