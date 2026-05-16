use crate::*;

/// Reactive state for the browser API demo page.
///
/// Aggregates all signals needed for the localStorage, sessionStorage,
/// clipboard, window, navigator, location, and console sections.
///
/// # Fields
///
/// - `Signal<String>`: The localStorage key input.
/// - `Signal<String>`: The localStorage value input.
/// - `Signal<String>`: The localStorage operation result.
/// - `Signal<String>`: The sessionStorage key input.
/// - `Signal<String>`: The sessionStorage value input.
/// - `Signal<String>`: The sessionStorage operation result.
/// - `Signal<String>`: The clipboard text input.
/// - `Signal<String>`: The clipboard operation result.
/// - `Signal<String>`: The window size display.
/// - `Signal<String>`: The user agent string.
/// - `Signal<String>`: The navigator language.
/// - `Signal<String>`: The location href.
/// - `Signal<String>`: The location origin.
/// - `Signal<String>`: The location pathname.
/// - `Signal<String>`: The console message input.
#[derive(Clone, Copy)]
pub struct UseBrowserApi {
    pub local_key: Signal<String>,
    pub local_value: Signal<String>,
    pub local_result: Signal<String>,
    pub session_key: Signal<String>,
    pub session_value: Signal<String>,
    pub session_result: Signal<String>,
    pub clipboard_text: Signal<String>,
    pub clipboard_result: Signal<String>,
    pub window_size: Signal<String>,
    pub user_agent: Signal<String>,
    pub language: Signal<String>,
    pub location_url: Signal<String>,
    pub location_origin_val: Signal<String>,
    pub location_pathname_val: Signal<String>,
    pub console_input: Signal<String>,
}

/// Creates browser API demo state signals wrapped in a `UseBrowserApi` struct.
///
/// # Returns
///
/// - `UseBrowserApi`: The browser API demo state.
pub fn use_browser_api() -> UseBrowserApi {
    UseBrowserApi {
        local_key: use_signal(|| "euv-demo-key".to_string()),
        local_value: use_signal(|| "".to_string()),
        local_result: use_signal(|| "No data yet".to_string()),
        session_key: use_signal(|| "euv-session-key".to_string()),
        session_value: use_signal(|| "".to_string()),
        session_result: use_signal(|| "No data yet".to_string()),
        clipboard_text: use_signal(|| "".to_string()),
        clipboard_result: use_signal(|| "".to_string()),
        window_size: use_signal(|| {
            let (width, height): (i32, i32) = window_inner_size();
            format!("{} x {}", width, height)
        }),
        user_agent: use_signal(navigator_user_agent),
        language: use_signal(navigator_language),
        location_url: use_signal(location_href),
        location_origin_val: use_signal(location_origin),
        location_pathname_val: use_signal(location_pathname),
        console_input: use_signal(|| "".to_string()),
    }
}

/// Reads a value from the browser localStorage.
///
/// # Arguments
///
/// - `&str`: The key to look up.
///
/// # Returns
///
/// - `Option<String>`: The stored value if found, or None.
pub fn local_storage_get(key: &str) -> Option<String> {
    let window: Window = window().expect("no global window exists");
    let storage: Storage = window.local_storage().ok()??;
    storage.get_item(key).ok()?
}

/// Writes a key-value pair to the browser localStorage.
///
/// # Arguments
///
/// - `&str`: The key to store.
/// - `&str`: The value to store.
pub fn local_storage_set(key: &str, value: &str) {
    let window: Window = window().expect("no global window exists");
    let storage: Storage = match window.local_storage() {
        Ok(Some(s)) => s,
        _ => return,
    };
    let _ = storage.set_item(key, value);
}

/// Removes a key from the browser localStorage.
///
/// # Arguments
///
/// - `&str`: The key to remove.
pub fn local_storage_remove(key: &str) {
    let window: Window = window().expect("no global window exists");
    let storage: Storage = match window.local_storage() {
        Ok(Some(s)) => s,
        _ => return,
    };
    let _ = storage.remove_item(key);
}

/// Reads a value from the browser sessionStorage.
///
/// # Arguments
///
/// - `&str`: The key to look up.
///
/// # Returns
///
/// - `Option<String>`: The stored value if found, or None.
pub fn session_storage_get(key: &str) -> Option<String> {
    let window: Window = window().expect("no global window exists");
    let storage: Storage = window.session_storage().ok()??;
    storage.get_item(key).ok()?
}

