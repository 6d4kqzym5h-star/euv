use crate::*;

/// Converts a `VirtualNode` reference into an owned node.
impl AsNode for VirtualNode {
    /// Clones this virtual node into an owned `Option<VirtualNode>`.
    ///
    /// # Returns
    ///
    /// - `Option<VirtualNode>` - Always `Some` with the cloned node.
    fn as_node(&self) -> Option<VirtualNode> {
        Some(self.clone())
    }
}

/// Converts a `VirtualNode` reference into an owned node.
impl AsNode for &VirtualNode {
    /// Clones this virtual node reference into an owned `Option<VirtualNode>`.
    ///
    /// # Returns
    ///
    /// - `Option<VirtualNode>` - Always `Some` with the cloned node.
    fn as_node(&self) -> Option<VirtualNode> {
        Some((*self).clone())
    }
}

/// Converts a `String` into a text virtual node.
impl AsNode for String {
    /// Converts this string into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `Option<VirtualNode>` - Always `Some` with a text node.
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.clone(), None)))
    }
}

/// Converts a string slice into a text virtual node.
impl AsNode for &str {
    /// Converts this string slice into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `Option<VirtualNode>` - Always `Some` with a text node.
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts an `i32` into a text virtual node.
impl AsNode for i32 {
    /// Converts this integer into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `Option<VirtualNode>` - Always `Some` with a text node.
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts an `i64` into a text virtual node.
impl AsNode for i64 {
    /// Converts this integer into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `Option<VirtualNode>` - Always `Some` with a text node.
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts a `usize` into a text virtual node.
impl AsNode for usize {
    /// Converts this unsigned integer into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `Option<VirtualNode>` - Always `Some` with a text node.
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts an `f32` into a text virtual node.
impl AsNode for f32 {
    /// Converts this float into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `Option<VirtualNode>` - Always `Some` with a text node.
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts an `f64` into a text virtual node.
impl AsNode for f64 {
    /// Converts this float into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `Option<VirtualNode>` - Always `Some` with a text node.
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts a `bool` into a text virtual node.
impl AsNode for bool {
    /// Converts this boolean into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `Option<VirtualNode>` - Always `Some` with a text node.
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts a signal into a reactive text virtual node.
impl<T> AsNode for Signal<T>
where
    T: Clone + PartialEq + std::fmt::Display + 'static,
{
    /// Converts this signal into a reactive text virtual node.
    ///
    /// # Returns
    ///
    /// - `Option<VirtualNode>` - Always `Some` with a reactive text node.
    fn as_node(&self) -> Option<VirtualNode> {
        Some(self.as_reactive_text())
    }
}

/// Converts a `VirtualNode` into itself via `IntoNode`.
impl IntoNode for VirtualNode {
    /// Returns this virtual node as-is.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - This same virtual node.
    fn into_node(self) -> VirtualNode {
        self
    }
}

/// Wraps a `FnMut() -> VirtualNode` closure into a `DynamicNode` via `IntoNode`.
///
/// This enables writing `{move || html! { ... }}` directly in HTML markup
/// without explicit `DynamicNode` construction.
impl<F> IntoNode for F
where
    F: FnMut() -> VirtualNode + 'static,
{
    /// Wraps this closure into a `VirtualNode::Dynamic` with a fresh hook context.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A dynamic virtual node wrapping this closure.
    fn into_node(self) -> VirtualNode {
        VirtualNode::Dynamic(DynamicNode {
            render_fn: Rc::new(RefCell::new(self)),
            hook_context: crate::reactive::create_hook_context(),
        })
    }
}

/// Converts a `String` into a text virtual node via `IntoNode`.
impl IntoNode for String {
    /// Converts this string into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A text virtual node.
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self, None))
    }
}

/// Converts a `&str` into a text virtual node via `IntoNode`.
impl IntoNode for &str {
    /// Converts this string slice into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A text virtual node.
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self.to_string(), None))
    }
}

/// Converts an `i32` into a text virtual node via `IntoNode`.
impl IntoNode for i32 {
    /// Converts this integer into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A text virtual node.
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self.to_string(), None))
    }
}

