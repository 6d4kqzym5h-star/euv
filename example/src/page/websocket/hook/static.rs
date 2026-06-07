use crate::*;

thread_local! {
    pub(crate) static WS_INSTANCE: RefCell<Option<WebSocket>> = const { RefCell::new(None) };
}
