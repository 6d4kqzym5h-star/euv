use super::*;

/// Type alias for the handler registry value.
///
/// Stores a raw pointer to a heap-allocated `HandlerSlot`. The allocation
/// is owned by the registry and freed during cleanup. Direct pointer access
/// avoids `Rc<RefCell<>>` overhead in the event dispatch hot path.
pub(crate) type HandlerEntry = *mut HandlerSlot;

/// Type alias for the signal update registry value.
///
/// Stores a raw pointer to a heap-allocated `SignalUpdateSlot`. The allocation
/// is owned by the registry and freed during cleanup or sweep. Direct pointer
/// access avoids `Rc<RefCell<>>` overhead in the signal dispatch hot path.
pub(crate) type SignalUpdateEntry = *mut SignalUpdateSlot;

/// Type alias for the handler registry map.
///
/// Nested by element ID first so that removing all handlers for one element is a
/// single `HashMap::remove(&euv_id)` instead of a full-registry scan. Uses
/// `&'static str` for event names to avoid allocation on every dispatch lookup.
/// Known event names are compile-time constants; custom names are leaked once via `as_str()`.
pub(crate) type HandlerRegistryMap = HashMap<usize, HashMap<&'static str, HandlerEntry>>;

/// Type alias for a single window event handler entry in the proxy registry.
///
/// Each entry holds a unique handler ID and a raw pointer to a heap-allocated
/// callback. The ID allows targeted removal during cleanup without disrupting
/// other handlers.
pub(crate) type WindowEventHandlerEntry = (usize, *mut Box<dyn FnMut()>);

/// Type alias for the window event proxy registry map.
///
/// Maps event names to a list of handler entries. All handlers for the same
/// event name share a single `window.addEventListener` listener (the proxy),
/// which iterates this list and invokes each callback on every event.
pub(crate) type WindowEventRegistryMap = HashMap<String, Vec<WindowEventHandlerEntry>>;
