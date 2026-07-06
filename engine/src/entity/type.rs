use crate::*;

/// A reference-counted, interior-mutable handle to a `dyn Component` trait object.
pub type ComponentRc = Rc<RefCell<dyn Component>>;

/// A reference-counted, interior-mutable handle to an `Entity`.
pub type EntityRc = Rc<RefCell<Entity>>;
