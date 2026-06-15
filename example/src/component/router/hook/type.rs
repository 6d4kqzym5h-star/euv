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
