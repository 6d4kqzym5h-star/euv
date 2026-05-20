use crate::*;

/// Sets a DOM attribute or property on the given element.
///
/// # Arguments
///
/// - `&Element` - The DOM element to modify.
/// - `&str` - The attribute or property name.
/// - `&str` - The value to set.
pub fn set_dom_attr(element: &Element, name: &str, value: &str) {
    set_dom_attribute_or_property(element, name, value);
}

/// Removes a DOM attribute or property from the given element.
///
/// # Arguments
///
/// - `&Element` - The DOM element to modify.
/// - `&str` - The attribute or property name to remove.
pub fn remove_dom_attr(element: &Element, name: &str) {
    remove_dom_attribute_or_property(element, name);
}

/// Creates an empty text node used as an anchor for dynamic view insertion.
///
/// # Returns
///
/// - `Node` - An empty text node.
pub fn create_anchor() -> Node {
    let document: Document = window().unwrap().document().unwrap();
    document.create_text_node("").into()
}

/// Appends a child view into a parent element.
///
/// # Arguments
///
/// - `&Element` - The parent DOM element.
/// - `View` - The child view to append.
pub fn append_view(parent: &Element, child: View) {
    child.append_to(parent);
}

/// Binds a `Signal<String>` to a DOM attribute, updating it reactively.
///
/// Sets the initial value immediately and subscribes to signal changes.
/// Empty values remove the attribute unless it is a boolean property.
///
/// # Arguments
///
/// - `&Element` - The DOM element.
/// - `&str` - The attribute name.
/// - `Signal<String>` - The reactive string signal.
pub fn bind_attr_signal(element: &Element, attr_name: &str, signal: Signal<String>) {
    let initial: String = signal.get_untracked();
    if !initial.is_empty() || is_boolean_property(attr_name) {
        set_dom_attr(element, attr_name, &initial);
    }
    let element_clone: Element = element.clone();
    let attr_name_owned: String = attr_name.to_string();
    signal.replace_subscribe(move || {
        let new_value: String = signal.get_untracked();
        if new_value.is_empty() && !is_boolean_property(&attr_name_owned) {
            remove_dom_attr(&element_clone, &attr_name_owned);
        } else {
            set_dom_attr(&element_clone, &attr_name_owned, &new_value);
        }
    });
}

/// Binds a computed effect to a DOM attribute, re-evaluating reactively.
///
/// Creates a render effect that re-computes the attribute value whenever
/// tracked signals change. The effect is automatically disposed when the
/// current hook context is cleaned up.
///
/// # Arguments
///
/// - `&Element` - The DOM element.
/// - `&str` - The attribute name.
/// - `F` - A closure that computes the attribute value.
pub fn bind_attr_effect<F>(element: &Element, attr_name: &str, compute: F)
where
    F: FnMut() -> String + 'static,
{
    let element_clone: Element = element.clone();
    let attr_name_owned: String = attr_name.to_string();
    let mut compute_fn: F = compute;
    let effect: RenderEffect = create_render_effect(move || {
        let new_value: String = compute_fn();
        if new_value.is_empty() && !is_boolean_property(&attr_name_owned) {
            remove_dom_attr(&element_clone, &attr_name_owned);
        } else {
            set_dom_attr(&element_clone, &attr_name_owned, &new_value);
        }
    });
    let ctx: HookContext = get_current_hook_context();
    ctx.leak_mut().get_mut_cleanups().push(Box::new(move || {
        effect.dispose();
    }));
}

/// Attaches an event handler to a DOM element using a global registry.
///
/// If a handler for the same element and event type already exists,
/// it is replaced in place. Otherwise a new `Closure` is created and
/// registered via `addEventListener`.
///
/// # Arguments
///
/// - `&Element` - The DOM element.
/// - `NativeEventHandler` - The event handler to attach.
pub fn attach_event(element: &Element, handler: NativeEventHandler) {
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
    if let Some(existing_addr) = registry.get(&key) {
        let existing: &mut HandlerSlot = HandlerSlot::from_addr(*existing_addr);
        existing.set_handler(Some(handler));
    } else {
        let handler_slot: Box<HandlerSlot> = Box::new(HandlerSlot {
            handler: Some(handler.clone()),
        });
        let leaked: &mut HandlerSlot = Box::leak(handler_slot);
        let handler_addr: usize = leaked as *mut HandlerSlot as usize;
        let handler_entry: HandlerEntry = handler_addr;
        let event_name_for_closure: String = event_name.clone();
        let closure: Closure<dyn FnMut(Event)> = Closure::wrap(Box::new(move |event: Event| {
            pause_effect_notifications();
            let slot: &mut HandlerSlot = HandlerSlot::from_addr(handler_addr);
            let active_handler: NativeEventHandler = slot.get_handler();
                let euv_event: NativeEvent = convert_web_event(&event, &event_name_for_closure);
                let prevent_requested: Rc<Cell<bool>> = euv_event.get_prevent_default_requested().clone();
                active_handler.handle(euv_event);
                if prevent_requested.get() {
                    event.prevent_default();
                }
            }
            resume_effect_notifications();
        }));
        element
            .add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
        registry.insert(key, handler_entry);
    }
}

