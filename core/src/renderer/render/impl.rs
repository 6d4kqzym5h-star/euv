use crate::*;

impl<T> OwnedPtr<T> {
    /// Creates a new `OwnedPtr` from a `Box::into_raw` pointer.
    pub(crate) fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }

    /// Returns the raw pointer for direct access.
    pub(crate) fn get(&self) -> *mut T {
        self.ptr
    }
}

impl<T> Drop for OwnedPtr<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(self.ptr);
            }
        }
    }
}

/// Implementation of the virtual DOM renderer.
impl Renderer {
    /// Renders the given virtual DOM tree into the real DOM.
    ///
    /// If a previous tree exists, patches the existing DOM to match the new tree.
    /// Otherwise, creates new DOM nodes from scratch and appends them to the root.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The new virtual DOM tree to render.
    pub fn render(&mut self, vnode: VirtualNode) {
        let new_unwrapped: VirtualNode = Self::unwrap_component(&vnode);
        let old_tree: Option<VirtualNode> = self.try_get_current_tree().clone();
        if let Some(old_vnode) = old_tree {
            self.patch_root(&old_vnode, &new_unwrapped);
        } else {
            while let Some(child) = self.get_root().first_child() {
                if let Some(element) = child.dyn_ref::<Element>() {
                    Self::cleanup_dom_subtree(element);
                }
                let _ = self.get_root().remove_child(&child);
            }
            let dom_node: Node = self.create_dom_node(&new_unwrapped);
            let _ = self.get_root().append_child(&dom_node);
        }
        self.set_current_tree(Some(new_unwrapped));
    }

    /// Renders the given virtual DOM tree into the real DOM by fully replacing
    /// all existing content. Used when a match arm switch occurs (e.g. route
    /// change) where incremental patching would incorrectly align unrelated
    /// child nodes from the previous arm.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The new virtual DOM tree to render.
    pub fn render_full_replace(&mut self, vnode: VirtualNode) {
        let new_unwrapped: VirtualNode = Self::unwrap_component(&vnode);
        while let Some(child) = self.get_root().first_child() {
            if let Some(element) = child.dyn_ref::<Element>() {
                Self::cleanup_dom_subtree(element);
            }
            let _ = self.get_root().remove_child(&child);
        }
        let dom_node: Node = self.create_dom_node(&new_unwrapped);
        let _ = self.get_root().append_child(&dom_node);
        self.set_current_tree(Some(new_unwrapped));
    }

    /// Patches the root DOM tree by replacing the single child of `self.root`.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The old virtual node to patch from.
    /// - `&VirtualNode` - The new virtual node to patch to.
    fn patch_root(&mut self, old_node: &VirtualNode, new_node: &VirtualNode) {
        let dom_child: Option<Node> = self.get_root().first_child();
        let is_element: bool = if let Some(ref dom_child) = dom_child {
            dom_child.dyn_ref::<Element>().is_some()
        } else {
            false
        };
        if is_element {
            if let Some(dom_child) = dom_child
                && let Ok(element) = dom_child.dyn_into::<Element>()
            {
                self.patch_node(old_node, new_node, &element);
            }
        } else if let Some(dom_child) = dom_child {
            if let Some(element) = dom_child.dyn_ref::<Element>() {
                Self::cleanup_dom_subtree(element);
            }
            let new_dom_node: Node = self.create_dom_node(new_node);
            let _ = self.get_root().replace_child(&new_dom_node, &dom_child);
        } else {
            let new_dom_node: Node = self.create_dom_node(new_node);
            let _ = self.get_root().append_child(&new_dom_node);
        }
    }

