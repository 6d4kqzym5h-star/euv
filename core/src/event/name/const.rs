/// The internal event name used by euv for signal update dispatch.
///
/// This custom event name is used to trigger DOM update cycles after signal changes.
/// It is dispatched on the global `window` object and listened for by the signal update registry.
pub(crate) const EUV_SIGNAL_UPDATE: &str = "__euv_signal_update__";

/// The DOM event name for mouse click.
pub(crate) const EVENT_NAME_CLICK: &str = "click";

/// The DOM event name for mouse double-click.
pub(crate) const EVENT_NAME_DBLCLICK: &str = "dblclick";

/// The DOM event name for mouse down.
pub(crate) const EVENT_NAME_MOUSEDOWN: &str = "mousedown";

/// The DOM event name for mouse up.
pub(crate) const EVENT_NAME_MOUSEUP: &str = "mouseup";

/// The DOM event name for mouse move.
pub(crate) const EVENT_NAME_MOUSEMOVE: &str = "mousemove";

/// The DOM event name for mouse enter.
pub(crate) const EVENT_NAME_MOUSEENTER: &str = "mouseenter";

/// The DOM event name for mouse leave.
pub(crate) const EVENT_NAME_MOUSELEAVE: &str = "mouseleave";

/// The DOM event name for mouse over.
pub(crate) const EVENT_NAME_MOUSEOVER: &str = "mouseover";

/// The DOM event name for mouse out.
pub(crate) const EVENT_NAME_MOUSEOUT: &str = "mouseout";

/// The DOM event name for context menu.
pub(crate) const EVENT_NAME_CONTEXTMENU: &str = "contextmenu";

/// The DOM event name for input.
pub(crate) const EVENT_NAME_INPUT: &str = "input";

/// The DOM event name for key down.
pub(crate) const EVENT_NAME_KEYDOWN: &str = "keydown";

/// The DOM event name for key up.
pub(crate) const EVENT_NAME_KEYUP: &str = "keyup";

/// The DOM event name for key press.
pub(crate) const EVENT_NAME_KEYPRESS: &str = "keypress";

/// The DOM event name for focus.
pub(crate) const EVENT_NAME_FOCUS: &str = "focus";

/// The DOM event name for blur.
pub(crate) const EVENT_NAME_BLUR: &str = "blur";

/// The DOM event name for focus in.
pub(crate) const EVENT_NAME_FOCUSIN: &str = "focusin";

/// The DOM event name for focus out.
pub(crate) const EVENT_NAME_FOCUSOUT: &str = "focusout";

/// The DOM event name for form submit.
pub(crate) const EVENT_NAME_SUBMIT: &str = "submit";

/// The DOM event name for change.
pub(crate) const EVENT_NAME_CHANGE: &str = "change";

/// The DOM event name for drag.
pub(crate) const EVENT_NAME_DRAG: &str = "drag";

/// The DOM event name for drag start.
pub(crate) const EVENT_NAME_DRAGSTART: &str = "dragstart";

/// The DOM event name for drag end.
pub(crate) const EVENT_NAME_DRAGEND: &str = "dragend";

/// The DOM event name for drag over.
pub(crate) const EVENT_NAME_DRAGOVER: &str = "dragover";

/// The DOM event name for drag enter.
pub(crate) const EVENT_NAME_DRAGENTER: &str = "dragenter";

/// The DOM event name for drag leave.
pub(crate) const EVENT_NAME_DRAGLEAVE: &str = "dragleave";

/// The DOM event name for drop.
pub(crate) const EVENT_NAME_DROP: &str = "drop";

/// The DOM event name for touch start.
pub(crate) const EVENT_NAME_TOUCHSTART: &str = "touchstart";

/// The DOM event name for touch end.
pub(crate) const EVENT_NAME_TOUCHEND: &str = "touchend";

/// The DOM event name for touch move.
pub(crate) const EVENT_NAME_TOUCHMOVE: &str = "touchmove";

/// The DOM event name for touch cancel.
pub(crate) const EVENT_NAME_TOUCHCANCEL: &str = "touchcancel";

/// The DOM event name for wheel.
pub(crate) const EVENT_NAME_WHEEL: &str = "wheel";

/// The DOM event name for copy.
pub(crate) const EVENT_NAME_COPY: &str = "copy";

/// The DOM event name for cut.
pub(crate) const EVENT_NAME_CUT: &str = "cut";

/// The DOM event name for paste.
pub(crate) const EVENT_NAME_PASTE: &str = "paste";

/// The DOM event name for play.
pub(crate) const EVENT_NAME_PLAY: &str = "play";

/// The DOM event name for pause.
pub(crate) const EVENT_NAME_PAUSE: &str = "pause";

/// The DOM event name for media ended.
pub(crate) const EVENT_NAME_ENDED: &str = "ended";

/// The DOM event name for loaded data.
pub(crate) const EVENT_NAME_LOADEDDATA: &str = "loadeddata";

