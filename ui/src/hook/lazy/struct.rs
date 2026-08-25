//! `LazyComponent<T>` implementation.

use super::*;

/// A lazy component that defers factory invocation until
/// first access.
///
/// Use `LazyComponent::new(factory)` to construct, then
/// `get()` to read (which triggers the factory on first
/// call) or `prefetch()` to trigger without reading.
///
/// # Why use `Rc<dyn Fn() -> T>`?
///
/// Because the factory must be callable multiple times
/// (e.g. after a `reset()`), and `dyn Fn` lets the user
/// pass any closure that produces a `T`. The factory is
/// stored as `Rc<dyn Fn() -> T>` (not `Box<dyn Fn>`) so
/// `LazyComponent` can be cloned cheaply and shared
/// between hook contexts.
pub struct LazyComponent<T: Clone + PartialEq + 'static> {
    pub(crate) state: Signal<LoadState<T>>,
    pub(crate) factory: Rc<dyn Fn() -> T>,
}
