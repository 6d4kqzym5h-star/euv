use crate::*;

/// Implementation of ComponentContext construction.
impl ComponentContext {
    /// Creates a new empty component context.
    pub fn new() -> Self {
        let mut ctx: ComponentContext = ComponentContext::default();
        ctx.set_hooks(Vec::new());
        ctx.set_hook_index(0_usize);
        ctx
    }
}

/// Implementation of component context lifecycle and utility methods.
impl ComponentContext {
    /// Resets the hook index for a new render cycle.
    pub fn reset_hook_index(&mut self) {
        self.set_hook_index(0_usize);
    }
}

/// Implementation of ComponentHandle construction.
impl ComponentHandle {
    /// Creates a new component handle with the given identifier.
    pub fn new(id: usize) -> Self {
        let mut handle: ComponentHandle = ComponentHandle::default();
        handle.set_id(id);
        handle
    }
}
