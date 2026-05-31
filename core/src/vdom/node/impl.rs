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

/// Clones a `VirtualNode<T>` by deep-copying all fields.
impl<T: Clone> Clone for VirtualNode<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Element {
                tag,
                attributes,
                children,
                key,
                props,
            } => Self::Element {
                tag: tag.clone(),
                attributes: attributes.clone(),
                children: children.clone(),
                key: key.clone(),
                props: props.clone(),
            },
            Self::Text(text_node) => Self::Text(text_node.clone()),
            Self::Fragment(children) => Self::Fragment(children.clone()),
            Self::Dynamic(dynamic_node) => Self::Dynamic(dynamic_node.clone()),
            Self::Empty => Self::Empty,
        }
    }
}

/// Debug formatting for `VirtualNode<T>`.
///
/// Skips `Dynamic` inner details and `props` for brevity.
impl<T: std::fmt::Debug> std::fmt::Debug for VirtualNode<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Element {
                tag,
                attributes,
                children,
                key,
                props,
            } => formatter
                .debug_struct("Element")
                .field("tag", tag)
                .field("attributes", attributes)
                .field("children", children)
                .field("key", key)
                .field("props", props)
                .finish(),
            Self::Text(text_node) => formatter.debug_tuple("Text").field(text_node).finish(),
            Self::Fragment(children) => formatter.debug_tuple("Fragment").field(children).finish(),
            Self::Dynamic(_) => formatter.debug_tuple("Dynamic").finish(),
            Self::Empty => formatter.debug_tuple("Empty").finish(),
        }
    }
}

/// Default implementation returns `VirtualNode::Empty`.
impl<T> Default for VirtualNode<T> {
    fn default() -> Self {
        Self::Empty
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
impl<T: PartialEq> PartialEq for VirtualNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VirtualNode::Text(old_text), VirtualNode::Text(new_text)) => old_text == new_text,
            (
                VirtualNode::Element {
                    tag: old_tag,
                    attributes: old_attrs,
                    children: old_children,
                    props: old_props,
                    ..
                },
                VirtualNode::Element {
                    tag: new_tag,
                    attributes: new_attrs,
                    children: new_children,
                    props: new_props,
                    ..
                },
            ) => {
                old_tag == new_tag
                    && old_attrs.len() == new_attrs.len()
                    && old_attrs.iter().zip(new_attrs.iter()).all(
                        |(old_attr, new_attr): (&AttributeEntry, &AttributeEntry)| {
                            old_attr == new_attr
                        },
                    )
                    && old_children.len() == new_children.len()
                    && old_children.iter().zip(new_children.iter()).all(
                        |(old_child, new_child): (&VirtualNode, &VirtualNode)| {
                            old_child == new_child
                        },
                    )
                    && old_props == new_props
            }
            (VirtualNode::Fragment(old_children), VirtualNode::Fragment(new_children)) => {
                old_children.len() == new_children.len()
                    && old_children.iter().zip(new_children.iter()).all(
                        |(old_child, new_child): (&VirtualNode, &VirtualNode)| {
                            old_child == new_child
                        },
                    )
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
        Self::new(
            self.get_render_fn().clone(),
            self.get_hook_context().clone(),
        )
    }
}

/// Implementation of dynamic node accessor methods.
impl DynamicNode {
    /// Invokes the render closure and returns the produced virtual node.
    ///
    /// # Returns
    ///
    /// - `Self` - The virtual node produced by the render closure.
    pub fn render(&self) -> VirtualNode {
        self.get_render_fn()
            .try_borrow_mut()
            .map(|mut inner: RefMut<RenderFnInner>| (inner.get_mut_render_fn())())
            .unwrap_or_default()
    }
}

/// Implementation of virtual node construction and property extraction.
impl<T> VirtualNode<T> {
    /// Returns the tag name if this is an element or component node.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The tag name, or `None` if not an element.
    pub fn try_get_tag_name(&self) -> Option<String> {
        match self {
            Self::Element { tag, .. } => match tag {
                Tag::Element(name) => Some(name.clone()),
                Tag::Component(name) => Some(name.clone()),
            },
            _ => None,
        }
    }

    /// Returns a reference to the children of this node, if it has any.
    ///
    /// Returns `Some` for `Element` and `Fragment` variants, `None` otherwise.
    ///
    /// # Returns
    ///
    /// - `Option<&Vec<VirtualNode>>` - The children, or `None`.
    pub fn try_get_children(&self) -> Option<&Vec<VirtualNode>> {
        match self {
            Self::Element { children, .. } => Some(children),
            Self::Fragment(children) => Some(children),
            _ => None,
        }
    }

    /// Returns `true` if this node has non-empty children.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether this node has children.
    pub fn has_children(&self) -> bool {
        self.try_get_children()
            .is_some_and(|children: &Vec<VirtualNode>| !children.is_empty())
    }

    /// Clones the props of this node.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - The cloned props, or `None` if this node has no props.
    pub fn try_get_props(&self) -> Option<T>
    where
        T: Clone,
    {
        match self {
            Self::Element { props, .. } => props.as_deref().cloned(),
            _ => None,
        }
    }

    /// Returns the children of this node as a virtual node.
    ///
    /// Returns `VirtualNode::Empty` when there are no children, a single child
    /// when there is exactly one, or `VirtualNode::Fragment` when there are
    /// multiple children.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - The children as a virtual node.
    pub fn try_get_child_node(&self) -> VirtualNode {
        match self.try_get_children() {
            Some(children) => match children.len() {
                0 => VirtualNode::Empty,
                1 => children.first().cloned().unwrap_or_default(),
                _ => VirtualNode::Fragment(children.clone()),
            },
            None => VirtualNode::Empty,
        }
    }
}

/// Implementation of virtual node construction for `VirtualNode<()>`.
impl VirtualNode<()> {
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
        Self::Dynamic(DynamicNode::new(inner, hook_context))
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
        Self::Dynamic(DynamicNode::new(inner, hook_context))
    }
}