    /// Patches an existing DOM node to match the new virtual node.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The old virtual node.
    /// - `&VirtualNode` - The new virtual node.
    /// - `&Element` - The real DOM element to patch.
    fn patch_node(
        &mut self,
        old_node: &VirtualNode,
        new_node: &VirtualNode,
        dom_element: &Element,
    ) {
        match (old_node, new_node) {
            (VirtualNode::Text(old_text), VirtualNode::Text(new_text)) => {
                if old_text != new_text {
                    dom_element.set_text_content(Some(new_text.get_content()));
                }
            }
            (
                VirtualNode::Element {
                    tag: old_tag,
                    attributes: old_attrs,
                    children: old_children,
                    key: _old_key,
                    ..
                },
                VirtualNode::Element {
                    tag: new_tag,
                    attributes: new_attrs,
                    children: new_children,
                    key: _new_key,
                    ..
                },
            ) => {
                if old_tag != new_tag {
                    let new_dom_node: Node = self.create_dom_node(new_node);
                    if let Some(parent) = dom_element.parent_node() {
                        Self::cleanup_dom_subtree(dom_element);
                        let _ = parent.replace_child(&new_dom_node, dom_element);
                    }
                    return;
                }
                self.patch_children(dom_element, old_children, new_children);
                self.patch_attributes(dom_element, old_attrs, new_attrs);
            }
            (VirtualNode::Fragment(old_children), VirtualNode::Fragment(new_children)) => {
                self.patch_children(dom_element, old_children, new_children);
            }
            (VirtualNode::Dynamic(_old_dynamic), VirtualNode::Dynamic(_new_dynamic)) => {}
            (VirtualNode::Dynamic(_), _) => {
                let new_dom_node: Node = self.create_dom_node(new_node);
                if let Some(parent) = dom_element.parent_node() {
                    Self::cleanup_dom_subtree(dom_element);
                    let _ = parent.replace_child(&new_dom_node, dom_element);
                }
            }
            (_, VirtualNode::Dynamic(_)) => {
                let new_dom_node: Node = self.create_dom_node(new_node);
                if let Some(parent) = dom_element.parent_node() {
                    Self::cleanup_dom_subtree(dom_element);
                    let _ = parent.replace_child(&new_dom_node, dom_element);
                }
            }
            _ => {
                let new_dom_node: Node = self.create_dom_node(new_node);
                if let Some(parent) = dom_element.parent_node() {
                    Self::cleanup_dom_subtree(dom_element);
                    let _ = parent.replace_child(&new_dom_node, dom_element);
                }
            }
        }
    }

    /// Patches attributes of an element, adding, removing, or updating as needed.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The DOM element whose attributes to patch.
    /// - `&[AttributeEntry]` - The old attribute list.
    /// - `&[AttributeEntry]` - The new attribute list.
    fn patch_attributes(
        &mut self,
        element: &Element,
        old_attrs: &[AttributeEntry],
        new_attrs: &[AttributeEntry],
    ) {
        let old_map: HashMap<&str, &AttributeValue> = old_attrs
            .iter()
            .map(|attr: &AttributeEntry| (attr.get_name().as_str(), attr.get_value()))
            .collect();
        let new_map: HashMap<&str, ()> = new_attrs
            .iter()
            .map(|attr: &AttributeEntry| (attr.get_name().as_str(), ()))
            .collect();
        for old_attr in old_attrs {
            if !new_map.contains_key(old_attr.get_name().as_str()) {
                if let AttributeValue::Event(handler) = old_attr.get_value()
                    && let Some(euv_id_str) = element.get_attribute(DATA_EUV_ID)
                    && let Ok(euv_id) = euv_id_str.parse::<usize>()
                    && let Some(entry) =
                        ensure_handler_registry_mut().remove(&(euv_id, handler.get_event_name()))
                {
                    let slot: &mut HandlerSlot = unsafe { &mut *entry };
                    if let Some(listener_element) = slot.try_get_element().as_ref().cloned()
                        && let Some(listener_function) = slot.get_mut_listener_function().take()
                    {
                        let event_name: &str = handler.get_event_name();
                        let listener: &Function = listener_function.unchecked_ref::<Function>();
                        let _ = listener_element
                            .remove_event_listener_with_callback(event_name, listener);
                    }
                    slot.set_handler(None);
                    unsafe {
                        let _ = Box::from_raw(entry);
                    }
                }
                element.remove_attribute_or_property(old_attr.get_name());
            }
        }
        for new_attr in new_attrs {
            match new_attr.get_value() {
                AttributeValue::Event(handler) => {
                    self.attach_event_listener(element, handler);
                }
                _ => {
                    let old_value: Option<&AttributeValue> =
                        old_map.get(new_attr.get_name().as_str()).copied();
                    let should_set: bool = match old_value {
                        Some(old_val) => old_val != new_attr.get_value(),
                        None => true,
                    };
                    if should_set {
                        match new_attr.get_value() {
                            AttributeValue::Text(value) => {
                                element.set_attribute_or_property(new_attr.get_name(), value);
                            }
                            AttributeValue::Signal(signal) => {
                                let value: String = signal.get();
                                element.set_attribute_or_property(new_attr.get_name(), &value);
                            }
                            AttributeValue::Dynamic(_) => {}
                            AttributeValue::Css(css) => {
                                css.inject_style();
                                element
                                    .set_attribute_or_property(new_attr.get_name(), css.get_name());
                            }
                            AttributeValue::Event(_) => unreachable!(),
                        }
                    }
                }
            }
        }
    }