/// Writes a key-value pair to the browser sessionStorage.
///
/// # Arguments
///
/// - `&str`: The key to store.
/// - `&str`: The value to store.
pub fn session_storage_set(key: &str, value: &str) {
    let window: Window = window().expect("no global window exists");
    let storage: Storage = match window.session_storage() {
        Ok(Some(s)) => s,
        _ => return,
    };
    let _ = storage.set_item(key, value);
}

/// Removes a key from the browser sessionStorage.
///
/// # Arguments
///
/// - `&str`: The key to remove.
pub fn session_storage_remove(key: &str) {
    let window: Window = window().expect("no global window exists");
    let storage: Storage = match window.session_storage() {
        Ok(Some(s)) => s,
        _ => return,
    };
    let _ = storage.remove_item(key);
}

/// Reads text from the system clipboard asynchronously.
///
/// # Returns
///
/// - `String`: The clipboard text content, or an error message.
pub async fn clipboard_read_text() -> String {
    let window: Window = window().expect("no global window exists");
    let navigator: Navigator = window.navigator();
    let clipboard: Clipboard = navigator.clipboard();
    let promise: js_sys::Promise = clipboard.read_text();
    let future: wasm_bindgen_futures::JsFuture = wasm_bindgen_futures::JsFuture::from(promise);
    match future.await {
        Ok(value) => value
            .as_string()
            .unwrap_or_else(|| "No text content".to_string()),
        Err(_) => "Failed to read clipboard".to_string(),
    }
}

/// Writes text to the system clipboard asynchronously.
///
/// # Arguments
///
/// - `&str`: The text to write.
///
/// # Returns
///
/// - `bool`: Whether the write succeeded.
pub async fn clipboard_write_text(text: &str) -> bool {
    let window: Window = window().expect("no global window exists");
    let navigator: Navigator = window.navigator();
    let clipboard: Clipboard = navigator.clipboard();
    let promise: js_sys::Promise = clipboard.write_text(text);
    let future: wasm_bindgen_futures::JsFuture = wasm_bindgen_futures::JsFuture::from(promise);
    future.await.is_ok()
}

/// Reads the browser window inner dimensions.
///
/// # Returns
///
/// - `(i32, i32)`: A tuple of (inner_width, inner_height).
pub fn window_inner_size() -> (i32, i32) {
    let window: Window = window().expect("no global window exists");
    let width: i32 = window
        .inner_width()
        .ok()
        .and_then(|v| v.as_f64())
        .map(|v| v as i32)
        .unwrap_or(0);
    let height: i32 = window
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .map(|v| v as i32)
        .unwrap_or(0);
    (width, height)
}

/// Reads the browser navigator user agent string.
///
/// # Returns
///
/// - `String`: The user agent string.
pub fn navigator_user_agent() -> String {
    let window: Window = window().expect("no global window exists");
    window
        .navigator()
        .user_agent()
        .unwrap_or_else(|_| "Unknown".to_string())
}

/// Reads the browser navigator language.
///
/// # Returns
///
/// - `String`: The preferred language string.
pub fn navigator_language() -> String {
    let window: Window = window().expect("no global window exists");
    window
        .navigator()
        .language()
        .unwrap_or_else(|| "Unknown".to_string())
}

/// Reads the current browser location href.
///
/// # Returns
///
/// - `String`: The current full URL.
pub fn location_href() -> String {
    let window: Window = window().expect("no global window exists");
    window
        .location()
        .href()
        .unwrap_or_else(|_| "Unknown".to_string())
}

/// Reads the current browser location origin.
///
/// # Returns
///
/// - `String`: The origin portion of the URL.
pub fn location_origin() -> String {
    let window: Window = window().expect("no global window exists");
    window
        .location()
        .origin()
        .unwrap_or_else(|_| "Unknown".to_string())
}

/// Reads the current browser location pathname.
///
/// # Returns
///
/// - `String`: The pathname portion of the URL.
pub fn location_pathname() -> String {
    let window: Window = window().expect("no global window exists");
    window
        .location()
        .pathname()
        .unwrap_or_else(|_| "Unknown".to_string())
}

/// Creates a click event handler that sets a localStorage item.
///
/// # Arguments
///
/// - `UseBrowserApi`: The browser API state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to set the localStorage item.
pub fn local_storage_on_set(state: UseBrowserApi) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let key: String = state.local_key.get();
        let value: String = state.local_value.get();
        if !key.is_empty() {
            local_storage_set(&key, &value);
            state.local_result.set(format!("Set: {} = {}", key, value));
        }
    })
}

