use crate::*;

/// Returns a mutable reference to the global handler registry.
///
/// Lazily initializes the registry on first access via `Box::leak`.
/// The allocated memory lives for the remainder of the program.
///
/// # Returns
///
/// - `&'static mut HashMap<(usize, String), HandlerEntry>` - A mutable reference to the global handler registry.
///
/// # Panics
///
/// Panics if the registry pointer is invalid after lazy initialization.
pub(crate) fn get_handler_registry() -> &'static mut HashMap<(usize, String), HandlerEntry> {
    unsafe {
        if HANDLER_REGISTRY.is_null() {
            let registry: Box<HashMap<(usize, String), HandlerEntry>> = Box::default();
            HANDLER_REGISTRY = Box::leak(registry) as *mut HashMap<(usize, String), HandlerEntry>;
        }
        &mut *HANDLER_REGISTRY
    }
}

/// Returns a mutable reference to the global DynamicNode listener registry.
///
/// Lazily initializes the registry on first access via `Box::leak`.
/// Maps `data-euv-dynamic-id` values to the `JsValue` reference of the
/// corresponding `__euv_signal_update__` event listener closure.
///
/// # Returns
///
/// - `&'static mut HashMap<usize, JsValue>` - A mutable reference to the global DynamicNode listener registry.
#[cfg(target_arch = "wasm32")]
pub(crate) fn get_dynamic_listener_registry() -> &'static mut HashMap<usize, JsValue> {
    unsafe {
        if DYNAMIC_LISTENER_REGISTRY.is_null() {
            let registry: Box<HashMap<usize, JsValue>> = Box::default();
            DYNAMIC_LISTENER_REGISTRY = Box::leak(registry) as *mut HashMap<usize, JsValue>;
        }
        &mut *DYNAMIC_LISTENER_REGISTRY
    }
}

/// Returns a mutable reference to the global attribute signal listener registry.
///
/// Lazily initializes the registry on first access via `Box::leak`.
/// Maps `Signal<String>` inner pointer addresses to the `JsValue` reference
/// of the corresponding `__euv_signal_update__` event listener closure.
///
/// # Returns
///
/// - `&'static mut HashMap<usize, JsValue>` - A mutable reference to the global attribute signal listener registry.
#[cfg(target_arch = "wasm32")]
pub(crate) fn get_attr_signal_listener_registry() -> &'static mut HashMap<usize, JsValue> {
    unsafe {
        if ATTR_SIGNAL_LISTENER_REGISTRY.is_null() {
            let registry: Box<HashMap<usize, JsValue>> = Box::default();
            ATTR_SIGNAL_LISTENER_REGISTRY = Box::leak(registry) as *mut HashMap<usize, JsValue>;
        }
        &mut *ATTR_SIGNAL_LISTENER_REGISTRY
    }
}

/// Registers a `__euv_signal_update__` event listener for a DynamicNode placeholder.
///
/// If a previous listener was registered for the same `dynamic_id`, it is removed
/// from the window before the new one is added. This prevents listener accumulation
/// when match arms switch during route changes.
///
/// # Arguments
///
/// - `usize` - The `data-euv-dynamic-id` value identifying the placeholder element.
/// - `Closure<dyn FnMut()>` - The re-render closure to register as the event listener.
#[cfg(target_arch = "wasm32")]
pub(crate) fn register_dynamic_listener(dynamic_id: usize, closure: Closure<dyn FnMut()>) {
    let event_name: String = NativeEventName::EuvSignalUpdate.to_string();
    let registry: &mut HashMap<usize, JsValue> = get_dynamic_listener_registry();
    if let Some(old_js_value) = registry.remove(&dynamic_id) {
        let window: Window = window().unwrap();
        let _ =
            window.remove_event_listener_with_callback(&event_name, old_js_value.unchecked_ref());
    }
    let js_value: JsValue = closure.as_ref().clone();
    let window: Window = window().unwrap();
    window
        .add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
    registry.insert(dynamic_id, js_value);
}

/// Registers a `__euv_signal_update__` event listener for an attribute signal.
///
/// If a previous listener was registered for the same signal pointer, it is removed
/// from the window before the new one is added. This prevents listener accumulation
/// when the same signal slot is reused across match arm switches.
///
/// # Arguments
///
/// - `usize` - The inner pointer address of the `Signal<String>`.
/// - `Closure<dyn FnMut()>` - The attribute recomputation closure to register.
#[cfg(target_arch = "wasm32")]
pub(crate) fn register_attr_signal_listener(signal_key: usize, closure: Closure<dyn FnMut()>) {
    let event_name: String = NativeEventName::EuvSignalUpdate.to_string();
    let registry: &mut HashMap<usize, JsValue> = get_attr_signal_listener_registry();
    if let Some(old_js_value) = registry.remove(&signal_key) {
        let window: Window = window().unwrap();
        let _ =
            window.remove_event_listener_with_callback(&event_name, old_js_value.unchecked_ref());
    }
    let js_value: JsValue = closure.as_ref().clone();
    let window: Window = window().unwrap();
    window
        .add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();
    registry.insert(signal_key, js_value);
}

/// No-op stub for `register_dynamic_listener` on non-WASM targets.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn register_dynamic_listener(_dynamic_id: usize, _closure: Closure<dyn FnMut()>) {}
