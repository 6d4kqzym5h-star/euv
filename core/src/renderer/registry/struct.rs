use crate::*;

/// A wrapper around `Option<NativeEventHandler>` that enables `From<usize>` conversions.
#[derive(CustomDebug, Data, New)]
pub(crate) struct HandlerSlot {
    /// The optional event handler stored in this slot.
    #[debug(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) handler: Option<NativeEventHandler>,
}

/// Stores a signal update callback and its cleanup flag.
#[derive(CustomDebug, Data, New)]
pub(crate) struct SignalUpdateSlot {
    /// The callback to invoke when `__euv_signal_update__` fires.
    #[debug(skip)]
    #[get(skip)]
    #[set(pub(crate))]
    pub(crate) callback: Option<Box<dyn FnMut()>>,
    /// Whether this slot has been marked for removal.
    #[get(type(copy))]
    #[set(pub(crate))]
    pub(crate) removed: bool,
}

/// A `Sync` wrapper for single-threaded global `HashMap` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct HandlerRegistryCell(
    /// Interior-mutable storage for the handler registry.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) UnsafeCell<Option<HandlerRegistryMap>>,
);

/// A `Sync` wrapper for single-threaded global `HashSet` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct DelegatedEventsCell(
    /// Interior-mutable storage for the delegated events set.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) UnsafeCell<Option<HashSet<Cow<'static, str>>>>,
);

/// A `Sync` wrapper for single-threaded global `HashMap` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct SignalUpdateRegistryCell(
    /// Interior-mutable storage for the signal update registry.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) UnsafeCell<Option<HashMap<usize, SignalUpdateEntry>>>,
);

/// A `Sync` wrapper for single-threaded global `bool` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct SignalUpdateListenerRegisteredCell(
    /// Interior-mutable storage for the flag.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) UnsafeCell<bool>,
);
