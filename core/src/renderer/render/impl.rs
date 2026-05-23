use crate::*;

/// Implementation of the virtual DOM renderer.
impl Renderer {
    /// Renders the given virtual DOM tree into the real DOM.
    ///
    /// If a previous tree exists, patches the existing DOM to match the new tree.
    /// Otherwise, creates new DOM nodes from scratch and appends them to the root.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode`: The new virtual DOM tree to render.
    pub fn render(&mut self, vnode: VirtualNode) {
        let new_unwrapped: VirtualNode = self.unwrap_component(&vnode);
        if let Some(old_vnode) = self.try_get_current_tree() {
            let old_unwrapped: VirtualNode = self.unwrap_component(old_vnode);
            self.patch_root(&old_unwrapped, &new_unwrapped);
        } else {
            let dom_node: Node = self.create_dom_node(&new_unwrapped);
            while let Some(child) = self.get_root().first_child() {
                self.get_root().remove_child(&child).unwrap();
            }
            self.get_root().append_child(&dom_node).unwrap();
        }
        self.set_current_tree(Some(vnode));
    }

    /// Patches the root DOM tree by replacing the single child of `self.root`.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode`: The old virtual node to patch from.
    /// - `&VirtualNode`: The new virtual node to patch to.
    fn patch_root(&mut self, old_node: &VirtualNode, new_node: &VirtualNode) {
        let dom_child: Option<Node> = self.get_root().first_child();
        let is_element: bool = if let Some(ref dom_child) = dom_child {
            dom_child.dyn_ref::<Element>().is_some()
        } else {
            false
        };
        if is_element {
            let element: Element = dom_child.unwrap().dyn_into::<Element>().unwrap();
            self.patch_node(old_node, new_node, &element);
        } else if let Some(dom_child) = dom_child {
            let new_dom: Node = self.create_dom_node(new_node);
            self.get_root().replace_child(&new_dom, &dom_child).unwrap();
        } else {
            let new_dom: Node = self.create_dom_node(new_node);
            self.get_root().append_child(&new_dom).unwrap();
        }
    }

