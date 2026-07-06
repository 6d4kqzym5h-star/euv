use crate::*;

/// Default camera angles providing a pleasant initial orbit view.
impl Default for CameraAngles {
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
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

/// Pointer equality – the wrapped `Vec` is mutated in-place via `RefCell`,
/// so `PartialEq` always returns `true` to avoid unnecessary re-renders.
impl PartialEq for CubeStore {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
