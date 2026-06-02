use crate::*;

/// Global registry holding heap-allocated `SignalInner<T>` entries to manage
/// their lifecycle.
///
/// Each signal's inner state is tracked by its pointer address. Entries are
/// removed during DOM cleanup to free memory. The registry stores a type-erased
/// drop function alongside each pointer so that `Box::from_raw` can be called
/// with the correct type.
///
/// SAFETY: Must only be accessed from the main thread (WASM single-threaded context).
pub(crate) static mut SIGNAL_INNER_REGISTRY: SignalInnerRegistryCell =
    SignalInnerRegistryCell(UnsafeCell::new(None));
