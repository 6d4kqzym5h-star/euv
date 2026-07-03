use crate::*;

/// A single entry in the modal stack, pairing a modal's visibility signal
/// with its close callback.
///
/// The visibility signal acts as a stable identity so that a specific modal
/// can be located and removed from the stack when closed through the UI
/// (rather than via the system back gesture).
pub(crate) type ModalStackEntry = (Signal<bool>, Rc<dyn Fn()>);

/// The internal storage type for the modal stack, holding an ordered list
/// of currently open modals inside a `RefCell` for interior mutability.
pub(crate) type ModalStack = RefCell<Vec<ModalStackEntry>>;

/// A single entry in the unified overlay history stack.
///
/// Holds the close callback for an overlay (modal, panel, or drawer) so that
/// a system back gesture can dismiss the most recently opened overlay first,
/// regardless of its type.
pub(crate) struct OverlayEntry {
    pub(crate) closer: Rc<dyn Fn()>,
}

/// The internal storage type for the unified overlay stack, holding an ordered
/// list of all open overlays inside a `RefCell` for interior mutability.
pub(crate) type OverlayStack = RefCell<Vec<OverlayEntry>>;

/// A guard callback that is invoked on every `popstate` event.
///
/// Returning `true` means the guard has consumed the event and the
/// `popstate` should **not** be forwarded to the overlay stack or
/// normal navigation logic. Returning `false` lets the next guard (or
/// the default overlay-stack handler) process the event.
///
/// Guards are called in registration order; the first guard that returns
/// `true` wins and stops further processing.
pub(crate) type PopstateGuard = Rc<dyn Fn() -> bool>;

/// A single entry in the `popstate` guard list, pairing a unique ID with
/// the guard callback. The ID is used for stable unregistration regardless
/// of other entries being removed.
pub(crate) type PopstateGuardEntry = (usize, PopstateGuard);

/// The internal storage type for the registered `popstate` guard list.
///
/// Stored in a `RefCell` for interior mutability so that guards can be
/// registered or unregistered from anywhere within the single-threaded
/// WASM context.
pub(crate) type PopstateGuardList = RefCell<Vec<PopstateGuardEntry>>;