/// The DOM event name for can play.
pub(crate) const EVENT_NAME_CANPLAY: &str = "canplay";

/// The DOM event name for volume change.
pub(crate) const EVENT_NAME_VOLUMECHANGE: &str = "volumechange";

/// The DOM event name for time update.
pub(crate) const EVENT_NAME_TIMEUPDATE: &str = "timeupdate";

/// The DOM event name for hash change.
pub(crate) const EVENT_NAME_HASHCHANGE: &str = "hashchange";

/// The DOM event name for pop state.
pub(crate) const EVENT_NAME_POPSTATE: &str = "popstate";

/// The DOM event name for resize.
pub(crate) const EVENT_NAME_RESIZE: &str = "resize";

/// The DOM event name for scroll.
pub(crate) const EVENT_NAME_SCROLL: &str = "scroll";

/// The DOM event name for load.
pub(crate) const EVENT_NAME_LOAD: &str = "load";

/// The DOM event name for unload.
pub(crate) const EVENT_NAME_UNLOAD: &str = "unload";

/// The DOM event name for before unload.
pub(crate) const EVENT_NAME_BEFOREUNLOAD: &str = "beforeunload";

/// The DOM event name for error.
pub(crate) const EVENT_NAME_ERROR: &str = "error";

/// The DOM event name for online.
pub(crate) const EVENT_NAME_ONLINE: &str = "online";

/// The DOM event name for offline.
pub(crate) const EVENT_NAME_OFFLINE: &str = "offline";

/// The DOM event name for visibility change.
pub(crate) const EVENT_NAME_VISIBILITYCHANGE: &str = "visibilitychange";

/// The DOM event name for animation start.
pub(crate) const EVENT_NAME_ANIMATIONSTART: &str = "animationstart";

/// The DOM event name for animation end.
pub(crate) const EVENT_NAME_ANIMATIONEND: &str = "animationend";

/// The DOM event name for animation iteration.
pub(crate) const EVENT_NAME_ANIMATIONITERATION: &str = "animationiteration";

/// The DOM event name for transition start.
pub(crate) const EVENT_NAME_TRANSITIONSTART: &str = "transitionstart";

/// The DOM event name for transition end.
pub(crate) const EVENT_NAME_TRANSITIONEND: &str = "transitionend";

/// The DOM event name for transition run.
pub(crate) const EVENT_NAME_TRANSITIONRUN: &str = "transitionrun";

/// All DOM event names that should be delegated at the window level.
///
/// Excludes `EuvSignalUpdate` (internal framework dispatch) and `Other`
/// (user-defined events which are registered on-demand). These are
/// registered once at mount time so that no per-element
/// `addEventListener` calls are ever needed.
pub(crate) const DELEGATABLE_EVENT_NAMES: [&str; 46] = [
    EVENT_NAME_CLICK,
    EVENT_NAME_DBLCLICK,
    EVENT_NAME_MOUSEDOWN,
    EVENT_NAME_MOUSEUP,
    EVENT_NAME_MOUSEMOVE,
    EVENT_NAME_MOUSEENTER,
    EVENT_NAME_MOUSELEAVE,
    EVENT_NAME_MOUSEOVER,
    EVENT_NAME_MOUSEOUT,
    EVENT_NAME_CONTEXTMENU,
    EVENT_NAME_INPUT,
    EVENT_NAME_KEYDOWN,
    EVENT_NAME_KEYUP,
    EVENT_NAME_KEYPRESS,
    EVENT_NAME_FOCUS,
    EVENT_NAME_BLUR,
    EVENT_NAME_FOCUSIN,
    EVENT_NAME_FOCUSOUT,
    EVENT_NAME_SUBMIT,
    EVENT_NAME_CHANGE,
    EVENT_NAME_DRAG,
    EVENT_NAME_DRAGSTART,
    EVENT_NAME_DRAGEND,
    EVENT_NAME_DRAGOVER,
    EVENT_NAME_DRAGENTER,
    EVENT_NAME_DRAGLEAVE,
    EVENT_NAME_DROP,
    EVENT_NAME_TOUCHSTART,
    EVENT_NAME_TOUCHEND,
    EVENT_NAME_TOUCHMOVE,
    EVENT_NAME_TOUCHCANCEL,
    EVENT_NAME_WHEEL,
    EVENT_NAME_COPY,
    EVENT_NAME_CUT,
    EVENT_NAME_PASTE,
    EVENT_NAME_PLAY,
    EVENT_NAME_PAUSE,
    EVENT_NAME_ENDED,
    EVENT_NAME_LOADEDDATA,
    EVENT_NAME_CANPLAY,
    EVENT_NAME_VOLUMECHANGE,
    EVENT_NAME_TIMEUPDATE,
    EVENT_NAME_SCROLL,
    EVENT_NAME_ANIMATIONSTART,
    EVENT_NAME_ANIMATIONEND,
    EVENT_NAME_TRANSITIONEND,
];
