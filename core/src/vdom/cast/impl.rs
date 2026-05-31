use crate::*;

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

/// Converts a `Vec<VirtualNode>` into a `VirtualNode::Fragment` via `IntoNode`.
///
/// This enables using a `Vec<VirtualNode>` directly in the `html!` macro
/// without manually wrapping it in `VirtualNode::Fragment(...)`.
///
/// # Returns
///
/// - `VirtualNode` - A `VirtualNode::Fragment` containing the nodes, or
///   `VirtualNode::Empty` if the vector is empty.
impl IntoNode for Vec<VirtualNode> {
    fn into_node(self) -> VirtualNode {
        if self.is_empty() {
            VirtualNode::Empty
        } else {
            VirtualNode::Fragment(self)
        }
    }
}

/// Converts an `Option<VirtualNode>` into a `VirtualNode` via `IntoNode`.
///
/// `Some(node)` returns the inner node, `None` returns `VirtualNode::Empty`.
///
/// # Returns
///
/// - `VirtualNode` - The inner node if `Some`, otherwise `VirtualNode::Empty`.
impl IntoNode for Option<VirtualNode> {
    fn into_node(self) -> VirtualNode {
        match self {
            Some(node) => node,
            None => VirtualNode::Empty,
        }
    }
}

/// Converts an `Option<Vec<VirtualNode>>` into a `VirtualNode` via `IntoNode`.
///
/// `Some(vec)` converts the vector into a `VirtualNode::Fragment` (or `Empty`
/// if the vector is empty), `None` returns `VirtualNode::Empty`.
///
/// # Returns
///
/// - `VirtualNode` - A `VirtualNode::Fragment` if `Some` with nodes,
///   `VirtualNode::Empty` if `None` or the vector is empty.
impl IntoNode for Option<Vec<VirtualNode>> {
    fn into_node(self) -> VirtualNode {
        match self {
            Some(nodes) => nodes.into_node(),
            None => VirtualNode::Empty,
        }
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
        let render_fn_inner: Rc<RefCell<RenderFnInner>> =
            Rc::new(RefCell::new(RenderFnInner::new(Box::new(self))));
        VirtualNode::Dynamic(DynamicNode::new(
            render_fn_inner,
            crate::reactive::create_hook_context(),
        ))
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
    T: Clone + PartialEq + Display + 'static,
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
    T: Clone + PartialEq + Display + 'static,
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
        let initial: String = self.get().to_string();
        let string_signal: Signal<String> = Signal::create(initial.clone());
        let string_signal_clone: Signal<String> = string_signal;
        self.replace_subscribe({
            let source: Signal<T> = *self;
            move || {
                string_signal_clone.set_silent(source.get().to_string());
            }
        });
        VirtualNode::Text(TextNode::new(initial, Some(string_signal)))
    }
}

/// Constructs an `EventAdapter` that wraps any event-compatible value.
impl<T> EventAdapter<T> {
    /// Returns the inner wrapped value, consuming the adapter.
    ///
    /// # Returns
    ///
    /// - `T` - The inner value.
    pub(crate) fn into_inner(self) -> T {
        self.inner
    }
}

/// Adapts a `FnMut(Event)` closure into an `AttributeValue::Event`.
///
/// Wraps the closure into a `NativeEventHandler` and returns it as an
/// event attribute value. This replaces the `__EventWrapper<F>` type
/// that was previously generated inline by the `html!` macro.
impl<F> EventAdapter<F>
where
    F: FnMut(Event) + 'static,
{
    /// Converts the wrapped closure into an event `AttributeValue`.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name string to associate with the handler.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An `AttributeValue::Event` wrapping the handler.
    pub fn into_attribute(self, event_name: &'static str) -> AttributeValue {
        AttributeValue::Event(NativeEventHandler::create(event_name, self.into_inner()))
    }
}

/// Adapts an owned `NativeEventHandler` into an `AttributeValue::Event` directly.
///
/// When the user already provides a `NativeEventHandler`, the handler is
/// re-wrapped with the given `event_name` to ensure the DOM event listener
/// is bound to the correct event type (e.g., "click" rather than "onclick").
impl EventAdapter<NativeEventHandler> {
    /// Converts the wrapped handler into an event `AttributeValue`.
    ///
    /// Re-wraps the handler with the provided `event_name` so that the
    /// DOM event listener uses the correct event type string.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name to bind the handler to.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An `AttributeValue::Event` containing the re-wrapped handler.
    pub fn into_attribute(self, event_name: &'static str) -> AttributeValue {
        let mut handler: NativeEventHandler = self.into_inner();
        handler.set_event_name(event_name);
        AttributeValue::Event(handler)
    }
}

