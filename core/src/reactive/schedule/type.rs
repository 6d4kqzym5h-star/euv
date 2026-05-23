use crate::*;

/// Type alias for the shared hook context inner reference.
pub(crate) type HookContextRc = Rc<RefCell<HookContextInner>>;
