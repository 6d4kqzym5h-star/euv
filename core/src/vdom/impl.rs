use crate::*;

/// Visual equality comparison for attribute values.
///
/// Compares values by their visual output rather than identity. `Signal`
/// values are compared by their current resolved string, `Event` values
/// are always considered equal (re-binding is handled by the handler
/// registry), and `CssClass` values are compared by class name.
impl PartialEq for AttributeValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AttributeValue::Text(a_val), AttributeValue::Text(b_val)) => a_val == b_val,
            (AttributeValue::Signal(a_sig), AttributeValue::Signal(b_sig)) => {
                a_sig.get() == b_sig.get()
            }
            (AttributeValue::Signal(a_sig), AttributeValue::Text(b_val)) => a_sig.get() == *b_val,
            (AttributeValue::Text(a_val), AttributeValue::Signal(b_sig)) => *a_val == b_sig.get(),
            (AttributeValue::Event(_), AttributeValue::Event(_)) => true,
            (AttributeValue::Css(a_css), AttributeValue::Css(b_css)) => {
                a_css.get_name() == b_css.get_name()
            }
            (AttributeValue::Dynamic(a_dyn), AttributeValue::Dynamic(b_dyn)) => a_dyn == b_dyn,
            _ => false,
        }
    }
}

/// Visual equality comparison for attribute entries.
///
/// Two attribute entries are equal when their names match and their values
/// are visually equal as defined by `AttributeValue::eq`.
impl PartialEq for AttributeEntry {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name() && self.get_value() == other.get_value()
    }
}

/// Visual equality comparison for text nodes.
///
/// Only compares the text content; the backing signal is not considered
/// because it does not affect visual output.
impl PartialEq for TextNode {
    fn eq(&self, other: &Self) -> bool {
        self.get_content() == other.get_content()
    }
}

/// Visual equality comparison for CSS classes.
///
/// Two CSS classes are considered equal when their class names match,
/// since the name uniquely identifies the visual style rule.
impl PartialEq for CssClass {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
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
            (VirtualNode::Text(a_text), VirtualNode::Text(b_text)) => a_text == b_text,
            (
                VirtualNode::Element {
                    tag: a_tag,
                    attributes: a_attrs,
                    children: a_children,
                    ..
                },
                VirtualNode::Element {
                    tag: b_tag,
                    attributes: b_attrs,
                    children: b_children,
                    ..
                },
            ) => {
                a_tag == b_tag
                    && a_attrs.len() == b_attrs.len()
                    && a_attrs.iter().zip(b_attrs.iter()).all(|(a, b)| a == b)
                    && a_children.len() == b_children.len()
                    && a_children
                        .iter()
                        .zip(b_children.iter())
                        .all(|(a, b)| a == b)
            }
            (VirtualNode::Fragment(a_children), VirtualNode::Fragment(b_children)) => {
                a_children.len() == b_children.len()
                    && a_children
                        .iter()
                        .zip(b_children.iter())
                        .all(|(a, b)| a == b)
            }
            (VirtualNode::Dynamic(_), VirtualNode::Dynamic(_)) => false,
            (VirtualNode::Empty, VirtualNode::Empty) => true,
            _ => false,
        }
    }
}

/// Maps each `Attribute` variant to its corresponding DOM attribute string.
impl Attribute {
    /// Returns the string representation of this attribute name for DOM binding.
    ///
    /// Static variants return `Cow::Borrowed` (zero allocation), while
    /// `Data` and `Other` variants return `Cow::Owned` (heap allocation).
    ///
    /// # Returns
    ///
    /// - `Cow<'static, str>` - The attribute name as a static or owned string.
    pub fn as_str(&self) -> Cow<'static, str> {
        match self {
            Attribute::AccessKey => Cow::Borrowed("accesskey"),
            Attribute::Action => Cow::Borrowed("action"),
            Attribute::Alt => Cow::Borrowed("alt"),
            Attribute::AriaLabel => Cow::Borrowed("aria-label"),
            Attribute::AutoComplete => Cow::Borrowed("autocomplete"),
            Attribute::AutoFocus => Cow::Borrowed("autofocus"),
            Attribute::Checked => Cow::Borrowed("checked"),
            Attribute::Class => Cow::Borrowed("class"),
            Attribute::Cols => Cow::Borrowed("cols"),
            Attribute::ContentEditable => Cow::Borrowed("contenteditable"),
            Attribute::Data(name) => Cow::Owned(format!("data-{}", name)),
            Attribute::Dir => Cow::Borrowed("dir"),
            Attribute::Disabled => Cow::Borrowed("disabled"),
            Attribute::Draggable => Cow::Borrowed("draggable"),
            Attribute::EncType => Cow::Borrowed("enctype"),
            Attribute::For => Cow::Borrowed("for"),
            Attribute::Form => Cow::Borrowed("form"),
            Attribute::Height => Cow::Borrowed("height"),
            Attribute::Hidden => Cow::Borrowed("hidden"),
            Attribute::Href => Cow::Borrowed("href"),
            Attribute::Id => Cow::Borrowed("id"),
            Attribute::Lang => Cow::Borrowed("lang"),
            Attribute::Max => Cow::Borrowed("max"),
            Attribute::MaxLength => Cow::Borrowed("maxlength"),
            Attribute::Method => Cow::Borrowed("method"),
            Attribute::Min => Cow::Borrowed("min"),
            Attribute::MinLength => Cow::Borrowed("minlength"),
            Attribute::Multiple => Cow::Borrowed("multiple"),
            Attribute::Name => Cow::Borrowed("name"),
            Attribute::Pattern => Cow::Borrowed("pattern"),
            Attribute::Placeholder => Cow::Borrowed("placeholder"),
            Attribute::ReadOnly => Cow::Borrowed("readonly"),
            Attribute::Required => Cow::Borrowed("required"),
            Attribute::Rows => Cow::Borrowed("rows"),
            Attribute::Selected => Cow::Borrowed("selected"),
            Attribute::Size => Cow::Borrowed("size"),
            Attribute::SpellCheck => Cow::Borrowed("spellcheck"),
            Attribute::Src => Cow::Borrowed("src"),
            Attribute::Step => Cow::Borrowed("step"),
            Attribute::Style => Cow::Borrowed("style"),
            Attribute::TabIndex => Cow::Borrowed("tabindex"),
            Attribute::Target => Cow::Borrowed("target"),
            Attribute::Title => Cow::Borrowed("title"),
            Attribute::Type => Cow::Borrowed("type"),
            Attribute::Value => Cow::Borrowed("value"),
            Attribute::Width => Cow::Borrowed("width"),
            Attribute::Other(name) => Cow::Owned(name.clone()),
        }
    }
}