/// Converts a `usize` into a text virtual node via `IntoNode`.
impl IntoNode for usize {
    /// Converts this unsigned integer into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A text virtual node.
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self.to_string(), None))
    }
}

/// Converts a `bool` into a text virtual node via `IntoNode`.
impl IntoNode for bool {
    /// Converts this boolean into a text virtual node.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A text virtual node.
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self.to_string(), None))
    }
}

/// Converts a signal into a reactive text virtual node via `IntoNode`.
impl<T> IntoNode for Signal<T>
where
    T: Clone + PartialEq + std::fmt::Display + 'static,
{
    /// Converts this signal into a reactive text virtual node.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A reactive text virtual node.
    fn into_node(self) -> VirtualNode {
        self.as_reactive_text()
    }
}

/// Converts a signal into a reactive text node with listener wiring.
impl<T> AsReactiveText for Signal<T>
where
    T: Clone + PartialEq + std::fmt::Display + 'static,
{
    /// Creates a reactive text node that auto-updates when the signal changes.
    ///
    /// Internally creates a bridge `Signal<String>` that subscribes to the
    /// source signal and updates the text content on every change.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - A text virtual node with reactive signal binding.
    fn as_reactive_text(&self) -> VirtualNode {
        let signal: Signal<T> = *self;
        let initial: String = signal.get().to_string();
        let string_signal: Signal<String> = {
            let boxed: Box<SignalInner<String>> = Box::new(SignalInner::new(initial.clone()));
            Signal::from_inner(Box::leak(boxed) as *mut SignalInner<String>)
        };
        let source_signal: Signal<T> = *self;
        let string_signal_clone: Signal<String> = string_signal;
        source_signal.subscribe({
            let source_signal: Signal<T> = source_signal;
            move || {
                let new_value: String = source_signal.get().to_string();
                string_signal_clone.set(new_value);
            }
        });
        VirtualNode::Text(TextNode::new(initial, Some(string_signal)))
    }
}

/// Constructs an `EventAdapter` that wraps any event-compatible value.
impl<T> EventAdapter<T> {
    /// Creates a new `EventAdapter` wrapping the given value.
    ///
    /// # Arguments
    ///
    /// - `T` - The value to wrap for event attribute adaptation.
    ///
    /// # Returns
    ///
    /// - `EventAdapter<T>` - A new adapter wrapping the value.
    pub fn new(inner: T) -> Self {
        EventAdapter { inner }
    }
}

/// Adapts a `FnMut(NativeEvent)` closure into an `AttributeValue::Event`.
///
/// Wraps the closure into a `NativeEventHandler` and returns it as an
/// event attribute value. This replaces the `__EventWrapper<F>` type
/// that was previously generated inline by the `html!` macro.
impl<F> EventAdapter<F>
where
    F: FnMut(NativeEvent) + 'static,
{
    /// Converts the wrapped closure into an event `AttributeValue`.
    ///
    /// # Arguments
    ///
    /// - `NativeEventName` - The event name enum variant to associate with the handler.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An `AttributeValue::Event` wrapping the handler.
    pub fn into_attribute(self, event_name: NativeEventName) -> AttributeValue {
        AttributeValue::Event(NativeEventHandler::new(event_name, self.inner))
    }
}

/// Adapts an owned `NativeEventHandler` into an `AttributeValue::Event` directly.
///
/// When the user already provides a `NativeEventHandler`, no wrapping is needed;
/// the handler is returned as-is. This replaces the `impl __EventWrapper<NativeEventHandler>`
/// that was previously generated inline.
impl EventAdapter<NativeEventHandler> {
    /// Converts the wrapped handler into an event `AttributeValue`.
    ///
    /// # Arguments
    ///
    /// - `NativeEventName` - The event name (unused, since the handler already carries it).
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An `AttributeValue::Event` containing the handler.
    pub fn into_attribute(self, _event_name: NativeEventName) -> AttributeValue {
        AttributeValue::Event(self.inner)
    }
}

