use crate::*;

/// Maps each `NativeEventName` variant to its corresponding DOM event string.
impl NativeEventName {
    /// All DOM event names that should be delegated at the window level.
    ///
    /// Excludes `EuvSignalUpdate` (internal framework dispatch) and `Other`
    /// (user-defined events which are registered on-demand). These are
    /// registered once at mount time so that no per-element
    /// `addEventListener` calls are ever needed.
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

    /// Returns the string representation of this event name for DOM binding.
    ///
    /// Static variants return `Cow::Borrowed` (zero allocation), while
    /// `Other` variants return `Cow::Owned` (heap allocation).
    ///
    /// # Returns
    ///
    /// - `Cow<'static, str>` - The event name as a static or owned string.
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            NativeEventName::Click => Cow::Borrowed("click"),
            NativeEventName::DblClick => Cow::Borrowed("dblclick"),
            NativeEventName::MouseDown => Cow::Borrowed("mousedown"),
            NativeEventName::MouseUp => Cow::Borrowed("mouseup"),
            NativeEventName::MouseMove => Cow::Borrowed("mousemove"),
            NativeEventName::MouseEnter => Cow::Borrowed("mouseenter"),
            NativeEventName::MouseLeave => Cow::Borrowed("mouseleave"),
            NativeEventName::MouseOver => Cow::Borrowed("mouseover"),
            NativeEventName::MouseOut => Cow::Borrowed("mouseout"),
            NativeEventName::ContextMenu => Cow::Borrowed("contextmenu"),
            NativeEventName::Input => Cow::Borrowed("input"),
            NativeEventName::KeyDown => Cow::Borrowed("keydown"),
            NativeEventName::KeyUp => Cow::Borrowed("keyup"),
            NativeEventName::KeyPress => Cow::Borrowed("keypress"),
            NativeEventName::Focus => Cow::Borrowed("focus"),
            NativeEventName::Blur => Cow::Borrowed("blur"),
            NativeEventName::FocusIn => Cow::Borrowed("focusin"),
            NativeEventName::FocusOut => Cow::Borrowed("focusout"),
            NativeEventName::Submit => Cow::Borrowed("submit"),
            NativeEventName::Change => Cow::Borrowed("change"),
            NativeEventName::Drag => Cow::Borrowed("drag"),
            NativeEventName::DragStart => Cow::Borrowed("dragstart"),
            NativeEventName::DragEnd => Cow::Borrowed("dragend"),
            NativeEventName::DragOver => Cow::Borrowed("dragover"),
            NativeEventName::DragEnter => Cow::Borrowed("dragenter"),
            NativeEventName::DragLeave => Cow::Borrowed("dragleave"),
            NativeEventName::Drop => Cow::Borrowed("drop"),
            NativeEventName::TouchStart => Cow::Borrowed("touchstart"),
            NativeEventName::TouchEnd => Cow::Borrowed("touchend"),
            NativeEventName::TouchMove => Cow::Borrowed("touchmove"),
            NativeEventName::TouchCancel => Cow::Borrowed("touchcancel"),
            NativeEventName::Wheel => Cow::Borrowed("wheel"),
            NativeEventName::Copy => Cow::Borrowed("copy"),
            NativeEventName::Cut => Cow::Borrowed("cut"),
            NativeEventName::Paste => Cow::Borrowed("paste"),
            NativeEventName::Play => Cow::Borrowed("play"),
            NativeEventName::Pause => Cow::Borrowed("pause"),
            NativeEventName::Ended => Cow::Borrowed("ended"),
            NativeEventName::LoadedData => Cow::Borrowed("loadeddata"),
            NativeEventName::CanPlay => Cow::Borrowed("canplay"),
            NativeEventName::VolumeChange => Cow::Borrowed("volumechange"),
            NativeEventName::TimeUpdate => Cow::Borrowed("timeupdate"),
            NativeEventName::HashChange => Cow::Borrowed("hashchange"),
            NativeEventName::Resize => Cow::Borrowed("resize"),
            NativeEventName::Scroll => Cow::Borrowed("scroll"),
            NativeEventName::Load => Cow::Borrowed("load"),
            NativeEventName::Unload => Cow::Borrowed("unload"),
            NativeEventName::BeforeUnload => Cow::Borrowed("beforeunload"),
            NativeEventName::Error => Cow::Borrowed("error"),
            NativeEventName::Online => Cow::Borrowed("online"),
            NativeEventName::Offline => Cow::Borrowed("offline"),
            NativeEventName::VisibilityChange => Cow::Borrowed("visibilitychange"),
            NativeEventName::AnimationStart => Cow::Borrowed("animationstart"),
            NativeEventName::AnimationEnd => Cow::Borrowed("animationend"),
            NativeEventName::AnimationIteration => Cow::Borrowed("animationiteration"),
            NativeEventName::TransitionStart => Cow::Borrowed("transitionstart"),
            NativeEventName::TransitionEnd => Cow::Borrowed("transitionend"),
            NativeEventName::TransitionRun => Cow::Borrowed("transitionrun"),
            NativeEventName::EuvSignalUpdate => Cow::Borrowed("__euv_signal_update__"),
            NativeEventName::Other(name) => Cow::Owned(name.clone()),
        }
    }
}

/// Implements `Display` for `NativeEventName` by delegating to `as_str`.
impl std::fmt::Display for NativeEventName {
    /// Formats this event name as a string.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>`: The formatter.
    ///
    /// # Returns
    ///
    /// - `std::fmt::Result`: The formatting result.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
