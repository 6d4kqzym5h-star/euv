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
    loading: Signal<bool>,
    error: Signal<String>,
    data: Signal<String>,
) -> VirtualNode {
    let is_loading: bool = loading.get();
    let error_text: String = error.get();
    let data_text: String = data.get();
    if is_loading {
        html! {
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
        html! {
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
        html! {
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
    let data: Signal<String> = use_signal(|| "Click fetch to load data".to_string());
    let error: Signal<String> = use_signal(|| "".to_string());
    html! {
        div {
            class: c_page_container()
            { page_header("Async Data", "Simulating network requests with loading states.") }
            my_card {
                title: "Fetch Data"
                p {
                    class: c_fetch_hint()
                    "Click the button below to fetch data from httpbin.org"
                }
                primary_button {
                    label: "Fetch"
                    onclick: move |_event: NativeEvent| {
                        loading.set(true);
                        error.set("".to_string());
                        data.set("".to_string());
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
                                            data.set(json_string);
                                        }
                                        Err(_) => {
                                            error.set("Failed to parse JSON".to_string());
                                        }
                                    }
                                }
                                Err(_) => {
                                    error.set("Network request failed".to_string());
                                }
                            }
                            loading.set(false);
                        });
                    }
                    "Fetch"
                }
                build_status_node(loading, error, data)
            }
        }
    }
}
