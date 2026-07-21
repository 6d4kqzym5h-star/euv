use super::*;

/// Type alias for a single non-passive event listener closure paired with its
/// event name, used for direct `addEventListener` registration on the canvas.
pub(crate) type CanvasGuardListener = (Closure<dyn FnMut(Event)>, &'static str);

/// Type alias for the canvas scroll guard entry, containing all non-passive
/// event listener closures and the canvas element needed for cleanup.
pub(crate) type CanvasGuardEntry = (Vec<CanvasGuardListener>, Element);

/// Type alias for the shared canvas scroll guard cell used across the 3D game
/// loop startup and cleanup closures.
pub(crate) type CanvasGuardCell = Rc<RefCell<Option<CanvasGuardEntry>>>;
