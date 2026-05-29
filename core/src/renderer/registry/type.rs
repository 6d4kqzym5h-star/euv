use crate::*;

/// Type alias for the handler registry value.
pub(crate) type HandlerEntry = Rc<RefCell<HandlerSlot>>;

/// Type alias for the signal update registry value.
pub(crate) type SignalUpdateEntry = Rc<RefCell<SignalUpdateSlot>>;

/// Type alias for the handler registry map.
///
/// Uses `&'static str` for event names to avoid allocation on every dispatch lookup.
/// Known event names are compile-time constants; custom names are leaked once via `as_str()`.
pub(crate) type HandlerRegistryMap = HashMap<(usize, &'static str), HandlerEntry>;