    /// Patches an existing DOM node to match the new virtual node.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode`: The old virtual node.
    /// - `&VirtualNode`: The new virtual node.
    /// - `&Element`: The real DOM element to patch.
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
                },
                VirtualNode::Element {
                    tag: new_tag,
                    attributes: new_attrs,
                    children: new_children,
                    key: _new_key,
                },
            ) => {
                if old_tag != new_tag {
                    let new_dom: Node = self.create_dom_node(new_node);
                    if let Some(parent) = dom_element.parent_node() {
                        self.cleanup_dom_subtree(dom_element);
                        parent.replace_child(&new_dom, dom_element).unwrap();
                    }
                    return;
                }
                self.patch_attributes(dom_element, old_attrs, new_attrs);
                self.patch_children(dom_element, old_children, new_children);
            }
            (VirtualNode::Fragment(old_children), VirtualNode::Fragment(new_children)) => {
                self.patch_children(dom_element, old_children, new_children);
            }
            (VirtualNode::Dynamic(_), VirtualNode::Dynamic(_)) => {}
            _ => {
                let new_dom: Node = self.create_dom_node(new_node);
                if let Some(parent) = dom_element.parent_node() {
                    self.cleanup_dom_subtree(dom_element);
                    parent.replace_child(&new_dom, dom_element).unwrap();
                }
            }
        }
    }

    /// Patches attributes of an element, adding, removing, or updating as needed.
    ///
    /// # Arguments
    ///
    /// - `&Element`: The DOM element whose attributes to patch.
    /// - `&[AttributeEntry]`: The old attribute list.
    /// - `&[AttributeEntry]`: The new attribute list.
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
                if let AttributeValue::Event(_) = old_attr.get_value()
                    && let Some(euv_id_str) = element.get_attribute(DATA_EUV_ID)
                    && let Ok(euv_id) = euv_id_str.parse::<usize>()
                {
                    cleanup_event_handler(euv_id, old_attr.get_name());
                }
                remove_dom_attribute_or_property(element, old_attr.get_name());
            }
        }
        for new_attr in new_attrs {
            let old_value: Option<&AttributeValue> =
                old_map.get(new_attr.get_name().as_str()).copied();
            let should_set: bool = match old_value {
                Some(old_val) => old_val != new_attr.get_value(),
                None => true,
            };
            if should_set {
                match new_attr.get_value() {
                    AttributeValue::Text(value) => {
                        if value.is_empty() {
                            remove_dom_attribute_or_property(element, new_attr.get_name());
                        } else {
                            set_dom_attribute_or_property(element, new_attr.get_name(), value);
                        }
                    }
                    AttributeValue::Signal(signal) => {
                        let value: String = signal.get();
                        if value.is_empty() && !is_boolean_property(new_attr.get_name()) {
                            remove_dom_attribute_or_property(element, new_attr.get_name());
                        } else {
                            set_dom_attribute_or_property(element, new_attr.get_name(), &value);
                        }
                    }
                    AttributeValue::Event(handler) => {
                        self.attach_event_listener(element, handler);
                    }
                    AttributeValue::Dynamic(_) => {}
                    AttributeValue::Css(css_class) => {
                        css_class.inject_style();
                        set_dom_attribute_or_property(
                            element,
                            new_attr.get_name(),
                            css_class.get_name(),
                        );
                    }
                }
            }
        }
    }

    /// Gets a child node at the given index.
    ///
    /// # Arguments
    ///
    /// - `&Element`: The parent element.
    /// - `u32`: The child index.
    ///
    /// # Returns
    ///
    /// - `Option<Node>`: The child node at the given index, if it exists.
    fn get_child_node(parent: &Element, index: u32) -> Option<Node> {
        parent.child_nodes().get(index)
    }

    /// Patches children of an element using a positional diff algorithm.
    ///
    /// # Arguments
    ///
    /// - `&Element`: The parent DOM element.
    /// - `&[VirtualNode]`: The old children list.
    /// - `&[VirtualNode]`: The new children list.
    fn patch_children(
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
                    let new_dom: Node = self.create_dom_node(new_child);
                    if let Some(parent_node) = dom_child.parent_node() {
                        if let Some(child_element) = dom_child.dyn_ref::<Element>() {
                            self.cleanup_dom_subtree(child_element);
                        }
                        let _ = parent_node.replace_child(&new_dom, &dom_child);
                    }
                }
            }
        }
        if new_len > old_len {
            for new_child in new_children.iter().skip(common_len) {
                let new_dom: Node = self.create_dom_node(new_child);
                parent.append_child(&new_dom).unwrap();
            }
        } else if old_len > new_len {
            for _ in common_len..old_len {
                if let Some(last_child) = parent.last_child()
                    && let Some(element) = last_child.dyn_ref::<Element>()
                {
                    self.cleanup_dom_subtree(element);
                }
                if let Some(last_child) = parent.last_child() {
                    parent.remove_child(&last_child).unwrap();
                }
            }
        }
    }

    /// Creates a real DOM node from a virtual node.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode`: The virtual node to materialize.
    ///
    /// # Returns
    ///
    /// - `Node`: The created DOM node.
    ///
    /// # Panics
    ///
    /// Panics if `window()` or `document()` is unavailable.
    fn create_dom_node(&mut self, node: &VirtualNode) -> Node {
        let document: Document = window().unwrap().document().unwrap();
        self.create_dom_node_with_document(node, &document)
    }

    /// Creates a real DOM node using a pre-acquired document reference.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode`: The virtual node to materialize.
    /// - `&Document`: The document reference for creating DOM elements.
    ///
    /// # Returns
    ///
    /// - `Node`: The created DOM node.
    fn create_dom_node_with_document(&mut self, node: &VirtualNode, document: &Document) -> Node {
        match node {
            VirtualNode::Element {
                tag,
                attributes,
                children,
                ..
            } => {
                let element: Element = match tag {
                    Tag::Element(name) => document.create_element(name).unwrap(),
                    Tag::Component(_) => {
                        let unwrapped: VirtualNode = self.unwrap_component(node);
                        return self.create_dom_node_with_document(&unwrapped, document);
                    }
                };
                for attr in attributes {
                    match attr.get_value() {
                        AttributeValue::Text(value) => {
                            if !value.is_empty() || is_boolean_property(attr.get_name()) {
                                set_dom_attribute_or_property(&element, attr.get_name(), value);
                            }
                        }
                        AttributeValue::Signal(signal) => {
                            let initial_value: String = signal.get();
                            if !initial_value.is_empty() || is_boolean_property(attr.get_name()) {
                                set_dom_attribute_or_property(
                                    &element,
                                    attr.get_name(),
                                    &initial_value,
                                );
                            }
                            let signal_addr: usize = signal.get_inner_addr();
                            let existing_addrs: String = element
                                .get_attribute("data-euv-signal-addrs")
                                .unwrap_or_default();
                            let updated_addrs: String = if existing_addrs.is_empty() {
                                signal_addr.to_string()
                            } else {
                                format!("{},{}", existing_addrs, signal_addr)
                            };
                            let _ = element.set_attribute("data-euv-signal-addrs", &updated_addrs);
                            let attr_name: String = attr.get_name().clone();
                            let element_clone: Element = element.clone();
                            let signal_for_sub: Signal<String> = *signal;
                            let sub_signal: Signal<String> = signal_for_sub;
                            signal_for_sub.replace_subscribe(move || {
                                if !is_node_connected(&element_clone) {
                                    sub_signal.clear_listeners();
                                    return;
                                }
                                let new_value: String = sub_signal.get();
                                if new_value.is_empty() && !is_boolean_property(&attr_name) {
                                    remove_dom_attribute_or_property(&element_clone, &attr_name);
                                } else {
                                    set_dom_attribute_or_property(
                                        &element_clone,
                                        &attr_name,
                                        &new_value,
                                    );
                                }
                            });
                        }
                        AttributeValue::Event(handler) => {
                            self.attach_event_listener(&element, handler);
                        }
                        AttributeValue::Dynamic(_) => {}
                        AttributeValue::Css(css_class) => {
                            css_class.inject_style();
                            set_dom_attribute_or_property(
                                &element,
                                attr.get_name(),
                                css_class.get_name(),
                            );
                        }
                    }
                }
                for child in children {
                    let child_node: Node = self.create_dom_node_with_document(child, document);
                    element.append_child(&child_node).unwrap();
                }
                element.into()
            }
            VirtualNode::Text(text_node) => {
                let text: Text = document.create_text_node(text_node.get_content());
                if let Some(signal) = text_node.try_get_signal() {
                    let text_clone: Text = text.clone();
                    let signal_clone: Signal<String> = *signal;
                    let sub_signal: Signal<String> = signal_clone;
                    signal_clone.replace_subscribe({
                        move || {
                            if !is_node_connected(&text_clone) {
                                sub_signal.clear_listeners();
                                return;
                            }
                            let new_value: String = sub_signal.get();
                            text_clone.set_text_content(Some(&new_value));
                        }
                    });
                }
                text.into()
            }
            VirtualNode::Fragment(children) => {
                let fragment: Element = document.create_element("slot").unwrap();
                let _ = fragment.set_attribute("style", "display:contents");
                for child in children {
                    let child_node: Node = self.create_dom_node_with_document(child, document);
                    fragment.append_child(&child_node).unwrap();
                }
                fragment.into()
            }
            VirtualNode::Dynamic(dynamic_node) => {
                let placeholder: Element = document.create_element("div").unwrap();
                let style: &str = "display: contents;";
                let _ = placeholder.set_attribute("style", style);
                let dynamic_id: usize = Self::assign_dynamic_id(&placeholder);
                let initial_dom: Node =
                    self.setup_dynamic_node(dynamic_node, dynamic_id, &placeholder, true);
                placeholder.append_child(&initial_dom).unwrap();
                placeholder.into()
            }
            VirtualNode::Empty => document.create_text_node("").into(),
        }
    }

    /// Initializes a DynamicNode: runs the initial render, creates a sub-renderer,
    /// and registers the re-render callback in the signal update registry.
    ///
    /// # Arguments
    ///
    /// - `&DynamicNode`: The dynamic node to set up.
    /// - `usize`: The unique dynamic ID assigned to the placeholder.
    /// - `&Element`: The placeholder DOM element.
    /// - `bool`: Whether to skip rendering if the output is unchanged.
    ///
    /// # Returns
    ///
    /// - `Node`: The initial rendered DOM node.
    fn setup_dynamic_node(
        &mut self,
        dynamic_node: &DynamicNode,
        dynamic_id: usize,
        placeholder: &Element,
        skip_equal: bool,
    ) -> Node {
        let mut hook_context: HookContext = dynamic_node.get_hook_context_value();
        hook_context.reset_hook_index();
        let initial_vnode: VirtualNode =
            with_hook_context(hook_context.clone(), || dynamic_node.render());
        let initial_unwrapped: VirtualNode = self.unwrap_component(&initial_vnode);
        let initial_dom: Node = self.create_dom_node(&initial_unwrapped);
        let render_fn: Rc<RefCell<RenderFnInner>> = dynamic_node.get_render_fn().clone();
        let placeholder_clone: Element = placeholder.clone();
        let mut renderer_for_sub: Renderer = Renderer::new(placeholder_clone.clone());
        renderer_for_sub.set_current_tree(Some(initial_unwrapped));
        let renderer_rc: Rc<RefCell<Renderer>> = Rc::new(RefCell::new(renderer_for_sub));
        let callback: Box<dyn FnMut()> = Box::new(move || {
            if placeholder_clone.parent_node().is_none() {
                return;
            }
            hook_context.reset_hook_index();
            let new_vnode: VirtualNode = with_hook_context(hook_context.clone(), || {
                let mut inner: std::cell::RefMut<RenderFnInner> = render_fn.borrow_mut();
                (inner.get_mut_render_fn())()
            });
            if skip_equal {
                let renderer_ref: std::cell::Ref<Renderer> = renderer_rc.borrow();
                if let Some(old_vnode) = renderer_ref.try_get_current_tree() {
                    let new_unwrapped: VirtualNode = Renderer::unwrap_component_static(&new_vnode);
                    if old_vnode == &new_unwrapped {
                        return;
                    }
                }
            }
            let mut renderer_mut: std::cell::RefMut<Renderer> = renderer_rc.borrow_mut();
            renderer_mut.render(new_vnode);
        });
        register_dynamic_listener(dynamic_id, callback);
        initial_dom
    }

    /// Recursively unwraps component nodes into their rendered output.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode`: The virtual node to unwrap.
    ///
    /// # Returns
    ///
    /// - `VirtualNode`: The unwrapped virtual node with all components expanded.
    fn unwrap_component(&self, node: &VirtualNode) -> VirtualNode {
        match node {
            VirtualNode::Element {
                tag: Tag::Component(_),
                children,
                ..
            } => {
                if children.len() == 1 {
                    self.unwrap_component(&children[0])
                } else {
                    VirtualNode::Fragment(children.clone())
                }
            }
            VirtualNode::Element {
                tag,
                attributes,
                children,
                key,
            } => {
                if !children.iter().any(Self::subtree_has_component) {
                    return node.clone();
                }
                let unwrapped_children: Vec<VirtualNode> = children
                    .iter()
                    .map(|child| self.unwrap_component(child))
                    .collect();
                VirtualNode::Element {
                    tag: tag.clone(),
                    attributes: attributes.clone(),
                    children: unwrapped_children,
                    key: key.clone(),
                }
            }
            VirtualNode::Fragment(children) => {
                if !children.iter().any(Self::subtree_has_component) {
                    return node.clone();
                }
                let unwrapped_children: Vec<VirtualNode> = children
                    .iter()
                    .map(|child| self.unwrap_component(child))
                    .collect();
                VirtualNode::Fragment(unwrapped_children)
            }
            other => other.clone(),
        }
    }

    /// Static version of `unwrap_component`.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode`: The virtual node to unwrap.
    ///
    /// # Returns
    ///
    /// - `VirtualNode`: The unwrapped virtual node with all components expanded.
    fn unwrap_component_static(node: &VirtualNode) -> VirtualNode {
        match node {
            VirtualNode::Element {
                tag: Tag::Component(_),
                children,
                ..
            } => {
                if children.len() == 1 {
                    Self::unwrap_component_static(&children[0])
                } else {
                    VirtualNode::Fragment(children.clone())
                }
            }
            VirtualNode::Element {
                tag,
                attributes,
                children,
                key,
            } => {
                if !children.iter().any(Self::subtree_has_component) {
                    return node.clone();
                }
                let unwrapped_children: Vec<VirtualNode> =
                    children.iter().map(Self::unwrap_component_static).collect();
                VirtualNode::Element {
                    tag: tag.clone(),
                    attributes: attributes.clone(),
                    children: unwrapped_children,
                    key: key.clone(),
                }
            }
            VirtualNode::Fragment(children) => {
                if !children.iter().any(Self::subtree_has_component) {
                    return node.clone();
                }
                let unwrapped_children: Vec<VirtualNode> =
                    children.iter().map(Self::unwrap_component_static).collect();
                VirtualNode::Fragment(unwrapped_children)
            }
            other => other.clone(),
        }
    }

    /// Returns `true` if the given subtree contains any `Tag::Component` nodes.
    ///
    /// # Arguments
    ///
    /// - `&VirtualNode`: The virtual node to check.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the subtree contains a component node.
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

    /// Assigns a new `data-euv-dynamic-id` to a newly created DynamicNode placeholder.
    ///
    /// # Arguments
    ///
    /// - `&Element`: The placeholder DOM element.
    ///
    /// # Returns
    ///
    /// - `usize`: The assigned dynamic ID.
    fn assign_dynamic_id(placeholder: &Element) -> usize {
        let dynamic_id: usize = NEXT_EUV_DYNAMIC_ID.fetch_add(1, Ordering::Relaxed);
        let _ = placeholder.set_attribute("data-euv-dynamic-id", &dynamic_id.to_string());
        dynamic_id
    }

    /// Recursively cleans up framework resources associated with a DOM subtree.
    ///
    /// Removes event handlers, dynamic node listeners, and signal listeners
    /// for the given element and all of its descendants.
    ///
    /// # Arguments
    ///
    /// - `&Element`: The DOM element to clean up.
    fn cleanup_dom_subtree(&self, element: &Element) {
        if let Some(euv_id_str) = element.get_attribute(DATA_EUV_ID)
            && let Ok(euv_id) = euv_id_str.parse::<usize>()
        {
            cleanup_element_handlers(euv_id);
        }
        if let Some(dynamic_id_str) = element.get_attribute("data-euv-dynamic-id")
            && let Ok(dynamic_id) = dynamic_id_str.parse::<usize>()
        {
            cleanup_dynamic_node(dynamic_id);
        }
        if let Some(signal_addrs_str) = element.get_attribute("data-euv-signal-addrs") {
            for addr_str in signal_addrs_str.split(',') {
                if let Ok(addr) = addr_str.parse::<usize>() {
                    clear_signal_listeners_by_addr(addr);
                }
            }
        }
        let child_nodes: NodeList = element.child_nodes();
        let length: u32 = child_nodes.length();
        for i in 0..length {
            if let Some(child) = child_nodes.get(i) {
                if let Some(child_element) = child.dyn_ref::<Element>() {
                    self.cleanup_dom_subtree(child_element);
                } else if let Some(text) = child.dyn_ref::<Text>() {
                    cleanup_text_signal_listeners(text);
                }
            }
        }
    }

    /// Registers an event handler for a DOM element using global event delegation.
    ///
    /// # Arguments
    ///
    /// - `&Element`: The DOM element to attach the handler to.
    /// - `&NativeEventHandler`: The event handler to register.
    fn attach_event_listener(&self, element: &Element, handler: &NativeEventHandler) {
        let euv_id: usize = match element.get_attribute(DATA_EUV_ID) {
            Some(id_str) => id_str.parse::<usize>().unwrap_or_else(|_| {
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
        let event_name: Cow<'static, str> = handler.get_event_name().clone();
        if !NativeEventName::DELEGATABLE_EVENT_NAMES.contains(&&*event_name) {
            ensure_delegated_listener(event_name.clone());
        }
        let key: (usize, Cow<'static, str>) = (euv_id, event_name);
        let registry_ref: &mut HashMap<(usize, Cow<'static, str>), HandlerEntry> =
            ensure_handler_registry_mut();
        if let Some(existing_entry) = registry_ref.get(&key) {
            let mut slot: std::cell::RefMut<HandlerSlot> = existing_entry.borrow_mut();
            slot.set_handler(Some(handler.clone()));
        } else {
            let handler_slot: HandlerEntry =
                Rc::new(RefCell::new(HandlerSlot::new(Some(handler.clone()))));
            registry_ref.insert(key, handler_slot);
        }
    }
}
