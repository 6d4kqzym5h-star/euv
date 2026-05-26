use crate::*;

/// Represents the value of an HTML attribute.
///
/// Attributes can be static text, reactive signals, event handlers, dynamic expressions,
/// or CSS class references.
#[derive(Clone, CustomDebug)]
pub enum AttributeValue {
    /// A static string value.
    Text(String),
    /// A dynamic signal-backed value.
    #[debug(skip)]
    Signal(Signal<String>),
    /// An event handler callback.
    #[debug(skip)]
    Event(NativeEventHandler),
    /// A dynamic expression value of any type (for component props).
    Dynamic(String),
    /// A CSS class reference created by the `class!` macro.
    Css(Css),
}
