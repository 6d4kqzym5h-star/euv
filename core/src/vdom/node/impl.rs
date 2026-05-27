use crate::*;

/// Visual equality comparison for text nodes.
///
/// Only compares the text content; the backing signal is not considered
/// because it does not affect visual output.
impl PartialEq for TextNode {
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
    fn default() -> Self {
        let render_fn_inner: Rc<RefCell<RenderFnInner>> =
            Rc::new(RefCell::new(RenderFnInner::new(Box::new(|| {
                VirtualNode::Empty
            }))));
        Self::new(render_fn_inner, HookContext::default())
    }
}

/// Clones a `DynamicNode` by cloning the shared references.
impl Clone for DynamicNode {
    fn clone(&self) -> Self {
        let cloned: Self = Self::new(
            self.get_render_fn().clone(),
            self.get_hook_context().clone(),
        );
        cloned
    }
}

/// Implementation of dynamic node accessor methods.
impl DynamicNode {
    /// Returns the hook context for this dynamic node.
    ///
    /// # Returns
    ///
    /// - `HookContext` - The hook context.
    pub(crate) fn get_hook_context_value(&self) -> HookContext {
        self.get_hook_context().clone()
    }

    /// Invokes the render closure and returns the produced virtual node.
    ///
    /// # Returns
    ///
    /// - `Self` - The virtual node produced by the render closure.
    pub fn render(&self) -> VirtualNode {
        let mut inner: RefMut<RenderFnInner> = self.get_render_fn().borrow_mut();
        (inner.get_mut_render_fn())()
    }
}

/// Implementation of virtual node construction and property extraction.
impl VirtualNode {
    /// Constructs a `Self::Dynamic` from a render closure with hook context management.
    ///
    /// # Arguments
    ///
    /// - `FnMut() -> Self + 'static` - The render closure that produces
    ///   a virtual node tree. Called on initial render and on every signal update.
    ///
    /// # Returns
    ///
    /// - `Self` - A `Self::Dynamic` wrapping the render closure
    ///   with a fresh `HookContext`.
    pub fn create_dynamic<F>(mut render_fn: F) -> Self
    where
        F: FnMut() -> Self + 'static,
    {
        let hook_context: HookContext = create_hook_context();
        let mut hook_context_for_closure: HookContext = hook_context.clone();
        let inner: Rc<RefCell<RenderFnInner>> =
            Rc::new(RefCell::new(RenderFnInner::new(Box::new(move || {
                hook_context_for_closure.reset_hook_index();
                render_fn()
            }))));
        let dynamic_node: DynamicNode = DynamicNode::new(inner, hook_context);
        Self::Dynamic(dynamic_node)
    }

    /// Constructs a `Self::Dynamic` for match expressions where arm hook
    /// isolation is required. The render closure receives a `&mut HookContext`
    /// so it can call `set_arm_changed` before each arm body.
    ///
    /// # Arguments
    ///
    /// - `FnMut(&mut HookContext) -> Self + 'static` - The render closure
    ///   that receives a mutable reference to the hook context.
    ///
    /// # Returns
    ///
    /// - `Self` - A `Self::Dynamic` wrapping the render closure
    ///   with a fresh `HookContext`.
    pub fn create_dynamic_with_context<F>(mut render_fn: F) -> Self
    where
        F: FnMut(&mut HookContext) -> Self + 'static,
    {
        let hook_context: HookContext = create_hook_context();
        let mut hook_context_for_closure: HookContext = hook_context.clone();
        let inner: Rc<RefCell<RenderFnInner>> =
            Rc::new(RefCell::new(RenderFnInner::new(Box::new(move || {
                hook_context_for_closure.reset_hook_index();
                render_fn(&mut hook_context_for_closure)
            }))));
        let dynamic_node: DynamicNode = DynamicNode::new(inner, hook_context);
        Self::Dynamic(dynamic_node)
    }

