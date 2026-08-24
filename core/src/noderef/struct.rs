use super::*;

/// A reactive handle to a mounted DOM element.
///
/// `NodeRef` is created via [`App::use_node_ref`] (which routes through the
/// current [`HookContext`]) and is populated by the renderer after the
/// corresponding virtual node is mounted into the real DOM. Before
/// the first mount the inner value is `None`; after unmount it is reset
/// to `None` again, so consumers can rely on `get()` returning `None`
/// to detect the unmounted state.
///
/// The type parameter `T` is purely a phantom marker that names the
/// expected element type (e.g. `NodeRef<HtmlInputElement>`). The runtime
/// stores the element as a raw `JsValue`; calling [`get_cloned`] performs
/// the `dyn_into` cast on demand. This avoids pulling in `web_sys` types
/// in the core hot path and keeps the type zero-cost when the consumer
/// only needs the raw `JsValue`.
///
/// `NodeRef` is `Clone` and cheap to copy (it is an `Rc` clone). All clones
/// share the same underlying cell, so setting the value through one clone
/// is visible through every other clone.
///
/// [`get_cloned`]: NodeRef::get_cloned
pub struct NodeRef<T: ?Sized> {
    /// Shared interior mutability cell holding the (optional) raw DOM
    /// element as a `JsValue`.
    pub(crate) inner: Rc<UnsafeCell<Option<JsValue>>>,
    /// Phantom marker for the expected element type. Not used at runtime
    /// — `get_cloned` only inspects the `T: Into<JsValue>` bound.
    pub(crate) _marker: PhantomData<fn() -> T>,
}

// `Debug` is implemented manually as well: we want to skip the
// non-`Debug` `JsValue` payload but still expose the phantom marker
// type, which is useful for assertions in tests.
