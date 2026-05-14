use crate::*;

/// A list of reactive signal listeners.
///
/// Each listener is a reference-counted mutable closure invoked when the signal changes.
pub(crate) type ListenerList = Vec<Rc<RefCell<dyn FnMut()>>>;
