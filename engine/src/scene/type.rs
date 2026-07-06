use crate::*;

/// A reference-counted, interior-mutable scene trait object.
pub type SceneRc = Rc<RefCell<dyn Scene>>;
