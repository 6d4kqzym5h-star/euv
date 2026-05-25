use crate::*;

/// Creates todo list state signals wrapped in a `UseTodoList` struct.
///
/// # Returns
///
/// - `UseTodoList` - The todo list state.
pub(crate) fn use_todo_list() -> UseTodoList {
    UseTodoList::new(
        use_signal(|| {
            let mut items: Vec<String> = Vec::with_capacity(1000);
            for index in 1..=1000 {
                items.push(format!("Item {}", index));
            }
            items
        }),
        use_signal(String::new),
        use_signal(String::new),
    )
}

/// Validates the new item input and updates the error signal.
///
/// # Arguments
///
/// - `UseTodoList` - The todo list state.
pub(crate) fn validate_todo_new_item(state: UseTodoList) {
    let new_item_value: String = state.get_new_item().get();
    if new_item_value.trim().is_empty() {
        state
            .get_add_error()
            .set("Please enter an item name.".to_string());
    } else if new_item_value.trim().len() > 50 {
        state
            .get_add_error()
            .set("Item name is too long (max 50 chars).".to_string());
    } else {
        state.get_add_error().set(String::new());
    }
}

/// Creates an input event handler that updates the new item and validates it.
///
/// # Arguments
///
/// - `UseTodoList` - The todo list state.
///
/// # Returns
///
/// - `NativeEventHandler` - An input handler.
pub(crate) fn todo_list_on_input_new_item(state: UseTodoList) -> NativeEventHandler {
    NativeEventHandler::create(NativeEventName::Input, move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            state.get_new_item().set(input.value());
        }
        validate_todo_new_item(state);
    })
}

/// Creates a click event handler that adds a new item to the list.
///
/// # Arguments
///
/// - `UseTodoList` - The todo list state.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler to add a new item.
pub(crate) fn todo_list_on_add(state: UseTodoList) -> NativeEventHandler {
    NativeEventHandler::create(NativeEventName::Click, move |_event: Event| {
        validate_todo_new_item(state);
        let add_error_value: String = state.get_add_error().get();
        if add_error_value.is_empty() {
            let text: String = state.get_new_item().get();
            let mut current: Vec<String> = state.get_items().get();
            current.push(text.trim().to_string());
            state.get_items().set(current);
            state.get_new_item().set(String::new());
        }
    })
}

/// Creates a click event handler that removes an item at the given index.
///
/// # Arguments
///
/// - `Signal<Vec<String>>` - The items signal.
/// - `usize` - The index of the item to remove.
///
/// # Returns
///
/// - `NativeEventHandler` - A click handler to remove the item.
pub(crate) fn todo_list_on_remove(items: Signal<Vec<String>>, index: usize) -> NativeEventHandler {
    NativeEventHandler::create(NativeEventName::Click, move |_event: Event| {
        let mut current: Vec<String> = items.get();
        if index < current.len() {
            current.remove(index);
            items.set(current);
        }
    })
}

