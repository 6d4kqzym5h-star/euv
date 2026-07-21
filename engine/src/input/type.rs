use super::*;

/// A set of keyboard key code strings currently in a given state.
pub type KeyStateSet = HashSet<String>;

/// A map from touch identifier to touch position.
pub type TouchPointMap = HashMap<i32, Vector2D>;