/// Injects the CSS class styles and sets the class attribute on the element.
///
/// # Arguments
///
/// - `&Element` - The DOM element.
/// - `&str` - The attribute name (typically `"class"`).
/// - `&CssClass` - The CSS class to apply.
pub fn apply_css_class(element: &Element, attr_name: &str, css_class: &CssClass) {
    css_class.inject_style();
    set_dom_attr(element, attr_name, css_class.get_name());
}

/// Creates a conditional view that re-renders when tracked signals change.
///
/// Uses a render effect to call `render_fn` whenever dependencies update.
/// The previous view is removed and the new one is inserted at the anchor.
///
/// # Arguments
///
/// - `F` - A closure that produces a new `View` each time it is called.
///
/// # Returns
///
/// - `View` - A view containing the anchor and the conditional content.
pub fn create_conditional_view<F>(mut render_fn: F) -> View
where
    F: FnMut() -> View + 'static,
{
    let anchor_node: Node = create_anchor();
    let shared_view: Rc<RefCell<Option<View>>> = Rc::new(RefCell::new(None));
    let shared_view_clone: Rc<RefCell<Option<View>>> = Rc::clone(&shared_view);
    let anchor_for_insert: Node = anchor_node.clone();
    let initial_view_nodes: Rc<RefCell<Option<View>>> = Rc::new(RefCell::new(None));
    let initial_view_nodes_clone: Rc<RefCell<Option<View>>> = Rc::clone(&initial_view_nodes);
    let mut first_run: bool = true;
    let effect: RenderEffect = create_render_effect(move || {
        if !first_run && anchor_for_insert.parent_node().is_none() {
            return;
        }
        first_run = false;
        let new_view: View = render_fn();
        if let Some(ref mut old_view) = shared_view_clone.borrow_mut().as_mut() {
            old_view.remove();
        }
        if anchor_for_insert.parent_node().is_some() {
            new_view.insert_before_ref(&anchor_for_insert);
        } else {
            *initial_view_nodes_clone.borrow_mut() = Some(new_view.node_refs());
        }
        *shared_view_clone.borrow_mut() = Some(new_view);
    });
    let shared_view_for_cleanup: Rc<RefCell<Option<View>>> = Rc::clone(&shared_view);
    let mut result: View = View::from_node(anchor_node);
    if let Some(initial) = initial_view_nodes.borrow_mut().take() {
        result.merge(initial);
    }
    result.get_mut_cleanups().push(Box::new(move || {
        effect.dispose();
        if let Some(mut view) = shared_view_for_cleanup.borrow_mut().take() {
            view.remove();
        }
    }));
    result
}

/// Creates a dynamic view with an explicit hook context passed to the render function.
///
/// The render function receives a mutable reference to a `HookContext`, allowing
/// hooks to be used inside the dynamic view. The hook context is reset on each
/// re-render so that hooks are called in stable order.
///
/// # Arguments
///
/// - `F` - A closure that receives `&mut HookContext` and returns a `View`.
///
/// # Returns
///
/// - `View` - A view containing the anchor and the dynamic content.
pub fn create_dynamic_view_with_context<F>(mut render_fn: F) -> View
where
    F: FnMut(&mut HookContext) -> View + 'static,
{
    let hook_context: HookContext = create_hook_context();
    let mut hook_context_for_closure: HookContext = hook_context;
    let anchor_node: Node = create_anchor();
    let shared_view: Rc<RefCell<Option<View>>> = Rc::new(RefCell::new(None));
    let shared_view_clone: Rc<RefCell<Option<View>>> = Rc::clone(&shared_view);
    let anchor_for_insert: Node = anchor_node.clone();
    let initial_view_nodes: Rc<RefCell<Option<View>>> = Rc::new(RefCell::new(None));
    let initial_view_nodes_clone: Rc<RefCell<Option<View>>> = Rc::clone(&initial_view_nodes);
    let mut first_run: bool = true;
    let effect: RenderEffect = create_render_effect(move || {
        if !first_run && anchor_for_insert.parent_node().is_none() {
            return;
        }
        first_run = false;
        hook_context_for_closure.reset_hook_index();
        let new_view: View = with_hook_context(hook_context_for_closure, || {
            render_fn(&mut hook_context_for_closure)
        });
        if let Some(ref mut old_view) = shared_view_clone.borrow_mut().as_mut() {
            old_view.remove();
        }
        if anchor_for_insert.parent_node().is_some() {
            new_view.insert_before_ref(&anchor_for_insert);
        } else {
            *initial_view_nodes_clone.borrow_mut() = Some(new_view.node_refs());
        }
        *shared_view_clone.borrow_mut() = Some(new_view);
    });
    let shared_view_for_cleanup: Rc<RefCell<Option<View>>> = Rc::clone(&shared_view);
    let mut result: View = View::from_node(anchor_node);
    if let Some(initial) = initial_view_nodes.borrow_mut().take() {
        result.merge(initial);
    }
    result.get_mut_cleanups().push(Box::new(move || {
        effect.dispose();
        if let Some(mut view) = shared_view_for_cleanup.borrow_mut().take() {
            view.remove();
        }
        hook_context.leak_mut().get_mut_hooks().clear();
        let cleanups: Vec<Box<dyn FnOnce()>> = take(hook_context.leak_mut().get_mut_cleanups());
        for cleanup in cleanups {
            cleanup();
        }
    }));
    result
}

