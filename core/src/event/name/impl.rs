use crate::*;

/// Implements `Display` for `NativeEventName` to provide string representation.
///
/// This also provides `ToString::to_string()` via the standard blanket implementation,
/// which is the preferred way to obtain the event name as a `String`.
impl Display for NativeEventName {
    /// Formats this event name as a string.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>`- The formatter.
    ///
    /// # Returns
    ///
    /// - `fmt::Result`- The formatting result.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NativeEventName::Click => write!(formatter, "{EVENT_NAME_CLICK}"),
            NativeEventName::DblClick => write!(formatter, "{EVENT_NAME_DBLCLICK}"),
            NativeEventName::MouseDown => write!(formatter, "{EVENT_NAME_MOUSEDOWN}"),
            NativeEventName::MouseUp => write!(formatter, "{EVENT_NAME_MOUSEUP}"),
            NativeEventName::MouseMove => write!(formatter, "{EVENT_NAME_MOUSEMOVE}"),
            NativeEventName::MouseEnter => write!(formatter, "{EVENT_NAME_MOUSEENTER}"),
            NativeEventName::MouseLeave => write!(formatter, "{EVENT_NAME_MOUSELEAVE}"),
            NativeEventName::MouseOver => write!(formatter, "{EVENT_NAME_MOUSEOVER}"),
            NativeEventName::MouseOut => write!(formatter, "{EVENT_NAME_MOUSEOUT}"),
            NativeEventName::ContextMenu => write!(formatter, "{EVENT_NAME_CONTEXTMENU}"),
            NativeEventName::Input => write!(formatter, "{EVENT_NAME_INPUT}"),
            NativeEventName::KeyDown => write!(formatter, "{EVENT_NAME_KEYDOWN}"),
            NativeEventName::KeyUp => write!(formatter, "{EVENT_NAME_KEYUP}"),
            NativeEventName::KeyPress => write!(formatter, "{EVENT_NAME_KEYPRESS}"),
            NativeEventName::Focus => write!(formatter, "{EVENT_NAME_FOCUS}"),
            NativeEventName::Blur => write!(formatter, "{EVENT_NAME_BLUR}"),
            NativeEventName::FocusIn => write!(formatter, "{EVENT_NAME_FOCUSIN}"),
            NativeEventName::FocusOut => write!(formatter, "{EVENT_NAME_FOCUSOUT}"),
            NativeEventName::Submit => write!(formatter, "{EVENT_NAME_SUBMIT}"),
            NativeEventName::Change => write!(formatter, "{EVENT_NAME_CHANGE}"),
            NativeEventName::Drag => write!(formatter, "{EVENT_NAME_DRAG}"),
            NativeEventName::DragStart => write!(formatter, "{EVENT_NAME_DRAGSTART}"),
            NativeEventName::DragEnd => write!(formatter, "{EVENT_NAME_DRAGEND}"),
            NativeEventName::DragOver => write!(formatter, "{EVENT_NAME_DRAGOVER}"),
            NativeEventName::DragEnter => write!(formatter, "{EVENT_NAME_DRAGENTER}"),
            NativeEventName::DragLeave => write!(formatter, "{EVENT_NAME_DRAGLEAVE}"),
            NativeEventName::Drop => write!(formatter, "{EVENT_NAME_DROP}"),
            NativeEventName::TouchStart => write!(formatter, "{EVENT_NAME_TOUCHSTART}"),
            NativeEventName::TouchEnd => write!(formatter, "{EVENT_NAME_TOUCHEND}"),
            NativeEventName::TouchMove => write!(formatter, "{EVENT_NAME_TOUCHMOVE}"),
            NativeEventName::TouchCancel => write!(formatter, "{EVENT_NAME_TOUCHCANCEL}"),
            NativeEventName::Wheel => write!(formatter, "{EVENT_NAME_WHEEL}"),
            NativeEventName::Copy => write!(formatter, "{EVENT_NAME_COPY}"),
            NativeEventName::Cut => write!(formatter, "{EVENT_NAME_CUT}"),
            NativeEventName::Paste => write!(formatter, "{EVENT_NAME_PASTE}"),
            NativeEventName::Play => write!(formatter, "{EVENT_NAME_PLAY}"),
            NativeEventName::Pause => write!(formatter, "{EVENT_NAME_PAUSE}"),
            NativeEventName::Ended => write!(formatter, "{EVENT_NAME_ENDED}"),
            NativeEventName::LoadedData => write!(formatter, "{EVENT_NAME_LOADEDDATA}"),
            NativeEventName::CanPlay => write!(formatter, "{EVENT_NAME_CANPLAY}"),
            NativeEventName::VolumeChange => write!(formatter, "{EVENT_NAME_VOLUMECHANGE}"),
            NativeEventName::TimeUpdate => write!(formatter, "{EVENT_NAME_TIMEUPDATE}"),
            NativeEventName::HashChange => write!(formatter, "{EVENT_NAME_HASHCHANGE}"),
            NativeEventName::PopState => write!(formatter, "{EVENT_NAME_POPSTATE}"),
            NativeEventName::Resize => write!(formatter, "{EVENT_NAME_RESIZE}"),
            NativeEventName::Scroll => write!(formatter, "{EVENT_NAME_SCROLL}"),
            NativeEventName::Load => write!(formatter, "{EVENT_NAME_LOAD}"),
            NativeEventName::Unload => write!(formatter, "{EVENT_NAME_UNLOAD}"),
            NativeEventName::BeforeUnload => write!(formatter, "{EVENT_NAME_BEFOREUNLOAD}"),
            NativeEventName::Error => write!(formatter, "{EVENT_NAME_ERROR}"),
            NativeEventName::Online => write!(formatter, "{EVENT_NAME_ONLINE}"),
            NativeEventName::Offline => write!(formatter, "{EVENT_NAME_OFFLINE}"),
            NativeEventName::VisibilityChange => write!(formatter, "{EVENT_NAME_VISIBILITYCHANGE}"),
            NativeEventName::AnimationStart => write!(formatter, "{EVENT_NAME_ANIMATIONSTART}"),
            NativeEventName::AnimationEnd => write!(formatter, "{EVENT_NAME_ANIMATIONEND}"),
            NativeEventName::AnimationIteration => {
                write!(formatter, "{EVENT_NAME_ANIMATIONITERATION}")
            }
            NativeEventName::TransitionStart => write!(formatter, "{EVENT_NAME_TRANSITIONSTART}"),
            NativeEventName::TransitionEnd => write!(formatter, "{EVENT_NAME_TRANSITIONEND}"),
            NativeEventName::TransitionRun => write!(formatter, "{EVENT_NAME_TRANSITIONRUN}"),
            NativeEventName::EuvSignalUpdate => write!(formatter, "{EUV_SIGNAL_UPDATE}"),
            NativeEventName::Other(name) => write!(formatter, "{name}"),
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
impl FromStr for NativeEventName {
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
            EVENT_NAME_CLICK => Ok(NativeEventName::Click),
            EVENT_NAME_DBLCLICK => Ok(NativeEventName::DblClick),
            EVENT_NAME_MOUSEDOWN => Ok(NativeEventName::MouseDown),
            EVENT_NAME_MOUSEUP => Ok(NativeEventName::MouseUp),
            EVENT_NAME_MOUSEMOVE => Ok(NativeEventName::MouseMove),
            EVENT_NAME_MOUSEENTER => Ok(NativeEventName::MouseEnter),
            EVENT_NAME_MOUSELEAVE => Ok(NativeEventName::MouseLeave),
            EVENT_NAME_MOUSEOVER => Ok(NativeEventName::MouseOver),
            EVENT_NAME_MOUSEOUT => Ok(NativeEventName::MouseOut),
            EVENT_NAME_CONTEXTMENU => Ok(NativeEventName::ContextMenu),
            EVENT_NAME_INPUT => Ok(NativeEventName::Input),
            EVENT_NAME_KEYDOWN => Ok(NativeEventName::KeyDown),
            EVENT_NAME_KEYUP => Ok(NativeEventName::KeyUp),
            EVENT_NAME_KEYPRESS => Ok(NativeEventName::KeyPress),
            EVENT_NAME_FOCUS => Ok(NativeEventName::Focus),
            EVENT_NAME_BLUR => Ok(NativeEventName::Blur),
            EVENT_NAME_FOCUSIN => Ok(NativeEventName::FocusIn),
            EVENT_NAME_FOCUSOUT => Ok(NativeEventName::FocusOut),
            EVENT_NAME_SUBMIT => Ok(NativeEventName::Submit),
            EVENT_NAME_CHANGE => Ok(NativeEventName::Change),
            EVENT_NAME_DRAG => Ok(NativeEventName::Drag),
            EVENT_NAME_DRAGSTART => Ok(NativeEventName::DragStart),
            EVENT_NAME_DRAGEND => Ok(NativeEventName::DragEnd),
            EVENT_NAME_DRAGOVER => Ok(NativeEventName::DragOver),
            EVENT_NAME_DRAGENTER => Ok(NativeEventName::DragEnter),
            EVENT_NAME_DRAGLEAVE => Ok(NativeEventName::DragLeave),
            EVENT_NAME_DROP => Ok(NativeEventName::Drop),
            EVENT_NAME_TOUCHSTART => Ok(NativeEventName::TouchStart),
            EVENT_NAME_TOUCHEND => Ok(NativeEventName::TouchEnd),
            EVENT_NAME_TOUCHMOVE => Ok(NativeEventName::TouchMove),
            EVENT_NAME_TOUCHCANCEL => Ok(NativeEventName::TouchCancel),
            EVENT_NAME_WHEEL => Ok(NativeEventName::Wheel),
            EVENT_NAME_COPY => Ok(NativeEventName::Copy),
            EVENT_NAME_CUT => Ok(NativeEventName::Cut),
            EVENT_NAME_PASTE => Ok(NativeEventName::Paste),
            EVENT_NAME_PLAY => Ok(NativeEventName::Play),
            EVENT_NAME_PAUSE => Ok(NativeEventName::Pause),
            EVENT_NAME_ENDED => Ok(NativeEventName::Ended),
            EVENT_NAME_LOADEDDATA => Ok(NativeEventName::LoadedData),
            EVENT_NAME_CANPLAY => Ok(NativeEventName::CanPlay),
            EVENT_NAME_VOLUMECHANGE => Ok(NativeEventName::VolumeChange),
            EVENT_NAME_TIMEUPDATE => Ok(NativeEventName::TimeUpdate),
            EVENT_NAME_HASHCHANGE => Ok(NativeEventName::HashChange),
            EVENT_NAME_POPSTATE => Ok(NativeEventName::PopState),
            EVENT_NAME_RESIZE => Ok(NativeEventName::Resize),
            EVENT_NAME_SCROLL => Ok(NativeEventName::Scroll),
            EVENT_NAME_LOAD => Ok(NativeEventName::Load),
            EVENT_NAME_UNLOAD => Ok(NativeEventName::Unload),
            EVENT_NAME_BEFOREUNLOAD => Ok(NativeEventName::BeforeUnload),
            EVENT_NAME_ERROR => Ok(NativeEventName::Error),
            EVENT_NAME_ONLINE => Ok(NativeEventName::Online),
            EVENT_NAME_OFFLINE => Ok(NativeEventName::Offline),
            EVENT_NAME_VISIBILITYCHANGE => Ok(NativeEventName::VisibilityChange),
            EVENT_NAME_ANIMATIONSTART => Ok(NativeEventName::AnimationStart),
            EVENT_NAME_ANIMATIONEND => Ok(NativeEventName::AnimationEnd),
            EVENT_NAME_ANIMATIONITERATION => Ok(NativeEventName::AnimationIteration),
            EVENT_NAME_TRANSITIONSTART => Ok(NativeEventName::TransitionStart),
            EVENT_NAME_TRANSITIONEND => Ok(NativeEventName::TransitionEnd),
            EVENT_NAME_TRANSITIONRUN => Ok(NativeEventName::TransitionRun),
            "" => Err(ParseNativeEventNameError::new(data.to_string())),
            other => Ok(NativeEventName::Other(other.to_string())),
        }
    }
}
