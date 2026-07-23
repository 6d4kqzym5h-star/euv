use super::*;

/// A reference-counted, shared-mutable list of asset load callbacks.
///
/// Stores `Closure` objects to prevent them from being dropped prematurely,
/// avoiding memory leaks when registering `onload`/`onerror` handlers.
///
/// Held behind `EngineCell` (an `UnsafeCell`-backed `Sync` newtype)
/// rather than `RefCell` so the storage matches the rest of the
/// engine's `static mut` convention. Access via [`EngineCell::get_mut`]
/// to obtain the underlying `Vec` for push / clear operations.
pub type AssetClosures = Rc<EngineCell<Vec<Closure<dyn FnMut()>>>>;
