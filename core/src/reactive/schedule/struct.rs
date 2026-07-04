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
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) UnsafeCell<Option<HookContextRc>>,
);

/// A zero-sized struct providing static methods for scheduling
/// signal update dispatches and batching.
///
/// All methods are crate-internal associated functions that manage
/// the global scheduling flags and dispatch closure.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct Scheduler;