    /// Gets a child node at the given index.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The parent element.
    /// - `u32` - The child index.
    ///
    /// # Returns
    ///
    /// - `Option<Node>` - The child node at the given index, if it exists.
    fn get_child_node(parent: &Element, index: u32) -> Option<Node> {
        parent.child_nodes().get(index)
    }

    /// Patches children of an element using a keyed diff algorithm when keys
    /// are available, falling back to positional diff when no keys exist.
    ///
    /// When all children in both old and new lists have keys, this method
    /// builds a key-to-index map and applies a minimal set of DOM moves,
    /// insertions, and removals. This avoids the O(N) per-child re-patch
    /// that the naive positional algorithm incurs when items are reordered.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The parent DOM element.
    /// - `&[VirtualNode]` - The old children list.
    /// - `&[VirtualNode]` - The new children list.
    fn patch_children(
        &mut self,
        parent: &Element,
        old_children: &[VirtualNode],
        new_children: &[VirtualNode],
    ) {
        let old_has_keys: bool =
            !old_children.is_empty() && old_children.iter().all(Self::node_has_key);
        let new_has_keys: bool =
            !new_children.is_empty() && new_children.iter().all(Self::node_has_key);
        if old_has_keys && new_has_keys {
            self.patch_children_keyed(parent, old_children, new_children);
        } else {
            self.patch_children_positional(parent, old_children, new_children);
        }
    }

    /// Returns `true` if the virtual node has a non-empty key.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The node to check.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the node has a key.
    fn node_has_key(node: &VirtualNode) -> bool {
        match node {
            VirtualNode::Element { key, .. } => key.is_some(),
            _ => false,
        }
    }

    /// Extracts the key from a virtual node.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The node to extract the key from.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - The key string, if present.
    fn get_node_key(node: &VirtualNode) -> Option<&str> {
        match node {
            VirtualNode::Element { key, .. } => key.as_deref(),
            _ => None,
        }
    }

    /// Keyed diffing algorithm that minimizes DOM operations.
    ///
    /// Builds a mapping from old keys to their DOM indices, then walks the
    /// new children list. For each new child:
    ///
    /// - If its key existed in the old list, patches the existing DOM node.
    /// - Otherwise, creates a new DOM node.
    ///
    /// After processing all new children, removes any old DOM nodes whose
    /// keys are no longer present in the new list.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The parent DOM element.
    /// - `&[VirtualNode]` - The old children list.
    /// - `&[VirtualNode]` - The new children list.
    fn patch_children_keyed(
        &mut self,
        parent: &Element,
        old_children: &[VirtualNode],
        new_children: &[VirtualNode],
    ) {
        let child_nodes: NodeList = parent.child_nodes();
        let dom_child_count: u32 = child_nodes.length();
        let mut old_key_to_node: HashMap<&str, (usize, Node)> =
            HashMap::with_capacity(old_children.len());
        for (index, old_child) in old_children.iter().enumerate() {
            if let Some(key) = Self::get_node_key(old_child) {
                let dom_index: u32 = index as u32;
                if dom_index < dom_child_count
                    && let Some(node) = child_nodes.get(dom_index)
                {
                    old_key_to_node.insert(key, (index, node));
                }
            }
        }
        let mut new_key_set: HashSet<&str> = HashSet::with_capacity(new_children.len());
        for new_child in new_children.iter() {
            if let Some(key) = Self::get_node_key(new_child) {
                new_key_set.insert(key);
            }
        }
        for (index, old_child) in old_children.iter().enumerate() {
            if let Some(key) = Self::get_node_key(old_child) {
                if !new_key_set.contains(key)
                    && let Some((_old_index, dom_node)) = old_key_to_node.remove(key)
                {
                    if let Some(element) = dom_node.dyn_ref::<Element>() {
                        Self::cleanup_dom_subtree(element);
                    }
                    let _ = parent.remove_child(&dom_node);
                }
            } else {
                let dom_index: u32 = index as u32;
                if dom_index < dom_child_count
                    && let Some(dom_node) = child_nodes.get(dom_index)
                {
                    if let Some(element) = dom_node.dyn_ref::<Element>() {
                        Self::cleanup_dom_subtree(element);
                    }
                    let _ = parent.remove_child(&dom_node);
                }
            }
        }
        for (new_index, new_child) in new_children.iter().enumerate() {
            let new_key: &str = Self::get_node_key(new_child).unwrap_or_default();
            if let Some((old_vnode_index, dom_node)) = old_key_to_node.remove(new_key) {
                let old_child: &VirtualNode = &old_children[old_vnode_index];
                if let Some(element) = dom_node.dyn_ref::<Element>() {
                    self.patch_node(old_child, new_child, element);
                }
                let current_children: NodeList = parent.child_nodes();
                let target_index: u32 = new_index as u32;
                let current_at_target: Option<Node> = current_children.get(target_index);
                if current_at_target.as_ref() != Some(&dom_node) {
                    if let Some(reference_node) = current_at_target {
                        let _ = parent.insert_before(&dom_node, Some(&reference_node));
                    } else {
                        let _ = parent.append_child(&dom_node);
                    }
                }
            } else {
                let new_dom_node: Node = self.create_dom_node(new_child);
                let current_children: NodeList = parent.child_nodes();
                let target_index: u32 = new_index as u32;
                if let Some(reference_node) = current_children.get(target_index) {
                    let _ = parent.insert_before(&new_dom_node, Some(&reference_node));
                } else {
                    let _ = parent.append_child(&new_dom_node);
                }
            }
        }
    }

