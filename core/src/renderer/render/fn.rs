use crate::*;

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
