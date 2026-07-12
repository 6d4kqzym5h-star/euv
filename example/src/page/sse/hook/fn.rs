use crate::*;

/// Creates SSE state signals wrapped in a `UseSse` struct.
///
/// # Returns
///
/// - `UseSse` - The SSE connection state.
pub(crate) fn use_sse() -> UseSse {
    UseSse::new(
        App::use_signal(|| SSE_DEFAULT_URL.to_string()),
        App::use_signal(|| false),
        App::use_signal(|| false),
        App::use_signal(Vec::new),
        App::use_signal(String::new),
    )
}

/// Opens an SSE connection to the specified URL and listens for events.
///
/// Creates a browser `EventSource` to the URL, and on each incoming `message`
/// event appends the data to the `messages` signal (up to `SSE_MAX_MESSAGES`).
/// The EventSource is stored in a thread-local so it can be closed later.
///
/// # Arguments
///
/// - `UseSse` - The SSE connection state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to open the SSE connection.
pub(crate) fn sse_on_connect(state: UseSse) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        if state.get_connecting().get() || state.get_connected().get() {
            return;
        }
        let url: String = state.get_url().get();
        if url.is_empty() {
            state
                .get_error()
                .set("Please enter a valid SSE URL".to_string());
            return;
        }
        state.get_connecting().set(true);
        sse_close(state, true);
        sse_open_source(state, &url);
    }))
}

/// Closes the stored SSE EventSource connection and resets state signals.
///
/// # Arguments
///
/// - `UseSse` - The SSE connection state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click handler to disconnect the SSE connection.
pub(crate) fn sse_on_disconnect(state: UseSse) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        sse_close(state, true);
    }))
}

/// Registers a cleanup callback that closes the SSE connection when the
/// component unmounts or the page route switches away.
///
/// # Arguments
///
/// - `UseSse` - The SSE connection state.
pub(crate) fn sse_cleanup(state: UseSse) {
    App::use_cleanup(move || {
        sse_close(state, false);
    });
}

/// Creates an `EventSource` connected to the given URL and registers open,
/// message, and error handlers. On error, the connection is closed
/// immediately without any reconnection attempt.
///
/// # Arguments
///
/// - `UseSse` - The SSE connection state.
/// - `&str` - The SSE endpoint URL.
fn sse_open_source(state: UseSse, url: &str) {
    let event_source: EventSource = match EventSource::new(url) {
        Ok(source) => source,
        Err(_) => {
            state.get_connecting().set(false);
            state
                .get_error()
                .set("Failed to create EventSource".to_string());
            return;
        }
    };
    SSE_SOURCE.with(|source: &RefCell<Option<EventSource>>| {
        *source.borrow_mut() = Some(event_source.clone());
    });
    let on_open: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new({
        let state: UseSse = state;
        move |_: JsValue| {
            state.get_connected().set(true);
            state.get_connecting().set(false);
        }
    }));
    event_source.set_onopen(Some(on_open.as_ref().unchecked_ref()));
    on_open.forget();
    let on_message: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new({
        let state: UseSse = state;
        move |event_value: JsValue| {
            let message_event: MessageEvent = event_value.unchecked_into();
            let js_value: JsValue = message_event.data();
            let raw: String = js_value.as_string().unwrap_or_default();
            let display: String = JSON::parse(&raw)
                .ok()
                .and_then(|parsed: JsValue| Reflect::get(&parsed, &JsValue::from_str("data")).ok())
                .and_then(|value: JsValue| value.as_string())
                .unwrap_or(raw);
            let mut current: Vec<String> = state.get_messages().get();
            current.push(display);
            if current.len() > SSE_MAX_MESSAGES {
                current.drain(0..current.len() - SSE_MAX_MESSAGES);
            }
            state.get_messages().set(current);
        }
    }));
    event_source.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();
    let on_error: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new({
        let state: UseSse = state;
        move |_: JsValue| {
            sse_close(state, false);
        }
    }));
    event_source.set_onerror(Some(on_error.as_ref().unchecked_ref()));
    on_error.forget();
}

/// Closes the stored EventSource and resets the connection state signals.
///
/// When `clear_messages` is `true`, the messages list is also cleared
/// (e.g. on explicit disconnect or reconnect). When `false`, messages
/// are preserved (e.g. on error or page cleanup).
///
/// # Arguments
///
/// - `UseSse` - The SSE connection state.
/// - `bool` - Whether to clear the messages list.
fn sse_close(state: UseSse, clear_messages: bool) {
    SSE_SOURCE.with(|source: &RefCell<Option<EventSource>>| {
        if let Some(event_source) = source.borrow_mut().take() {
            event_source.close();
        }
    });
    state.get_connected().set(false);
    state.get_connecting().set(false);
    state.get_error().set(String::new());
    if clear_messages {
        state.get_messages().set(Vec::new());
    }
}
