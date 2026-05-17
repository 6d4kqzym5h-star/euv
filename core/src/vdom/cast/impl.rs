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
