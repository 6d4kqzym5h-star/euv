use crate::*;

/// Creates fetch state signals wrapped in a `UseFetch` struct.
///
/// # Returns
///
/// - `UseFetch` - The fetch state containing loading, data, and error signals.
pub(crate) fn use_fetch() -> UseFetch {
    UseFetch::new(
        use_signal(|| false),
        use_signal(|| "Click fetch to load data".to_string()),
        use_signal(String::new),
    )
}

/// Creates a click event handler that fetches data from httpbin.org.
///
/// Sets loading to true, clears error and data, then spawns an async
/// task that performs the HTTP fetch and updates the state signals.
///
/// # Arguments
///
/// - `UseFetch` - The fetch state.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler to trigger the fetch.
pub(crate) fn fetch_on_fetch(state: UseFetch) -> NativeEventHandler {
    NativeEventHandler::create(NativeEventName::Click, move |_event: Event| {
        state.get_loading().set(true);
        state.get_error().set(String::new());
        state.get_data().set(String::new());
        let data_signal: Signal<String> = state.get_data();
        let error_signal: Signal<String> = state.get_error();
        let loading_signal: Signal<bool> = state.get_loading();
        spawn_local(async move {
            let window: Window = window().expect("no global window exists");
            let promise: Promise = window.fetch_with_str("https://httpbin.org/get");
            let future: JsFuture = JsFuture::from(promise);
            match future.await {
                Ok(response) => {
                    let response_value: Response = response.dyn_into().unwrap();
                    let json_promise: Promise = response_value.json().unwrap();
                    let json_future: JsFuture = JsFuture::from(json_promise);
                    match json_future.await {
                        Ok(json) => {
                            let json_string: String = JSON::stringify(&json)
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