    /// Positional diffing algorithm (original behavior).
    ///
    /// Patches children by index position. Used as a fallback when keys
    /// are not available on all children.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The parent DOM element.
    /// - `&[VirtualNode]` - The old children list.
    /// - `&[VirtualNode]` - The new children list.
    fn patch_children_positional(
        &mut self,
        parent: &Element,
        old_children: &[VirtualNode],
        new_children: &[VirtualNode],
    ) {
        let old_len: usize = old_children.len();
        let new_len: usize = new_children.len();
        let common_len: usize = old_len.min(new_len);
        for index in 0..common_len {
            let old_child: &VirtualNode = &old_children[index];
            let new_child: &VirtualNode = &new_children[index];
            if let Some(dom_child) = Self::get_child_node(parent, index as u32) {
                if let Some(element) = dom_child.dyn_ref::<Element>() {
                    self.patch_node(old_child, new_child, element);
                } else if let (VirtualNode::Text(old_text), VirtualNode::Text(new_text)) =
                    (old_child, new_child)
                {
                    if old_text != new_text {
                        dom_child.set_text_content(Some(new_text.get_content()));
                    }
                } else {
                    let new_dom_node: Node = self.create_dom_node(new_child);
                    if let Some(parent_node) = dom_child.parent_node() {
                        if let Some(child_element) = dom_child.dyn_ref::<Element>() {
                            Self::cleanup_dom_subtree(child_element);
                        }
                        let _ = parent_node.replace_child(&new_dom_node, &dom_child);
                    }
                }
            }
        }
        if new_len > old_len {
            for new_child in new_children.iter().skip(common_len) {
                let new_dom_node: Node = self.create_dom_node(new_child);
                let _ = parent.append_child(&new_dom_node);
            }
        } else if old_len > new_len {
            for _ in common_len..old_len {
                if let Some(last_child) = parent.last_child()
                    && let Some(element) = last_child.dyn_ref::<Element>()
                {
                    Self::cleanup_dom_subtree(element);
                }
                if let Some(last_child) = parent.last_child() {
                    let _ = parent.remove_child(&last_child);
                }
            }
        }
    }

    /// Creates a real DOM node from a virtual node.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The virtual node to materialize.
    ///
    /// # Returns
    ///
    /// - `Node` - The created DOM node.
    ///
    fn create_dom_node(&mut self, node: &VirtualNode) -> Node {
        let window_value: Window = match window() {
            Some(window_instance) => window_instance,
            None => return JsValue::UNDEFINED.into(),
        };
        let document: Document = match window_value.document() {
            Some(document_instance) => document_instance,
            None => return JsValue::UNDEFINED.into(),
        };
        self.create_dom_node_with_document(node, &document)
    }

