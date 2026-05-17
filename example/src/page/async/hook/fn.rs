use crate::*;

/// Reactive state for an async data fetch feature.
#[derive(Clone, Copy, Data, New)]
pub struct UseFetch {
    /// Whether data is currently being fetched.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) loading: Signal<bool>,
    /// The fetched data content.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) data: Signal<String>,
    /// The error message, empty if no error.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) error: Signal<String>,
}

/// Creates fetch state signals wrapped in a `UseFetch` struct.
///
/// # Returns
///
/// - `UseFetch` - The fetch state containing loading, data, and error signals.
pub fn use_fetch() -> UseFetch {
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
pub fn fetch_on_fetch(state: UseFetch) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        state.get_loading().set(true);
        state.get_error().set("".to_string());
        state.get_data().set("".to_string());
        let data_signal: Signal<String> = state.get_data();
        let error_signal: Signal<String> = state.get_error();
        let loading_signal: Signal<bool> = state.get_loading();
        spawn_local(async move {
            let window: Window = window().expect("no global window exists");
            let promise: Promise = window.fetch_with_str("https://httpbin.org/get");
            let future: JsFuture = JsFuture::from(promise);
            match future.await {
                Ok(response) => {
                    let resp: Response = response.dyn_into().unwrap();
                    let json_promise: Promise = resp.json().unwrap();
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
