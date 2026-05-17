use crate::*;

/// Represents a text node in the virtual DOM.
///
/// Text nodes may optionally be bound to a reactive signal for automatic updates.
#[derive(Clone, Data, Debug, New)]
pub struct TextNode {
    /// The text content.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content: String,
    /// An optional signal that drives reactive text updates.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) signal: Option<Signal<String>>,
}

/// A closure-based dynamic node that re-renders when its dependency signals change.
///
/// Holds a boxed closure that produces a fresh `VirtualNode` on each evaluation.
/// The renderer subscribes to the closure's signals and patches the DOM automatically.
/// Contains a `HookContext` that persists hook state (like `use_signal`) across
/// re-renders, ensuring that signal values are not reset when the render function
/// is called again.
#[derive(CustomDebug, Data)]
pub struct DynamicNode {
    /// The closure that generates the dynamic virtual node tree.
    #[debug(skip)]
    #[get(pub)]
    #[set(pub)]
    pub(crate) render_fn: Rc<RefCell<dyn FnMut() -> VirtualNode>>,
    /// Persistent hook context for this dynamic node, storing signal
    /// state and other hook values across render cycles.
    ///
    /// Implements `Copy`; all copies share the same underlying state.
    /// When the `arm_changed` flag inside is toggled (by `match` arm switching),
    /// the hooks array is cleared to prevent signal leakage between arms.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) hook_context: HookContext,
}
