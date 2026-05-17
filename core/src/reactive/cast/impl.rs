use crate::*;

/// Converts a static `String` into a text attribute value.
impl IntoReactiveValue for String {
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Text(self)
    }
}

/// Converts a string slice into a text attribute value.
impl IntoReactiveValue for &str {
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Text(self.to_string())
    }
}

/// Converts a string signal into a reactive attribute value.
impl IntoReactiveValue for Signal<String> {
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Signal(self)
    }
}

/// Converts a mutable bool signal into a reactive attribute value.
///
/// The signal is mapped to a `Signal<String>` that yields `"true"` or `"false"`,
/// enabling boolean attributes like `checked` to reactively update the DOM.
impl IntoReactiveValue for Signal<bool> {
    fn into_reactive_value(self) -> AttributeValue {
        bool_signal_to_string_attribute_value(self)
    }
}

/// Converts a CSS class reference into an attribute value.
impl IntoReactiveValue for CssClass {
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Css(self)
    }
}

/// Converts a reference to a CSS class into an attribute value by cloning.
impl IntoReactiveValue for &'static CssClass {
    fn into_reactive_value(self) -> AttributeValue {
        AttributeValue::Css(self.clone())
    }
}

/// Converts a `String` into its own value for reactive string storage.
impl IntoReactiveString for String {
    fn into_reactive_string(self) -> String {
        self
    }
}

/// Converts a string slice into an owned string for reactive string storage.
impl IntoReactiveString for &str {
    fn into_reactive_string(self) -> String {
        self.to_string()
    }
}

/// Converts a `CssClass` into its class name for reactive string storage.
impl IntoReactiveString for CssClass {
    fn into_reactive_string(self) -> String {
        self.get_name().to_string()
    }
}

/// Converts a reference to a `CssClass` into its class name for reactive string storage.
impl IntoReactiveString for &'static CssClass {
    fn into_reactive_string(self) -> String {
        self.get_name().to_string()
    }
}

/// Converts a `bool` into `"true"` or `"false"` for reactive string storage.
impl IntoReactiveString for bool {
    fn into_reactive_string(self) -> String {
        self.to_string()
    }
}

/// Converts an `i32` into a string for reactive string storage.
impl IntoReactiveString for i32 {
    fn into_reactive_string(self) -> String {
        self.to_string()
    }
}

/// Converts a `u32` into a string for reactive string storage.
impl IntoReactiveString for u32 {
    fn into_reactive_string(self) -> String {
        self.to_string()
    }
}

/// Converts a `f64` into a string for reactive string storage.
impl IntoReactiveString for f64 {
    fn into_reactive_string(self) -> String {
        self.to_string()
    }
}

/// Converts a string signal into a reactive string by resolving its current value.
impl IntoReactiveString for Signal<String> {
    fn into_reactive_string(self) -> String {
        self.get()
    }
}

/// Converts a bool signal into a reactive string by resolving its current value.
impl IntoReactiveString for Signal<bool> {
    fn into_reactive_string(self) -> String {
        self.get().to_string()
    }
}

/// Converts a closure into a callback attribute value.
impl<F> IntoCallbackAttribute for F
where
    F: FnMut(NativeEvent) + 'static,
{
    fn into_callback_attribute(self) -> AttributeValue {
        AttributeValue::Event(NativeEventHandler::new(
            NativeEventName::Other("callback".to_string()),
            self,
        ))
    }
}

/// Converts an owned event handler into a callback attribute value.
impl IntoCallbackAttribute for NativeEventHandler {
    fn into_callback_attribute(self) -> AttributeValue {
        AttributeValue::Event(self)
    }
}

/// Converts an optional event handler into a callback attribute value.
impl IntoCallbackAttribute for Option<NativeEventHandler> {
    fn into_callback_attribute(self) -> AttributeValue {
        match self {
            Some(handler) => AttributeValue::Event(handler),
            None => AttributeValue::Text(String::new()),
        }
    }
}