/// Creates a dynamic view that re-renders when tracked signals change.
///
/// Internally creates a hook context and sets it as current before each
/// invocation of `render_fn`, so hooks can be used transparently.
///
/// # Arguments
///
/// - `F` - A closure that produces a `View` each time it is called.
///
/// # Returns
///
/// - `View` - A view containing the anchor and the dynamic content.
pub fn create_dynamic_view<F>(mut render_fn: F) -> View
where
    F: FnMut() -> View + 'static,
{
    let hook_context: HookContext = create_hook_context();
    let mut hook_context_for_closure: HookContext = hook_context;
    let anchor_node: Node = create_anchor();
    let shared_view: Rc<RefCell<Option<View>>> = Rc::new(RefCell::new(None));
    let shared_view_clone: Rc<RefCell<Option<View>>> = Rc::clone(&shared_view);
    let anchor_for_insert: Node = anchor_node.clone();
    let initial_view_nodes: Rc<RefCell<Option<View>>> = Rc::new(RefCell::new(None));
    let initial_view_nodes_clone: Rc<RefCell<Option<View>>> = Rc::clone(&initial_view_nodes);
    let mut first_run: bool = true;
    let effect: RenderEffect = create_render_effect(move || {
        if !first_run && anchor_for_insert.parent_node().is_none() {
            return;
        }
        first_run = false;
        hook_context_for_closure.reset_hook_index();
        let new_view: View = with_hook_context(hook_context_for_closure, &mut render_fn);
        if let Some(ref mut old_view) = shared_view_clone.borrow_mut().as_mut() {
            old_view.remove();
        }
        if anchor_for_insert.parent_node().is_some() {
            new_view.insert_before_ref(&anchor_for_insert);
        } else {
            *initial_view_nodes_clone.borrow_mut() = Some(new_view.node_refs());
        }
        *shared_view_clone.borrow_mut() = Some(new_view);
    });
    let shared_view_for_cleanup: Rc<RefCell<Option<View>>> = Rc::clone(&shared_view);
    let mut result: View = View::from_node(anchor_node);
    if let Some(initial) = initial_view_nodes.borrow_mut().take() {
        result.merge(initial);
    }
    result.get_mut_cleanups().push(Box::new(move || {
        effect.dispose();
        if let Some(mut view) = shared_view_for_cleanup.borrow_mut().take() {
            view.remove();
        }
        hook_context.leak_mut().get_mut_hooks().clear();
        let cleanups: Vec<Box<dyn FnOnce()>> = take(hook_context.leak_mut().get_mut_cleanups());
        for cleanup in cleanups {
            cleanup();
        }
    }));
    result
}

/// Mounts a view into the DOM at the element matching the given CSS selector.
///
/// The selector supports `"body"`, `#id`, `.class`, and tag name lookups.
///
/// # Arguments
///
/// - `&str` - The CSS selector for the mount target.
/// - `F` - A closure that produces the root `View`.
///
/// # Panics
///
/// Panics if no matching element is found in the document.
pub fn mount_view<F>(selector: &str, render_fn: F)
where
    F: FnOnce() -> View,
{
    let window: Window = web_sys::window().expect("no global window exists");
    let document: Document = window.document().expect("should have a document");
    let target: Element = if selector == "body" {
        document.body().expect("document should have a body").into()
    } else if let Some(id) = selector.strip_prefix('#') {
        document
            .get_element_by_id(id)
            .unwrap_or_else(|| panic!("no element found with id '{}'", id))
    } else if let Some(class) = selector.strip_prefix('.') {
        document
            .get_elements_by_class_name(class)
            .item(0)
            .unwrap_or_else(|| panic!("no element found with class '{}'", class))
    } else {
        document
            .get_elements_by_tag_name(selector)
            .item(0)
            .unwrap_or_else(|| panic!("no element found with tag '{}'", selector))
    };
    let view: View = render_fn();
    view.append_to(&target);
}
