use crate::*;

/// Manages the rendering of virtual DOM nodes to the real DOM.
///
/// Maintains a mapping between virtual nodes and real DOM elements,
/// and handles creation, diffing, and patching of the DOM tree.
#[derive(Data, New)]
pub struct Renderer {
    /// Mapping from virtual node IDs to real DOM elements.
    #[new(skip)]
    node_map: HashMap<usize, Element>,
    /// The root DOM element.
    root: Element,
    /// The current virtual DOM tree.
    #[new(skip)]
    current_tree: Option<VirtualNode>,
    /// Counter for generating unique node IDs.
    #[new(skip)]
    next_id: Rc<RefCell<usize>>,
}