    /// Creates a real DOM node using a pre-acquired document reference.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The virtual node to materialize.
    /// - `&Document` - The document reference for creating DOM elements.
    ///
    /// # Returns
    ///
    /// - `Node` - The created DOM node.
    fn create_dom_node_with_document(&mut self, node: &VirtualNode, document: &Document) -> Node {
        match node {
            VirtualNode::Element {
                tag,
                attributes,
                children,
                ..
            } => {
                let element: Element = match tag {
                    Tag::Element(name) => match document.create_element(name) {
                        Ok(created_element) => created_element,
                        Err(_err) => return document.create_text_node(EMPTY_STRING).into(),
                    },
                    Tag::Component(_) => {
                        let unwrapped: VirtualNode = Self::unwrap_component(node);
                        return self.create_dom_node_with_document(&unwrapped, document);
                    }
                };
                for child in children {
                    let child_node: Node = self.create_dom_node_with_document(child, document);
                    let _ = element.append_child(&child_node);
                    if let VirtualNode::Text(text_node) = child
                        && let Some(signal) = text_node.try_get_signal()
                    {
                        element.track_signal_addr(signal.get_inner());
                    }
                }
                for attr in attributes {
                    match attr.get_value() {
                        AttributeValue::Text(value) => {
                            element.set_attribute_or_property(attr.get_name(), value);
                        }
                        AttributeValue::Signal(signal) => {
                            let initial_value: String = signal.get();
                            element.set_attribute_or_property(attr.get_name(), &initial_value);
                            element.track_signal_addr(signal.get_inner());
                            let attr_name: String = attr.get_name().clone();
                            let element_clone: Element = element.clone();
                            let signal_for_sub: Signal<String> = *signal;
                            let subscribe_signal: Signal<String> = signal_for_sub;
                            signal_for_sub.replace_subscribe(move || {
                                if !is_node_connected(&element_clone) {
                                    // The element has been removed from the DOM.
                                    // Do NOT call `deactivate()` here — the signal
                                    // may be a user-created signal shared with other
                                    // DynamicNodes. Deactivating it would permanently
                                    // break all other dependents. Simply return; the
                                    // listener will be cleaned up by
                                    // `clear_signal_listeners_by_addr` during
                                    // `cleanup_dom_subtree`, or replaced by
                                    // `replace_subscribe` if the element is recreated.
                                    return;
                                }
                                let new_value: String = subscribe_signal.get();
                                element_clone.set_attribute_or_property(&attr_name, &new_value);
                            });
                        }
                        AttributeValue::Event(handler) => {
                            self.attach_event_listener(&element, handler);
                        }
                        AttributeValue::Dynamic(_) => {}
                        AttributeValue::Css(css) => {
                            css.inject_style();
                            element.set_attribute_or_property(attr.get_name(), css.get_name());
                        }
                    }
                }
                element.into()
            }
            VirtualNode::Text(text_node) => {
                let text: Text = document.create_text_node(text_node.get_content());
                if let Some(signal) = text_node.try_get_signal() {
                    let text_clone: Text = text.clone();
                    let signal_for_sub: Signal<String> = *signal;
                    let subscribe_signal: Signal<String> = signal_for_sub;
                    signal_for_sub.replace_subscribe(move || {
                        if !is_node_connected(&text_clone) {
                            return;
                        }
                        let new_value: String = subscribe_signal.get();
                        text_clone.set_text_content(Some(&new_value));
                    });
                }
                text.into()
            }
            VirtualNode::Fragment(children) => {
                let fragment: Element = match document.create_element(FRAGMENT_TAG) {
                    Ok(created_element) => created_element,
                    Err(_err) => return document.create_text_node(EMPTY_STRING).into(),
                };
                let _ = fragment.set_attribute(ATTR_STYLE, FRAGMENT_STYLE);
                for child in children {
                    let child_node: Node = self.create_dom_node_with_document(child, document);
                    let _ = fragment.append_child(&child_node);
                    if let VirtualNode::Text(text_node) = child
                        && let Some(signal) = text_node.try_get_signal()
                    {
                        fragment.track_signal_addr(signal.get_inner());
                    }
                }
                fragment.into()
            }
            VirtualNode::Dynamic(dynamic_node) => {
                let placeholder: Element = match document.create_element(DYNAMIC_PLACEHOLDER_TAG) {
                    Ok(created_element) => created_element,
                    Err(_err) => return document.create_text_node(EMPTY_STRING).into(),
                };
                let _ = placeholder.set_attribute(ATTR_STYLE, DISPLAY_CONTENTS_STYLE);
                let dynamic_id: usize = Self::assign_dynamic_id(&placeholder);
                let initial_dom: Node =
                    self.setup_dynamic_node(dynamic_node, dynamic_id, &placeholder, true);
                let _ = placeholder.append_child(&initial_dom);
                placeholder.into()
            }
            VirtualNode::Empty => document.create_text_node(EMPTY_STRING).into(),
        }
    }

