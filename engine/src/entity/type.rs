use super::*;

/// A reference-counted, interior-mutable handle to a `dyn Component` trait object.
pub type ComponentRc = Rc<RefCell<dyn Component>>;

/// A reference-counted, interior-mutable handle to an `Entity`.
pub type EntityRc = Rc<RefCell<Entity>>;

/// A handler function that processes an `EntityEvent`.
pub type EventHandler = Rc<dyn Fn(&EntityEvent)>;

/// A map from event name to the list of registered handler closures.
pub type EventHandlers = HashMap<String, Vec<EventHandler>>;
