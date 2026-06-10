use crate::*;

/// Creates browser API demo state signals wrapped in a `UseBrowserApi` struct.
///
/// # Returns
///
/// - `UseBrowserApi` - The browser API demo state.
pub(crate) fn use_browser_api() -> UseBrowserApi {
    let mut state: UseBrowserApi = UseBrowserApi::default();
    state.set_local_key(use_signal(|| "euv-demo-key".to_string()));
    state.set_local_value(use_signal(String::new));
    state.set_local_result(use_signal(|| "No data yet".to_string()));
    state.set_session_key(use_signal(|| "euv-session-key".to_string()));
    state.set_session_value(use_signal(String::new));
    state.set_session_result(use_signal(|| "No data yet".to_string()));
    state.set_clipboard_text(use_signal(String::new));
    state.set_clipboard_result(use_signal(String::new));
    state.set_window_size(use_signal(|| {
        let (width, height): (i32, i32) = window_inner_size();
        format!("{} x {}", width, height)
    }));
    state.set_user_agent(use_signal(navigator_user_agent));
    state.set_language(use_signal(navigator_language));
    state.set_location_url(use_signal(location_href));
    state.set_location_origin_val(use_signal(location_origin));
    state.set_location_pathname_val(use_signal(location_pathname));
    state.set_console_input(use_signal(String::new));
    state
}

/// Reads a value from the browser localStorage.
///
/// # Arguments
///
/// - `&str` - The key to look up.
///
/// # Returns
///
/// - `Option<String>` - The stored value if found, or None.
pub(crate) fn local_storage_get(key: &str) -> Option<String> {
    let window: Window = window().expect("no global window exists");
    let storage: Storage = window.local_storage().ok()??;
    storage.get_item(key).ok()?
}

/// Writes a key-value pair to the browser localStorage.
///
/// # Arguments
///
/// - `&str` - The key to store.
/// - `&str` - The value to store.
pub(crate) fn local_storage_set(key: &str, value: &str) {
    let window: Window = window().expect("no global window exists");
    let storage: Storage = match window.local_storage() {
        Ok(Some(local_storage)) => local_storage,
        _ => return,
    };
    let _ = storage.set_item(key, value);
}

/// Removes a key from the browser localStorage.
///
/// # Arguments
///
/// - `&str` - The key to remove.
pub(crate) fn local_storage_remove(key: &str) {
    let window: Window = window().expect("no global window exists");
    let storage: Storage = match window.local_storage() {
        Ok(Some(local_storage)) => local_storage,
        _ => return,
    };
    let _ = storage.remove_item(key);
}

/// Reads a value from the browser sessionStorage.
///
/// # Arguments
///
/// - `&str` - The key to look up.
///
/// # Returns
///
/// - `Option<String>` - The stored value if found, or None.
pub(crate) fn session_storage_get(key: &str) -> Option<String> {
    let window: Window = window().expect("no global window exists");
    let storage: Storage = window.session_storage().ok()??;
    storage.get_item(key).ok()?
}

/// Writes a key-value pair to the browser sessionStorage.
///
/// # Arguments
///
/// - `&str` - The key to store.
/// - `&str` - The value to store.
pub(crate) fn session_storage_set(key: &str, value: &str) {
    let window: Window = window().expect("no global window exists");
    let storage: Storage = match window.session_storage() {
        Ok(Some(session_storage)) => session_storage,
        _ => return,
    };
    let _ = storage.set_item(key, value);
}

/// Removes a key from the browser sessionStorage.
///
/// # Arguments
///
/// - `&str` - The key to remove.
pub(crate) fn session_storage_remove(key: &str) {
    let window: Window = window().expect("no global window exists");
    let storage: Storage = match window.session_storage() {
        Ok(Some(session_storage)) => session_storage,
        _ => return,
    };
    let _ = storage.remove_item(key);
}

