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
    /// - `&mut Formatter<'_>`- The formatter.
    ///
    /// # Returns
    ///
    /// - `std::fmt::Result`- The formatting result.
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
            NativeEventName::PopState => write!(f, "popstate"),
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

/// Implements `FromStr` for `NativeEventName`, enabling parsing from Web-standard
/// lowercase event name strings (e.g., `"click"`, `"dblclick"`, `"mouseenter"`).
///
/// This is the standard library trait for string-to-enum parsing, used by the
/// `html!` macro to convert event attribute keys (e.g., `onclick`) into the
/// corresponding `NativeEventName` variant at runtime.
///
/// # Supported names
///
/// All standard DOM event names in lowercase: `click`, `dblclick`, `mousedown`,
/// `mouseup`, `mousemove`, `mouseenter`, `mouseleave`, `mouseover`, `mouseout`,
/// `contextmenu`, `input`, `keydown`, `keyup`, `keypress`, `focus`, `blur`,
/// `focusin`, `focusout`, `submit`, `change`, `drag`, `dragstart`, `dragend`,
/// `dragover`, `dragenter`, `dragleave`, `drop`, `touchstart`, `touchend`,
/// `touchmove`, `touchcancel`, `wheel`, `copy`, `cut`, `paste`, `play`,
/// `pause`, `ended`, `loadeddata`, `canplay`, `volumechange`, `timeupdate`,
/// `hashchange`, `popstate`, `resize`, `scroll`, `load`, `unload`,
/// `beforeunload`, `error`, `online`, `offline`, `visibilitychange`,
/// `animationstart`, `animationend`, `animationiteration`, `transitionstart`,
/// `transitionend`, `transitionrun`.
///
/// Unknown names fall back to `NativeEventName::Other(name)`.
impl std::str::FromStr for NativeEventName {
    type Err = ParseNativeEventNameError;

    /// Parses a Web-standard event name string into a `NativeEventName`.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name string (e.g., `"click"`, `"mouseenter"`).
    ///
    /// # Returns
    ///
    /// - `Result<NativeEventName, ParseNativeEventNameError>` - The parsed enum variant,
    ///   or an error if the input is empty.
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        match data {
            "click" => Ok(NativeEventName::Click),
            "dblclick" => Ok(NativeEventName::DblClick),
            "mousedown" => Ok(NativeEventName::MouseDown),
            "mouseup" => Ok(NativeEventName::MouseUp),
            "mousemove" => Ok(NativeEventName::MouseMove),
            "mouseenter" => Ok(NativeEventName::MouseEnter),
            "mouseleave" => Ok(NativeEventName::MouseLeave),
            "mouseover" => Ok(NativeEventName::MouseOver),
            "mouseout" => Ok(NativeEventName::MouseOut),
            "contextmenu" => Ok(NativeEventName::ContextMenu),
            "input" => Ok(NativeEventName::Input),
            "keydown" => Ok(NativeEventName::KeyDown),
            "keyup" => Ok(NativeEventName::KeyUp),
            "keypress" => Ok(NativeEventName::KeyPress),
            "focus" => Ok(NativeEventName::Focus),
            "blur" => Ok(NativeEventName::Blur),
            "focusin" => Ok(NativeEventName::FocusIn),
            "focusout" => Ok(NativeEventName::FocusOut),
            "submit" => Ok(NativeEventName::Submit),
            "change" => Ok(NativeEventName::Change),
            "drag" => Ok(NativeEventName::Drag),
            "dragstart" => Ok(NativeEventName::DragStart),
            "dragend" => Ok(NativeEventName::DragEnd),
            "dragover" => Ok(NativeEventName::DragOver),
            "dragenter" => Ok(NativeEventName::DragEnter),
            "dragleave" => Ok(NativeEventName::DragLeave),
            "drop" => Ok(NativeEventName::Drop),
            "touchstart" => Ok(NativeEventName::TouchStart),
            "touchend" => Ok(NativeEventName::TouchEnd),
            "touchmove" => Ok(NativeEventName::TouchMove),
            "touchcancel" => Ok(NativeEventName::TouchCancel),
            "wheel" => Ok(NativeEventName::Wheel),
            "copy" => Ok(NativeEventName::Copy),
            "cut" => Ok(NativeEventName::Cut),
            "paste" => Ok(NativeEventName::Paste),
            "play" => Ok(NativeEventName::Play),
            "pause" => Ok(NativeEventName::Pause),
            "ended" => Ok(NativeEventName::Ended),
            "loadeddata" => Ok(NativeEventName::LoadedData),
            "canplay" => Ok(NativeEventName::CanPlay),
            "volumechange" => Ok(NativeEventName::VolumeChange),
            "timeupdate" => Ok(NativeEventName::TimeUpdate),
            "hashchange" => Ok(NativeEventName::HashChange),
            "popstate" => Ok(NativeEventName::PopState),
            "resize" => Ok(NativeEventName::Resize),
            "scroll" => Ok(NativeEventName::Scroll),
            "load" => Ok(NativeEventName::Load),
            "unload" => Ok(NativeEventName::Unload),
            "beforeunload" => Ok(NativeEventName::BeforeUnload),
            "error" => Ok(NativeEventName::Error),
            "online" => Ok(NativeEventName::Online),
            "offline" => Ok(NativeEventName::Offline),
            "visibilitychange" => Ok(NativeEventName::VisibilityChange),
            "animationstart" => Ok(NativeEventName::AnimationStart),
            "animationend" => Ok(NativeEventName::AnimationEnd),
            "animationiteration" => Ok(NativeEventName::AnimationIteration),
            "transitionstart" => Ok(NativeEventName::TransitionStart),
            "transitionend" => Ok(NativeEventName::TransitionEnd),
            "transitionrun" => Ok(NativeEventName::TransitionRun),
            "" => Err(ParseNativeEventNameError::new(data.to_string())),
            other => Ok(NativeEventName::Other(other.to_string())),
        }
    }
}
