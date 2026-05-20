use crate::*;

/// Manual Debug implementation that skips cleanups (dyn FnOnce has no Debug).
impl std::fmt::Debug for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("View")
            .field("nodes", &self.get_nodes())
            .field("cleanups", &format!("{} cleanups", self.get_cleanups().len()))
            .finish()
    }
}

/// Manual Debug implementation that skips cleanups (dyn FnOnce has no Debug).
impl std::fmt::Debug for NodeView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeView")
            .field("node", &self.get_node())
            .field("cleanups", &format!("{} cleanups", self.get_cleanups().len()))
            .finish()
    }
}

/// Implementation of `View` construction and DOM insertion.
impl View {
    /// Creates an empty view with no nodes and no cleanups.
    ///
    /// # Returns
    ///
    /// - `Self` - An empty view.
    pub fn empty() -> Self {
        View {
            nodes: Vec::new(),
            cleanups: Vec::new(),
        }
    }

    /// Creates a view from a single DOM node with no cleanups.
    ///
    /// # Arguments
    ///
    /// - `Node` - The DOM node.
    ///
    /// # Returns
    ///
    /// - `Self` - A view wrapping the single node.
    pub fn from_node(node: Node) -> Self {
        View {
            nodes: vec![node],
            cleanups: Vec::new(),
        }
    }

    /// Creates a view from a vector of DOM nodes.
    ///
    /// # Arguments
    ///
    /// - `Vec<Node>` - The DOM nodes.
    ///
    /// # Returns
    ///
    /// - `Self` - A view wrapping the nodes.
    pub fn from_nodes(nodes: Vec<Node>) -> Self {
        View {
            nodes,
            cleanups: Vec::new(),
        }
    }

    /// Appends all nodes of this view into the given parent element.
    ///
    /// # Arguments
    ///
    /// - `&Element` - The parent element to append into.
    pub fn append_to(&self, parent: &Element) {
        for node in self.get_nodes() {
            let _ = parent.append_child(node);
        }
    }

    /// Inserts all nodes of this view before the given reference node.
    ///
    /// # Arguments
    ///
    /// - `&Node` - The reference node to insert before.
    pub fn insert_before_ref(&self, ref_node: &Node) {
        if let Some(parent) = ref_node.parent_node() {
            for node in self.get_nodes() {
                let _ = parent.insert_before(node, Some(ref_node));
            }
        }
    }

    /// Removes all nodes of this view from their parent and runs cleanups.
    pub fn remove(&mut self) {
        for node in self.get_nodes() {
            if let Some(parent) = node.parent_node() {
                let _ = parent.remove_child(node);
            }
        }
        for cleanup in self.get_mut_cleanups().drain(..) {
            cleanup();
        }
    }

    /// Runs all cleanup closures without removing DOM nodes.
    pub fn dispose(&mut self) {
        for cleanup in self.get_mut_cleanups().drain(..) {
            cleanup();
        }
    }

    /// Returns the first node, if any.
    ///
    /// # Returns
    ///
    /// - `Option<&Node>` - The first node reference.
    pub fn first_node(&self) -> Option<&Node> {
        self.get_nodes().first()
    }

    /// Returns true if this view has no DOM nodes.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the view is empty.
    pub fn is_empty(&self) -> bool {
        self.get_nodes().is_empty()
    }

    /// Merges another view into this one, taking its nodes and cleanups.
    ///
    /// # Arguments
    ///
    /// - `View` - The other view to merge into this one.
    pub fn merge(&mut self, other: View) {
        self.get_mut_nodes().extend(other.get_nodes().clone());
        self.get_mut_cleanups().extend(other.get_cleanups().clone());
    }

    /// Creates a shallow copy of this view containing only node references
    /// without cleanup closures.
    ///
    /// Used internally when a reference to the current view's DOM nodes
    /// must be stored for later removal, but the cleanup closures must
    /// remain owned by the original view to prevent double-execution.
    ///
    /// # Returns
    ///
    /// - `View` - A view with cloned node references and no cleanups.
    pub fn node_refs(&self) -> View {
        View {
            nodes: self.get_nodes().clone(),
            cleanups: Vec::new(),
        }
    }

    /// Converts this View into a NodeView by taking the first node.
    /// Panics if the view has no nodes.
    ///
    /// # Returns
    ///
    /// - `NodeView` - A single-node view.
    pub fn into_node_view(mut self) -> NodeView {
        let node: Node = self.get_mut_nodes().remove(0);
        NodeView {
            node,
            cleanups: std::mem::take(self.get_mut_cleanups()),
        }
    }
}

/// Implementation of `NodeView` helpers.
impl NodeView {
    /// Creates a NodeView from a single DOM node with no cleanups.
    ///
    /// # Arguments
    ///
    /// - `Node` - The DOM node.
    ///
    /// # Returns
    ///
    /// - `Self` - A single-node view.
    pub fn from_node(node: Node) -> Self {
        NodeView {
            node,
            cleanups: Vec::new(),
        }
    }

    /// Converts this NodeView back into a full View.
    ///
    /// # Returns
    ///
    /// - `View` - A view with this node.
    pub fn into_view(self) -> View {
        View {
            nodes: vec![self.get_node().clone()],
            cleanups: self.get_cleanups().clone(),
        }
    }
}

/// Converts a `View` into `IntoView` trivially.
impl IntoView for View {
    fn into_view(self) -> View {
        self
    }
}

/// Converts a `NodeView` into `IntoView`.
impl IntoView for NodeView {
    fn into_view(self) -> View {
        self.into_view()
    }
}

/// Converts an `Element` into a `View`.
impl IntoView for Element {
    fn into_view(self) -> View {
        View::from_node(self.into())
    }
}

/// Converts a `String` into a text `View`.
impl IntoView for String {
    fn into_view(self) -> View {
        let document: Document = window().unwrap().document().unwrap();
        let text: Text = document.create_text_node(&self);
        View::from_node(text.into())
    }
}

/// Converts a `&str` into a text `View`.
impl IntoView for &str {
    fn into_view(self) -> View {
        self.to_string().into_view()
    }
}

/// Converts an `i32` into a text `View`.
impl IntoView for i32 {
    fn into_view(self) -> View {
        self.to_string().into_view()
    }
}

/// Converts a `usize` into a text `View`.
impl IntoView for usize {
    fn into_view(self) -> View {
        self.to_string().into_view()
    }
}

/// Converts an `f64` into a text `View`.
impl IntoView for f64 {
    fn into_view(self) -> View {
        self.to_string().into_view()
    }
}

/// Converts a `bool` into a text `View`.
impl IntoView for bool {
    fn into_view(self) -> View {
        self.to_string().into_view()
    }
}

/// Converts a `Signal<T>` into a reactive text `View`.
///
/// Creates a text node whose content updates whenever the signal changes.
impl<T> IntoView for Signal<T>
where
    T: Clone + PartialEq + std::fmt::Display + 'static,
{
    fn into_view(self) -> View {
        let document: Document = window().unwrap().document().unwrap();
        let initial: String = self.get_untracked().to_string();
        let text: Text = document.create_text_node(&initial);
        let text_clone: Text = text.clone();
        let signal_inner: Signal<T> = self;
        signal_inner.replace_subscribe(move || {
            let new_value: String = signal_inner.get_untracked().to_string();
            text_clone.set_text_content(Some(&new_value));
        });
        View::from_node(text.into())
    }
}
