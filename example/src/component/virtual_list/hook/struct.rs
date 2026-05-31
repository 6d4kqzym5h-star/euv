use crate::*;

/// Reactive state for the virtual list scroll tracking.
///
/// Contains only `Copy` signal fields so the struct can be freely
/// captured inside `html!` closures without causing `FnOnce` issues.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseVirtualList {
    /// The current scroll offset in pixels.
    #[get(type(copy))]
    pub(crate) scroll_offset: Signal<i32>,
    /// The current viewport height in pixels.
    #[get(type(copy))]
    pub(crate) viewport_height: Signal<i32>,
}
