use crate::*;

/// Global registry holding `Rc<RefCell<SignalInner<T>>>` references to prevent
/// premature deallocation.
///
/// Each signal's inner state is tracked by its pointer address.
/// The `Rc` references are never removed, ensuring the `Copy`-based
/// `Signal` handles always point to valid memory.
///
/// SAFETY: Must only be accessed from the main thread (WASM single-threaded context).
pub(crate) static mut SIGNAL_INNER_REGISTRY: SignalInnerRegistryCell =
    SignalInnerRegistryCell(UnsafeCell::new(None));
