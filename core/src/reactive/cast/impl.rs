use crate::*;

/// Converts a static `String` into a text attribute value.
impl IntoReactiveValue for String {
    /// Converts this string into an `AttributeValue::Text`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A text attribute value containing this string.
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Text(self)
    }
}

/// Converts a string slice into a text attribute value.
impl IntoReactiveValue for &str {
    /// Converts this string slice into an `AttributeValue::Text`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A text attribute value containing the owned string.
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Text(self.to_string())
    }
}

/// Converts a string signal into a reactive attribute value.
impl IntoReactiveValue for Signal<String> {
    /// Converts this string signal into an `AttributeValue::Signal`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A signal-backed attribute value.
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Signal(self)
    }
}

/// Converts a mutable bool signal into a reactive attribute value.
///
/// The signal is mapped to a `Signal<String>` that yields `"true"` or `"false"`,
/// enabling boolean attributes like `checked` to reactively update the DOM.
impl IntoReactiveValue for Signal<bool> {
    /// Converts this bool signal into an `AttributeValue` via string mapping.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A signal-backed attribute value yielding `"true"` or `"false"`.
    fn into_reactive_value(self) -> AttributeValue {
        bool_to_attr(self)
    }
}

/// Converts a `bool` into a dynamic attribute value.
///
/// Stored as `AttributeValue::Dynamic("true"/"false")` so components can
/// extract the original boolean via `try_get_typed_prop`.
impl IntoReactiveValue for bool {
    /// Converts this boolean into an `AttributeValue::Dynamic`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A dynamic attribute value containing the boolean string.
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Dynamic(self.to_string())
    }
}

/// Converts an `i32` into a dynamic attribute value.
///
/// Stored as `AttributeValue::Dynamic` so components can extract the
/// original integer via `try_get_typed_prop`.
impl IntoReactiveValue for i32 {
    /// Converts this integer into an `AttributeValue::Dynamic`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A dynamic attribute value containing the integer string.
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Dynamic(self.to_string())
    }
}

/// Converts an `f64` into a dynamic attribute value.
///
/// Stored as `AttributeValue::Dynamic` so components can extract the
/// original float via `try_get_typed_prop`.
impl IntoReactiveValue for f64 {
    /// Converts this float into an `AttributeValue::Dynamic`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A dynamic attribute value containing the float string.
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Dynamic(self.to_string())
    }
}

/// Converts a CSS class reference into an attribute value.
impl IntoReactiveValue for Css {
    /// Converts this CSS class into an `AttributeValue::Css`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A CSS class attribute value.
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Css(self)
    }
}

/// Converts a reference to a CSS class into an attribute value by cloning.
impl IntoReactiveValue for &'static Css {
    /// Converts this CSS class reference into an `AttributeValue::Css` by cloning.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - A CSS class attribute value.
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Css(self.clone())
    }
}

/// Converts a `String` into its own value for reactive string storage.
impl IntoReactiveString for String {
    /// Returns this string as-is.
    ///
    /// # Returns
    ///
    /// - `String` - The same string.
    fn into_reactive_string(self) -> String {
        self
    }
}

/// Converts a string slice into an owned string for reactive string storage.
impl IntoReactiveString for &str {
    /// Converts this string slice into an owned `String`.
    ///
    /// # Returns
    ///
    /// - `String` - The owned string.
    fn into_reactive_string(self) -> String {
        self.to_string()
    }
}

/// Converts a `Css` into its class name for reactive string storage.
impl IntoReactiveString for Css {
    /// Returns the class name as an owned string.
    ///
    /// # Returns
    ///
    /// - `String` - The class name.
    fn into_reactive_string(self) -> String {
        self.get_name().to_string()
    }
}

/// Converts a reference to a `Css` into its class name for reactive string storage.
impl IntoReactiveString for &'static Css {
    /// Returns the class name as an owned string.
    ///
    /// # Returns
    ///
    /// - `String` - The class name.
    fn into_reactive_string(self) -> String {
        self.get_name().to_string()
    }
}

/// Converts a `bool` into `"true"` or `"false"` for reactive string storage.
impl IntoReactiveString for bool {
    /// Returns `"true"` or `"false"` as a string.
    ///
    /// # Returns
    ///
    /// - `String` - The boolean as a string.
    fn into_reactive_string(self) -> String {
        self.to_string()
    }
}

/// Converts an `i32` into a string for reactive string storage.
impl IntoReactiveString for i32 {
    /// Converts this integer into its string representation.
    ///
    /// # Returns
    ///
    /// - `String` - The integer as a string.
    fn into_reactive_string(self) -> String {
        self.to_string()
    }
}

/// Converts a `u32` into a string for reactive string storage.
impl IntoReactiveString for u32 {
    /// Converts this unsigned integer into its string representation.
    ///
    /// # Returns
    ///
    /// - `String` - The unsigned integer as a string.
    fn into_reactive_string(self) -> String {
        self.to_string()
    }
}

/// Converts a `f64` into a string for reactive string storage.
impl IntoReactiveString for f64 {
    /// Converts this float into its string representation.
    ///
    /// # Returns
    ///
    /// - `String` - The float as a string.
    fn into_reactive_string(self) -> String {
        self.to_string()
    }
}

/// Converts a string signal into a reactive string by resolving its current value.
impl IntoReactiveString for Signal<String> {
    /// Resolves the current value of this signal.
    ///
    /// # Returns
    ///
    /// - `String` - The current value of the signal.
    fn into_reactive_string(self) -> String {
        self.get()
    }
}

/// Converts a bool signal into a reactive string by resolving its current value.
impl IntoReactiveString for Signal<bool> {
    /// Resolves the current value of this bool signal as a string.
    ///
    /// # Returns
    ///
    /// - `String` - The current boolean value as `"true"` or `"false"`.
    fn into_reactive_string(self) -> String {
        self.get().to_string()
    }
}

/// Converts a closure into a callback attribute value.
impl<F> IntoCallbackAttribute for F
where
    F: FnMut(Event) + 'static,
{
    /// Wraps this closure into an `AttributeValue::Event` with a generic "callback" event name.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value wrapping this closure.
    fn into_callback_attribute(self) -> AttributeValue {
        AttributeValue::Event(NativeEventHandler::create(CALLBACK_EVENT_NAME, self))
    }
}

/// Converts an owned event handler into a callback attribute value.
///
/// Re-wraps the handler with a generic "callback" event name so that
/// subsequent `EventAdapter::into_attribute` calls can override it with
/// the correct DOM event type.
impl IntoCallbackAttribute for NativeEventHandler {
    /// Converts this event handler into an `AttributeValue::Event`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value with a generic callback event name.
    fn into_callback_attribute(self) -> AttributeValue {
        AttrValueAdapter::new(self).into_callback_attribute_value()
    }
}

/// Converts an optional event handler into a callback attribute value.
///
/// Re-wraps a `Some` handler with a generic "callback" event name so that
/// subsequent `EventAdapter::into_attribute` calls can override it with
/// the correct DOM event type.
impl IntoCallbackAttribute for Option<NativeEventHandler> {
    /// Converts this optional handler into an `AttributeValue::Event` or `AttributeValue::Text` if `None`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value if `Some`, otherwise an empty text attribute.
    fn into_callback_attribute(self) -> AttributeValue {
        AttrValueAdapter::new(self).into_callback_attribute_value()
    }
}
