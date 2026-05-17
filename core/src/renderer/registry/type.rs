use crate::*;

/// Type alias for the handler registry value.
///
/// Uses a raw pointer instead of `Rc<RefCell<...>>` for page-lifetime data.
/// Allocated via `Box::leak`, valid for the program lifetime.
pub(crate) type HandlerEntry = *mut HandlerSlot;
