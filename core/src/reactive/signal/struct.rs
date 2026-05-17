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
