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
    /// (and `clear_signal_listeners_by_addr`) to make subsequent `set()` calls
    /// complete no-ops (no value update, no listener invocation, no
    /// `schedule_signal_update()`), ensuring stale closures like orphaned
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
#[derive(CustomDebug, Data, Eq, New, PartialEq)]
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
    pub(crate) _marker: std::marker::PhantomData<fn() -> T>,
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

/// An entry in the signal inner registry.
///
/// Stores the raw pointer and a type-erased drop function so that the
/// allocation can be correctly freed without knowing the concrete `T`.
pub(crate) struct SignalRegistryEntry {
    /// The raw pointer to the `SignalInner<T>` allocation (as `*mut ()`).
    pub(crate) ptr: *mut (),
    /// A function that drops the `Box<SignalInner<T>>` when called with `ptr`.
    pub(crate) drop_fn: unsafe fn(*mut ()),
}

impl SignalRegistryEntry {
    /// Creates a new registry entry.
    pub(crate) fn new(ptr: *mut (), drop_fn: unsafe fn(*mut ())) -> Self {
        Self { ptr, drop_fn }
    }

    /// Returns the raw pointer.
    pub(crate) fn get_ptr(&self) -> *mut () {
        self.ptr
    }

    /// Returns the drop function.
    pub(crate) fn get_drop_fn(&self) -> unsafe fn(*mut ()) {
        self.drop_fn
    }
}

/// A `Sync` wrapper for single-threaded global `HashMap` access.
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
    pub(crate) UnsafeCell<Option<HashMap<usize, SignalRegistryEntry>>>,
);
