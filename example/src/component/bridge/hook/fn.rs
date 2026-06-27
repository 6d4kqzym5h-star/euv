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
        use_signal(String::new),
    )
}

/// Creates cache update state signals wrapped in a `UseCacheUpdate` struct.
///
/// Initializes `doc_status` to `false`, `version` to an empty string,
/// and `updating` to `false`. The actual data is loaded asynchronously
/// via `load_cache_update`.
///
/// # Returns
///
/// - `UseCacheUpdate` - The cache update state.
pub(crate) fn use_cache_update() -> UseCacheUpdate {
    UseCacheUpdate::new(
        use_signal(|| false),
        use_signal(String::new),
        use_signal(|| false),
    )
}

/// Checks whether the bridge native bridge is available on the current platform.
///
/// Looks up `window.___.core` via `Reflect` to determine if the
/// bridge runtime is present. Returns `false` if the property chain does not exist
/// or if any reflection error occurs.
///
/// # Returns
///
/// - `bool` - `true` if the bridge core module is available.
pub(crate) fn is_bridge_available() -> bool {
    let window_value: Window = window().expect("no global window exists");
    let bridge_key: JsValue = JsValue::from_str(BRIDGE_GLOBAL_KEY);
    let bridge_obj: JsValue = match Reflect::get(&window_value, &bridge_key) {
        Ok(value) => value,
        Err(_) => return false,
    };
    if bridge_obj.is_undefined() || bridge_obj.is_null() {
        return false;
    }
    let core_key: JsValue = JsValue::from_str(CORE_KEY);
    let core_obj: JsValue = match Reflect::get(&bridge_obj, &core_key) {
        Ok(value) => value,
        Err(_) => return false,
    };
    !core_obj.is_undefined() && !core_obj.is_null()
}

/// Invokes a bridge core command by name via `window.___.core.invoke`.
///
/// Resolves the `___` → `core` → `invoke` property chain on the global
/// `window` object, then calls `invoke` with the given command name and
/// optional arguments object. Returns the resulting `Promise`, or an
/// error string if any step in the reflection chain fails.
///
/// # Arguments
///
/// - `&str` - The bridge command name to invoke.
/// - `Option<&JsValue>` - Optional arguments object to pass to the command.
///
/// # Returns
///
/// - `Result<Promise, String>` - The promise returned by the invoke call, or an error message.
pub(crate) fn bridge_invoke(command: &str, args: Option<&JsValue>) -> Result<Promise, String> {
    let window_value: Window = window().expect("no global window exists");
    let bridge_key: JsValue = JsValue::from_str(BRIDGE_GLOBAL_KEY);
    let bridge_obj: JsValue =
        Reflect::get(&window_value, &bridge_key).map_err(|error: JsValue| format!("{error:?}"))?;
    let core_key: JsValue = JsValue::from_str(CORE_KEY);
    let core_obj: JsValue =
        Reflect::get(&bridge_obj, &core_key).map_err(|error: JsValue| format!("{error:?}"))?;
    let invoke_key: JsValue = JsValue::from_str(INVOKE_KEY);
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
        .dyn_into::<Promise>()
        .map_err(|error: JsValue| format!("{error:?}"))
}

/// Asynchronously loads native bridge data and updates the provided state signals.
///
/// First checks platform availability via `is_bridge_available`. If unavailable,
/// sets `available` to `false` and returns. Otherwise, invokes the
/// `resolve_bridge_group_permissions` command, then populates the corresponding
/// signal from the result. If the invoke fails, sets `available` to `false`
/// so the card is hidden.
///
/// # Arguments
///
/// - `UseNativeBridge` - The native bridge state to populate.
pub(crate) fn load_native_bridge_data(state: UseNativeBridge) {
    if !is_bridge_available() {
        state.get_available().set(false);
        state.get_loading().set(false);
        return;
    }
    let permissions_state: UseNativeBridge = state;
    spawn_local(async move {
        let args_obj: Object = Object::new();
        Reflect::set(
            &args_obj,
            &JsValue::from_str(BRIDGE_GROUP_KEY),
            &JsValue::from_str(BRIDGE_GROUP_ALL),
        )
        .unwrap_or(false);
        let permissions_result: Result<JsValue, String> =
            match bridge_invoke(INVOKE_RESOLVE_BRIDGE_GROUP_PERMISSIONS, Some(&args_obj)) {
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
                    .dyn_into::<Array>()
                    .map(|array: Array| {
                        array
                            .iter()
                            .filter_map(|item: JsValue| item.as_string())
                            .collect::<Vec<String>>()
                    })
                    .unwrap_or_default();
                permissions_state
                    .get_permissions()
                    .set(permissions_array.join(", "));
                permissions_state.get_available().set(true);
            }
            Err(_) => {
                permissions_state.get_available().set(false);
            }
        }
        permissions_state.get_loading().set(false);
    });
}

