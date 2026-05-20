use crate::*;

/// Implementation of the virtual DOM renderer.
impl Renderer {
    /// Creates a new `Renderer` targeting the given root DOM element.
    ///
    /// # Arguments
    ///
    /// - `Element` - The root DOM element to render into.
    ///
    /// # Returns
    ///
    /// - `Self` - A new renderer instance.
    pub fn new(root: Element) -> Self {
        Renderer {
            root,
            current_tree: None,
        }
    }

    /// Renders the given virtual DOM tree into the real DOM.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The virtual DOM tree to render.
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
    fn patch_node(
        &mut self,
        old_node: &VirtualNode,
        new_node: &VirtualNode,
        dom_element: &Element,
    ) {
        match (old_node, new_node) {
            (VirtualNode::Text(old_text), VirtualNode::Text(new_text)) => {
                if old_text.get_content() != new_text.get_content() {
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
                if !Self::tags_equal(old_tag, new_tag) {
                    let new_dom: Node = self.create_dom_node(new_node);
                    if let Some(parent) = dom_element.parent_node() {
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
                    parent.replace_child(&new_dom, dom_element).unwrap();
                }
            }
        }
    }

    /// Patches attributes of an element, adding, removing, or updating as needed.
    fn patch_attributes(
        &mut self,
        element: &Element,
        old_attrs: &[AttributeEntry],
        new_attrs: &[AttributeEntry],
    ) {
        for old_attr in old_attrs {
            let removed: bool = !new_attrs
                .iter()
                .any(|new_attr| new_attr.get_name() == old_attr.get_name());
            if removed {
                remove_dom_attribute_or_property(element, old_attr.get_name());
            }
        }
        for new_attr in new_attrs {
            let old_value: Option<&AttributeValue> = old_attrs
                .iter()
                .find(|old_attr| old_attr.get_name() == new_attr.get_name())
                .map(AttributeEntry::get_value);
            let should_set: bool = match old_value {
                Some(old_val) => !Self::attribute_values_equal(old_val, new_attr.get_value()),
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

    /// Compares two tags for equality.
    fn tags_equal(a: &Tag, b: &Tag) -> bool {
        match (a, b) {
            (Tag::Element(a_name), Tag::Element(b_name)) => a_name == b_name,
            (Tag::Component(a_name), Tag::Component(b_name)) => a_name == b_name,
            _ => false,
        }
    }

    /// Compares two attribute values for equality.
    fn attribute_values_equal(a: &AttributeValue, b: &AttributeValue) -> bool {
        match (a, b) {
            (AttributeValue::Text(a_val), AttributeValue::Text(b_val)) => a_val == b_val,
            (AttributeValue::Signal(_a_sig), AttributeValue::Signal(_b_sig)) => false,
            (AttributeValue::Event(_a_ev), AttributeValue::Event(_b_ev)) => false,
            (AttributeValue::Dynamic(a_dyn), AttributeValue::Dynamic(b_dyn)) => a_dyn == b_dyn,
            (AttributeValue::Css(a_css), AttributeValue::Css(b_css)) => {
                a_css.get_name() == b_css.get_name()
            }
            _ => false,
        }
    }

    /// Gets a child node at the given index by traversing child nodes.
    fn get_child_node(parent: &Element, index: u32) -> Option<Node> {
        let mut current: Option<Node> = parent.first_child();
        let mut current_index: u32 = 0;
        while let Some(node) = current {
            if current_index == index {
                return Some(node);
            }
            current = node.next_sibling();
            current_index += 1;
        }
        None
    }

    /// Patches children of an element using a positional diff algorithm.
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
                    if old_text.get_content() != new_text.get_content() {
                        dom_child.set_text_content(Some(new_text.get_content()));
                    }
                } else {
                    let new_dom: Node = self.create_dom_node(new_child);
                    if let Some(parent_node) = dom_child.parent_node() {
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
                if let Some(last_child) = parent.last_child() {
                    parent.remove_child(&last_child).unwrap();
                }
            }
        }
    }

    /// Creates a real DOM node from a virtual node.
    fn create_dom_node(&mut self, node: &VirtualNode) -> Node {
        let document: Document = window().unwrap().document().unwrap();
        self.create_dom_node_with_document(node, &document)
    }

    /// Creates a real DOM node using a pre-acquired document reference.
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
                            let attr_name: String = attr.get_name().clone();
                            let element_clone: Element = element.clone();
                            let signal_for_sub: Signal<String> = *signal;
                            let signal_inner: Signal<String> = signal_for_sub;
                            signal_for_sub.replace_subscribe(move || {
                                let new_value: String = signal_inner.get();
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
                    signal_clone.replace_subscribe({
                        let signal_inner: Signal<String> = signal_clone;
                        move || {
                            let new_value: String = signal_inner.get();
                            text_clone.set_text_content(Some(&new_value));
                        }
                    });
                }
                text.into()
            }
            VirtualNode::Fragment(children) => {
                let fragment: Element = document.create_element("div").unwrap();
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
    /// and registers the re-render closure as a `__euv_signal_update__` listener.
    fn setup_dynamic_node(
        &mut self,
        dynamic_node: &DynamicNode,
        dynamic_id: usize,
        placeholder: &Element,
        skip_equal: bool,
    ) -> Node {
        let mut hook_context: HookContext = dynamic_node.get_hook_context_value();
        hook_context.reset_hook_index();
        let initial_vnode: VirtualNode = with_hook_context(hook_context, || dynamic_node.render());
        let initial_unwrapped: VirtualNode = self.unwrap_component(&initial_vnode);
        let initial_dom: Node = self.create_dom_node(&initial_unwrapped);
        let render_fn_addr: usize = usize::from(dynamic_node);
        let placeholder_clone: Element = placeholder.clone();
        let mut renderer_for_sub: Renderer = Renderer::new(placeholder_clone.clone());
        renderer_for_sub.set_current_tree(Some(initial_unwrapped));
        let renderer_addr: usize = Box::leak(Box::new(renderer_for_sub)) as *mut Renderer as usize;
        let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            if placeholder_clone.parent_node().is_none() {
                return;
            }
            hook_context.reset_hook_index();
            let new_vnode: VirtualNode = with_hook_context(hook_context, || {
                let inner: &mut RenderFnInner = render_fn_addr.into();
                (inner.render_fn)()
            });
            if skip_equal {
                let renderer: &Renderer = renderer_addr.into();
                if let Some(old_vnode) = renderer.try_get_current_tree() {
                    let new_unwrapped: VirtualNode = Renderer::unwrap_component_static(&new_vnode);
                    if old_vnode == &new_unwrapped {
                        return;
                    }
                }
            }
            let renderer: &mut Renderer = renderer_addr.into();
            renderer.render(new_vnode);
        }));
        register_dynamic_listener(dynamic_id, closure);
        initial_dom
    }

    /// Recursively unwraps component nodes into their rendered output.
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
                let unwrapped_children: Vec<VirtualNode> = children
                    .iter()
                    .map(|child| self.unwrap_component(child))
                    .collect();
                VirtualNode::Fragment(unwrapped_children)
            }
            other => other.clone(),
        }
    }

    /// Static version of `unwrap_component` that does not require `&self`.
    ///
    /// Used inside closures where only a static method is available.
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
                let unwrapped_children: Vec<VirtualNode> =
                    children.iter().map(Self::unwrap_component_static).collect();
                VirtualNode::Fragment(unwrapped_children)
            }
            other => other.clone(),
        }
    }

    /// Assigns a new `data-euv-dynamic-id` to a newly created DynamicNode placeholder.
    fn assign_dynamic_id(placeholder: &Element) -> usize {
        let dynamic_id: usize = NEXT_EUV_DYNAMIC_ID.fetch_add(1, Ordering::Relaxed);
        let _ = placeholder.set_attribute("data-euv-dynamic-id", &dynamic_id.to_string());
        dynamic_id
    }

    /// Attaches an event listener to a DOM element.
    fn attach_event_listener(&self, element: &Element, handler: &NativeEventHandler) {
        let euv_id: usize = match element.get_attribute("data-euv-id") {
            Some(id_str) => id_str.parse::<usize>().unwrap_or_else(|_| {
                let new_id: usize = NEXT_EUV_ID.fetch_add(1, Ordering::Relaxed);
                let _ = element.set_attribute("data-euv-id", &new_id.to_string());
                new_id
            }),
            None => {
                let new_id: usize = NEXT_EUV_ID.fetch_add(1, Ordering::Relaxed);
                let _ = element.set_attribute("data-euv-id", &new_id.to_string());
                new_id
            }
        };
        let event_name: String = handler.get_event_name().clone();
        let key: (usize, String) = (euv_id, event_name.clone());
        let registry: &mut HashMap<(usize, String), HandlerEntry> = get_handler_registry();
        if let Some(existing_ptr) = registry.get(&key) {
            let existing: &mut HandlerSlot = (*existing_ptr as usize).into();
            existing.set_handler(Some(handler.clone()));
        } else {
            let handler_slot: Box<HandlerSlot> = Box::new(HandlerSlot {
                handler: Some(handler.clone()),
            });
            let handler_entry: HandlerEntry = Box::leak(handler_slot) as *mut HandlerSlot;
            let handler_addr: usize = handler_entry as usize;
            let event_name_for_closure: String = event_name.clone();
            let closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    let slot: &mut HandlerSlot = handler_addr.into();
                    let active_handler: NativeEventHandler = slot.get_handler();
                    let euv_event: NativeEvent = convert_web_event(&event, &event_name_for_closure);
                    active_handler.handle(euv_event);
                    event.stop_propagation();
                }));
            element
                .add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
            registry.insert(key, handler_entry);
        }
    }
}

/// Implementation of `From` trait for converting `usize` address into `&'static mut Renderer`.
impl From<usize> for &'static mut Renderer {
    /// Converts a memory address into a mutable reference to `Renderer`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `Renderer` instance.
    ///
    /// # Returns
    ///
    /// - `&'static mut Renderer` - A mutable reference to the `Renderer` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Renderer` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> Self {
        unsafe { &mut *(address as *mut Renderer) }
    }
}

/// Implementation of `From` trait for converting `usize` address into `&'static Renderer`.
impl From<usize> for &'static Renderer {
    /// Converts a memory address into a reference to `Renderer`.
    ///
    /// # Arguments
    ///
    /// - `usize` - The memory address of the `Renderer` instance.
    ///
    /// # Returns
    ///
    /// - `&'static Renderer` - A reference to the `Renderer` at the given address.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Renderer` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    #[inline(always)]
    fn from(address: usize) -> Self {
        unsafe { &*(address as *const Renderer) }
    }
}
