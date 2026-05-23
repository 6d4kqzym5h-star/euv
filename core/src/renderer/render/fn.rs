use crate::*;

/// Mounts the given virtual DOM tree to the document body.
///
/// # Arguments
///
/// - `FnOnce() -> VirtualNode + 'static` - A closure that returns the virtual DOM tree to render.
///
/// # Panics
///
/// Panics if the document body cannot be found.
pub fn mount_body<F>(render_fn: F)
where
    F: FnOnce() -> VirtualNode,
{
    mount("body", render_fn);
}

/// Mounts the given virtual DOM tree to a specific element matched by a CSS selector.
///
/// Supported selector syntax:
/// - `"#id"` — select by element ID
/// - `".class"` — select by class name (uses the first match)
/// - `"tag"` — select by tag name (uses the first match)
///
/// # Arguments
///
/// - `&str` - A CSS selector string to locate the target element.
/// - `FnOnce() -> VirtualNode + 'static` - A closure that returns the virtual DOM tree to render.
///
/// # Panics
///
/// Panics if no global `window` or `document` exists, or if the selector does not match any element.
pub fn mount<F>(selector: &str, render_fn: F)
where
    F: FnOnce() -> VirtualNode,
{
    init_event_delegation();
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
    let mut renderer: Renderer = Renderer::new(target);
    let vnode: VirtualNode = render_fn();
    renderer.render(vnode);
}

/// Destroys the application mounted at the given CSS selector.
///
/// Removes all child nodes from the target element, cleans up all
/// framework-global state (handlers, signal update registry, delegated events),
/// resets scheduling state, and clears injected CSS classes.
///
/// # Arguments
///
/// - `&str` - A CSS selector string identifying the root element to unmount.
///
/// # Panics
///
/// Panics if no global `window` or `document` exists.
pub fn destroy(selector: &str) {
    let window: Window = window().expect("no global window exists");
    let document: Document = window.document().expect("should have a document");
    let target: Option<Element> = if selector == "body" {
        document.body().map(|body: HtmlElement| body.into())
    } else if let Some(id) = selector.strip_prefix('#') {
        document.get_element_by_id(id)
    } else if let Some(class) = selector.strip_prefix('.') {
        document.get_elements_by_class_name(class).item(0)
    } else {
        document.get_elements_by_tag_name(selector).item(0)
    };
    if let Some(element) = target {
        while let Some(child) = element.first_child() {
            let _ = element.remove_child(&child);
        }
    }
    cleanup_all();
    reset_schedule_state();
    reset_injected_classes();
}
