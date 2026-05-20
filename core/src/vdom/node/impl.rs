use crate::*;

/// Implementation of `From` trait for converting `usize` address into `&'static mut RenderFnInner`.
impl From<usize> for &'static mut RenderFnInner {
    /// Converts a memory address into a mutable reference to `RenderFnInner`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `RenderFnInner` instance.
    ///
    /// # Returns
    ///
    /// - `&'static mut RenderFnInner` - A mutable reference to the `RenderFnInner` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `RenderFnInner` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> Self {
        unsafe { &mut *(address as *mut RenderFnInner) }
    }
}

/// Implementation of `From` trait for converting `usize` address into `&'static RenderFnInner`.
impl From<usize> for &'static RenderFnInner {
    /// Converts a memory address into a reference to `RenderFnInner`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `RenderFnInner` instance.
    ///
    /// # Returns
    ///
    /// - `&'static RenderFnInner` - A reference to the `RenderFnInner` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `RenderFnInner` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> Self {
        unsafe { &*(address as *const RenderFnInner) }
    }
}

/// Visual equality comparison for text nodes.
///
/// Only compares the text content; the backing signal is not considered
/// because it does not affect visual output.
impl PartialEq for TextNode {
    /// Compares two text nodes by their content.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The first text node.
    /// - `&Self` - The second text node.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the text content is equal.
    fn eq(&self, other: &Self) -> bool {
        self.get_content() == other.get_content()
    }
}

/// Visual equality comparison for virtual DOM nodes.
///
/// Used by DynamicNode re-rendering to skip unnecessary DOM patches when
/// the rendered output has not changed. Event attributes are always
/// considered equal because re-binding event listeners is handled
/// separately by the handler registry and does not affect visual output.
/// Dynamic nodes manage their own subtree re-rendering, so two Dynamic
/// variants are always considered equal — the inner renderer handles
/// patching when the dynamic content actually changes.
impl PartialEq for VirtualNode {
    /// Compares two virtual nodes for visual equality.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The first virtual node.
    /// - `&Self` - The second virtual node.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the virtual nodes are visually equal.
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VirtualNode::Text(old_text), VirtualNode::Text(new_text)) => old_text == new_text,
            (
                VirtualNode::Element {
                    tag: old_tag,
                    attributes: old_attrs,
                    children: old_children,
                    ..
                },
                VirtualNode::Element {
                    tag: new_tag,
                    attributes: new_attrs,
                    children: new_children,
                    ..
                },
            ) => {
                old_tag == new_tag
                    && old_attrs.len() == new_attrs.len()
                    && old_attrs
                        .iter()
                        .zip(new_attrs.iter())
                        .all(|(old_attr, new_attr)| old_attr == new_attr)
                    && old_children.len() == new_children.len()
                    && old_children
                        .iter()
                        .zip(new_children.iter())
                        .all(|(old_child, new_child)| old_child == new_child)
            }
            (VirtualNode::Fragment(old_children), VirtualNode::Fragment(new_children)) => {
                old_children.len() == new_children.len()
                    && old_children
                        .iter()
                        .zip(new_children.iter())
                        .all(|(old_child, new_child)| old_child == new_child)
            }
            (VirtualNode::Dynamic(_), VirtualNode::Dynamic(_)) => false,
            (VirtualNode::Empty, VirtualNode::Empty) => true,
            _ => false,
        }
    }
}

/// Provides a default empty dynamic node with a no-op render function.
impl Default for DynamicNode {
    /// Returns a default `DynamicNode` with a no-op render function and empty hook context.
    ///
    /// # Returns
    ///
    /// - `Self` - A default dynamic node.
    fn default() -> Self {
        let inner: Box<RenderFnInner> = Box::new(RenderFnInner {
            render_fn: Box::new(|| VirtualNode::Empty),
        });
        let node: DynamicNode = DynamicNode {
            render_fn: Box::leak(inner) as *mut RenderFnInner,
            hook_context: HookContext::default(),
        };
        node
    }
}

/// Copies a `DynamicNode` by bitwise copy of its raw pointer and hook context.
///
/// A `DynamicNode` is just raw pointers; copying is a trivial bitwise copy.
impl Clone for DynamicNode {
    /// Returns a clone of this dynamic node sharing the same render function.
    ///
    /// # Returns
    ///
    /// - `Self` - A cloned dynamic node.
    fn clone(&self) -> Self {
        *self
    }
}

/// Copies a `DynamicNode` by bitwise copy of its raw pointer and hook context.
///
/// A `DynamicNode` is just raw pointers; copying is a trivial bitwise copy.
impl Copy for DynamicNode {}

/// Implementation of `From` trait for converting `&DynamicNode` into `usize` address.
impl From<&DynamicNode> for usize {
    /// Converts a reference to `DynamicNode` into its render_fn pointer address.
    ///
    /// # Arguments
    ///
    /// - `&DynamicNode` - The reference to the dynamic node.
    ///
    /// # Returns
    ///
    /// - `usize` - The memory address of the render_fn pointer.
    #[inline(always)]
    fn from(node: &DynamicNode) -> Self {
        *node.get_render_fn() as usize
    }
}

