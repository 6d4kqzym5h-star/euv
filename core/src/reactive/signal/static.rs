use crate::*;

/// Global registry tracking heap-allocated `SignalInner<T>` pointer addresses.
///
/// Each signal's inner state is tracked by its pointer address. This registry
/// is used to check whether a signal allocation is still alive, preventing
/// use-after-free when listeners free signals during dispatch.
///
/// SAFETY: Must only be accessed from the main thread (WASM single-threaded context).
pub(crate) static mut SIGNAL_INNER_REGISTRY: LazyLock<SignalInnerRegistryCell> =
    LazyLock::new(|| SignalInnerRegistryCell(UnsafeCell::new(HashSet::new())));
