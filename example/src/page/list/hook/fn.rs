use crate::*;

/// Creates todo list state signals wrapped in a `UseTodoList` struct.
///
/// # Returns
///
/// - `UseTodoList` - The todo list state.
pub fn use_todo_list() -> UseTodoList {
    UseTodoList::new(
        use_signal(|| {
            vec![
                "Learn Rust".to_string(),
                "Build a UI framework".to_string(),
                "Write documentation".to_string(),
            ]
        }),
        use_signal(String::new),
        use_signal(String::new),
    )
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
pub fn todo_list_on_add(state: UseTodoList) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let text: String = state.get_new_item().get();
        if text.trim().is_empty() {
            state
                .get_add_error()
                .set("Please enter an item name.".to_string());
        } else if text.trim().len() > 50 {
            state
                .get_add_error()
                .set("Item name is too long (max 50 chars).".to_string());
        } else {
            let mut current: Vec<String> = state.get_items().get();
            current.push(text.trim().to_string());
            state.get_items().set(current);
            state.get_new_item().set(String::new());
            state.get_add_error().set(String::new());
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
pub fn todo_list_on_remove(items: Signal<Vec<String>>, index: usize) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let mut current: Vec<String> = items.get();
        if index < current.len() {
            current.remove(index);
            items.set(current);
        }
    })
}
