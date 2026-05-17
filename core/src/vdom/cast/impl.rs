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

    /// Returns the inner wrapped value, consuming the adapter.
    ///
    /// # Returns
    ///
    /// - `T` - The inner value.
    pub(crate) fn into_inner(self) -> T {
        self.inner
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
        AttributeValue::Event(NativeEventHandler::new(event_name, self.into_inner()))
    }
}

/// Adapts an owned `NativeEventHandler` into an `AttributeValue::Event` directly.
///
/// When the user already provides a `NativeEventHandler`, the handler is
/// re-wrapped with the given `event_name` to ensure the DOM event listener
/// is bound to the correct event type (e.g., "click" rather than "onclick").
/// This replaces the `impl __EventWrapper<NativeEventHandler>` that was
/// previously generated inline.
impl EventAdapter<NativeEventHandler> {
    /// Converts the wrapped handler into an event `AttributeValue`.
    ///
    /// Re-wraps the handler with the provided `event_name` so that the
    /// DOM event listener uses the correct event type string.
    ///
    /// # Arguments
    ///
    /// - `NativeEventName` - The event name to bind the handler to.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An `AttributeValue::Event` containing the re-wrapped handler.
    pub fn into_attribute(self, event_name: NativeEventName) -> AttributeValue {
        let handler: NativeEventHandler = self.into_inner();
        AttributeValue::Event(NativeEventHandler::new(
            event_name,
            move |event: NativeEvent| {
                handler.handle(event);
            },
        ))
    }
}

/// Adapts an `Option<NativeEventHandler>` into an `AttributeValue`.
///
/// `Some(handler)` becomes `AttributeValue::Event(handler)` re-wrapped with the
/// given event name, and `None` becomes `AttributeValue::Text(String::new())`.
/// This replaces the `impl __EventWrapper<Option<NativeEventHandler>>` that was
/// previously generated inline by the `html!` macro.
impl EventAdapter<Option<NativeEventHandler>> {
    /// Converts the wrapped optional handler into an attribute value.
    ///
    /// Re-wraps a `Some` handler with the provided `event_name` so that the
    /// DOM event listener uses the correct event type string.
    ///
    /// # Arguments
    ///
    /// - `NativeEventName` - The event name to bind the handler to.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute if `Some`, otherwise an empty text attribute.
    pub fn into_attribute(self, event_name: NativeEventName) -> AttributeValue {
        match self.into_inner() {
            Some(handler) => EventAdapter::new(handler).into_attribute(event_name),
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

    /// Returns the inner wrapped value, consuming the adapter.
    ///
    /// # Returns
    ///
    /// - `T` - The inner value.
    pub(crate) fn into_inner(self) -> T {
        self.inner
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
        self.into_inner().into_callback_attribute()
    }

    /// Converts the wrapped closure into a callback `AttributeValue` with a
    /// custom event name for component props.
    ///
    /// # Arguments
    ///
    /// - `String` - The custom attribute name (e.g., "on-increment", "on-change").
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value with the custom name.
    pub fn into_callback_attribute_value_with_name(self, name: String) -> AttributeValue {
        AttributeValue::Event(NativeEventHandler::new(
            NativeEventName::Other(name),
            self.into_inner(),
        ))
    }
}

/// Adapts an owned `NativeEventHandler` into an `AttributeValue::Event` directly.
///
/// When the user already provides a `NativeEventHandler`, the handler is
/// re-wrapped with a generic "callback" event name so that subsequent
/// `EventAdapter::into_attribute` calls can override it with the correct
/// DOM event type. This replaces the `__IsClosure for NativeEventHandler`
/// impl that was previously generated inline by the `html!` macro.
impl AttrValueAdapter<NativeEventHandler> {
    /// Converts the wrapped handler into an event `AttributeValue`.
    ///
    /// Re-wraps the handler with a generic callback event name so that the
    /// DOM event type can be correctly resolved when the handler is later
    /// bound to a real element via `EventAdapter::into_attribute`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An `AttributeValue::Event` containing the re-wrapped handler.
    pub fn into_callback_attribute_value(self) -> AttributeValue {
        let handler: NativeEventHandler = self.into_inner();
        AttributeValue::Event(NativeEventHandler::new(
            NativeEventName::Other("callback".to_string()),
            move |event: NativeEvent| {
                handler.handle(event);
            },
        ))
    }

    /// Converts the wrapped handler into a callback `AttributeValue` with a
    /// custom event name for component props.
    ///
    /// # Arguments
    ///
    /// - `String` - The custom attribute name.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value with the custom name.
    pub fn into_callback_attribute_value_with_name(self, name: String) -> AttributeValue {
        let handler: NativeEventHandler = self.into_inner();
        AttributeValue::Event(NativeEventHandler::new(
            NativeEventName::Other(name),
            move |event: NativeEvent| {
                handler.handle(event);
            },
        ))
    }
}

/// Adapts an `Option<NativeEventHandler>` into an `AttributeValue`.
///
/// `Some(handler)` becomes `AttributeValue::Event(handler)` re-wrapped with
/// a generic callback event name, and `None` becomes `AttributeValue::Text(String::new())`.
/// This replaces the `__IsClosure for Option<NativeEventHandler>` impl that was
/// previously generated inline by the `html!` macro.
impl AttrValueAdapter<Option<NativeEventHandler>> {
    /// Converts the wrapped optional handler into an attribute value.
    ///
    /// Re-wraps a `Some` handler with a generic callback event name so that the
    /// DOM event type can be correctly resolved when the handler is later bound
    /// to a real element via `EventAdapter::into_attribute`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute if `Some`, otherwise an empty text attribute.
    pub fn into_callback_attribute_value(self) -> AttributeValue {
        match self.into_inner() {
            Some(handler) => AttrValueAdapter::new(handler).into_callback_attribute_value(),
            None => AttributeValue::Text(String::new()),
        }
    }

    /// Converts this optional handler into a callback `AttributeValue` with a
    /// custom event name for component props.
    ///
    /// # Arguments
    ///
    /// - `String` - The custom attribute name.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute with the custom name if `Some`,
    ///   otherwise an empty text attribute.
    pub fn into_callback_attribute_value_with_name(self, name: String) -> AttributeValue {
        match self.into_inner() {
            Some(handler) => {
                AttrValueAdapter::new(handler).into_callback_attribute_value_with_name(name)
            }
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
        self.into_inner().into_reactive_value()
    }
}
