use crate::*;

/// Trait for types that can be converted into an event attribute value.
///
/// Allows event handlers and optional handlers to be used as attribute values.
pub trait IntoEventAttribute {
    /// Converts this value into an `AttributeValue`, or an empty text if None.
    fn into_event_attribute(self) -> AttributeValue;
}
