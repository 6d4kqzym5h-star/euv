use super::*;

/// A zero-sized namespace struct providing static input event extraction methods.
///
/// This struct follows the same pattern as `App`, serving as a namespace for
/// free-standing input utility functions that extract data from DOM events.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct Input;

/// Tracks the current state of all input devices (keyboard, mouse, touch)
/// for a single game frame. The state should be updated by event handlers
/// and cleared of per-frame data at the end of each frame.
#[derive(Clone, Data, Debug, New, PartialEq)]
pub struct InputState {
    /// Key codes that were pressed during this frame.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) keys_pressed: KeyStateSet,
    /// Key codes that are currently held down.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) keys_held: KeyStateSet,
    /// Key codes that were released during this frame.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) keys_released: KeyStateSet,
    /// Mouse buttons that were pressed during this frame.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) mouse_buttons_pressed: HashSet<MouseButton>,
    /// Mouse buttons that are currently held down.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) mouse_buttons_held: HashSet<MouseButton>,
    /// Mouse buttons that were released during this frame.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) mouse_buttons_released: HashSet<MouseButton>,
    /// The current mouse position in screen coordinates.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) mouse_position: Vector2D,
    /// The mouse position delta (movement) since the last frame.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) mouse_delta: Vector2D,
    /// Whether the mouse has moved during this frame.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) mouse_moved: bool,
    /// Active touch points mapped by identifier to screen position.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) touch_points: TouchPointMap,
    /// Touch point identifiers that started during this frame.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) touch_started: HashSet<i32>,
    /// Touch point identifiers that ended during this frame.
    #[get_mut(pub(crate))]
    #[new(skip)]
    pub(crate) touch_ended: HashSet<i32>,
}
