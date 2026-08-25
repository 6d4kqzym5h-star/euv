use super::*;

/// Represents a standard mouse button identifier.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum MouseButton {
    /// The left mouse button (button 0).
    #[default]
    Left,
    /// The middle mouse button / scroll wheel click (button 1).
    Middle,
    /// The right mouse button (button 2).
    Right,
    /// Mouse button 3 (browser back).
    Button4,
    /// Mouse button 4 (browser forward).
    Button5,
}

/// Represents a high-level input action category.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum InputAction {
    /// The key or button was pressed during this frame.
    #[default]
    Pressed,
    /// The key or button is currently held down.
    Held,
    /// The key or button was released during this frame.
    Released,
    /// The key or button is currently idle (not pressed).
    Idle,
}
