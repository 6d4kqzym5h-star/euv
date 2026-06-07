use crate::*;

/// Creates native bridge state signals wrapped in a `UseNativeBridge` struct.
///
/// Initializes all signals with default values and sets `available` to `false`.
/// The actual data is loaded asynchronously via `load_native_bridge_data`.
///
/// # Returns
///
/// - `UseNativeBridge` - The native bridge state.
pub(crate) fn use_native_bridge() -> UseNativeBridge {
    UseNativeBridge::new(
        use_signal(|| false),
        use_signal(|| true),
        use_signal(|| false),
        use_signal(String::new),
        use_signal(String::new),
        use_signal(String::new),
        use_signal(String::new),
    )
}

/// Checks whether the Tauri native bridge is available on the current platform.
///
/// Looks up `window.__TAURI__.core` via `js_sys::Reflect` to determine if the
/// Tauri runtime is present. Returns `false` if the property chain does not exist
/// or if any reflection error occurs.
///
/// # Returns
///
/// - `bool` - `true` if the Tauri core module is available.
pub(crate) fn is_tauri_available() -> bool {
    let window_value: Window = window().expect("no global window exists");
    let tauri_key: JsValue = JsValue::from_str("__TAURI__");
    let tauri_obj: JsValue = match Reflect::get(&window_value, &tauri_key) {
        Ok(value) => value,
        Err(_) => return false,
    };
    if tauri_obj.is_undefined() || tauri_obj.is_null() {
        return false;
    }
    let core_key: JsValue = JsValue::from_str("core");
    let core_obj: JsValue = match Reflect::get(&tauri_obj, &core_key) {
        Ok(value) => value,
        Err(_) => return false,
    };
    !core_obj.is_undefined() && !core_obj.is_null()
}

/// Invokes a Tauri core command by name via `window.__TAURI__.core.invoke`.
///
/// Resolves the `__TAURI__` → `core` → `invoke` property chain on the global
/// `window` object, then calls `invoke` with the given command name and
/// optional arguments object. Returns the resulting `js_sys::Promise`, or an
/// error string if any step in the reflection chain fails.
///
/// # Arguments
///
/// - `&str` - The Tauri command name to invoke.
/// - `Option<&JsValue>` - Optional arguments object to pass to the command.
///
/// # Returns
///
/// - `Result<js_sys::Promise, String>` - The promise returned by the invoke call, or an error message.
pub(crate) fn tauri_invoke(
    command: &str,
    args: Option<&JsValue>,
) -> Result<js_sys::Promise, String> {
    let window_value: Window = window().expect("no global window exists");
    let tauri_key: JsValue = JsValue::from_str("__TAURI__");
    let tauri_obj: JsValue =
        Reflect::get(&window_value, &tauri_key).map_err(|error: JsValue| format!("{error:?}"))?;
    let core_key: JsValue = JsValue::from_str("core");
    let core_obj: JsValue =
        Reflect::get(&tauri_obj, &core_key).map_err(|error: JsValue| format!("{error:?}"))?;
    let invoke_key: JsValue = JsValue::from_str("invoke");
    let invoke_fn: JsValue =
        Reflect::get(&core_obj, &invoke_key).map_err(|error: JsValue| format!("{error:?}"))?;
    let invoke_function: Function = invoke_fn
        .dyn_into::<Function>()
        .map_err(|error: JsValue| format!("{error:?}"))?;
    let command_value: JsValue = JsValue::from_str(command);
    let result: JsValue = match args {
        Some(arguments) => invoke_function
            .call2(&core_obj, &command_value, arguments)
            .map_err(|error: JsValue| format!("{error:?}"))?,
        None => invoke_function
            .call1(&core_obj, &command_value)
            .map_err(|error: JsValue| format!("{error:?}"))?,
    };
    result
        .dyn_into::<js_sys::Promise>()
        .map_err(|error: JsValue| format!("{error:?}"))
}

