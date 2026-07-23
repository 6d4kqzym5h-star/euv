use super::*;

/// A reference-counted, shared-mutable handle to a `dyn Component` trait object.
///
/// Held behind `EngineCell` (an `UnsafeCell`-backed `Sync` newtype) so the
/// storage matches the rest of the engine's `static mut` convention.
/// Use [`EngineCell::get_mut`] for exclusive mutable access.
pub type ComponentRc = Rc<EngineCell<dyn Component>>;

/// A reference-counted, shared-mutable handle to an `Entity`.
///
/// Mirrors [`ComponentRc`] - the underlying storage is `EngineCell<Entity>`
/// rather than `Rc<RefCell<Entity>>`. The naming and ownership semantics
/// stay the same; only the interior mutability mechanism changes.
pub type EntityRc = Rc<EngineCell<Entity>>;

/// A handler function that processes an `EntityEvent`.
pub type EventHandler = Rc<dyn Fn(&EntityEvent)>;

/// A handler function that processes an `EntityEvent`.
pub type EventHandlers = HashMap<String, Vec<EventHandler>>;
