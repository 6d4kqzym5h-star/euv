use crate::*;

/// Type alias for the handler registry value.
pub(crate) type HandlerEntry = Rc<RefCell<Option<NativeEventHandler>>>;
