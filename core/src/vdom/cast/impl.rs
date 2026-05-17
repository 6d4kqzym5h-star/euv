use crate::*;

/// Converts a `VirtualNode` reference into an owned node.
impl AsNode for VirtualNode {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(self.clone())
    }
}

/// Converts a `VirtualNode` reference into an owned node.
impl AsNode for &VirtualNode {
    fn as_node(&self) -> Option<VirtualNode> {
        Some((*self).clone())
    }
}

/// Converts a `String` into a text virtual node.
impl AsNode for String {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.clone(), None)))
    }
}

/// Converts a string slice into a text virtual node.
impl AsNode for &str {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts an `i32` into a text virtual node.
impl AsNode for i32 {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts an `i64` into a text virtual node.
impl AsNode for i64 {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts a `usize` into a text virtual node.
impl AsNode for usize {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts an `f32` into a text virtual node.
impl AsNode for f32 {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts an `f64` into a text virtual node.
impl AsNode for f64 {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts a `bool` into a text virtual node.
impl AsNode for bool {
    fn as_node(&self) -> Option<VirtualNode> {
        Some(VirtualNode::Text(TextNode::new(self.to_string(), None)))
    }
}

/// Converts a signal into a reactive text virtual node.
impl<T> AsNode for Signal<T>
where
    T: Clone + PartialEq + std::fmt::Display + 'static,
{
    fn as_node(&self) -> Option<VirtualNode> {
        Some(self.as_reactive_text())
    }
}

/// Converts a `VirtualNode` into itself via `IntoNode`.
impl IntoNode for VirtualNode {
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
    fn into_node(self) -> VirtualNode {
        VirtualNode::Dynamic(DynamicNode {
            render_fn: Rc::new(RefCell::new(self)),
            hook_context: crate::reactive::create_hook_context(),
        })
    }
}

/// Converts a `String` into a text virtual node via `IntoNode`.
impl IntoNode for String {
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self, None))
    }
}

/// Converts a `&str` into a text virtual node via `IntoNode`.
impl IntoNode for &str {
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self.to_string(), None))
    }
}

/// Converts an `i32` into a text virtual node via `IntoNode`.
impl IntoNode for i32 {
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self.to_string(), None))
    }
}

/// Converts a `usize` into a text virtual node via `IntoNode`.
impl IntoNode for usize {
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self.to_string(), None))
    }
}

/// Converts a `bool` into a text virtual node via `IntoNode`.
impl IntoNode for bool {
    fn into_node(self) -> VirtualNode {
        VirtualNode::Text(TextNode::new(self.to_string(), None))
    }
}

/// Converts a signal into a reactive text virtual node via `IntoNode`.
impl<T> IntoNode for Signal<T>
where
    T: Clone + PartialEq + std::fmt::Display + 'static,
{
    fn into_node(self) -> VirtualNode {
        self.as_reactive_text()
    }
}

/// Converts a signal into a reactive text node with listener wiring.
impl<T> AsReactiveText for Signal<T>
where
    T: Clone + PartialEq + std::fmt::Display + 'static,
{
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
