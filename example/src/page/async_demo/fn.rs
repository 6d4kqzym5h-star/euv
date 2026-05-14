use crate::*;

/// Builds the status display node based on current async state.
///
/// # Arguments
///
/// - `Signal<bool>`: The loading state signal.
/// - `Signal<String>`: The error message signal.
/// - `Signal<String>`: The fetched data signal.
///
/// # Returns
///
/// - `VirtualNode`: The status indicator virtual DOM tree.
fn build_status_node(
    loading_read: Signal<bool>,
    error_read: Signal<String>,
    data_read: Signal<String>,
) -> VirtualNode {
    let is_loading: bool = loading_read.get();
    let error_text: String = error_read.get();
    let data_text: String = data_read.get();
    if is_loading {
        rsx! {
            div {
                class: c_loading_container()
                div {
                    class: c_spinner()
                }
                div {
                    class: c_loading_text_col()
                    span {
                        class: c_loading_title()
                        "Loading..."
                    }
                    span {
                        class: c_loading_subtitle()
                        "Fetching data from server"
                    }
                }
            }
        }
    } else if !error_text.is_empty() {
        rsx! {
            div {
                class: c_error_container()
                div {
                    class: c_error_icon()
                    "!"
                }
                span {
                    class: c_error_text()
                    error_text
                }
            }
        }
    } else {
        rsx! {
            div {
                class: c_data_box()
                pre {
                    class: c_data_pre()
                    data_text
                }
            }
        }
    }
}

/// An async data demo page simulating network requests.
///
/// # Returns
///
/// - `VirtualNode`: The async demo page virtual DOM tree.
pub fn page_async_demo() -> VirtualNode {
    let loading: Signal<bool> = use_signal(|| false);
    let loading_updater: Signal<bool> = loading;
    let loading_read: Signal<bool> = loading;
    let data: Signal<String> = use_signal(|| "Click fetch to load data".to_string());
    let data_updater: Signal<String> = data;
    let data_read: Signal<String> = data;
    let error: Signal<String> = use_signal(|| "".to_string());
    let error_updater: Signal<String> = error;
    let error_read: Signal<String> = error;
    rsx! {
        div {
            class: c_page_container()
            div {
                class: c_page_header()
                h1 {
                    class: c_page_title()
                    "Async Data"
                }
                p {
                    class: c_page_subtitle()
                    "Simulating network requests with loading states."
                }
            }
            my_card {
                title: "Fetch Data"
                p {
                    class: c_fetch_hint()
                    "Click the button below to fetch data from httpbin.org"
                }
                primary_button {
                    label: "Fetch"
                    onclick: move |_event: NativeEvent| {
                        loading_updater.set(true);
                        error_updater.set("".to_string());
                        data_updater.set("".to_string());
                        let data_updater_clone: Signal<String> = data_updater;
                        let loading_updater_clone: Signal<bool> = loading_updater;
                        let error_updater_clone: Signal<String> = error_updater;
                        wasm_bindgen_futures::spawn_local(async move {
                            let window: web_sys::Window = web_sys::window().expect("no global window exists");
                            let promise: js_sys::Promise = window.fetch_with_str("https://httpbin.org/get");
                            let future: wasm_bindgen_futures::JsFuture = wasm_bindgen_futures::JsFuture::from(promise);
                            match future.await {
                                Ok(response) => {
                                    let resp: web_sys::Response = response.dyn_into().unwrap();
                                    let json_promise: js_sys::Promise = resp.json().unwrap();
                                    let json_future: wasm_bindgen_futures::JsFuture = wasm_bindgen_futures::JsFuture::from(json_promise);
                                    match json_future.await {
                                        Ok(json) => {
                                            let json_string: String = js_sys::JSON::stringify(&json).unwrap().as_string().unwrap_or_default();
                                            data_updater_clone.set(json_string);
                                        }
                                        Err(_) => {
                                            error_updater_clone.set("Failed to parse JSON".to_string());
                                        }
                                    }
                                }
                                Err(_) => {
                                    error_updater_clone.set("Network request failed".to_string());
                                }
                            }
                            loading_updater_clone.set(false);
                        });
                    }
                    "Fetch Data"
                }
                build_status_node(loading_read, error_read, data_read)
            }
        }
    }
}
