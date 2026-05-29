#[allow(unused_imports)]
use super::*;

/// All DOM event names that should be delegated at the window level.
///
/// These are registered once at mount time so that no per-element
/// `addEventListener` calls are ever needed for standard DOM events.
/// Non-delegated events (hashchange, popstate, resize, etc.) are registered
/// directly via `addEventListener` on-demand when used in components.
pub(crate) const DELEGATABLE_EVENT_NAMES: [&str; 46] = [
    "click",
    "dblclick",
    "mousedown",
    "mouseup",
    "mousemove",
    "mouseenter",
    "mouseleave",
    "mouseover",
    "mouseout",
    "contextmenu",
    "input",
    "keydown",
    "keyup",
    "keypress",
    "focus",
    "blur",
    "focusin",
    "focusout",
    "submit",
    "change",
    "drag",
    "dragstart",
    "dragend",
    "dragover",
    "dragenter",
    "dragleave",
    "drop",
    "touchstart",
    "touchend",
    "touchmove",
    "touchcancel",
    "wheel",
    "copy",
    "cut",
    "paste",
    "play",
    "pause",
    "ended",
    "loadeddata",
    "canplay",
    "volumechange",
    "timeupdate",
    "scroll",
    "animationstart",
    "animationend",
    "transitionend",
];
