use crate::*;

/// Inner storage for a dynamic node render closure.
///
/// Boxes a `dyn FnMut() -> VirtualNode` so it can be stored behind `Rc<RefCell<>>`.
#[derive(CustomDebug, Data, New)]
pub(crate) struct RenderFnInner {
    /// The boxed render closure.
    #[debug(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) render_fn: Box<dyn FnMut() -> VirtualNode>,
}

/// Represents a text node in the virtual DOM.
///
/// Text nodes may optionally be bound to a reactive signal for automatic updates.
#[derive(Clone, CustomDebug, Data, New)]
pub struct TextNode {
    /// The text content.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content: String,
    /// An optional signal that drives reactive text updates.
    #[debug(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) signal: Option<Signal<String>>,
}

/// A closure-based dynamic node that re-renders when its dependency signals change.
///
/// Holds a shared reference to a heap-allocated render closure that produces a fresh
/// `VirtualNode` on each evaluation. The renderer subscribes to the closure's
/// signals and patches the DOM automatically.
/// Contains a `HookContext` that persists hook state (like `use_signal`) across
/// re-renders, ensuring that signal values are not reset when the render function
/// is called again.
#[derive(CustomDebug, Data, New)]
pub struct DynamicNode {
    /// Shared reference to the heap-allocated render closure inner state.
    #[debug(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) render_fn: Rc<RefCell<RenderFnInner>>,
    /// Persistent hook context for this dynamic node, storing signal
    /// state and other hook values across render cycles.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) hook_context: HookContext,
}
