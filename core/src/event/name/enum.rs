/// Represents the name of a DOM event.
///
/// Covers mouse, keyboard, focus, form, drag, touch, wheel, clipboard, and media events.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NativeEventName {
    /// Mouse click event.
    Click,
    /// Mouse double-click event.
    DblClick,
    /// Mouse down event.
    MouseDown,
    /// Mouse up event.
    MouseUp,
    /// Mouse move event.
    MouseMove,
    /// Mouse enter event.
    MouseEnter,
    /// Mouse leave event.
    MouseLeave,
    /// Mouse over event.
    MouseOver,
    /// Mouse out event.
    MouseOut,
    /// Context menu event.
    ContextMenu,
    /// Input value change event.
    Input,
    /// Key down event.
    KeyDown,
    /// Key up event.
    KeyUp,
    /// Key press event.
    KeyPress,
    /// Focus event.
    Focus,
    /// Blur event.
    Blur,
    /// Focus in event.
    FocusIn,
    /// Focus out event.
    FocusOut,
    /// Form submit event.
    Submit,
    /// Change event for select/checkbox/radio.
    Change,
    /// Drag event.
    Drag,
    /// Drag start event.
    DragStart,
    /// Drag end event.
    DragEnd,
    /// Drag over event.
    DragOver,
    /// Drag enter event.
    DragEnter,
    /// Drag leave event.
    DragLeave,
    /// Drop event.
    Drop,
    /// Touch start event.
    TouchStart,
    /// Touch end event.
    TouchEnd,
    /// Touch move event.
    TouchMove,
    /// Touch cancel event.
    TouchCancel,
    /// Wheel/scroll event.
    Wheel,
    /// Copy event.
    Copy,
    /// Cut event.
    Cut,
    /// Paste event.
    Paste,
    /// Play media event.
    Play,
    /// Pause media event.
    Pause,
    /// Media ended event.
    Ended,
    /// Media loaded data event.
    LoadedData,
    /// Media can play event.
    CanPlay,
    /// Volume change event.
    VolumeChange,
    /// Time update event.
    TimeUpdate,
    /// Hash change event.
    HashChange,
    /// Pop state event.
    PopState,
    /// Window resize event.
    Resize,
    /// Scroll event.
    Scroll,
    /// Window load event.
    Load,
    /// Window unload event.
    Unload,
    /// Window before unload event.
    BeforeUnload,
    /// Window error event.
    Error,
    /// Window online event.
    Online,
    /// Window offline event.
    Offline,
    /// Visibility change event.
    VisibilityChange,
    /// Animation start event.
    AnimationStart,
    /// Animation end event.
    AnimationEnd,
    /// Animation iteration event.
    AnimationIteration,
    /// Transition start event.
    TransitionStart,
    /// Transition end event.
    TransitionEnd,
    /// Transition run event.
    TransitionRun,
    /// Internal euv signal update event.
    EuvSignalUpdate,
    /// A custom event with an arbitrary name.
    Other(String),
}
