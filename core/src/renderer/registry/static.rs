use super::*;

/// Global auto-incrementing ID counter for DOM elements.
pub(crate) static NEXT_EUV_ID: AtomicUsize = AtomicUsize::new(0);

/// Global auto-incrementing ID counter for DynamicNode placeholder elements.
pub(crate) static NEXT_EUV_DYNAMIC_ID: AtomicUsize = AtomicUsize::new(0);

/// Whether `dispatch_updates` is currently executing.
pub(crate) static SIGNAL_UPDATE_DISPATCHING: AtomicBool = AtomicBool::new(false);

/// Global handler registry, mapping (element_id, event_name) to HandlerEntry.
pub(crate) static mut HANDLER_REGISTRY: LazyLock<HandlerRegistryCell> =
    LazyLock::new(|| HandlerRegistryCell(UnsafeCell::new(HashMap::new())));

/// Global set of event names that have already been delegated at the window level.
pub(crate) static mut DELEGATED_EVENTS: LazyLock<DelegatedEventsCell> =
    LazyLock::new(|| DelegatedEventsCell(UnsafeCell::new(HashSet::new())));

/// Global signal update callback registry, mapping keys to SignalUpdateEntry.
pub(crate) static mut SIGNAL_UPDATE_REGISTRY: LazyLock<SignalUpdateRegistryCell> =
    LazyLock::new(|| SignalUpdateRegistryCell(UnsafeCell::new(HashMap::new())));

/// Global auto-incrementing ID counter for window event handler entries.
pub(crate) static NEXT_WINDOW_HANDLER_ID: AtomicUsize = AtomicUsize::new(0);

/// Global window event proxy registry, mapping event names to handler lists.
pub(crate) static mut WINDOW_EVENT_REGISTRY: LazyLock<WindowEventRegistryCell> =
    LazyLock::new(|| WindowEventRegistryCell(UnsafeCell::new(HashMap::new())));
