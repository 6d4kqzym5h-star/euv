use crate::*;

/// Inner storage for a native event callback closure.
///
/// Boxes a `dyn FnMut(Event)` so it can be stored behind `Rc<RefCell<>>`.
#[derive(CustomDebug, Data, New)]
pub(crate) struct NativeEventCallbackInner {
    /// The boxed callback closure.
    #[debug(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) callback: Box<dyn FnMut(Event)>,
}

/// A wrapper around an event callback.
///
/// Stores the event name and a shared reference to the heap-allocated callback closure.
#[derive(Clone, CustomDebug, Data, New)]
pub struct NativeEventHandler {
    /// The name of the event (e.g., "click", "input").
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) event_name: Cow<'static, str>,
    /// Shared reference to the heap-allocated callback closure inner state.
    #[debug(skip)]
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) callback: Rc<RefCell<NativeEventCallbackInner>>,
}