/// Adapts an `Option<NativeEventHandler>` into an `AttributeValue`.
///
/// `Some(handler)` becomes `AttributeValue::Event(handler)`, and `None` becomes
/// `AttributeValue::Text(String::new())`. This replaces the
/// `impl __EventWrapper<Option<NativeEventHandler>>` that was previously
/// generated inline by the `html!` macro.
impl EventAdapter<Option<NativeEventHandler>> {
    /// Converts the wrapped optional handler into an attribute value.
    ///
    /// # Arguments
    ///
    /// - `NativeEventName` - The event name (unused when handler is `None`).
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute if `Some`, otherwise an empty text attribute.
    pub fn into_attribute(self, _event_name: NativeEventName) -> AttributeValue {
        match self.inner {
            Some(handler) => AttributeValue::Event(handler),
            None => AttributeValue::Text(String::new()),
        }
    }
}

/// Constructs an `AttrValueAdapter` that wraps any attribute-compatible value.
impl<T> AttrValueAdapter<T> {
    /// Creates a new `AttrValueAdapter` wrapping the given value.
    ///
    /// # Arguments
    ///
    /// - `T` - The value to wrap for attribute adaptation.
    ///
    /// # Returns
    ///
    /// - `AttrValueAdapter<T>` - A new adapter wrapping the value.
    pub fn new(inner: T) -> Self {
        AttrValueAdapter { inner }
    }
}

/// Adapts a `FnMut(NativeEvent)` closure into a callback `AttributeValue`.
///
/// This handles the case where a closure is used as a component callback prop.
/// The closure is converted via `IntoCallbackAttribute::into_callback_attribute()`.
/// This replaces the `__IsClosure for F` impl that was previously generated inline.
impl<F> AttrValueAdapter<F>
where
    F: FnMut(NativeEvent) + 'static,
{
    /// Converts the wrapped closure into a callback `AttributeValue`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value wrapping the adapted closure.
    pub fn into_callback_attribute_value(self) -> AttributeValue {
        self.inner.into_callback_attribute()
    }
}

/// Adapts an owned `NativeEventHandler` into an `AttributeValue::Event` directly.
///
/// When the user already provides a `NativeEventHandler`, it is returned as-is.
/// This replaces the `__IsClosure for NativeEventHandler` impl that was previously
/// generated inline by the `html!` macro.
impl AttrValueAdapter<NativeEventHandler> {
    /// Converts the wrapped handler into an event `AttributeValue`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An `AttributeValue::Event` containing the handler.
    pub fn into_callback_attribute_value(self) -> AttributeValue {
        AttributeValue::Event(self.inner)
    }
}

/// Adapts an `Option<NativeEventHandler>` into an `AttributeValue`.
///
/// `Some(handler)` becomes `AttributeValue::Event(handler)`, and `None` becomes
/// `AttributeValue::Text(String::new())`. This replaces the
/// `__IsClosure for Option<NativeEventHandler>` impl that was previously
/// generated inline by the `html!` macro.
impl AttrValueAdapter<Option<NativeEventHandler>> {
    /// Converts the wrapped optional handler into an attribute value.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute if `Some`, otherwise an empty text attribute.
    pub fn into_callback_attribute_value(self) -> AttributeValue {
        match self.inner {
            Some(handler) => AttributeValue::Event(handler),
            None => AttributeValue::Text(String::new()),
        }
    }
}

/// Adapts any `IntoReactiveValue` type into an `AttributeValue`.
///
/// This is the fallback path for non-closure attribute values (strings, signals,
/// CSS classes, etc.). The value is converted via `IntoReactiveValue::into_reactive_value()`.
/// This replaces the `__ValuePicker` / `__FallbackHelper` hierarchy that was previously
/// generated inline by the `html!` macro.
impl<T> AttrValueAdapter<T>
where
    T: IntoReactiveValue,
{
    /// Converts the wrapped value into an `AttributeValue` via reactive value adaptation.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - The reactive attribute value.
    pub fn into_reactive_attribute_value(self) -> AttributeValue {
        self.inner.into_reactive_value()
    }
}
