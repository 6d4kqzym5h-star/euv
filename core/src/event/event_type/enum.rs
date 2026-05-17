use crate::*;

/// Represents the name of a DOM event.
///
/// Covers mouse, keyboard, focus, form, drag, touch, wheel, clipboard, and media events.
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
    /// Internal euv signal update event.
    EuvSignalUpdate,
    /// A custom event with an arbitrary name.
    Other(String),
}

/// Represents different kinds of UI events.
///
/// Each variant wraps the specific data associated with that event type.
#[derive(Clone, Debug, PartialEq)]
pub enum NativeEvent {
    /// A mouse-related event.
    Mouse(NativeMouseEvent),
    /// An input change event.
    Input(NativeInputEvent),
    /// A keyboard event.
    Keyboard(NativeKeyboardEvent),
    /// A focus event.
    Focus(NativeFocusEvent),
    /// A form submit event.
    Submit(NativeSubmitEvent),
    /// A change event for select/checkbox/radio.
    Change(NativeChangeEvent),
    /// A drag and drop event.
    Drag(NativeDragEvent),
    /// A touch event.
    Touch(NativeTouchEvent),
    /// A wheel/scroll event.
    Wheel(NativeWheelEvent),
    /// A clipboard event.
    Clipboard(NativeClipboardEvent),
    /// A media event.
    Media(NativeMediaEvent),
    /// A generic event with no specific data.
    Generic,
}