/// Disconnects the previous `IntersectionObserver` stored on `window.__euv_list_observer`
/// and creates a fresh one that observes all elements matching `selector`.
///
/// When an element enters or leaves the viewport, its tag name, `data-index`
/// attribute (if present), and intersection state are logged via `Console::log`.
///
/// # Arguments
///
/// - `&str` - A CSS selector string to identify the elements to observe.
fn bind_observer(selector: &str) {
    let win: Window = window().expect("no global window exists");
    let doc: Document = win.document().expect("should have a document");
    let obs_key: JsValue = JsValue::from_str("__euv_list_observer");
    if let Some(old_observer) = Reflect::get(&win, &obs_key)
        .ok()
        .and_then(|v| v.dyn_into::<IntersectionObserver>().ok())
    {
        old_observer.disconnect();
    }
    let callback: Closure<dyn FnMut(Array)> = Closure::wrap(Box::new(move |entries: Array| {
        let w: Window = window().expect("no global window exists");
        let p_key: JsValue = JsValue::from_str("__euv_list_observer_pending_entries");
        let t_key: JsValue = JsValue::from_str("__euv_list_observer_throttle");
        let existing: Array = Reflect::get(&w, &p_key)
            .unwrap_or(JsValue::UNDEFINED)
            .dyn_into::<Array>()
            .unwrap_or_else(|_| Array::new());
        for index in 0..entries.length() {
            existing.push(&entries.get(index));
        }
        let _ = Reflect::set(&w, &p_key, &existing);
        if !Reflect::get(&w, &t_key)
            .unwrap_or(JsValue::UNDEFINED)
            .is_undefined()
        {
            return;
        }
        let throttle_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            let w2: Window = window().expect("no global window exists");
            let p_key2: JsValue = JsValue::from_str("__euv_list_observer_pending_entries");
            let t_key2: JsValue = JsValue::from_str("__euv_list_observer_throttle");
            let _ = Reflect::set(&w2, &t_key2, &JsValue::UNDEFINED);
            let pending: Array = Reflect::get(&w2, &p_key2)
                .unwrap_or(JsValue::UNDEFINED)
                .dyn_into::<Array>()
                .unwrap_or_else(|_| Array::new());
            let _ = Reflect::set(&w2, &p_key2, &Array::new());
            for index in 0..pending.length() {
                let entry: JsValue = pending.get(index);
                let intersection_entry: IntersectionObserverEntry =
                    entry.dyn_into::<IntersectionObserverEntry>().unwrap();
                if !intersection_entry.is_intersecting() {
                    continue;
                }
                let target: Element = intersection_entry.target();
                let tag_name: String = target.tag_name();
                let data_index: Option<String> = target.get_attribute("data-index");
                let intersection_ratio: f64 = intersection_entry.intersection_ratio();
                let index_info: String = match data_index {
                    Some(idx) => format!("index={}, ", idx),
                    None => String::new(),
                };
                Console::log(&format!(
                    "[IntersectionObserver] <{}> {}intersection_ratio={:.2}",
                    tag_name, index_info, intersection_ratio
                ));
            }
        }));
        let _ = Reflect::set(&w, &t_key, &JsValue::TRUE);
        let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
            throttle_closure.as_ref().unchecked_ref(),
            100,
        );
        throttle_closure.forget();
    }));
    let observer: IntersectionObserver =
        IntersectionObserver::new(callback.as_ref().unchecked_ref()).unwrap();
    let _ = Reflect::set(&win, &obs_key, observer.as_ref());
    callback.forget();
    let elements: NodeList = doc.query_selector_all(selector).unwrap();
    for index in 0..elements.length() {
        if let Some(node) = elements.item(index)
            && let Ok(element) = node.dyn_into::<Element>()
        {
            observer.observe(&element);
        }
    }
}

/// Schedules a `bind_observer` call via `requestAnimationFrame` so that the
/// DOM is fully painted before querying elements.
///
/// Uses a guard flag (`window.__euv_list_observer_pending`) to debounce
/// multiple schedule requests within the same frame into a single
/// `requestAnimationFrame` callback.
///
/// # Arguments
///
/// - `String` - A CSS selector string to identify the elements to observe.
fn schedule_bind_observer(selector: String) {
    let pending_key: JsValue = JsValue::from_str("__euv_list_observer_pending");
    let win: Window = window().expect("no global window exists");
    if !Reflect::get(&win, &pending_key)
        .unwrap_or(JsValue::UNDEFINED)
        .is_undefined()
    {
        return;
    }
    let _ = Reflect::set(&win, &pending_key, &JsValue::TRUE);
    let raf_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let win: Window = window().expect("no global window exists");
        let key: JsValue = JsValue::from_str("__euv_list_observer_pending");
        let _ = Reflect::set(&win, &key, &JsValue::UNDEFINED);
        bind_observer(&selector);
    }));
    let _ = win.request_animation_frame(raf_closure.as_ref().unchecked_ref());
    raf_closure.forget();
}

/// Observes elements matching the given CSS selector for viewport intersection changes.
///
/// On first call, schedules an initial binding via `requestAnimationFrame` so
/// that the DOM is fully rendered and painted before querying elements.
/// Also registers a `__euv_signal_update__` event listener on `window` so
/// that the observer is re-bound after every signal-driven DOM patch, again
/// deferred via `requestAnimationFrame` to ensure the DOM update has been
/// painted.
///
/// A guard flag (`window.__euv_list_observer_listener`) ensures both the
/// initial `requestAnimationFrame` and the event listener are only
/// registered once.
///
/// # Arguments
///
/// - `&str` - A CSS selector string to identify the elements to observe.
pub(crate) fn use_intersection_observer(selector: &str) {
    let selector_owned: String = selector.to_string();
    let rebind_selector: String = selector_owned.clone();
    let init_selector: String = selector_owned.clone();
    let win: Window = window().expect("no global window exists");
    let listener_key: JsValue = JsValue::from_str("__euv_list_observer_listener");
    if Reflect::get(&win, &listener_key)
        .unwrap_or(JsValue::UNDEFINED)
        .is_undefined()
    {
        let _ = Reflect::set(&win, &listener_key, &JsValue::TRUE);
        let rebind_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            schedule_bind_observer(rebind_selector.clone())
        }));
        let _ = win.add_event_listener_with_callback(
            "__euv_signal_update__",
            rebind_closure.as_ref().unchecked_ref(),
        );
        rebind_closure.forget();
        schedule_bind_observer(init_selector);
    }
}