    /// Initializes a DynamicNode: runs the initial render, creates a sub-renderer,
    /// and registers the re-render callback in the signal update registry.
    ///
    /// Sets up dependency tracking so that signals accessed during the render
    /// function automatically register this dynamic node as a dependent,
    /// enabling precise dirty marking on subsequent signal changes.
    ///
    /// # Arguments
    ///
    /// - `&DynamicNode` - The dynamic node to set up.
    /// - `usize` - The unique dynamic ID assigned to the placeholder.
    /// - `&Element` - The placeholder DOM element.
    /// - `bool` - Whether to skip rendering if the output is unchanged.
    ///
    /// # Returns
    ///
    /// - `Node` - The initial rendered DOM node.
    fn setup_dynamic_node(
        &mut self,
        dynamic_node: &DynamicNode,
        dynamic_id: usize,
        placeholder: &Element,
        skip_equal: bool,
    ) -> Node {
        let mut hook_context: HookContext = dynamic_node.get_hook_context().clone();
        hook_context.reset_hook_index();
        CURRENT_TRACKING_DYNAMIC_ID.store(dynamic_id, Ordering::Relaxed);
        let initial_vnode: VirtualNode =
            with_hook_context(hook_context.clone(), || dynamic_node.render());
        let initial_unwrapped: VirtualNode = Self::unwrap_component(&initial_vnode);
        CURRENT_TRACKING_DYNAMIC_ID.store(usize::MAX, Ordering::Relaxed);
        let initial_dom: Node = self.create_dom_node(&initial_unwrapped);
        let render_fn_rc: Rc<UnsafeCell<RenderFnInner>> = dynamic_node.get_render_fn().clone();
        let placeholder_clone: Element = placeholder.clone();
        let mut renderer_for_sub: Self = Self::new(placeholder_clone.clone());
        renderer_for_sub.set_current_tree(Some(initial_unwrapped));
        // Wrap heap allocations in OwnedPtr so they are freed when the closure drops.
        let renderer_owned: OwnedPtr<Renderer> =
            OwnedPtr::new(Box::into_raw(Box::new(renderer_for_sub)));
        let initial_arm: usize = hook_context
            .get_inner()
            .try_borrow()
            .map(|inner: Ref<HookContextInner>| inner.get_arm_changed())
            .unwrap_or(0);
        let last_arm_owned: OwnedPtr<usize> = OwnedPtr::new(Box::into_raw(Box::new(initial_arm)));
        let callback: Box<dyn FnMut()> = Box::new(move || {
            if placeholder_clone.parent_node().is_none() {
                return;
            }
            hook_context.reset_hook_index();
            let prev_arm: usize = unsafe { *last_arm_owned.get() };
            CURRENT_TRACKING_DYNAMIC_ID.store(dynamic_id, Ordering::Relaxed);
            let new_vnode: VirtualNode = with_hook_context(hook_context.clone(), || {
                let inner: &mut RenderFnInner = unsafe { &mut *render_fn_rc.get() };
                (inner.get_mut_render_fn())()
            });
            let current_arm: usize = hook_context
                .get_inner()
                .try_borrow()
                .map(|inner: Ref<HookContextInner>| inner.get_arm_changed())
                .unwrap_or(0);
            let arm_switched: bool = prev_arm != current_arm;
            unsafe {
                *last_arm_owned.get() = current_arm;
            }
            if skip_equal && !arm_switched {
                let renderer_ref: &Renderer = unsafe { &*renderer_owned.get() };
                if let Some(old_vnode) = renderer_ref.try_get_current_tree() {
                    let new_unwrapped: VirtualNode = Self::unwrap_component(&new_vnode);
                    if Self::visual_eq(old_vnode, &new_unwrapped) {
                        CURRENT_TRACKING_DYNAMIC_ID.store(usize::MAX, Ordering::Relaxed);
                        return;
                    }
                }
            }
            let renderer_mut: &mut Renderer = unsafe { &mut *renderer_owned.get() };
            if arm_switched {
                renderer_mut.render_full_replace(new_vnode);
            } else {
                renderer_mut.render(new_vnode);
            }
            CURRENT_TRACKING_DYNAMIC_ID.store(usize::MAX, Ordering::Relaxed);
        });
        register_dynamic_listener(dynamic_id, callback);
        initial_dom
    }