/// Implementation of dynamic node accessor methods.
impl DynamicNode {
    /// Returns a mutable reference to the inner render closure state by going
    /// through `usize` intermediate conversion.
    ///
    /// # Returns
    ///
    /// - `&'static mut RenderFnInner` - A mutable reference to the inner render closure state.
    pub(crate) fn leak_mut(&self) -> &'static mut RenderFnInner {
        let address: usize = self.into();
        address.into()
    }

    /// Returns the hook context for this dynamic node.
    ///
    /// # Returns
    ///
    /// - `HookContext` - The hook context (Copy).
    pub(crate) fn get_hook_context_value(&self) -> HookContext {
        *self.get_hook_context()
    }

    /// Invokes the render closure and returns the produced virtual node.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - The virtual node produced by the render closure.
    pub fn render(&self) -> VirtualNode {
        let inner: &mut RenderFnInner = self.leak_mut();
        (inner.get_mut_render_fn())()
    }
}

/// Implementation of virtual node construction and property extraction.
impl VirtualNode {
    /// Determines whether the DOM needs to be patched when transitioning
    /// from `old` to `new`.
    ///
    /// Unlike `PartialEq`, this method treats two `Dynamic` variants as
    /// **different** so that the renderer always re-evaluates dynamic
    /// subtrees. This is essential for route-based `match` expressions
    /// where different pages may occupy the same DynamicNode slot.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The old virtual node.
    /// - `&VirtualNode` - The new virtual node.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the DOM needs to be patched.
    pub fn needs_patch(old: &VirtualNode, new: &VirtualNode) -> bool {
        match (old, new) {
            (VirtualNode::Text(old_text), VirtualNode::Text(new_text)) => {
                old_text.get_content() != new_text.get_content()
            }
            (
                VirtualNode::Element {
                    tag: old_tag,
                    attributes: old_attrs,
                    children: old_children,
                    key: _old_key,
                },
                VirtualNode::Element {
                    tag: new_tag,
                    attributes: new_attrs,
                    children: new_children,
                    key: _new_key,
                },
            ) => {
                if old_tag != new_tag {
                    return true;
                }
                if old_attrs.len() != new_attrs.len() {
                    return true;
                }
                for (old_attr, new_attr) in old_attrs.iter().zip(new_attrs.iter()) {
                    if old_attr.get_name() != new_attr.get_name()
                        || old_attr.get_value() != new_attr.get_value()
                    {
                        return true;
                    }
                }
                if old_children.len() != new_children.len() {
                    return true;
                }
                for (old_child, new_child) in old_children.iter().zip(new_children.iter()) {
                    if Self::needs_patch(old_child, new_child) {
                        return true;
                    }
                }
                false
            }
            (VirtualNode::Fragment(old_children), VirtualNode::Fragment(new_children)) => {
                if old_children.len() != new_children.len() {
                    return true;
                }
                for (old_child, new_child) in old_children.iter().zip(new_children.iter()) {
                    if Self::needs_patch(old_child, new_child) {
                        return true;
                    }
                }
                false
            }
            (VirtualNode::Dynamic(_), VirtualNode::Dynamic(_)) => false,
            (VirtualNode::Empty, VirtualNode::Empty) => false,
            _ => true,
        }
    }

    /// Creates a new element node with the given tag name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The tag name for the element.
    ///
    /// # Returns
    ///
    /// - `Self` - A new element virtual node.
    pub fn get_element_node(tag_name: &str) -> Self {
        VirtualNode::Element {
            tag: Tag::Element(tag_name.to_string()),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
        }
    }

    /// Creates a new text node with the given content.
    ///
    /// # Arguments
    ///
    /// - `&str` - The text content.
    ///
    /// # Returns
    ///
    /// - `Self` - A new text virtual node.
    pub fn get_text_node(content: &str) -> Self {
        VirtualNode::Text(TextNode::new(content.to_string(), None))
    }

    /// Adds an attribute to this node if it is an element.
    ///
    /// # Arguments
    ///
    /// - `&str` - The attribute name.
    /// - `AttributeValue` - The attribute value.
    ///
    /// # Returns
    ///
    /// - `Self` - This node with the attribute added.
    pub fn with_attribute(mut self, name: &str, value: AttributeValue) -> Self {
        if let VirtualNode::Element {
            ref mut attributes, ..
        } = self
        {
            attributes.push(AttributeEntry::new(name.to_string(), value));
        }
        self
    }

    /// Adds a child node to this node if it is an element.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The child node to add.
    ///
    /// # Returns
    ///
    /// - `Self` - This node with the child added.
    pub fn with_child(mut self, child: VirtualNode) -> Self {
        if let VirtualNode::Element {
            ref mut children, ..
        } = self
        {
            children.push(child);
        }
        self
    }