    /// Creates a new element node with the given tag name.
    ///
    /// # Arguments
    ///
    /// - `&str`- The HTML tag name.
    ///
    /// # Returns
    ///
    /// - `Self`- A new element virtual node.
    pub fn get_element_node(tag_name: &str) -> Self {
        Self::Element {
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
    /// - `&str`- The text content.
    ///
    /// # Returns
    ///
    /// - `Self`- A new text virtual node.
    pub fn get_text_node(content: &str) -> Self {
        Self::Text(TextNode::new(content.to_string(), None))
    }

    /// Adds an attribute to this node if it is an element.
    ///
    /// # Arguments
    ///
    /// - `&str`- The attribute name.
    /// - `AttributeValue`- The attribute value.
    ///
    /// # Returns
    ///
    /// - `Self`- This node with the attribute added.
    pub fn with_attribute(mut self, name: &str, value: AttributeValue) -> Self {
        if let Self::Element {
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
    /// - `VirtualNode`- The child node to add.
    ///
    /// # Returns
    ///
    /// - `Self`- This node with the child added.
    pub fn with_child(mut self, child: VirtualNode) -> Self {
        if let Self::Element {
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
    /// - `bool`- `true` if this is a component element.
    pub fn is_component(&self) -> bool {
        matches!(
            self,
            Self::Element {
                tag: Tag::Component(_),
                ..
            }
        )
    }

    /// Returns the tag name if this is an element or component node.
    ///
    /// # Returns
    ///
    /// - `Option<String>`- The tag name, or `None` if not an element.
    pub fn tag_name(&self) -> Option<String> {
        match self {
            Self::Element { tag, .. } => match tag {
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
    /// - `&str`- The attribute name to look for.
    ///
    /// # Returns
    ///
    /// - `Option<String>`- The attribute value as a string, or `None`.
    pub fn try_get_prop(&self, name: &str) -> Option<String> {
        if let Self::Element { attributes, .. } = self {
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
    /// # Arguments
    ///
    /// - `&str`- The attribute name to look for.
    ///
    /// # Returns
    ///
    /// - `Option<T>`- The parsed value, or `None` if not found or parsing fails.
    pub fn try_get_typed_prop<T>(&self, name: &str) -> Option<T>
    where
        T: FromStr,
    {
        if let Self::Element { attributes, .. } = self {
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
    /// # Arguments
    ///
    /// - `&str`- The attribute name to look for.
    ///
    /// # Returns
    ///
    /// - `Option<Signal<String>>`- The signal, or `None` if not found.
    pub fn try_get_signal_prop(&self, name: &str) -> Option<Signal<String>> {
        if let Self::Element { attributes, .. } = self {
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
    /// - `Vec<Self>`- The children, or an empty vec if not an element.
    pub fn get_children(&self) -> Vec<Self> {
        if let Self::Element { children, .. } = self {
            children.clone()
        } else {
            Vec::new()
        }
    }

    /// Extracts text content from this node.
    ///
    /// # Returns
    ///
    /// - `Option<String>`- The text content, or `None` if unavailable.
    pub fn try_get_text(&self) -> Option<String> {
        match self {
            Self::Text(text_node) => Some(text_node.get_content().clone()),
            Self::Element { children, .. } => children.first().and_then(Self::try_get_text),
            _ => None,
        }
    }

    /// Extracts an event handler from this node if it is an element with the named event attribute.
    ///
    /// # Arguments
    ///
    /// - `&str`- The event attribute name to look for.
    ///
    /// # Returns
    ///
    /// - `Option<NativeEventHandler>`- The handler, or `None` if not found.
    pub fn try_get_event(&self, name: &str) -> Option<NativeEventHandler> {
        if let Self::Element { attributes, .. } = self {
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
    /// - `&str`- The custom callback attribute name to look for.
    ///
    /// # Returns
    ///
    /// - `Option<NativeEventHandler>`- The handler, or `None` if not found.
    pub fn try_get_callback(&self, name: &str) -> Option<NativeEventHandler> {
        if let Self::Element { attributes, .. } = self {
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
