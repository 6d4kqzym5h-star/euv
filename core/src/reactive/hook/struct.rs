use crate::*;

/// Internal storage for hook state, holding boxed `Any` values.
///
/// This struct is not exposed directly; use `HookContext` instead.
/// The `arm_changed` flag tracks whether a `match` arm switch occurred;
/// when toggled, the hook array is cleared to prevent signal leakage
/// between different match arms.
#[derive(Data)]
pub struct HookContextInner {
    /// Storage for hook state values (signals, etc.).
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) hooks: Vec<Box<dyn Any>>,
    /// Whether the match arm has changed since the last render.
    /// Toggled on each `match` arm entry; when the value differs from
    /// the previous render, hooks are cleared.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) arm_changed: bool,
    /// Current hook index, incremented on each hook call and reset per render.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) hook_index: usize,
    /// Cleanup closures registered by hooks (e.g., `use_signal`) that must
    /// be executed when the hook context is cleared due to a `match` arm
    /// switch. Each closure typically clears signal listeners so that
    /// stale `setInterval` closures become no-ops.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cleanups: Vec<Box<dyn FnOnce()>>,
}

/// Manages hook state across render cycles for a DynamicNode.
///
/// Stores boxed `Any` values keyed by hook call order, enabling `use_signal`
/// and similar hooks to persist state between re-renders of the same
/// dynamic node.
///
/// Implements `Copy` for ergonomic use; all copies share the same underlying state.
///
/// SAFETY: The inner pointer is allocated via `Box::leak` and lives for the
/// entire program. This is safe in single-threaded WASM contexts where no
/// concurrent access can occur.
pub struct HookContext {
    /// Raw pointer to the heap-allocated hook context inner state.
    pub(crate) inner: *mut HookContextInner,
}

/// A `Sync` wrapper for single-threaded global `HookContextInner` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
pub struct HookContextCell(
    /// Interior-mutable storage for the hook context inner state.
    pub(crate) UnsafeCell<HookContextInner>,
);
