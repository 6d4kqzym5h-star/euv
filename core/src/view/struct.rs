use crate::*;

/// A rendered view that holds real DOM nodes.
///
/// Unlike `VirtualNode` which represents a virtual DOM tree that must be
/// converted to real DOM, `View` wraps actual `web_sys::Node` instances
/// that are already attached (or ready to be attached) to the document.
///
/// This is the output type of the `html!` macro in the new architecture.
/// Each `View` carries one or more DOM nodes plus optional cleanup logic
/// that runs when the view is removed from the document.
#[derive(CustomDebug, Data)]
pub struct View {
    /// The DOM nodes belonging to this view.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) nodes: Vec<Node>,
    /// Cleanup closures that run when this view is disposed.
    #[debug(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cleanups: Vec<Box<dyn FnOnce()>>,
}

/// A thin wrapper for a single DOM node that can be inserted into a parent.
///
/// Used internally by the `html!` macro to pass child views into parent
/// element construction without allocating a `Vec`.
#[derive(CustomDebug, Data)]
pub struct NodeView {
    /// The single DOM node.
    #[get(pub(crate))]
    pub(crate) node: Node,
    /// Cleanup closures inherited from the child view.
    #[debug(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cleanups: Vec<Box<dyn FnOnce()>>,
}
