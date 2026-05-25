use crate::*;

/// Implements `Display` for `NativeEventName` to provide string representation.
///
/// This also provides `ToString::to_string()` via the standard blanket implementation,
/// which is the preferred way to obtain the event name as a `String`.
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
        match self {
            NativeEventName::Click => write!(f, "click"),
            NativeEventName::DblClick => write!(f, "dblclick"),
            NativeEventName::MouseDown => write!(f, "mousedown"),
            NativeEventName::MouseUp => write!(f, "mouseup"),
            NativeEventName::MouseMove => write!(f, "mousemove"),
            NativeEventName::MouseEnter => write!(f, "mouseenter"),
            NativeEventName::MouseLeave => write!(f, "mouseleave"),
            NativeEventName::MouseOver => write!(f, "mouseover"),
            NativeEventName::MouseOut => write!(f, "mouseout"),
            NativeEventName::ContextMenu => write!(f, "contextmenu"),
            NativeEventName::Input => write!(f, "input"),
            NativeEventName::KeyDown => write!(f, "keydown"),
            NativeEventName::KeyUp => write!(f, "keyup"),
            NativeEventName::KeyPress => write!(f, "keypress"),
            NativeEventName::Focus => write!(f, "focus"),
            NativeEventName::Blur => write!(f, "blur"),
            NativeEventName::FocusIn => write!(f, "focusin"),
            NativeEventName::FocusOut => write!(f, "focusout"),
            NativeEventName::Submit => write!(f, "submit"),
            NativeEventName::Change => write!(f, "change"),
            NativeEventName::Drag => write!(f, "drag"),
            NativeEventName::DragStart => write!(f, "dragstart"),
            NativeEventName::DragEnd => write!(f, "dragend"),
            NativeEventName::DragOver => write!(f, "dragover"),
            NativeEventName::DragEnter => write!(f, "dragenter"),
            NativeEventName::DragLeave => write!(f, "dragleave"),
            NativeEventName::Drop => write!(f, "drop"),
            NativeEventName::TouchStart => write!(f, "touchstart"),
            NativeEventName::TouchEnd => write!(f, "touchend"),
            NativeEventName::TouchMove => write!(f, "touchmove"),
            NativeEventName::TouchCancel => write!(f, "touchcancel"),
            NativeEventName::Wheel => write!(f, "wheel"),
            NativeEventName::Copy => write!(f, "copy"),
            NativeEventName::Cut => write!(f, "cut"),
            NativeEventName::Paste => write!(f, "paste"),
            NativeEventName::Play => write!(f, "play"),
            NativeEventName::Pause => write!(f, "pause"),
            NativeEventName::Ended => write!(f, "ended"),
            NativeEventName::LoadedData => write!(f, "loadeddata"),
            NativeEventName::CanPlay => write!(f, "canplay"),
            NativeEventName::VolumeChange => write!(f, "volumechange"),
            NativeEventName::TimeUpdate => write!(f, "timeupdate"),
            NativeEventName::HashChange => write!(f, "hashchange"),
            NativeEventName::Resize => write!(f, "resize"),
            NativeEventName::Scroll => write!(f, "scroll"),
            NativeEventName::Load => write!(f, "load"),
            NativeEventName::Unload => write!(f, "unload"),
            NativeEventName::BeforeUnload => write!(f, "beforeunload"),
            NativeEventName::Error => write!(f, "error"),
            NativeEventName::Online => write!(f, "online"),
            NativeEventName::Offline => write!(f, "offline"),
            NativeEventName::VisibilityChange => write!(f, "visibilitychange"),
            NativeEventName::AnimationStart => write!(f, "animationstart"),
            NativeEventName::AnimationEnd => write!(f, "animationend"),
            NativeEventName::AnimationIteration => write!(f, "animationiteration"),
            NativeEventName::TransitionStart => write!(f, "transitionstart"),
            NativeEventName::TransitionEnd => write!(f, "transitionend"),
            NativeEventName::TransitionRun => write!(f, "transitionrun"),
            NativeEventName::EuvSignalUpdate => write!(f, "__euv_signal_update__"),
            NativeEventName::Other(name) => write!(f, "{}", name),
        }
    }
}