    /// Returns true if this node is a component node.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if this is a component node.
    pub fn is_component(&self) -> bool {
        matches!(
            self,
            VirtualNode::Element {
                tag: Tag::Component(_),
                ..
            }
        )
    }

    /// Returns the tag name if this is an element or component node.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The tag name, or `None` if not an element node.
    pub fn tag_name(&self) -> Option<String> {
        match self {
            VirtualNode::Element { tag, .. } => match tag {
                Tag::Element(name) => Some(name.clone()),
                Tag::Component(name) => Some(name.clone()),
            },
            _ => None,
        }
    }

    /// Extracts a string property from this node if it is an element with the named attribute.
    ///
    /// # Arguments
    ///
    /// - `&str` - The attribute name to look up.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The attribute value as a string, or `None` if not found.
    pub fn try_get_prop(&self, name: &str) -> Option<String> {
        if let VirtualNode::Element { attributes, .. } = self {
            for attr in attributes {
                if attr.get_name() == name {
                    match attr.get_value() {
                        AttributeValue::Text(value) => return Some(value.clone()),
                        AttributeValue::Signal(signal) => return Some(signal.get()),
                        AttributeValue::Dynamic(value) => return Some(value.clone()),
                        _ => {}
                    }
                }
            }
        }
        None
    }

    /// Extracts a typed property from this node by parsing the attribute value string.
    ///
    /// Supports `Text`, `Signal`, and `Dynamic` attribute values. The string
    /// representation is parsed into the target type `T` via `FromStr`.
    ///
    /// # Arguments
    ///
    /// - `&str` - The attribute name to look up.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - The parsed value, or `None` if not found or parsing fails.
    pub fn try_get_typed_prop<T>(&self, name: &str) -> Option<T>
    where
        T: std::str::FromStr,
    {
        if let VirtualNode::Element { attributes, .. } = self {
            for attr in attributes {
                if attr.get_name() == name {
                    let raw: String = match attr.get_value() {
                        AttributeValue::Text(value) => value.clone(),
                        AttributeValue::Signal(signal) => signal.get(),
                        AttributeValue::Dynamic(value) => value.clone(),
                        _ => continue,
                    };
                    return raw.parse::<T>().ok();
                }
            }
        }
        None
    }

    /// Extracts a signal property from this node if it is an element with the named attribute.
    ///
    /// Returns the raw `Signal<String>` so components can reactively read the current value
    /// and subscribe to future changes, rather than receiving a snapshot string.
    ///
    /// # Arguments
    ///
    /// - `&str` - The attribute name to look up.
    ///
    /// # Returns
    ///
    /// - `Option<Signal<String>>` - The signal if found, or `None`.
    pub fn try_get_signal_prop(&self, name: &str) -> Option<Signal<String>> {
        if let VirtualNode::Element { attributes, .. } = self {
            for attr in attributes {
                if attr.get_name() == name
                    && let AttributeValue::Signal(signal) = attr.get_value()
                {
                    return Some(*signal);
                }
            }
        }
        None
    }

    /// Extracts children from this node if it is an element.
    ///
    /// # Returns
    ///
    /// - `Vec<VirtualNode>` - The children, or an empty vec if not an element.
    pub fn get_children(&self) -> Vec<VirtualNode> {
        if let VirtualNode::Element { children, .. } = self {
            children.clone()
        } else {
            Vec::new()
        }
    }

    /// Extracts text content from this node.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The text content, or `None` if not a text node.
    pub fn try_get_text(&self) -> Option<String> {
        match self {
            VirtualNode::Text(text_node) => Some(text_node.get_content().clone()),
            VirtualNode::Element { children, .. } => {
                children.first().and_then(VirtualNode::try_get_text)
            }
            _ => None,
        }
    }

    /// Extracts an event handler from this node if it is an element with the named event attribute.
    ///
    /// # Arguments
    ///
    /// - `&str` - The event name to look up.
    ///
    /// # Returns
    ///
    /// - `Option<NativeEventHandler>` - The event handler if found, or `None`.
    pub fn try_get_event(&self, name: &str) -> Option<NativeEventHandler> {
        if let VirtualNode::Element { attributes, .. } = self {
            for attr in attributes {
                if attr.get_name() == name
                    && let AttributeValue::Event(handler) = attr.get_value()
                {
                    return Some(handler.clone());
                }
            }
        }
        None
    }

    /// Extracts an event handler from this node by a custom attribute name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The custom attribute name to look up.
    ///
    /// # Returns
    ///
    /// - `Option<NativeEventHandler>` - The event handler if found, or `None`.
    pub fn try_get_callback(&self, name: &str) -> Option<NativeEventHandler> {
        if let VirtualNode::Element { attributes, .. } = self {
            for attr in attributes {
                if attr.get_name() == name
                    && let AttributeValue::Event(handler) = attr.get_value()
                {
                    return Some(handler.clone());
                }
            }
        }
        None
    }
}
