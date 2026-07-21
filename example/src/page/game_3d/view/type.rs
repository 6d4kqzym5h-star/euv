use super::*;

/// Type alias for the pointer position signal used in 3D camera drag.
pub(crate) type PointerPositionSignal = Signal<Rc<Cell<Option<(f64, f64)>>>>;
