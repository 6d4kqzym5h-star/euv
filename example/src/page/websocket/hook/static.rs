use crate::*;

thread_local! {
    pub(crate) static WS_INSTANCE: RefCell<Option<WebSocket>> = const { RefCell::new(None) };
    pub(crate) static WS_CONNECTION_ID: Cell<usize> = const { Cell::new(0) };
}