/// Creates a click event handler that gets a localStorage item.
///
/// # Arguments
///
/// - `UseBrowserApi`: The browser API state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to get the localStorage item.
pub fn local_storage_on_get(state: UseBrowserApi) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let key: String = state.local_key.get();
        let value: Option<String> = local_storage_get(&key);
        match value {
            Some(v) => state.local_result.set(format!("Get: {} = {}", key, v)),
            None => state.local_result.set(format!("Key '{}' not found", key)),
        }
    })
}

/// Creates a click event handler that removes a localStorage item.
///
/// # Arguments
///
/// - `UseBrowserApi`: The browser API state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to remove the localStorage item.
pub fn local_storage_on_remove(state: UseBrowserApi) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let key: String = state.local_key.get();
        local_storage_remove(&key);
        state.local_result.set(format!("Removed key: {}", key));
    })
}

/// Creates a click event handler that sets a sessionStorage item.
///
/// # Arguments
///
/// - `UseBrowserApi`: The browser API state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to set the sessionStorage item.
pub fn session_storage_on_set(state: UseBrowserApi) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let key: String = state.session_key.get();
        let value: String = state.session_value.get();
        if !key.is_empty() {
            session_storage_set(&key, &value);
            state
                .session_result
                .set(format!("Set: {} = {}", key, value));
        }
    })
}

/// Creates a click event handler that gets a sessionStorage item.
///
/// # Arguments
///
/// - `UseBrowserApi`: The browser API state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to get the sessionStorage item.
pub fn session_storage_on_get(state: UseBrowserApi) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let key: String = state.session_key.get();
        let value: Option<String> = session_storage_get(&key);
        match value {
            Some(v) => state.session_result.set(format!("Get: {} = {}", key, v)),
            None => state.session_result.set(format!("Key '{}' not found", key)),
        }
    })
}

/// Creates a click event handler that removes a sessionStorage item.
///
/// # Arguments
///
/// - `UseBrowserApi`: The browser API state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to remove the sessionStorage item.
pub fn session_storage_on_remove(state: UseBrowserApi) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let key: String = state.session_key.get();
        session_storage_remove(&key);
        state.session_result.set(format!("Removed key: {}", key));
    })
}

/// Creates a click event handler that copies text to clipboard.
///
/// # Arguments
///
/// - `UseBrowserApi`: The browser API state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to copy text to clipboard.
pub fn clipboard_on_copy(state: UseBrowserApi) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let text: String = state.clipboard_text.get();
        let text_clone: String = text.clone();
        let result: Signal<String> = state.clipboard_result;
        if text.is_empty() {
            result.set("Please enter text to copy".to_string());
        } else {
            wasm_bindgen_futures::spawn_local(async move {
                let success: bool = clipboard_write_text(&text_clone).await;
                if success {
                    result.set("Copied to clipboard!".to_string());
                } else {
                    result.set("Failed to copy".to_string());
                }
            });
        }
    })
}

/// Creates a click event handler that reads text from clipboard.
///
/// # Arguments
///
/// - `UseBrowserApi`: The browser API state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to read text from clipboard.
pub fn clipboard_on_paste(state: UseBrowserApi) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let result: Signal<String> = state.clipboard_result;
        wasm_bindgen_futures::spawn_local(async move {
            let text: String = clipboard_read_text().await;
            result.set(format!("Pasted: {}", text));
        });
    })
}

/// Creates a click event handler that refreshes the window size display.
///
/// # Arguments
///
/// - `UseBrowserApi`: The browser API state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to refresh the window size.
pub fn window_on_refresh_size(state: UseBrowserApi) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let (width, height): (i32, i32) = window_inner_size();
        state.window_size.set(format!("{} x {}", width, height));
    })
}

/// Creates a click event handler that logs a console message.
///
/// # Arguments
///
/// - `Signal<String>`: The console input signal.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler for console.log.
pub fn console_on_log(console_input: Signal<String>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let message: String = console_input.get();
        Console::log(&message);
    })
}

/// Creates a click event handler that warns a console message.
///
/// # Arguments
///
/// - `Signal<String>`: The console input signal.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler for console.warn.
pub fn console_on_warn(console_input: Signal<String>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let message: String = console_input.get();
        Console::warn(&message);
    })
}

/// Creates a click event handler that errors a console message.
///
/// # Arguments
///
/// - `Signal<String>`: The console input signal.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler for console.error.
pub fn console_on_error(console_input: Signal<String>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let message: String = console_input.get();
        Console::error(&message);
    })
}
