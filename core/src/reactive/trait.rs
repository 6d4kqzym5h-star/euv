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

/// Trait for types that can be converted into a callback attribute value.
///
/// This allows closures to be passed as custom component props.
pub trait IntoCallbackAttribute {
    /// Converts this value into an `AttributeValue`, wrapping the callback
    /// for use as a component prop.
    fn into_callback_attribute(self) -> AttributeValue;
}
