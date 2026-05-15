use crate::*;

/// Reads a value from the browser localStorage.
///
/// # Arguments
///
/// - `&str`: The key to look up.
///
/// # Returns
///
/// - `Option<String>`: The stored value if found, or None.
fn local_storage_get(key: &str) -> Option<String> {
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
fn local_storage_set(key: &str, value: &str) {
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
fn local_storage_remove(key: &str) {
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
fn session_storage_get(key: &str) -> Option<String> {
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
fn session_storage_set(key: &str, value: &str) {
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
fn session_storage_remove(key: &str) {
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
async fn clipboard_read_text() -> String {
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
async fn clipboard_write_text(text: &str) -> bool {
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
fn window_inner_size() -> (i32, i32) {
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
fn navigator_user_agent() -> String {
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
fn navigator_language() -> String {
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
fn location_href() -> String {
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
fn location_origin() -> String {
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
fn location_pathname() -> String {
    let window: Window = window().expect("no global window exists");
    window
        .location()
        .pathname()
        .unwrap_or_else(|_| "Unknown".to_string())
}

/// A browser API demo page showcasing localStorage, sessionStorage, clipboard, window, navigator, and location.
///
/// # Returns
///
/// - `VirtualNode`: The browser API demo page virtual DOM tree.
pub fn page_browser() -> VirtualNode {
    let local_key: Signal<String> = use_signal(|| "euv-demo-key".to_string());
    let local_value: Signal<String> = use_signal(|| "".to_string());
    let local_result: Signal<String> = use_signal(|| "No data yet".to_string());
    let session_key: Signal<String> = use_signal(|| "euv-session-key".to_string());
    let session_value: Signal<String> = use_signal(|| "".to_string());
    let session_result: Signal<String> = use_signal(|| "No data yet".to_string());
    let clipboard_text: Signal<String> = use_signal(|| "".to_string());
    let clipboard_result: Signal<String> = use_signal(|| "".to_string());
    let window_size: Signal<String> = use_signal(|| {
        let (width, height): (i32, i32) = window_inner_size();
        format!("{} x {}", width, height)
    });
    let user_agent: Signal<String> = use_signal(navigator_user_agent);
    let language: Signal<String> = use_signal(navigator_language);
    let location_url: Signal<String> = use_signal(location_href);
    let location_origin_val: Signal<String> = use_signal(location_origin);
    let location_pathname_val: Signal<String> = use_signal(location_pathname);
    let console_input: Signal<String> = use_signal(|| "".to_string());
    html! {
        div {
            class: c_page_container()
            div {
                class: c_page_header()
                h1 {
                    class: c_page_title()
                    "Browser APIs"
                }
                p {
                    class: c_page_subtitle()
                    "Interact with localStorage, sessionStorage, clipboard, window, navigator, and location."
                }
            }
            my_card {
                title: "localStorage"
                p {
                    class: c_demo_text()
                    "Store and retrieve persistent data in the browser."
                }
                div {
                    class: c_browser_api_row()
                    div {
                        class: c_form_input_wrapper()
                        label {
                            class: c_form_label()
                            "Key"
                        }
                        input {
                            r#type: "text"
                            placeholder: "Storage key..."
                            value: local_key
                            class: c_form_input_no_transition()
                            oninput: move |event: NativeEvent| {
                                if let NativeEvent::Input(input_event) = event {
                                    local_key.set(input_event.get_value().clone());
                                }
                            }
                        }
                    }
                    div {
                        class: c_form_input_wrapper()
                        label {
                            class: c_form_label()
                            "Value"
                        }
                        input {
                            r#type: "text"
                            placeholder: "Storage value..."
                            value: local_value
                            class: c_form_input_no_transition()
                            oninput: move |event: NativeEvent| {
                                if let NativeEvent::Input(input_event) = event {
                                    local_value.set(input_event.get_value().clone());
                                }
                            }
                        }
                    }
                }
                div {
                    class: c_browser_api_actions()
                    primary_button {
                        label: "Set"
                        onclick: move |_event: NativeEvent| {
                            let key: String = local_key.get();
                            let value: String = local_value.get();
                            if !key.is_empty() {
                                local_storage_set(&key, &value);
                                local_result.set(format!("Set: {} = {}", key, value));
                            }
                        }
                        "Set Item"
                    }
                    primary_button {
                        label: "Get"
                        onclick: move |_event: NativeEvent| {
                            let key: String = local_key.get();
                            let value: Option<String> = local_storage_get(&key);
                            match value {
                                Some(v) => local_result.set(format!("Get: {} = {}", key, v)),
                                None => local_result.set(format!("Key '{}' not found", key)),
                            }
                        }
                        "Get Item"
                    }
                    primary_button {
                        label: "Remove"
                        onclick: move |_event: NativeEvent| {
                            let key: String = local_key.get();
                            local_storage_remove(&key);
                            local_result.set(format!("Removed key: {}", key));
                        }
                        "Remove Item"
                    }
                }
                div {
                    class: c_browser_result_box()
                    span {
                        class: c_browser_result_label()
                        "Result: "
                    }
                    span {
                        class: c_browser_result_value()
                        local_result
                    }
                }
            }
            my_card {
                title: "sessionStorage"
                p {
                    class: c_demo_text()
                    "Store data for the duration of the page session."
                }
                div {
                    class: c_browser_api_row()
                    div {
                        class: c_form_input_wrapper()
                        label {
                            class: c_form_label()
                            "Key"
                        }
                        input {
                            r#type: "text"
                            placeholder: "Session key..."
                            value: session_key
                            class: c_form_input_no_transition()
                            oninput: move |event: NativeEvent| {
                                if let NativeEvent::Input(input_event) = event {
                                    session_key.set(input_event.get_value().clone());
                                }
                            }
                        }
                    }
                    div {
                        class: c_form_input_wrapper()
                        label {
                            class: c_form_label()
                            "Value"
                        }
                        input {
                            r#type: "text"
                            placeholder: "Session value..."
                            value: session_value
                            class: c_form_input_no_transition()
                            oninput: move |event: NativeEvent| {
                                if let NativeEvent::Input(input_event) = event {
                                    session_value.set(input_event.get_value().clone());
                                }
                            }
                        }
                    }
                }
                div {
                    class: c_browser_api_actions()
                    primary_button {
                        label: "Set"
                        onclick: move |_event: NativeEvent| {
                            let key: String = session_key.get();
                            let value: String = session_value.get();
                            if !key.is_empty() {
                                session_storage_set(&key, &value);
                                session_result.set(format!("Set: {} = {}", key, value));
                            }
                        }
                        "Set Item"
                    }
                    primary_button {
                        label: "Get"
                        onclick: move |_event: NativeEvent| {
                            let key: String = session_key.get();
                            let value: Option<String> = session_storage_get(&key);
                            match value {
                                Some(v) => session_result.set(format!("Get: {} = {}", key, v)),
                                None => session_result.set(format!("Key '{}' not found", key)),
                            }
                        }
                        "Get Item"
                    }
                    primary_button {
                        label: "Remove"
                        onclick: move |_event: NativeEvent| {
                            let key: String = session_key.get();
                            session_storage_remove(&key);
                            session_result.set(format!("Removed key: {}", key));
                        }
                        "Remove Item"
                    }
                }
                div {
                    class: c_browser_result_box()
                    span {
                        class: c_browser_result_label()
                        "Result: "
                    }
                    span {
                        class: c_browser_result_value()
                        session_result
                    }
                }
            }
            my_card {
                title: "Clipboard API"
                p {
                    class: c_demo_text()
                    "Read from and write to the system clipboard."
                }
                div {
                    class: c_form_input_wrapper()
                    label {
                        class: c_form_label()
                        "Text to copy"
                    }
                    input {
                        r#type: "text"
                        placeholder: "Enter text to copy..."
                        value: clipboard_text
                        class: c_form_input_no_transition()
                        oninput: move |event: NativeEvent| {
                            if let NativeEvent::Input(input_event) = event {
                                clipboard_text.set(input_event.get_value().clone());
                            }
                        }
                    }
                }
                div {
                    class: c_browser_api_actions()
                    primary_button {
                        label: "Copy"
                        onclick: move |_event: NativeEvent| {
                            let text: String = clipboard_text.get();
                            let text_clone: String = text.clone();
                            let result: Signal<String> = clipboard_result;
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
                        }
                        "Copy to Clipboard"
                    }
                    primary_button {
                        label: "Paste"
                        onclick: move |_event: NativeEvent| {
                            let result: Signal<String> = clipboard_result;
                            wasm_bindgen_futures::spawn_local(async move {
                                let text: String = clipboard_read_text().await;
                                result.set(format!("Pasted: {}", text));
                            });
                        }
                        "Read Clipboard"
                    }
                }
                div {
                    class: c_browser_result_box()
                    span {
                        class: c_browser_result_label()
                        "Result: "
                    }
                    span {
                        class: c_browser_result_value()
                        clipboard_result
                    }
                }
            }
            my_card {
                title: "Window"
                p {
                    class: c_demo_text()
                    "Read the current window dimensions."
                }
                primary_button {
                    label: "Refresh"
                    onclick: move |_event: NativeEvent| {
                        let (width, height): (i32, i32) = window_inner_size();
                        window_size.set(format!("{} x {}", width, height));
                    }
                    "Refresh Size"
                }
                div {
                    class: c_browser_info_grid()
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Inner Size"
                        }
                        span {
                            class: c_browser_info_value()
                            window_size
                        }
                    }
                }
            }
            my_card {
                title: "Navigator"
                p {
                    class: c_demo_text()
                    "Read browser and device information."
                }
                div {
                    class: c_browser_info_grid()
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "User Agent"
                        }
                        span {
                            class: c_browser_info_value()
                            user_agent
                        }
                    }
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Language"
                        }
                        span {
                            class: c_browser_info_value()
                            language
                        }
                    }
                }
            }
            my_card {
                title: "Location"
                p {
                    class: c_demo_text()
                    "Read the current page URL information."
                }
                div {
                    class: c_browser_info_grid()
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Href"
                        }
                        span {
                            class: c_browser_info_value()
                            location_url
                        }
                    }
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Origin"
                        }
                        span {
                            class: c_browser_info_value()
                            location_origin_val
                        }
                    }
                    div {
                        class: c_browser_info_item()
                        span {
                            class: c_browser_info_label()
                            "Pathname"
                        }
                        span {
                            class: c_browser_info_value()
                            location_pathname_val
                        }
                    }
                }
            }
            my_card {
                title: "Console"
                p {
                    class: c_demo_text()
                    "Send messages to the browser developer console."
                }
                div {
                    class: c_form_input_wrapper()
                    label {
                        class: c_form_label()
                        "Console message"
                    }
                    input {
                        r#type: "text"
                        placeholder: "Type a message to log..."
                        value: console_input
                        class: c_form_input_no_transition()
                        oninput: move |event: NativeEvent| {
                            if let NativeEvent::Input(input_event) = event {
                                console_input.set(input_event.get_value().clone());
                            }
                        }
                    }
                }
                div {
                    class: c_browser_api_actions()
                    primary_button {
                        label: "Log"
                        onclick: move |_event: NativeEvent| {
                            let message: String = console_input.get();
                            Console::log(&message);
                        }
                        "console.log"
                    }
                    primary_button {
                        label: "Warn"
                        onclick: move |_event: NativeEvent| {
                            let message: String = console_input.get();
                            Console::warn(&message);
                        }
                        "console.warn"
                    }
                    primary_button {
                        label: "Error"
                        onclick: move |_event: NativeEvent| {
                            let message: String = console_input.get();
                            Console::error(&message);
                        }
                        "console.error"
                    }
                }
            }
        }
    }
}
