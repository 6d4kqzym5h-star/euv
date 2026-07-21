use super::*;

/// A reference-counted, interior-mutable list of asset load callbacks.
///
/// Stores `Closure` objects to prevent them from being dropped prematurely,
/// avoiding memory leaks when registering `onload`/`onerror` handlers.
pub type AssetClosures = Rc<RefCell<Vec<Closure<dyn FnMut()>>>>;
