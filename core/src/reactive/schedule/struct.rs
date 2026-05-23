use crate::*;

/// A `Sync` wrapper for single-threaded global `Option<HookContextRc>` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct CurrentHookContextCell(
    /// Interior-mutable storage for the current hook context.
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) UnsafeCell<Option<HookContextRc>>,
);
