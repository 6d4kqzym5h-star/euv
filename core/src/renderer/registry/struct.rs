use crate::*;

/// A wrapper around `Option<NativeEventHandler>` that enables `From<usize>` conversions.
///
/// For non-bubbling events, also stores the JavaScript `Function` reference
/// and `Element` needed to call `removeEventListener` during cleanup.
#[derive(CustomDebug, Data, New)]
pub(crate) struct HandlerSlot {
    /// The optional event handler stored in this slot.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) handler: Option<NativeEventHandler>,
    /// The JavaScript `Function` reference for non-bubbling event listeners.
    ///
    /// When a non-bubbling event is attached directly on an element via
    /// `addEventListener`, the closure's JS `Function` must be kept alive so
    /// it can be passed to `removeEventListener` during cleanup. For bubbling
    /// events that use global delegation, this is `None`.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) listener_function: Option<JsValue>,
    /// The DOM element on which a non-bubbling event listener was registered.
    ///
    /// Stored here so that `removeEventListener` can be called during cleanup
    /// without needing to re-look-up the element. `None` for bubbling events.
    #[debug(skip)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) element: Option<Element>,
}

/// Stores a signal update callback and its cleanup flag.
#[derive(CustomDebug, Data, New)]
pub(crate) struct SignalUpdateSlot {
    /// The callback to invoke when signal update events fire.
    #[debug(skip)]
    #[get(skip)]
    #[set(pub(crate))]
    pub(crate) callback: Option<Box<dyn FnMut()>>,
    /// Whether this slot has been marked for removal.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) removed: bool,
    /// Whether this slot has pending changes that need dispatching.
    /// Only dirty slots are invoked during dispatch, avoiding O(N)
    /// broadcast to all dynamic nodes when only one signal changed.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dirty: bool,
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
    #[get_mut(pub(crate))]
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
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) UnsafeCell<Option<HashSet<&'static str>>>,
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
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) UnsafeCell<Option<HashMap<usize, SignalUpdateEntry>>>,
);

/// A `Sync` wrapper for single-threaded global `WindowEventRegistryMap` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct WindowEventRegistryCell(
    /// Interior-mutable storage for the window event handler registry.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) UnsafeCell<Option<WindowEventRegistryMap>>,
);
