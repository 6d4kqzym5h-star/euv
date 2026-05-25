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
    let window_value: Window = window().expect("no global window exists");
    let document_value: Document = window_value.document().expect("should have a document");
    let observer_key: JsValue = JsValue::from_str("__euv_list_observer");
    if let Some(existing_observer) = Reflect::get(&window_value, &observer_key)
        .ok()
        .and_then(|value: JsValue| value.dyn_into::<IntersectionObserver>().ok())
    {
        if let Some(container_element) = document_value.query_selector(selector).ok().flatten() {
            existing_observer.observe(&container_element);
            let children: NodeList = container_element
                .query_selector_all("[data-index]")
                .unwrap();
            for child_index in 0..children.length() {
                if let Some(child_node) = children.item(child_index)
                    && let Ok(child_element) = child_node.dyn_into::<Element>()
                {
                    existing_observer.observe(&child_element);
                }
            }
        }
        return;
    }
    let callback: Closure<dyn FnMut(Array)> = Closure::wrap(Box::new(move |entries: Array| {
        for index in 0..entries.length() {
            let entry: JsValue = entries.get(index);
            let intersection_entry: IntersectionObserverEntry =
                entry.dyn_into::<IntersectionObserverEntry>().unwrap();
            if !intersection_entry.is_intersecting() {
                continue;
            }
            let target: Element = intersection_entry.target();
            let tag_name: String = target.tag_name();
            let intersection_ratio: f64 = intersection_entry.intersection_ratio();
            let data_index: Option<String> = target.get_attribute("data-index");
            match data_index {
                Some(index_value) => {
                    Console::log(&format!(
                        "[IntersectionObserver] <{}> index={}, intersection_ratio={:.2}",
                        tag_name, index_value, intersection_ratio
                    ));
                }
                None => {
                    let children: NodeList = target.query_selector_all("[data-index]").unwrap();
                    let total_count: u32 = children.length();
                    let estimated_visible: u32 =
                        (intersection_ratio * total_count as f64).ceil() as u32;
                    Console::log(&format!(
                        "[IntersectionObserver] <{}> intersection_ratio={:.2}, total_items={}, estimated_visible_items={}",
                        tag_name, intersection_ratio, total_count, estimated_visible
                    ));
                }
            }
        }
    }));
    let observer: IntersectionObserver =
        IntersectionObserver::new(callback.as_ref().unchecked_ref()).unwrap();
    let _ = Reflect::set(&window_value, &observer_key, observer.as_ref());
    callback.forget();
    if let Some(container_element) = document_value.query_selector(selector).ok().flatten() {
        observer.observe(&container_element);
        let children: NodeList = container_element
            .query_selector_all("[data-index]")
            .unwrap();
        for child_index in 0..children.length() {
            if let Some(child_node) = children.item(child_index)
                && let Ok(child_element) = child_node.dyn_into::<Element>()
            {
                observer.observe(&child_element);
            }
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
    let window_value: Window = window().expect("no global window exists");
    if !Reflect::get(&window_value, &pending_key)
        .unwrap_or(JsValue::UNDEFINED)
        .is_undefined()
    {
        return;
    }
    let _ = Reflect::set(&window_value, &pending_key, &JsValue::TRUE);
    let request_animation_frame_closure: Closure<dyn FnMut()> =
        Closure::wrap(Box::new(move || {
            let window_value: Window = window().expect("no global window exists");
            let key: JsValue = JsValue::from_str("__euv_list_observer_pending");
            let _ = Reflect::set(&window_value, &key, &JsValue::UNDEFINED);
            bind_observer(&selector);
        }));
    let _ = window_value
        .request_animation_frame(request_animation_frame_closure.as_ref().unchecked_ref());
    request_animation_frame_closure.forget();
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
    let window_value: Window = window().expect("no global window exists");
    let listener_key: JsValue = JsValue::from_str("__euv_list_observer_listener");
    if Reflect::get(&window_value, &listener_key)
        .unwrap_or(JsValue::UNDEFINED)
        .is_undefined()
    {
        let _ = Reflect::set(&window_value, &listener_key, &JsValue::TRUE);
        let rebind_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            schedule_bind_observer(rebind_selector.clone());
        }));
        let _ = window_value.add_event_listener_with_callback(
            "__euv_signal_update__",
            rebind_closure.as_ref().unchecked_ref(),
        );
        rebind_closure.forget();
        schedule_bind_observer(init_selector);
    }
}
