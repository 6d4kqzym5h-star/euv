use super::*;

/// Global registry tracking heap-allocated `SignalInner<T>` pointer addresses.
///
/// Each signal's inner state is tracked by its pointer address. This registry
/// is used to check whether a signal allocation is still alive, preventing
/// use-after-free when listeners free signals during dispatch.
///
/// SAFETY: Must only be accessed from the main thread (WASM single-threaded context).
pub(crate) static mut SIGNAL_INNER_REGISTRY: LazyLock<SignalInnerRegistryCell> =
    LazyLock::new(|| SignalInnerRegistryCell(UnsafeCell::new(HashSet::new())));

/// Global reverse-index of `bridge_addr -> HashSet<source_addr>`.
///
/// Tracks which source signals currently hold a `subscribe` closure that
/// captures a given bridge signal's address. The bridge signal's heap
/// allocation can be safely freed only when the entry for that bridge is
/// empty AND `clear_listeners` has been called on the bridge; in any other
/// state, a stale closure could dereference the freed pointer.
///
/// This is set when a bridge is created (`Signal::track_bridge_dependency`)
/// and consulted from `clear_listeners` and `Signal::deactivate`. Entries
/// are removed when the bridge is fully reclaimed.
///
/// SAFETY: Must only be accessed from the main thread (WASM single-threaded
/// context).
pub(crate) static mut BRIDGE_REFS: LazyLock<BridgeRefsCell> =
    LazyLock::new(|| BridgeRefsCell(UnsafeCell::new(HashMap::new())));