/// Reads text from the system clipboard asynchronously.
///
/// # Returns
///
/// - `String` - The clipboard text content, or an error message.
pub(crate) async fn clipboard_read_text() -> String {
    let window: Window = window().expect("no global window exists");
    let navigator: Navigator = window.navigator();
    let clipboard: Clipboard = navigator.clipboard();
    let promise: Promise = clipboard.read_text();
    let future: JsFuture = JsFuture::from(promise);
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
/// - `&str` - The text to write.
///
/// # Returns
///
/// - `bool` - Whether the write succeeded.
pub(crate) async fn clipboard_write_text(text: &str) -> bool {
    let window: Window = window().expect("no global window exists");
    let navigator: Navigator = window.navigator();
    let clipboard: Clipboard = navigator.clipboard();
    let promise: Promise = clipboard.write_text(text);
    let future: JsFuture = JsFuture::from(promise);
    future.await.is_ok()
}

/// Reads the browser window inner dimensions.
///
/// # Returns
///
/// - `(i32, i32)` - A tuple of (inner_width, inner_height).
pub(crate) fn window_inner_size() -> (i32, i32) {
    let window: Window = window().expect("no global window exists");
    let width: i32 = window
        .inner_width()
        .ok()
        .map(|value: JsValue| Number::from(value).value_of() as i32)
        .unwrap_or_default();
    let height: i32 = window
        .inner_height()
        .ok()
        .map(|value: JsValue| Number::from(value).value_of() as i32)
        .unwrap_or_default();
    (width, height)
}

/// Reads the browser navigator user agent string.
///
/// # Returns
///
/// - `String` - The user agent string.
pub(crate) fn navigator_user_agent() -> String {
    let window: Window = window().expect("no global window exists");
    window
        .navigator()
        .user_agent()
        .unwrap_or_else(|_: JsValue| "Unknown".to_string())
}

/// Reads the browser navigator language.
///
/// # Returns
///
/// - `String` - The preferred language string.
pub(crate) fn navigator_language() -> String {
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
/// - `String` - The current full URL.
pub(crate) fn location_href() -> String {
    let window: Window = window().expect("no global window exists");
    window
        .location()
        .href()
        .unwrap_or_else(|_error: JsValue| "Unknown".to_string())
}

/// Reads the current browser location origin.
///
/// # Returns
///
/// - `String` - The origin portion of the URL.
pub(crate) fn location_origin() -> String {
    let window: Window = window().expect("no global window exists");
    window
        .location()
        .origin()
        .unwrap_or_else(|_error: JsValue| "Unknown".to_string())
}

/// Reads the current browser location pathname.
///
/// # Returns
///
/// - `String` - The pathname portion of the URL.
pub(crate) fn location_pathname() -> String {
    let window: Window = window().expect("no global window exists");
    window
        .location()
        .pathname()
        .unwrap_or_else(|_error: JsValue| "Unknown".to_string())
}

/// Creates a click event handler that sets a localStorage item.
///
/// # Arguments
///
/// - `UseBrowserApi` - The browser API state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to set the localStorage item.
pub(crate) fn local_storage_on_set(state: UseBrowserApi) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let key: String = state.get_local_key().get();
        let value: String = state.get_local_value().get();
        if !key.is_empty() {
            local_storage_set(&key, &value);
            state
                .get_local_result()
                .set(format!("Set: {} = {}", key, value));
        }
    }))
}

/// Creates a click event handler that gets a localStorage item.
///
/// # Arguments
///
/// - `UseBrowserApi` - The browser API state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to get the localStorage item.
pub(crate) fn local_storage_on_get(state: UseBrowserApi) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let key: String = state.get_local_key().get();
        let value: Option<String> = local_storage_get(&key);
        match value {
            Some(v) => state
                .get_local_result()
                .set(format!("Get: {} = {}", key, v)),
            None => state
                .get_local_result()
                .set(format!("Key '{}' not found", key)),
        }
    }))
}

/// Creates a click event handler that removes a localStorage item.
///
/// # Arguments
///
/// - `UseBrowserApi` - The browser API state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to remove the localStorage item.
pub(crate) fn local_storage_on_remove(state: UseBrowserApi) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let key: String = state.get_local_key().get();
        local_storage_remove(&key);
        state
            .get_local_result()
            .set(format!("Removed key: {}", key));
    }))
}

