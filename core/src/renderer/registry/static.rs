use crate::*;

/// Global auto-incrementing ID counter for DOM elements.
///
/// Used to assign a unique `data-euv-id` attribute to each element
/// that receives an event listener, providing a stable identity across
/// re-renders.
pub(crate) static NEXT_EUV_ID: AtomicUsize = AtomicUsize::new(0);

/// Global auto-incrementing ID counter for DynamicNode placeholder elements.
///
/// Used to assign a unique `data-euv-dynamic-id` attribute to each
/// DynamicNode placeholder `<div>`, enabling lifecycle management of
/// `__euv_signal_update__` event listeners tied to that placeholder.
pub(crate) static NEXT_EUV_DYNAMIC_ID: AtomicUsize = AtomicUsize::new(0);

/// Global pointer to the handler registry.
///
/// Lazily initialized on first access via `Box::leak`. Because WASM
/// is single-threaded, concurrent access is impossible and raw
/// pointer access is safe.
pub(crate) static mut HANDLER_REGISTRY: *mut HashMap<(usize, String), HandlerEntry> = null_mut();

/// Global pointer to the DynamicNode listener registry.
///
/// Maps a `data-euv-dynamic-id` value to the `JsValue` reference of the
/// `__euv_signal_update__` event listener closure previously registered
/// for that placeholder. When a new listener is registered for the same
/// placeholder, the old one is removed via `removeEventListener` first
/// to prevent listener accumulation and memory leaks on route changes.
///
/// Lazily initialized on first access via `Box::leak`.
#[cfg(target_arch = "wasm32")]
pub(crate) static mut DYNAMIC_LISTENER_REGISTRY: *mut HashMap<usize, JsValue> = null_mut();

/// Global pointer to the attribute signal listener registry.
///
/// Maps a `Signal<String>` inner pointer address to the `JsValue`
/// reference of the `__euv_signal_update__` event listener closure
/// registered by `subscribe_attr_signal`. When a new listener is
/// registered for the same signal, the old one is removed first
/// to prevent listener accumulation.
///
/// Lazily initialized on first access via `Box::leak`.
#[cfg(target_arch = "wasm32")]
pub(crate) static mut ATTR_SIGNAL_LISTENER_REGISTRY: *mut HashMap<usize, JsValue> = null_mut();
