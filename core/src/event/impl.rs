use crate::*;

/// Maps each `NativeEventName` variant to its corresponding DOM event string.
impl NativeEventName {
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
            NativeEventName::EuvSignalUpdate => Cow::Borrowed("__euv_signal_update__"),
            NativeEventName::Other(name) => Cow::Owned(name.clone()),
        }
    }
}

/// Implements `Display` for `NativeEventName` by delegating to `as_str`.
///
/// This automatically provides the `ToString` trait via blanket implementation.
impl std::fmt::Display for NativeEventName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Implementation of event handler construction, cloning, and invocation.
impl NativeEventHandler {
    /// Creates a new event handler from an `NativeEventName` enum and callback.
    pub fn new<F>(event_name: NativeEventName, callback: F) -> Self
    where
        F: FnMut(NativeEvent) + 'static,
    {
        NativeEventHandler {
            event_name: event_name.as_str().into_owned(),
            callback: Rc::new(RefCell::new(callback)),
        }
    }

    /// Invokes the underlying callback with the given event.
    pub fn handle(&self, event: NativeEvent) {
        let mut cb: RefMut<dyn FnMut(NativeEvent)> = self.get_callback().borrow_mut();
        cb(event);
    }
}

/// Clones the event handler, sharing the underlying callback reference.
impl Clone for NativeEventHandler {
    fn clone(&self) -> Self {
        NativeEventHandler {
            event_name: self.get_event_name().clone(),
            callback: Rc::clone(self.get_callback()),
        }
    }
}
