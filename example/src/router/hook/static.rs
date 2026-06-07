use crate::*;

thread_local! {
    pub(crate) static BACK_PENDING: Cell<bool> = const { Cell::new(false) };
    pub(crate) static NAVIGATE_AFTER_BACK: Cell<Option<String>> = const { Cell::new(None) };
}