/// Adapts an `Option<NativeEventHandler>` into an `AttributeValue`.
///
/// `Some(handler)` becomes `AttributeValue::Event(handler)` re-wrapped with the
/// given event name, and `None` becomes `AttributeValue::Text(String::new())`.
impl EventAdapter<Option<NativeEventHandler>> {
    /// Converts the wrapped optional handler into an attribute value.
    ///
    /// Re-wraps a `Some` handler with the provided `event_name` so that the
    /// DOM event listener uses the correct event type string.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name to bind the handler to.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute if `Some`, otherwise an empty text attribute.
    pub fn into_attribute(self, event_name: &'static str) -> AttributeValue {
        match self.into_inner() {
            Some(handler) => EventAdapter::new(handler).into_attribute(event_name),
            None => AttributeValue::Text(String::new()),
        }
    }
}

/// Adapts an `Option<Rc<dyn Fn(Event)>>` into an `AttributeValue`.
///
/// `Some(callback)` becomes `AttributeValue::Event` by wrapping the shared closure
/// into a `NativeEventHandler`, and `None` becomes `AttributeValue::Text(String::new())`.
/// This supports component Props that use `Option<Rc<dyn Fn(Event)>>` for event callbacks.
impl EventAdapter<Option<Rc<dyn Fn(Event)>>> {
    /// Converts the wrapped optional shared closure into an attribute value.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The event name to bind the handler to.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute if `Some`, otherwise an empty text attribute.
    pub fn into_attribute(self, event_name: &'static str) -> AttributeValue {
        match self.into_inner() {
            Some(callback) => {
                let wrapper = move |event: Event| {
                    callback(event);
                };
                AttributeValue::Event(NativeEventHandler::create(event_name, wrapper))
            }
            None => AttributeValue::Text(String::new()),
        }
    }
}

/// Constructs an `AttrValueAdapter` that wraps any attribute-compatible value.
impl<T> AttrValueAdapter<T> {
    /// Returns the inner wrapped value, consuming the adapter.
    ///
    /// # Returns
    ///
    /// - `T` - The inner value.
    pub(crate) fn into_inner(self) -> T {
        self.inner
    }
}

/// Adapts a `FnMut(Event)` closure into a callback `AttributeValue`.
///
/// This handles the case where a closure is used as a component callback prop.
/// The closure is converted via `IntoCallbackAttribute::into_callback_attribute()`.
impl<F> AttrValueAdapter<F>
where
    F: FnMut(Event) + 'static,
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
    /// - `&'static str` - The custom attribute name (e.g., "on-increment", "on-change").
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value with the custom name.
    pub fn into_callback_attribute_value_with_name(self, name: &'static str) -> AttributeValue {
        AttributeValue::Event(NativeEventHandler::create(name, self.into_inner()))
    }
}

/// Adapts an owned `NativeEventHandler` into an `AttributeValue::Event` directly.
impl AttrValueAdapter<NativeEventHandler> {
    /// Converts the wrapped handler into an event `AttributeValue`.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An `AttributeValue::Event` containing the re-wrapped handler.
    pub fn into_callback_attribute_value(self) -> AttributeValue {
        AttributeValue::Event(self.into_inner())
    }

    /// Converts the wrapped handler into a callback `AttributeValue` with a
    /// custom event name for component props.
    ///
    /// # Arguments
    ///
    /// - `&'static str` - The custom attribute name.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute value with the custom name.
    pub fn into_callback_attribute_value_with_name(self, name: &'static str) -> AttributeValue {
        let mut handler: NativeEventHandler = self.into_inner();
        handler.set_event_name(name);
        AttributeValue::Event(handler)
    }
}

/// Adapts an `Option<NativeEventHandler>` into an `AttributeValue`.
impl AttrValueAdapter<Option<NativeEventHandler>> {
    /// Converts the wrapped optional handler into an attribute value.
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
    /// - `&'static str` - The custom attribute name.
    ///
    /// # Returns
    ///
    /// - `AttributeValue` - An event attribute with the custom name if `Some`,
    ///   otherwise an empty text attribute.
    pub fn into_callback_attribute_value_with_name(self, name: &'static str) -> AttributeValue {
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