/// Creates a click event handler that sets a sessionStorage item.
///
/// # Arguments
///
/// - `UseBrowserApi` - The browser API state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to set the sessionStorage item.
pub(crate) fn session_storage_on_set(state: UseBrowserApi) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let key: String = state.get_session_key().get();
        let value: String = state.get_session_value().get();
        if !key.is_empty() {
            session_storage_set(&key, &value);
            state
                .get_session_result()
                .set(format!("Set: {} = {}", key, value));
        }
    }))
}

/// Creates a click event handler that gets a sessionStorage item.
///
/// # Arguments
///
/// - `UseBrowserApi` - The browser API state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to get the sessionStorage item.
pub(crate) fn session_storage_on_get(state: UseBrowserApi) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let key: String = state.get_session_key().get();
        let value: Option<String> = session_storage_get(&key);
        match value {
            Some(v) => state
                .get_session_result()
                .set(format!("Get: {} = {}", key, v)),
            None => state
                .get_session_result()
                .set(format!("Key '{}' not found", key)),
        }
    }))
}

/// Creates a click event handler that removes a sessionStorage item.
///
/// # Arguments
///
/// - `UseBrowserApi` - The browser API state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to remove the sessionStorage item.
pub(crate) fn session_storage_on_remove(state: UseBrowserApi) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let key: String = state.get_session_key().get();
        session_storage_remove(&key);
        state
            .get_session_result()
            .set(format!("Removed key: {}", key));
    }))
}

/// Creates a click event handler that copies text to clipboard.
///
/// # Arguments
///
/// - `UseBrowserApi` - The browser API state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to copy text to clipboard.
pub(crate) fn clipboard_on_copy(state: UseBrowserApi) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let text: String = state.get_clipboard_text().get();
        let text_clone: String = text.clone();
        let result: Signal<String> = state.get_clipboard_result();
        if text.is_empty() {
            result.set("Please enter text to copy".to_string());
        } else {
            spawn_local(async move {
                let success: bool = clipboard_write_text(&text_clone).await;
                if success {
                    result.set("Copied to clipboard!".to_string());
                } else {
                    result.set("Failed to copy".to_string());
                }
            });
        }
    }))
}

/// Creates a click event handler that reads text from clipboard.
///
/// # Arguments
///
/// - `UseBrowserApi` - The browser API state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to read text from clipboard.
pub(crate) fn clipboard_on_paste(state: UseBrowserApi) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let result: Signal<String> = state.get_clipboard_result();
        spawn_local(async move {
            let text: String = clipboard_read_text().await;
            result.set(format!("Pasted: {}", text));
        });
    }))
}

/// Creates a click event handler that refreshes the window size display.
///
/// # Arguments
///
/// - `UseBrowserApi` - The browser API state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to refresh the window size.
pub(crate) fn window_on_refresh_size(state: UseBrowserApi) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let (width, height): (i32, i32) = window_inner_size();
        state
            .get_window_size()
            .set(format!("{} x {}", width, height));
    }))
}

/// Creates a click event handler that logs a console message.
///
/// # Arguments
///
/// - `Signal<String>` - The console input signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler for console.log.
pub(crate) fn console_on_log(console_input: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let raw: String = console_input.get();
        let message: &str = if raw.is_empty() {
            CONSOLE_LOG_DEFAULT_MESSAGE
        } else {
            &raw
        };
        Console::log(message);
    }))
}

/// Creates a click event handler that warns a console message.
///
/// # Arguments
///
/// - `Signal<String>` - The console input signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler for console.warn.
pub(crate) fn console_on_warn(console_input: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let raw: String = console_input.get();
        let message: &str = if raw.is_empty() {
            CONSOLE_WARN_DEFAULT_MESSAGE
        } else {
            &raw
        };
        Console::warn(message);
    }))
}

/// Creates a click event handler that errors a console message.
///
/// # Arguments
///
/// - `Signal<String>` - The console input signal.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler for console.error.
pub(crate) fn console_on_error(console_input: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let raw: String = console_input.get();
        let message: &str = if raw.is_empty() {
            CONSOLE_ERROR_DEFAULT_MESSAGE
        } else {
            &raw
        };
        Console::error(message);
    }))
}
