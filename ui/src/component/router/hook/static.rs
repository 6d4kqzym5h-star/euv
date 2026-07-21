use super::*;

thread_local! {
    /// Flag indicating that the next `popstate` event should consume the
    /// overlay history entry instead of performing normal navigation.
    pub(crate) static BACK_PENDING: Cell<bool> = const { Cell::new(false) };
    /// Optional route to navigate to after the overlay history entry is
    /// consumed by the browser back gesture.
    pub(crate) static NAVIGATE_AFTER_BACK: Cell<Option<String>> = const { Cell::new(None) };
    /// A stack of currently open modals, ordered from oldest (bottom) to most
    /// recently opened (top), supporting nested modals.
    ///
    /// Each entry pairs the modal's visibility signal (used as a stable identity for
    /// precise removal when closed through the UI) and a close callback. Opening a modal
    /// pushes an entry by adding a browser history entry; a system back gesture pops
    /// the topmost entry and invokes its close callback, so the most recently opened
    /// modal is dismissed first instead of navigating away.
    pub(crate) static MODAL_STACK: ModalStack =
        const { RefCell::new(Vec::new()) };
    /// A unified stack of all open overlays (modals, panels, drawers) in the order
    /// they were opened. The topmost entry is the most recently opened overlay.
    /// On a system back gesture the topmost entry is popped and its close callback
    /// is invoked, so overlays close in reverse opening order regardless of type.
    pub(crate) static OVERLAY_STACK: OverlayStack =
        const { RefCell::new(Vec::new()) };
    /// Depth counter for re-entrant `hashchange` / `popstate` event dispatch.
    ///
    /// Incremented before each window event proxy callback runs (`hashchange`,
    /// `popstate`, etc.) and decremented after it finishes. Provides a way for
    /// any code to detect that it is running within a window event handler context.
    ///
    /// Note: `navigate()` always defers `set_hash()` to a microtask regardless
    /// of this counter's value, so recursive Closure invocation is avoided
    /// by construction rather than by a conditional check.
    pub(crate) static WINDOW_EVENT_DEPTH: Cell<usize> = const { Cell::new(0) };
    /// Pending route deferred to the next microtask.
    ///
    /// `navigate()` always stores the target route here and schedules a
    /// microtask that calls `set_hash()`. If multiple `navigate()` calls
    /// occur before the microtask fires, only the **last** one wins —
    /// earlier routes are overwritten because they were superseded by a
    /// more recent navigation intent.
    pub(crate) static DEFERRED_NAVIGATION: Cell<Option<String>> = const { Cell::new(None) };
    /// Registered `popstate` guard callbacks.
    ///
    /// Each guard is an `Rc<dyn Fn() -> bool>`. On a `popstate` event, guards
    /// are called in registration order. The first guard that returns `true`
    /// consumes the event, preventing the overlay stack and normal navigation
    /// from processing it. This allows external modules (e.g. native fullscreen,
    /// canvas fullscreen) to intercept the system back gesture without
    /// registering their own independent `popstate` listener.
    pub(crate) static POPSTATE_GUARDS: PopstateGuardList =
        const { RefCell::new(Vec::new()) };
    /// Monotonically increasing counter for `popstate` guard IDs.
    ///
    /// Each call to `register_popstate_guard` reads and increments this
    /// counter, ensuring every guard receives a unique, stable ID that
    /// remains valid even after other guards are unregistered.
    pub(crate) static NEXT_POPSTATE_GUARD_ID: Cell<usize> = const { Cell::new(0) };
}
