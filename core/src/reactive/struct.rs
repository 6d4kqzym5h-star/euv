use crate::*;

/// Inner state of a signal, holding the value and subscribed listeners.
///
/// This struct is not exposed directly; use `Signal` instead.
#[derive(Data)]
pub(crate) struct SignalInner<T>
where
    T: Clone,
{
    /// The current value of the signal.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) value: T,
    /// Callbacks to invoke when the value changes.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) listeners: Vec<Rc<RefCell<dyn FnMut()>>>,
}

/// A reactive signal handle.
///
/// Allows reading, writing, and subscribing to changes.
/// Implements `Copy` for ergonomic use; all copies share the same underlying state.
///
/// SAFETY: The inner pointer is allocated via `Box::leak` and lives for the
/// entire program. This is safe in single-threaded WASM contexts where no
/// concurrent access can occur.
pub struct Signal<T>
where
    T: Clone + PartialEq,
{
    /// Raw pointer to the heap-allocated signal inner state.
    pub(crate) inner: *mut SignalInner<T>,
}

/// A `Sync` wrapper for single-threaded global `Signal` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
pub struct SignalCell<T>
where
    T: Clone + PartialEq,
{
    /// Interior-mutable storage for an optional signal handle.
    pub(crate) inner: UnsafeCell<Option<Signal<T>>>,
}

/// Internal storage for hook state, holding boxed `Any` values.
///
/// This struct is not exposed directly; use `HookContext` instead.
/// The `current_id` field tracks which match arm owns the hooks;
/// when the ID changes, the hook array is cleared to prevent
/// signal leakage between different match arms.
#[derive(Data)]
pub struct HookContextInner {
    /// Storage for hook state values (signals, etc.).
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) hooks: Vec<Box<dyn Any>>,
    /// Current context ID, determined by the active match arm.
    /// When this changes, the hooks array is cleared.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) current_id: u64,
    /// Current hook index, incremented on each hook call and reset per render.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) hook_index: usize,
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
