use super::*;

/// Creates fetch state signals wrapped in a `UseFetch` struct.
///
/// # Returns
///
/// - `UseFetch` - The fetch state containing loading, data, and error signals.
pub(crate) fn use_fetch() -> UseFetch {
    UseFetch::new(
        App::use_signal(|| false),
        App::use_signal(|| "Click fetch to load data".to_string()),
        App::use_signal(String::new),
    )
}

/// Creates a click event handler that fetches data from ltpp.vip.
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
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to trigger the fetch.
pub(crate) fn fetch_on_fetch(state: UseFetch) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_loading().set(true);
        state.get_error().set(String::new());
        state.get_data().set(String::new());
        let data_signal: Signal<String> = state.get_data();
        let error_signal: Signal<String> = state.get_error();
        let loading_signal: Signal<bool> = state.get_loading();
        spawn_local(async move {
            let Some(window): Option<Window> = window() else {
                return;
            };
            let promise: Promise = window.fetch_with_str(ASYNC_FETCH_URL);
            let future: JsFuture = JsFuture::from(promise);
            match future.await {
                Ok(response) => {
                    let Ok(response_value): Result<Response, JsValue> = response.dyn_into() else {
                        return;
                    };
                    let Ok(text_promise): Result<Promise, JsValue> = response_value.text() else {
                        return;
                    };
                    let text_future: JsFuture = JsFuture::from(text_promise);
                    match text_future.await {
                        Ok(text) => {
                            let text_string: String = text.as_string().unwrap_or_default();
                            data_signal.set(text_string);
                        }
                        Err(_) => {
                            error_signal.set("Failed to read response text".to_string());
                        }
                    }
                }
                Err(_) => {
                    error_signal.set("Network request failed".to_string());
                }
            }
            loading_signal.set(false);
        });
    }))
}
