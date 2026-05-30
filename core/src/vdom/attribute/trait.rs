use crate::*;

/// Trait for types that can be converted into a reactive string value.
///
/// This allows both static strings and signals to be used interchangeably
/// in HTML attributes.
pub trait IntoReactiveValue {
    /// Converts this value into an `AttributeValue`, wrapping signals
    /// for reactive updates or converting static values to text.
    fn into_reactive_value(self) -> AttributeValue;
}

/// Trait for converting a value into a string for reactive attribute updates.
///
/// Used by the `html!` macro when an attribute value contains an `if` condition.
/// The result is stored in a `Signal<String>` that re-evaluates whenever any
/// signal changes — mirroring the DOM-level `DynamicNode` mechanism.
///
/// Implementations are provided for `String`, `&str`, `Css`, `bool`,
/// numeric types, and `Signal<T>` (which resolves the signal first).
pub trait IntoReactiveString {
    /// Converts this value into its string representation for attribute storage.
    fn into_reactive_string(self) -> String;
}

/// Trait for types that can be converted into a callback attribute value.
///
/// This allows closures to be passed as custom component props.
pub(crate) trait IntoCallbackAttribute {
    /// Converts this value into an `AttributeValue`, wrapping the callback
    /// for use as a component prop.
    fn into_callback_attribute(self) -> AttributeValue;
}
