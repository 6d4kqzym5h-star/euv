use crate::*;

/// Reactive state for an async data fetch feature.
///
/// # Fields
///
/// - `Signal<bool>`: Whether data is currently being fetched.
/// - `Signal<String>`: The fetched data content.
/// - `Signal<String>`: The error message, empty if no error.
#[derive(Clone, Copy)]
pub struct UseFetch {
    pub loading: Signal<bool>,
    pub data: Signal<String>,
    pub error: Signal<String>,
}

/// Creates fetch state signals wrapped in a `UseFetch` struct.
///
/// # Returns
///
/// - `UseFetch`: The fetch state containing loading, data, and error signals.
pub fn use_fetch() -> UseFetch {
    let loading: Signal<bool> = use_signal(|| false);
    let data: Signal<String> = use_signal(|| "Click fetch to load data".to_string());
    let error: Signal<String> = use_signal(|| "".to_string());
    UseFetch {
        loading,
        data,
        error,
    }
}

/// Creates a click event handler that fetches data from httpbin.org.
///
/// Sets loading to true, clears error and data, then spawns an async
/// task that performs the HTTP fetch and updates the state signals.
///
/// # Arguments
///
/// - `UseFetch`: The fetch state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to trigger the fetch.
pub fn fetch_on_fetch(state: UseFetch) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        state.loading.set(true);
        state.error.set("".to_string());
        state.data.set("".to_string());
        let data_signal: Signal<String> = state.data;
        let error_signal: Signal<String> = state.error;
        let loading_signal: Signal<bool> = state.loading;
        wasm_bindgen_futures::spawn_local(async move {
            let window: web_sys::Window = web_sys::window().expect("no global window exists");
            let promise: js_sys::Promise = window.fetch_with_str("https://httpbin.org/get");
            let future: wasm_bindgen_futures::JsFuture =
                wasm_bindgen_futures::JsFuture::from(promise);
            match future.await {
                Ok(response) => {
                    let resp: web_sys::Response = response.dyn_into().unwrap();
                    let json_promise: js_sys::Promise = resp.json().unwrap();
                    let json_future: wasm_bindgen_futures::JsFuture =
                        wasm_bindgen_futures::JsFuture::from(json_promise);
                    match json_future.await {
                        Ok(json) => {
                            let json_string: String = js_sys::JSON::stringify(&json)
                                .unwrap()
                                .as_string()
                                .unwrap_or_default();
                            data_signal.set(json_string);
                        }
                        Err(_) => {
                            error_signal.set("Failed to parse JSON".to_string());
                        }
                    }
                }
                Err(_) => {
                    error_signal.set("Network request failed".to_string());
                }
            }
            loading_signal.set(false);
        });
    })
}
