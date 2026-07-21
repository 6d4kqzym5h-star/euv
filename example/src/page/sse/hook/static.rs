use super::*;

thread_local! {
    pub(crate) static SSE_SOURCE: RefCell<Option<EventSource>> = const { RefCell::new(None) };
}
