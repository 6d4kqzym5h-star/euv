use super::*;

/// A set of keyboard key code strings currently in a given state.
pub type KeyStateSet = HashSet<String>;

/// A map from touch identifier to touch position.
pub type TouchPointMap = HashMap<i32, Vector2D>;

/// A shared, single-threaded cell holding the engine's [`InputState`].
///
/// Registered DOM event listeners clone this `Rc` to keep the state alive
/// for the lifetime of the document, while game code reads the same cell
/// through `EngineHandle::try_get_input_cell`.
pub type InputStateCell = Rc<EngineCell<InputState>>;
