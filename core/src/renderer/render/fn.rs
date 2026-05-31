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
pub fn mount<F>(selector: &str, render_fn: F)
where
    F: FnOnce() -> VirtualNode,
{
    let window: Window = match window() {
        Some(window_instance) => window_instance,
        None => return,
    };
    let document: Document = match window.document() {
        Some(document_instance) => document_instance,
        None => return,
    };
    let target: Element = if selector == BODY_TAG {
        match document.body() {
            Some(body) => body.into(),
            None => return,
        }
    } else if let Some(id) = selector.strip_prefix(ID_SELECTOR_PREFIX) {
        match document.get_element_by_id(id) {
            Some(element) => element,
            None => return,
        }
    } else if let Some(class) = selector.strip_prefix(CLASS_SELECTOR_PREFIX) {
        match document.get_elements_by_class_name(class).item(0) {
            Some(element) => element,
            None => return,
        }
    } else {
        match document.get_elements_by_tag_name(selector).item(0) {
            Some(element) => element,
            None => return,
        }
    };
    Renderer::new(target).render(render_fn());
}