/// Asynchronously loads native bridge data and updates the provided state signals.
///
/// First checks platform availability via `is_tauri_available`. If unavailable,
/// sets `available` to `false` and returns. Otherwise, invokes both
/// `load_cached_resource` and `resolve_bridge_group_permissions` commands,
/// then populates the corresponding signals from the results. If any invoke
/// fails, sets `available` to `false` so the card is hidden.
///
/// # Arguments
///
/// - `UseNativeBridge` - The native bridge state to populate.
pub(crate) fn load_native_bridge_data(state: UseNativeBridge) {
    if !is_tauri_available() {
        state.get_available().set(false);
        state.get_loading().set(false);
        return;
    }
    let cached_resource_state: UseNativeBridge = state;
    let permissions_state: UseNativeBridge = state;
    spawn_local(async move {
        let load_result: Result<JsValue, String> =
            match tauri_invoke(TAURI_INVOKE_LOAD_CACHED_RESOURCE, None) {
                Ok(promise) => {
                    let future: JsFuture = JsFuture::from(promise);
                    match future.await {
                        Ok(value) => Ok(value),
                        Err(error) => Err(format!("{error:?}")),
                    }
                }
                Err(error) => Err(error),
            };
        match load_result {
            Ok(value) => {
                let from_cache_value: bool = Reflect::get(&value, &JsValue::from_str("from_cache"))
                    .ok()
                    .and_then(|v: JsValue| v.as_bool())
                    .unwrap_or(false);
                let remote_url_value: String =
                    Reflect::get(&value, &JsValue::from_str("remote_url"))
                        .ok()
                        .and_then(|v: JsValue| v.as_string())
                        .unwrap_or_default();
                let source_value: String = Reflect::get(&value, &JsValue::from_str("source"))
                    .ok()
                    .and_then(|v: JsValue| v.as_string())
                    .unwrap_or_default();
                let cache_path_value: String =
                    Reflect::get(&value, &JsValue::from_str("cache_path"))
                        .ok()
                        .and_then(|v: JsValue| v.as_string())
                        .unwrap_or_default();
                cached_resource_state.get_from_cache().set(from_cache_value);
                cached_resource_state.get_remote_url().set(remote_url_value);
                cached_resource_state.get_source().set(source_value);
                cached_resource_state.get_cache_path().set(cache_path_value);
            }
            Err(_) => {
                cached_resource_state.get_available().set(false);
                cached_resource_state.get_loading().set(false);
                return;
            }
        }
        cached_resource_state.get_available().set(true);
        cached_resource_state.get_loading().set(false);
    });
    spawn_local(async move {
        let args_obj: js_sys::Object = js_sys::Object::new();
        Reflect::set(
            &args_obj,
            &JsValue::from_str("group"),
            &JsValue::from_str(TAURI_BRIDGE_GROUP_ALL),
        )
        .unwrap_or(false);
        let permissions_result: Result<JsValue, String> = match tauri_invoke(
            TAURI_INVOKE_RESOLVE_BRIDGE_GROUP_PERMISSIONS,
            Some(&args_obj),
        ) {
            Ok(promise) => {
                let future: JsFuture = JsFuture::from(promise);
                match future.await {
                    Ok(value) => Ok(value),
                    Err(error) => Err(format!("{error:?}")),
                }
            }
            Err(error) => Err(error),
        };
        match permissions_result {
            Ok(value) => {
                let permissions_array: Vec<String> = value
                    .dyn_into::<js_sys::Array>()
                    .map(|array: js_sys::Array| {
                        array
                            .iter()
                            .filter_map(|item: JsValue| item.as_string())
                            .collect::<Vec<String>>()
                    })
                    .unwrap_or_default();
                permissions_state
                    .get_permissions()
                    .set(permissions_array.join(", "));
            }
            Err(_) => {
                permissions_state.get_available().set(false);
            }
        }
    });
}