    /// Recursively unwraps component nodes into their rendered output.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The virtual node to unwrap.
    ///
    /// # Returns
    ///
    /// - `VirtualNode` - The unwrapped virtual node with all components expanded.
    fn unwrap_component(node: &VirtualNode) -> VirtualNode {
        match node {
            VirtualNode::Element {
                tag: Tag::Component(_),
                children,
                ..
            } => {
                if children.len() == 1 {
                    Self::unwrap_component(&children[0])
                } else {
                    VirtualNode::Fragment(children.clone())
                }
            }
            VirtualNode::Element {
                tag,
                attributes,
                children,
                key,
                ..
            } => {
                if !children.iter().any(Self::subtree_has_component) {
                    return node.clone();
                }
                let unwrapped_children: Vec<VirtualNode> =
                    children.iter().map(Self::unwrap_component).collect();
                VirtualNode::Element {
                    tag: tag.clone(),
                    attributes: attributes.clone(),
                    children: unwrapped_children,
                    key: key.clone(),
                    props: None,
                }
            }
            VirtualNode::Fragment(children) => {
                if !children.iter().any(Self::subtree_has_component) {
                    return node.clone();
                }
                let unwrapped_children: Vec<VirtualNode> =
                    children.iter().map(Self::unwrap_component).collect();
                VirtualNode::Fragment(unwrapped_children)
            }
            other => other.clone(),
        }
    }

    /// Returns `true` if the given subtree contains any `Tag::Component` nodes
    /// that need unwrapping.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The virtual node to check.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the subtree contains a component node.
    fn subtree_has_component(node: &VirtualNode) -> bool {
        match node {
            VirtualNode::Element {
                tag: Tag::Component(_),
                ..
            } => true,
            VirtualNode::Element { children, .. } => {
                children.iter().any(Self::subtree_has_component)
            }
            VirtualNode::Fragment(children) => children.iter().any(Self::subtree_has_component),
            _ => false,
        }
    }

