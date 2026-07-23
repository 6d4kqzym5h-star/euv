use super::*;

/// A reference-counted, shared-mutable scene trait object.
///
/// Holds a `dyn Scene` behind `EngineCell` (an `UnsafeCell`-backed
/// `Sync` newtype) so the storage matches the rest of the engine's
/// `static mut` convention. Use [`EngineCell::get_mut`] for
/// exclusive mutable access.
pub type SceneRc = Rc<EngineCell<dyn Scene>>;
