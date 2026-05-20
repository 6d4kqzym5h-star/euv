use crate::*;

/// Inner storage for a dynamic node render closure.
///
/// Boxes a `dyn FnMut() -> VirtualNode` so it can be stored behind a raw pointer.
/// Allocated via `Box::leak` and lives for the remainder of the program.
#[derive(CustomDebug, Data)]
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
/// Holds a raw pointer to a heap-allocated render closure that produces a fresh
/// `VirtualNode` on each evaluation. The renderer subscribes to the closure's
/// signals and patches the DOM automatically.
/// Contains a `HookContext` that persists hook state (like `use_signal`) across
/// re-renders, ensuring that signal values are not reset when the render function
/// is called again.
///
/// Implements `Copy` for ergonomic use; all copies share the same underlying state.
///
/// SAFETY: The inner pointer is allocated via `Box::leak` and lives for the
/// entire program. This is safe in single-threaded WASM contexts where no
/// concurrent access can occur.
#[derive(CustomDebug, Data, Eq, PartialEq)]
pub struct DynamicNode {
    /// Raw pointer to the heap-allocated render closure inner state.
    ///
    /// SAFETY: Allocated via `Box::leak`, valid for the program lifetime.
    #[debug(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) render_fn: *mut RenderFnInner,
    /// Persistent hook context for this dynamic node, storing signal
    /// state and other hook values across render cycles.
    ///
    /// Implements `Copy`; all copies share the same underlying state.
    /// When the `arm_changed` flag inside is toggled (by `match` arm switching),
    /// the hooks array is cleared to prevent signal leakage between arms.
    #[set(pub(crate))]
    pub(crate) hook_context: HookContext,
}
