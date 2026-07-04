use crate::*;

/// Inner state of a signal, holding the value and subscribed listeners.
///
/// This struct is not exposed directly; use `Signal` instead.
#[derive(CustomDebug, Data, New)]
pub(crate) struct SignalInner<T>
where
    T: Clone,
{
    /// The current value of the signal.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) value: T,
    /// Callbacks to invoke when the value changes.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) listeners: Vec<Box<dyn FnMut()>>,
    /// Whether this signal is still active. Set to `false` by `deactivate()`
    /// (and `clear_signal_listeners`) to make subsequent `set()` calls
    /// complete no-ops (no value update, no listener invocation, no
    /// dispatch scheduling), ensuring stale closures like orphaned
    /// `setInterval` handlers or pending `spawn_local` futures become harmless.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) alive: bool,
    /// IDs of dynamic nodes that depend on this signal for precise dirty marking.
    /// When this signal changes, only these dynamic nodes are marked dirty
    /// instead of broadcasting to all registered dynamic nodes.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) dependents: Vec<usize>,
    /// Flag indicating that `replace_subscribe` was called during
    /// `update_and_notify`'s swap-out phase. When a listener callback
    /// re-registers listeners via `replace_subscribe`, it intends to
    /// replace all existing listeners — but the old listeners have
    /// already been swapped out into the local variable. Without this
    /// flag, `update_and_notify` would incorrectly merge the old
    /// listeners back with the new ones, defeating `replace_subscribe`'s
    /// replacement semantics and causing listener accumulation.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    pub(crate) listeners_replaced: bool,
}

/// A reactive signal handle.
///
/// Allows reading, writing, and subscribing to changes.
/// Implements `Clone` and `Copy` for ergonomic use; all copies share the same
/// underlying state. The inner state is heap-allocated via `Box` and accessed
/// through a raw pointer stored as a `usize`. The allocation is tracked in a
/// global registry for lifecycle management. The `Copy` semantics are safe
/// because only the pointer address is copied — the actual heap allocation
/// is owned by the registry.
#[derive(CustomDebug, Data, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct Signal<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Address of the heap-allocated inner state (`*mut SignalInner<T>`).
    #[debug(skip)]
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inner: usize,
    /// Marker for the generic type parameter (uses fn pointer to be `Copy`
    /// regardless of `T`).
    #[debug(skip)]
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) _marker: PhantomData<fn() -> T>,
}

/// A `Sync` wrapper for single-threaded global `Signal` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(CustomDebug, Data, New)]
pub struct SignalCell<T>
where
    T: Clone + PartialEq + 'static,
{
    /// Interior-mutable storage for an optional signal handle.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) inner: UnsafeCell<Option<Signal<T>>>,
}

/// A `Sync` wrapper for single-threaded global `HashSet` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct SignalInnerRegistryCell(
    /// Interior-mutable storage for the signal inner registry.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) UnsafeCell<Option<HashSet<usize>>>,
);
