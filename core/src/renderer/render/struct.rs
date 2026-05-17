use crate::*;

/// Manages the rendering of virtual DOM nodes to the real DOM.
///
/// Maintains a mapping between virtual nodes and real DOM elements,
/// and handles creation, diffing, and patching of the DOM tree.
#[derive(Data, Debug, New)]
pub struct Renderer {
    /// Mapping from virtual node IDs to real DOM elements.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    node_map: HashMap<usize, Element>,
    /// The root DOM element.
    #[get(pub(crate))]
    #[set(pub(crate))]
    root: Element,
    /// The current virtual DOM tree.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    current_tree: Option<VirtualNode>,
    /// Counter for generating unique node IDs.
    #[get(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    next_id: Rc<RefCell<usize>>,
}
