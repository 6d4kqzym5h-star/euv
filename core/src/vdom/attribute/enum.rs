use crate::*;

/// Represents the value of an HTML attribute.
///
/// Attributes can be static text, reactive signals, event handlers, dynamic expressions,
/// or CSS class references.
#[derive(Clone, Debug)]
pub enum AttributeValue {
    /// A static string value.
    Text(String),
    /// A dynamic signal-backed value.
    Signal(Signal<String>),
    /// An event handler callback.
    Event(NativeEventHandler),
    /// A dynamic expression value of any type (for component props).
    Dynamic(String),
    /// A CSS class reference created by the `class!` macro.
    Css(CssClass),
}
