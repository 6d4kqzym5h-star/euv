use super::*;

/// Type alias for an event callback closure.
///
/// A boxed `FnMut(Event)` invoked whenever the associated DOM event fires.
pub(crate) type EventCallback = Box<dyn FnMut(Event)>;

/// Type alias for the shared, interior-mutable storage of an event callback.
///
/// Uses `Rc<UnsafeCell<>>` instead of `Rc<RefCell<>>` to avoid runtime borrow
/// checking overhead in the single-threaded WASM context. The `Rc` provides
/// automatic deallocation when the last reference drops.
///
/// SAFETY: Only accessed from the main thread in the WASM single-threaded
/// context; no concurrent access is possible.
pub(crate) type SharedEventCallback = Rc<UnsafeCell<EventCallback>>;
