/// The DOM attribute name used to store the unique euv identifier on an element.
///
/// This attribute is set on every element that registers an event listener
/// so the framework can look up the element's identity across re-renders.
pub(crate) const DATA_EUV_ID: &str = "data-euv-id";

/// Event names that do not bubble up to `window`.
///
/// These events must be attached directly on the target element
/// instead of using global event delegation on `window`.
///
/// Sources:
/// - W3C DOM Level 3 Events: `abort`, `blur`, `error`, `focus`, `load`, `resize`, `unload`
/// - Mouse: `mouseenter`, `mouseleave`
/// - Media (all non-bubbling): `loadstart`, `progress`, `loadend`, `emptied`, `stalled`,
///   `suspend`, `canplay`, `canplaythrough`, `loadedmetadata`, `waiting`, `playing`,
///   `pause`, `seeking`, `seeked`, `timeupdate`, `volumechange`, `durationchange`,
///   `ratechange`, `ended`
/// - UI: `beforeunload`, `scroll`, `resize`, `select`
/// - CSS: `transitionend`, `animationend`, `animationiteration`, `animationstart`
pub(crate) const NON_BUBBLING_EVENTS: [&str; 35] = [
    "abort",
    "animationend",
    "animationiteration",
    "animationstart",
    "beforeunload",
    "blur",
    "canplay",
    "canplaythrough",
    "durationchange",
    "emptied",
    "ended",
    "error",
    "focus",
    "mouseleave",
    "mouseenter",
    "load",
    "loadedmetadata",
    "loadend",
    "loadstart",
    "pause",
    "playing",
    "progress",
    "ratechange",
    "resize",
    "scroll",
    "seeked",
    "seeking",
    "select",
    "stalled",
    "suspend",
    "timeupdate",
    "transitionend",
    "unload",
    "volumechange",
    "waiting",
];
