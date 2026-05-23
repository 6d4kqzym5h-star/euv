use crate::*;

/// Global map from signal address to dependent effect addresses.
///
/// Each entry maps a signal's inner pointer address to a list of
/// `RenderEffect` inner pointer addresses that depend on that signal.
pub(crate) static mut EFFECT_SUBSCRIBERS: EffectSubscribersCell =
    EffectSubscribersCell(UnsafeCell::new(None));

/// The currently active RenderEffect being tracked, if any.
/// When `Signal::get()` is called and this is `Some`, the signal
/// registers itself as a dependency of the active effect.
pub(crate) static mut CURRENT_EFFECT: CurrentEffectCell = CurrentEffectCell(UnsafeCell::new(None));

/// Global registry mapping effect inner addresses to their `Rc<RefCell<>>`
/// references. This allows reconstructing a `RenderEffect` from just
/// the address key stored in pending queues.
pub(crate) static mut RENDER_EFFECT_REGISTRY: RenderEffectRegistryCell =
    RenderEffectRegistryCell(UnsafeCell::new(None));
