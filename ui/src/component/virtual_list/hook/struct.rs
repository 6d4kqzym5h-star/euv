use crate::*;

/// Reactive state for the virtual list scroll tracking.
///
/// Contains only `Copy` signal fields so the struct can be freely
/// captured inside `html!` closures without causing `FnOnce` issues.
#[derive(Clone, Copy, Data, Debug, Default, Eq, New, Ord, PartialEq, PartialOrd)]
pub struct UseVirtualList {
    /// The current scroll offset in pixels.
    #[get(type(copy))]
    pub scroll_offset: Signal<i32>,
    /// The current viewport height in pixels.
    #[get(type(copy))]
    pub viewport_height: Signal<i32>,
}

/// A `Sync` wrapper for single-threaded global `HashSet` access.
///
/// SAFETY: This type is only safe to use in single-threaded contexts
/// (e.g., WASM). It implements `Sync` to allow usage as a `static mut`
/// variable, but concurrent access from multiple threads would be
/// undefined behavior.
#[derive(Data, Debug, New)]
pub(crate) struct PendingMeasureCell(
    /// Interior-mutable storage for the pending measurement set.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) UnsafeCell<HashSet<String>>,
);