    /// Performs a visual equality comparison between two virtual node trees.
    ///
    /// Unlike `PartialEq`, this method recursively unwraps `VirtualNode::Dynamic`
    /// nodes by rendering their inner content and comparing the visual output.
    /// This is used by the `skip_equal` optimization in `setup_dynamic_node`
    /// to avoid unnecessary DOM patches when the rendered output is unchanged.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode` - The old virtual node.
    /// - `&VirtualNode` - The new virtual node.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the two nodes produce the same visual output.
    fn visual_eq(old_node: &VirtualNode, new_node: &VirtualNode) -> bool {
        match (old_node, new_node) {
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
                    && old_attrs.iter().zip(new_attrs.iter()).all(
                        |(old_attr, new_attr): (&AttributeEntry, &AttributeEntry)| {
                            old_attr == new_attr
                        },
                    )
                    && old_children.len() == new_children.len()
                    && old_children.iter().zip(new_children.iter()).all(
                        |(old_child, new_child): (&VirtualNode, &VirtualNode)| {
                            Self::visual_eq(old_child, new_child)
                        },
                    )
            }
            (VirtualNode::Fragment(old_children), VirtualNode::Fragment(new_children)) => {
                old_children.len() == new_children.len()
                    && old_children.iter().zip(new_children.iter()).all(
                        |(old_child, new_child): (&VirtualNode, &VirtualNode)| {
                            Self::visual_eq(old_child, new_child)
                        },
                    )
            }
            (VirtualNode::Dynamic(_), VirtualNode::Dynamic(_)) => true,
            (VirtualNode::Empty, VirtualNode::Empty) => true,
            _ => false,
        }
    }

    /// Assigns a new `data-euv-dynamic-id` to a newly created DynamicNode placeholder.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The placeholder DOM element.
    ///
    /// # Returns
    ///
    /// - `usize` - The assigned dynamic ID.
    fn assign_dynamic_id(placeholder: &Element) -> usize {
        let dynamic_id: usize = NEXT_EUV_DYNAMIC_ID.fetch_add(1, Ordering::Relaxed);
        let _ = placeholder.set_attribute(DATA_EUV_DYNAMIC_ID, &dynamic_id.to_string());
        dynamic_id
    }

    /// Recursively cleans up framework resources associated with a DOM subtree.
    ///
    /// Removes event handlers, dynamic node listeners, and signal listeners
    /// for the given element and all of its descendants.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The DOM element to clean up.
    fn cleanup_dom_subtree(element: &Element) {
        if let Some(euv_id_str) = element.get_attribute(DATA_EUV_ID)
            && let Ok(euv_id) = euv_id_str.parse::<usize>()
        {
            cleanup_element_handlers(euv_id);
        }
        if let Some(dynamic_id_str) = element.get_attribute(DATA_EUV_DYNAMIC_ID)
            && let Ok(dynamic_id) = dynamic_id_str.parse::<usize>()
        {
            cleanup_dynamic_node(dynamic_id);
        }
        if let Some(signal_addrs_str) = element.get_attribute(DATA_EUV_SIGNAL_ADDRS) {
            signal_addrs_str
                .split(CHAR_SIGNAL_ADDRS_SEPARATOR)
                .filter_map(|addr_str: &str| addr_str.parse::<usize>().ok())
                .for_each(clear_signal_listeners_by_addr);
        }
        let child_nodes: NodeList = element.child_nodes();
        let length: u32 = child_nodes.length();
        for child_index in 0..length {
            if let Some(child) = child_nodes.get(child_index)
                && let Some(child_element) = child.dyn_ref::<Element>()
            {
                Self::cleanup_dom_subtree(child_element);
            }
        }
    }

    /// Registers an event handler for a DOM element.
    ///
    /// For non-bubbling events (load, error, loadstart, etc.), attaches the
    /// listener directly on the element since global delegation on `window`
    /// cannot capture these events. For all other events, uses global event
    /// delegation via `ensure_delegated_listener`.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The DOM element to attach the handler to.
    /// - `&NativeEventHandler` - The event handler to register.
    fn attach_event_listener(&self, element: &Element, handler: &NativeEventHandler) {
        let euv_id: usize = match element.get_attribute(DATA_EUV_ID) {
            Some(id_str) => id_str
                .parse::<usize>()
                .unwrap_or_else(|_error: ParseIntError| {
                    let new_id: usize = NEXT_EUV_ID.fetch_add(1, Ordering::Relaxed);
                    let _ = element.set_attribute(DATA_EUV_ID, &new_id.to_string());
                    new_id
                }),
            None => {
                let new_id: usize = NEXT_EUV_ID.fetch_add(1, Ordering::Relaxed);
                let _ = element.set_attribute(DATA_EUV_ID, &new_id.to_string());
                new_id
            }
        };
        let event_name: &'static str = handler.get_event_name();
        if is_non_bubbling_event(event_name) {
            let key: (usize, &'static str) = (euv_id, event_name);
            let registry_ref: &mut HandlerRegistryMap = ensure_handler_registry_mut();
            if let Some(existing_entry) = registry_ref.get(&key) {
                let slot: &mut HandlerSlot = unsafe { &mut **existing_entry };
                slot.set_handler(Some(handler.clone()));
            } else {
                let closure: Closure<dyn FnMut(Event)> =
                    Closure::wrap(Box::new(move |event: Event| {
                        if let Some(entry) = ensure_handler_registry().get(&key) {
                            let slot: &HandlerSlot = unsafe { &**entry };
                            if let Some(active_handler) = slot.try_get_handler().as_ref().cloned() {
                                active_handler.handle(event);
                            }
                        }
                    }));
                let _ = element
                    .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());
                let listener_function: JsValue = closure.as_ref().clone();
                closure.forget();
                let handler_slot: HandlerEntry = Box::into_raw(Box::new(HandlerSlot::new(
                    Some(handler.clone()),
                    Some(listener_function),
                    Some(element.clone()),
                )));
                registry_ref.insert(key, handler_slot);
            }
        } else {
            ensure_delegated_listener(event_name);
            let key: (usize, &'static str) = (euv_id, event_name);
            let registry_ref: &mut HandlerRegistryMap = ensure_handler_registry_mut();
            if let Some(existing_entry) = registry_ref.get(&key) {
                let slot: &mut HandlerSlot = unsafe { &mut **existing_entry };
                slot.set_handler(Some(handler.clone()));
            } else {
                let handler_slot: HandlerEntry = Box::into_raw(Box::new(HandlerSlot::new(
                    Some(handler.clone()),
                    None,
                    None,
                )));
                registry_ref.insert(key, handler_slot);
            }
        }
    }
}
