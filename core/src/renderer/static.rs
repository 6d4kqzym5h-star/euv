use crate::*;

/// Global auto-incrementing ID counter for DOM elements.
///
/// Used to assign a unique `data-euv-id` attribute to each element
/// that receives an event listener, providing a stable identity across
/// re-renders.
pub static NEXT_EUV_ID: AtomicUsize = AtomicUsize::new(1);

/// Global pointer to the handler registry.
///
/// Lazily initialized on first access via `Box::leak`. Because WASM
/// is single-threaded, concurrent access is impossible and raw
/// pointer access is safe.
pub static mut HANDLER_REGISTRY: *mut HashMap<(usize, String), HandlerEntry> = null_mut();
