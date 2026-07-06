use crate::*;

/// A reference-counted, interior-mutable tick handler.
pub type TickHandlerRc = Rc<RefCell<dyn TickHandler>>;

/// A reference-counted closure cell used to keep the `requestAnimationFrame` closure alive.
pub type RafClosureCell = Rc<RefCell<Option<Closure<dyn FnMut()>>>>;
