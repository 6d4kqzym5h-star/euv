use crate::*;

/// Context provided to a component during rendering.
///
/// Holds hook state and tracks the current hook index for this render cycle.
#[derive(Data, Default)]
pub struct ComponentContext {
    /// Internal storage for hooks and state.
    #[set(pub(crate))]
    hooks: Vec<Box<dyn Any>>,
    /// Current hook index for this render cycle.
    #[set(pub(crate))]
    hook_index: usize,
}

/// A handle to a mounted component instance.
///
/// Uniquely identifies a component instance within the application.
#[derive(Data, Default)]
pub struct ComponentHandle {
    /// Unique identifier for this component instance.
    id: usize,
}
