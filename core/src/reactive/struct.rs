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
    pub(crate) listeners: Vec<Box<dyn FnMut()>>,
    /// Whether this signal is still active. Set to `false` by `clear_listeners()`
    /// to make subsequent `set()` calls complete no-ops (no value update, no
    /// listener invocation, no `schedule_signal_update()`), ensuring stale
    /// closures like orphaned `setInterval` handlers become harmless.
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) alive: bool,
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
