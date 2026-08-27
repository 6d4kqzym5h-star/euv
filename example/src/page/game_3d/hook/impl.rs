use super::*;

/// Default camera angles providing a pleasant initial orbit view.
impl Default for CameraAngles {
    /// Constructs a default [`CameraAngles`] value.
    fn default() -> Self {
        Self {
            yaw: Rc::new(Cell::new(0.3)),
            pitch: Rc::new(Cell::new(0.4)),
        }
    }
}

/// Pointer equality – two `CameraAngles` are always considered equal because
/// the actual values live inside `Cell`s that bypass the reactivity system.
impl PartialEq for CameraAngles {
    /// Returns `true` when `self` and `other` are equivalent by the [`PartialEq`] contract.
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

/// Pointer equality – the wrapped `Vec` is mutated in-place via `RefCell`,
/// so `PartialEq` always returns `true` to avoid unnecessary re-renders.
impl PartialEq for CubeStore {
    /// Returns `true` when `self` and `other` are equivalent by the [`PartialEq`] contract.
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