/// Asynchronously fetches the docs.rs status JSON and invokes the bridge
/// `update_cache` command to synchronize the local cache.
///
/// First fetches `DOCS_STATUS_URL` and parses the `{ "doc_status": bool,
/// "version": string }` JSON payload into the provided state signals.
/// If the remote version is greater than `EUV_VERSION` and the bridge
/// is available, invokes the `update_cache` command via
/// `window.bridge.core.invoke("update_cache")` and updates the
/// `updating` signal accordingly. Otherwise, logs that the current
/// version is already the latest.
///
/// If the network request or response parsing fails at any stage,
/// the function returns early without invoking the bridge.
///
/// # Arguments
///
/// - `UseCacheUpdate` - The cache update state to populate.
pub(crate) fn load_cache_update(state: UseCacheUpdate) {
    let doc_status_signal: Signal<bool> = state.get_doc_status();
    let version_signal: Signal<String> = state.get_version();
    let updating_signal: Signal<bool> = state.get_updating();
    spawn_local(async move {
        let window_value: Window = window().expect("no global window exists");
        let promise: Promise = window_value.fetch_with_str(DOCS_STATUS_URL);
        let future: JsFuture = JsFuture::from(promise);
        let response: JsValue = match future.await {
            Ok(value) => value,
            Err(error) => {
                Console::error(&format!(
                    "Failed to fetch version status: {}",
                    error.as_string().unwrap_or_default()
                ));
                return;
            }
        };
        let response_value: Response = match response.dyn_into() {
            Ok(value) => value,
            Err(error) => {
                Console::error(&format!(
                    "Failed to convert fetch response: {}",
                    error.as_string().unwrap_or_default()
                ));
                return;
            }
        };
        let text_promise: Promise = match response_value.text() {
            Ok(promise) => promise,
            Err(error) => {
                Console::error(&format!(
                    "Failed to get response text promise: {}",
                    error.as_string().unwrap_or_default()
                ));
                return;
            }
        };
        let text_future: JsFuture = JsFuture::from(text_promise);
        let text: JsValue = match text_future.await {
            Ok(value) => value,
            Err(error) => {
                Console::error(&format!(
                    "Failed to read response text: {}",
                    error.as_string().unwrap_or_default()
                ));
                return;
            }
        };
        let text_string: String = text.as_string().unwrap_or_default();
        Console::log(&text_string);
        let parsed: DocsStatus =
            serde_json::from_str::<DocsStatus>(&text_string).unwrap_or_default();
        doc_status_signal.set(parsed.get_doc_status());
        version_signal.set(parsed.get_version().clone());
        if !matches!(
            CompareVersion::compare_version(parsed.get_version(), EUV_VERSION),
            Ok(VersionLevel::Greater)
        ) {
            Console::log(&format!(
                "Current version v{EUV_VERSION} is already the latest version"
            ));
            return;
        }
        if !is_bridge_available() {
            return;
        }
        updating_signal.set(true);
        if let Ok(promise) = bridge_invoke(INVOKE_UPDATE_CACHE, None) {
            let future: JsFuture = JsFuture::from(promise);
            match future.await {
                Ok(result) => Console::log(&result.as_string().unwrap_or_default()),
                Err(error) => Console::error(&error.as_string().unwrap_or_default()),
            }
        }
        updating_signal.set(false);
    });
}
