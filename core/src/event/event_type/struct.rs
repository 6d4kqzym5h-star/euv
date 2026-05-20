use crate::*;

/// A wrapper around event data that provides event control capabilities.
///
/// Contains the event payload and a `prevent_default` flag that can be set by
/// event handlers to request the browser to prevent the default action.
///
/// Since the event type is already known at registration time (e.g., registering
/// `NativeEventName::Click` guarantees the callback receives mouse event data),
/// use the typed accessor methods to retrieve data directly without pattern matching:
///
/// - `as_mouse()` — for click, dblclick, mousedown, mouseup, mousemove, mouseenter,
///   mouseleave, mouseover, mouseout, contextmenu events
/// - `as_input()` — for input events
/// - `as_keyboard()` — for keydown, keyup, keypress events
/// - `as_focus()` — for focus, blur, focusin, focusout events
/// - `as_submit()` — for submit events
/// - `as_change()` — for change events
/// - `as_drag()` — for drag, dragstart, dragend, dragover, dragenter, dragleave, drop events
/// - `as_touch()` — for touchstart, touchend, touchmove, touchcancel events
/// - `as_wheel()` — for wheel events
/// - `as_clipboard()` — for copy, cut, paste events
/// - `as_media()` — for play, pause, ended, loadeddata, canplay, volumechange, timeupdate events
#[derive(CustomDebug, Data)]
pub struct NativeEvent {
    /// The event data.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) kind: NativeEventKind,
    /// Whether `prevent_default` has been requested.
    #[debug(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) prevent_default_requested: Rc<Cell<bool>>,
}
