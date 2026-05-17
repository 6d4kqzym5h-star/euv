use crate::*;

/// Manages the rendering of virtual DOM nodes to the real DOM.
///
/// Maintains a mapping between virtual nodes and real DOM elements,
/// and handles creation, diffing, and patching of the DOM tree.
#[derive(Debug)]
pub struct Renderer {
    /// The root DOM element.
    pub(crate) root: Element,
    /// The current virtual DOM tree.
    pub(crate) current_tree: Option<VirtualNode>,
}
