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
pub(crate) fn todo_list_on_input_new_item(state: UseTodoList) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        if let Some(target) = event.target()
            && let Ok(input) = target.clone().dyn_into::<HtmlInputElement>()
        {
            state.get_new_item().set(input.value());
        }
        validate_todo_new_item(state);
    }))
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
pub(crate) fn todo_list_on_add(state: UseTodoList) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        validate_todo_new_item(state);
        let add_error_value: String = state.get_add_error().get();
        if add_error_value.is_empty() {
            let text: String = state.get_new_item().get();
            let mut current: Vec<String> = state.get_items().get();
            current.push(text.trim().to_string());
            state.get_items().set(current);
            state.get_new_item().set(String::new());
        }
    }))
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
pub(crate) fn todo_list_on_remove(
    items: Signal<Vec<String>>,
    index: usize,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        let mut current: Vec<String> = items.get();
        if index < current.len() {
            current.remove(index);
            items.set(current);
        }
    }))
}

/// Creates an `IntersectionObserver` stored on `window.__euv_list_observer`
/// that observes the container element matching `selector`.
///
/// When the container enters or leaves the viewport, its tag name,
/// intersection state, and child count are logged via `Console::log`.
/// Only the container element itself is observed (not individual children)
/// to avoid O(N) performance overhead with large lists.
///
/// # Arguments
///
/// - `&str` - A CSS selector string to identify the elements to observe.
fn bind_observer(selector: &str) {
    let window_value: Window = window().expect("no global window exists");
    let document_value: Document = window_value.document().expect("should have a document");
    let observer_key: JsValue = JsValue::from_str("__euv_list_observer");
    if Reflect::get(&window_value, &observer_key)
        .ok()
        .and_then(|value: JsValue| value.dyn_into::<IntersectionObserver>().ok())
        .is_some()
    {
        return;
    }
    let callback: Closure<dyn FnMut(Array)> = Closure::wrap(Box::new(move |entries: Array| {
        batch_updates(|| {
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
        });
        schedule_signal_update();
    }));
    let observer: IntersectionObserver =
        IntersectionObserver::new(callback.as_ref().unchecked_ref()).unwrap();
    let _ = Reflect::set(&window_value, &observer_key, observer.as_ref());
    callback.forget();
    if let Some(container_element) = document_value.query_selector(selector).ok().flatten() {
        observer.observe(&container_element);
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

/// Observes the container element matching the given CSS selector for viewport
/// intersection changes.
///
/// On first call, schedules an initial binding via `requestAnimationFrame` so
/// that the DOM is fully rendered and painted before querying elements.
/// Only the container itself is observed (not individual children) to avoid
/// O(N) overhead with large lists.
///
/// A guard flag (`window.__euv_list_observer_listener`) ensures the initial
/// `requestAnimationFrame` is only registered once.
///
/// # Arguments
///
/// - `&str` - A CSS selector string to identify the container element to observe.
pub(crate) fn use_intersection_observer(selector: &str) {
    let init_selector: String = selector.to_string();
    let window_value: Window = window().expect("no global window exists");
    let listener_key: JsValue = JsValue::from_str("__euv_list_observer_listener");
    if Reflect::get(&window_value, &listener_key)
        .unwrap_or(JsValue::UNDEFINED)
        .is_undefined()
    {
        let _ = Reflect::set(&window_value, &listener_key, &JsValue::TRUE);
        schedule_bind_observer(init_selector);
    }
}
