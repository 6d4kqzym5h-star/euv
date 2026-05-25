use crate::*;

/// Type alias for the handler registry value.
pub(crate) type HandlerEntry = Rc<RefCell<HandlerSlot>>;

/// Type alias for the signal update registry value.
pub(crate) type SignalUpdateEntry = Rc<RefCell<SignalUpdateSlot>>;

/// Type alias for the handler registry map.
pub(crate) type HandlerRegistryMap = HashMap<(usize, String), HandlerEntry>;
