use crate::*;

thread_local! {
    pub(crate) static BACK_PENDING: Cell<bool> = const { Cell::new(false) };
    pub(crate) static NAVIGATE_AFTER_BACK: Cell<Option<String>> = const { Cell::new(None) };
    /// A stack of currently open modals, ordered from oldest (bottom) to most
    /// recently opened (top), supporting nested modals.
    ///
    /// Each entry pairs the modal's visibility signal (used as a stable
    /// identity for precise removal when closed through the UI) with a close
    /// callback. Opening a modal pushes an entry and adds a browser history
    /// entry; a system back gesture pops the topmost entry and invokes its
    /// callback, so the most recently opened modal is always closed first.
    pub(crate) static MODAL_STACK: RefCell<Vec<(Signal<bool>, Rc<dyn Fn()>)>> =
        const { RefCell::new(Vec::new()) };
}