/// Provides a default empty dynamic node with a no-op render function.
impl Default for DynamicNode {
    fn default() -> Self {
        let node: DynamicNode = DynamicNode {
            render_fn: Rc::new(RefCell::new(|| VirtualNode::Empty)),
            hook_context: HookContext::default(),
        };
        node
    }
}

/// Clones a `DynamicNode` by cloning its `HookContext` (Copy) and `render_fn` (Rc).
impl Clone for DynamicNode {
    fn clone(&self) -> Self {
        DynamicNode {
            render_fn: Rc::clone(self.get_render_fn()),
            hook_context: self.hook_context,
        }
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
            (VirtualNode::Dynamic(_), VirtualNode::Dynamic(_)) => true,
            (VirtualNode::Empty, VirtualNode::Empty) => false,
            _ => true,
        }
    }

    /// Creates a new element node with the given tag name.
    pub fn get_element_node(tag_name: &str) -> Self {
        VirtualNode::Element {
            tag: Tag::Element(tag_name.to_string()),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
        }
    }

    /// Creates a new text node with the given content.
    pub fn get_text_node(content: &str) -> Self {
        VirtualNode::Text(TextNode::new(content.to_string(), None))
    }

    /// Adds an attribute to this node if it is an element.
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
    pub fn try_get_prop(&self, name: &Attribute) -> Option<String> {
        let name_str: Cow<'static, str> = name.as_str();
        if let VirtualNode::Element { attributes, .. } = self {
            for attr in attributes {
                if attr.get_name() == &name_str {
                    match attr.get_value() {
                        AttributeValue::Text(value) => return Some(value.clone()),
                        AttributeValue::Signal(signal) => return Some(signal.get()),
                        _ => {}
                    }
                }
            }
        }
        None
    }

    /// Extracts a signal property from this node if it is an element with the named attribute.
    ///
    /// Returns the raw `Signal<String>` so components can reactively read the current value
    /// and subscribe to future changes, rather than receiving a snapshot string.
    pub fn try_get_signal_prop(&self, name: &Attribute) -> Option<Signal<String>> {
        let name_str: Cow<'static, str> = name.as_str();
        if let VirtualNode::Element { attributes, .. } = self {
            for attr in attributes {
                if attr.get_name() == &name_str
                    && let AttributeValue::Signal(signal) = attr.get_value()
                {
                    return Some(*signal);
                }
            }
        }
        None
    }

    /// Extracts children from this node if it is an element.
    pub fn get_children(&self) -> Vec<VirtualNode> {
        if let VirtualNode::Element { children, .. } = self {
            children.clone()
        } else {
            Vec::new()
        }
    }

    /// Extracts text content from this node.
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
    pub fn try_get_event(
        &self,
        name: &NativeEventName,
    ) -> Option<crate::event::NativeEventHandler> {
        let name_str: Cow<'static, str> = name.as_str();
        if let VirtualNode::Element { attributes, .. } = self {
            for attr in attributes {
                if attr.get_name() == &name_str
                    && let AttributeValue::Event(handler) = attr.get_value()
                {
                    return Some(handler.clone());
                }
            }
        }
        None
    }

    /// Extracts an event handler from this node by a custom attribute name.
    pub fn try_get_callback(&self, name: &str) -> Option<crate::event::NativeEventHandler> {
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
