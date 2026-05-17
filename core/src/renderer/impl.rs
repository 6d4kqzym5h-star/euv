use crate::*;

/// Implementation of the virtual DOM renderer.
impl Renderer {
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
            (VirtualNode::Dynamic(new_dynamic), VirtualNode::Dynamic(_old_dynamic)) => {
                while let Some(child) = dom_element.first_child() {
                    dom_element.remove_child(&child).unwrap();
                }
                let dynamic_id: usize = Self::ensure_dynamic_id(dom_element);
                let mut hook_context: HookContext = new_dynamic.get_hook_context();
                hook_context.reset_hook_index();
                let initial_vnode: VirtualNode = with_hook_context(hook_context, || {
                    let mut borrowed: RefMut<dyn FnMut() -> VirtualNode> =
                        new_dynamic.get_render_fn().borrow_mut();
                    borrowed()
                });
                let initial_unwrapped: VirtualNode = self.unwrap_component(&initial_vnode);
                let initial_dom: Node = self.create_dom_node(&initial_unwrapped);
                dom_element.append_child(&initial_dom).unwrap();
                let render_fn_clone: Rc<RefCell<dyn FnMut() -> VirtualNode>> =
                    new_dynamic.get_render_fn().clone();
                let placeholder_clone: Element = dom_element.clone();
                let mut renderer_for_sub: Renderer = Renderer::new(placeholder_clone.clone());
                renderer_for_sub.set_current_tree(Some(initial_unwrapped));
                let renderer_ref: Rc<RefCell<Renderer>> = Rc::new(RefCell::new(renderer_for_sub));
                let renderer_ref_for_sub: Rc<RefCell<Renderer>> = Rc::clone(&renderer_ref);
                let render_fn_for_sub: Rc<RefCell<dyn FnMut() -> VirtualNode>> =
                    Rc::clone(&render_fn_clone);
                let re_render_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                    if placeholder_clone.parent_node().is_none() {
                        return;
                    }
                    hook_context.reset_hook_index();
                    let new_vnode: VirtualNode =
                        with_hook_context(hook_context, || render_fn_for_sub.borrow_mut()());
                    renderer_ref_for_sub.borrow_mut().render(new_vnode);
                }));
                register_dynamic_listener(dynamic_id, re_render_closure);
            }
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
        let mut old_map: HashMap<&str, &AttributeValue> = HashMap::new();
        for attr in old_attrs {
            old_map.insert(attr.get_name(), attr.get_value());
        }
        let mut new_map: HashMap<&str, &AttributeValue> = HashMap::new();
        for attr in new_attrs {
            new_map.insert(attr.get_name(), attr.get_value());
        }
        for name in old_map.keys() {
            if !new_map.contains_key(*name) {
                Self::remove_dom_attribute_or_property(element, name);
            }
        }
        for attr in new_attrs {
            let should_set: bool = match old_map.get(attr.get_name().as_str()) {
                Some(old_value) => !Self::attribute_values_equal(old_value, attr.get_value()),
                None => true,
            };
            if should_set {
                match attr.get_value() {
                    AttributeValue::Text(value) => {
                        if value.is_empty() {
                            Self::remove_dom_attribute_or_property(element, attr.get_name());
                        } else {
                            Self::set_dom_attribute_or_property(element, attr.get_name(), value);
                        }
                    }
                    AttributeValue::Signal(signal) => {
                        let value: String = signal.get();
                        if value.is_empty() && !Self::is_boolean_property(attr.get_name()) {
                            Self::remove_dom_attribute_or_property(element, attr.get_name());
                        } else {
                            Self::set_dom_attribute_or_property(element, attr.get_name(), &value);
                        }
                    }
                    AttributeValue::Event(handler) => {
                        self.attach_event_listener(element, handler);
                    }
                    AttributeValue::Dynamic(_) => {}
                    AttributeValue::Css(css_class) => {
                        css_class.inject_style();
                        Self::set_dom_attribute_or_property(
                            element,
                            attr.get_name(),
                            css_class.get_name(),
                        );
                    }
                }
            }
        }
    }

    /// Returns true if the given attribute name is a boolean attribute that
    /// requires DOM property-based manipulation instead of HTML attribute strings.
    fn is_boolean_property(name: &str) -> bool {
        matches!(name, "checked" | "disabled" | "selected" | "readonly")
    }

    /// Removes or clears a DOM attribute/property, depending on the attribute name.
    ///
    /// For `value`, sets the DOM property to an empty string rather than calling
    /// `remove_attribute`, because `remove_attribute("value")` only removes the
    /// HTML attribute and does not clear the displayed value of input elements.
    /// For boolean properties (`checked`, `disabled`, `selected`, `readonly`),
    /// sets the DOM property to `false` rather than calling `remove_attribute`,
    /// because `remove_attribute` on a previously-set attribute may not correctly
    /// reset the property in all browsers.
    fn remove_dom_attribute_or_property(element: &Element, name: &str) {
        if name == "value" {
            if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
                input.set_value("");
                return;
            }
            if let Some(textarea) = element.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_value("");
                return;
            }
            if let Some(select) = element.dyn_ref::<HtmlSelectElement>() {
                select.set_value("");
                return;
            }
        }
        if name == "checked"
            && let Some(input) = element.dyn_ref::<HtmlInputElement>()
        {
            input.set_checked(false);
            return;
        }
        if name == "disabled" {
            if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
                input.set_disabled(false);
                return;
            }
            if let Some(button) = element.dyn_ref::<HtmlButtonElement>() {
                button.set_disabled(false);
                return;
            }
            if let Some(select) = element.dyn_ref::<HtmlSelectElement>() {
                select.set_disabled(false);
                return;
            }
            if let Some(textarea) = element.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_disabled(false);
                return;
            }
        }
        if name == "selected"
            && let Some(option) = element.dyn_ref::<HtmlOptionElement>()
        {
            option.set_selected(false);
            return;
        }
        if name == "readonly" {
            if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
                input.set_read_only(false);
                return;
            }
            if let Some(textarea) = element.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_read_only(false);
                return;
            }
        }
        let _ = element.remove_attribute(name);
    }

    /// Sets a DOM attribute or property, depending on the attribute name.
    ///
    /// For `value`, uses the DOM property to ensure input elements update correctly.
    /// For boolean attributes (`checked`, `disabled`, `selected`, `readonly`),
    /// uses the DOM property so that the browser honors the value correctly
    /// (HTML attributes are present-or-absent, not true/false strings).
    /// For all other attributes, uses `set_attribute`.
    fn set_dom_attribute_or_property(element: &Element, name: &str, value: &str) {
        if name == "value" {
            if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
                input.set_value(value);
                return;
            }
            if let Some(textarea) = element.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_value(value);
                return;
            }
            if let Some(select) = element.dyn_ref::<HtmlSelectElement>() {
                select.set_value(value);
                return;
            }
        }
        if name == "checked"
            && let Some(input) = element.dyn_ref::<HtmlInputElement>()
        {
            input.set_checked(value == "true");
            return;
        }
        if name == "disabled" {
            if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
                input.set_disabled(value == "true");
                return;
            }
            if let Some(button) = element.dyn_ref::<HtmlButtonElement>() {
                button.set_disabled(value == "true");
                return;
            }
            if let Some(select) = element.dyn_ref::<HtmlSelectElement>() {
                select.set_disabled(value == "true");
                return;
            }
            if let Some(textarea) = element.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_disabled(value == "true");
                return;
            }
        }
        if name == "selected"
            && let Some(option) = element.dyn_ref::<HtmlOptionElement>()
        {
            option.set_selected(value == "true");
            return;
        }
        if name == "readonly" {
            if let Some(input) = element.dyn_ref::<HtmlInputElement>() {
                input.set_read_only(value == "true");
                return;
            }
            if let Some(textarea) = element.dyn_ref::<HtmlTextAreaElement>() {
                textarea.set_read_only(value == "true");
                return;
            }
        }
        let _ = element.set_attribute(name, value);
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
    ///
    /// Event attributes are always considered unequal to ensure that
    /// event listeners are re-bound on every patch. This is critical
    /// because the underlying closure may capture different signal
    /// references after a route change, even though the event name
    /// remains the same.
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
    ///
    /// For each position, patches the old child into the new child in-place.
    /// Text nodes are updated by modifying their text content rather than
    /// being replaced, which preserves any reactive signal subscriptions
    /// already wired to the existing DOM text node.
    /// Appends any extra new children, and removes any trailing old children.
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
        match node {
            VirtualNode::Element {
                tag,
                attributes,
                children,
                ..
            } => {
                let document: Document = window().unwrap().document().unwrap();
                let element: Element = match tag {
                    Tag::Element(name) => document.create_element(name).unwrap(),
                    Tag::Component(_) => {
                        let unwrapped: VirtualNode = self.unwrap_component(node);
                        return self.create_dom_node(&unwrapped);
                    }
                };
                for attr in attributes {
                    match attr.get_value() {
                        AttributeValue::Text(value) => {
                            if !value.is_empty() || Self::is_boolean_property(attr.get_name()) {
                                Self::set_dom_attribute_or_property(
                                    &element,
                                    attr.get_name(),
                                    value,
                                );
                            }
                        }
                        AttributeValue::Signal(signal) => {
                            let initial_value: String = signal.get();
                            if !initial_value.is_empty()
                                || Self::is_boolean_property(attr.get_name())
                            {
                                Self::set_dom_attribute_or_property(
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
                                if new_value.is_empty() && !Self::is_boolean_property(&attr_name) {
                                    Self::remove_dom_attribute_or_property(
                                        &element_clone,
                                        &attr_name,
                                    );
                                } else {
                                    Self::set_dom_attribute_or_property(
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
                            Self::set_dom_attribute_or_property(
                                &element,
                                attr.get_name(),
                                css_class.get_name(),
                            );
                        }
                    }
                }
                for child in children {
                    let child_node: Node = self.create_dom_node(child);
                    element.append_child(&child_node).unwrap();
                }
                element.into()
            }
            VirtualNode::Text(text_node) => {
                let document: Document = window().unwrap().document().unwrap();
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
                let document: Document = window().unwrap().document().unwrap();
                let fragment: Element = document.create_element("div").unwrap();
                for child in children {
                    let child_node: Node = self.create_dom_node(child);
                    fragment.append_child(&child_node).unwrap();
                }
                fragment.into()
            }
            VirtualNode::Dynamic(dynamic_node) => {
                let document: Document = window().unwrap().document().unwrap();
                let placeholder: Element = document.create_element("div").unwrap();
                let style: &str = "display: contents;";
                let _ = placeholder.set_attribute("style", style);
                let dynamic_id: usize = Self::assign_dynamic_id(&placeholder);
                let mut hook_context: HookContext = dynamic_node.get_hook_context();
                hook_context.reset_hook_index();
                let initial_vnode: VirtualNode = with_hook_context(hook_context, || {
                    let mut borrowed: RefMut<dyn FnMut() -> VirtualNode> =
                        dynamic_node.get_render_fn().borrow_mut();
                    borrowed()
                });
                let initial_unwrapped: VirtualNode = self.unwrap_component(&initial_vnode);
                let initial_dom: Node = self.create_dom_node(&initial_unwrapped);
                placeholder.append_child(&initial_dom).unwrap();
                let render_fn_clone: Rc<RefCell<dyn FnMut() -> VirtualNode>> =
                    dynamic_node.get_render_fn().clone();
                let placeholder_clone: Element = placeholder.clone();
                let mut renderer_for_sub: Renderer = Renderer::new(placeholder_clone.clone());
                renderer_for_sub.set_current_tree(Some(initial_unwrapped));
                let renderer_ref: Rc<RefCell<Renderer>> = Rc::new(RefCell::new(renderer_for_sub));
                let renderer_ref_for_sub: Rc<RefCell<Renderer>> = Rc::clone(&renderer_ref);
                let render_fn_for_sub: Rc<RefCell<dyn FnMut() -> VirtualNode>> =
                    Rc::clone(&render_fn_clone);
                let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                    if placeholder_clone.parent_node().is_none() {
                        return;
                    }
                    hook_context.reset_hook_index();
                    let new_vnode: VirtualNode =
                        with_hook_context(hook_context, || render_fn_for_sub.borrow_mut()());
                    {
                        let renderer: Ref<'_, Renderer> = renderer_ref_for_sub.borrow();
                        if let Some(old_vnode) = renderer.try_get_current_tree() {
                            let new_unwrapped: VirtualNode = renderer.unwrap_component(&new_vnode);
                            if old_vnode == &new_unwrapped {
                                return;
                            }
                        }
                    }
                    renderer_ref_for_sub.borrow_mut().render(new_vnode);
                }));
                register_dynamic_listener(dynamic_id, closure);
                placeholder.into()
            }
            VirtualNode::Empty => {
                let document: Document = window().unwrap().document().unwrap();
                document.create_text_node("").into()
            }
        }
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

    /// Attaches an event listener to a DOM element.
    ///
    /// Uses a global auto-incrementing ID stored as `data-euv-id` on the element
    /// to uniquely identify it in the handler registry. This avoids the bug where
    /// `element.as_ref() as *const JsValue as usize` returns the address of the
    /// Rust-side temporary `JsValue` wrapper rather than a stable JS object identity,
    /// causing different DOM elements to collide on the same key.
    ///
    /// On first attach, allocates a new ID, creates a wrapper
    /// `Rc<RefCell<Option<NativeEventHandler>>>`, and registers a DOM
    /// `addEventListener` closure that reads from it. On subsequent patches
    /// for the same element+event, only updates the wrapper content.
    /// Assigns a new `data-euv-dynamic-id` to a newly created DynamicNode placeholder.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The placeholder element to assign the ID to.
    ///
    /// # Returns
    ///
    /// - `usize` - The assigned dynamic ID.
    fn assign_dynamic_id(placeholder: &Element) -> usize {
        let dynamic_id: usize = NEXT_EUV_DYNAMIC_ID.fetch_add(1, Ordering::Relaxed);
        let _ = placeholder.set_attribute("data-euv-dynamic-id", &dynamic_id.to_string());
        dynamic_id
    }

    /// Reads or assigns the `data-euv-dynamic-id` of an existing DynamicNode placeholder.
    ///
    /// During `patch_node`, the placeholder element already exists in the DOM
    /// and may already carry a `data-euv-dynamic-id`. If it does, the existing
    /// ID is returned so the old listener can be looked up and removed. If not,
    /// a new ID is assigned.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The existing placeholder element.
    ///
    /// # Returns
    ///
    /// - `usize` - The existing or newly assigned dynamic ID.
    fn ensure_dynamic_id(dom_element: &Element) -> usize {
        match dom_element.get_attribute("data-euv-dynamic-id") {
            Some(id_str) => id_str.parse::<usize>().unwrap_or_else(|_| {
                let new_id: usize = NEXT_EUV_DYNAMIC_ID.fetch_add(1, Ordering::Relaxed);
                let _ = dom_element.set_attribute("data-euv-dynamic-id", &new_id.to_string());
                new_id
            }),
            None => {
                let new_id: usize = NEXT_EUV_DYNAMIC_ID.fetch_add(1, Ordering::Relaxed);
                let _ = dom_element.set_attribute("data-euv-dynamic-id", &new_id.to_string());
                new_id
            }
        }
    }

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
        let registry: &mut HashMap<(usize, String), Rc<RefCell<Option<NativeEventHandler>>>> =
            get_handler_registry();
        if let Some(existing_wrapper) = registry.get(&key) {
            let mut wrapper: RefMut<Option<NativeEventHandler>> = existing_wrapper.borrow_mut();
            *wrapper = Some(handler.clone());
        } else {
            let handler_wrapper: Rc<RefCell<Option<NativeEventHandler>>> =
                Rc::new(RefCell::new(Some(handler.clone())));
            let wrapper_for_closure: Rc<RefCell<Option<NativeEventHandler>>> =
                Rc::clone(&handler_wrapper);
            let event_name_for_closure: String = event_name.clone();
            let closure: Closure<dyn FnMut(Event)> =
                Closure::wrap(Box::new(move |event: Event| {
                    if let Some(active_handler) = wrapper_for_closure.borrow_mut().as_ref() {
                        let euv_event: NativeEvent =
                            convert_web_event(&event, &event_name_for_closure);
                        active_handler.handle(euv_event);
                    }
                    event.stop_propagation();
                }));
            element
                .add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
            registry.insert(key, handler_wrapper);
        }
    }
}
