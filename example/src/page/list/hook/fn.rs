use crate::*;

/// Reactive state for a todo list feature.
///
/// # Fields
///
/// - `Signal<Vec<String>>`: The list of todo items.
/// - `Signal<String>`: The new item input text.
#[derive(Clone, Copy)]
pub struct UseTodoList {
    pub items: Signal<Vec<String>>,
    pub new_item: Signal<String>,
}

/// Creates todo list state signals wrapped in a `UseTodoList` struct.
///
/// # Returns
///
/// - `UseTodoList`: The todo list state.
pub fn use_todo_list() -> UseTodoList {
    UseTodoList {
        items: use_signal(|| {
            vec![
                "Learn Rust".to_string(),
                "Build a UI framework".to_string(),
                "Write documentation".to_string(),
            ]
        }),
        new_item: use_signal(|| "".to_string()),
    }
}

/// Creates a click event handler that adds a new item to the list.
///
/// # Arguments
///
/// - `UseTodoList`: The todo list state.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to add a new item.
pub fn todo_list_on_add(state: UseTodoList) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let text: String = state.new_item.get();
        if !text.trim().is_empty() {
            let mut current: Vec<String> = state.items.get();
            current.push(text.clone());
            state.items.set(current);
            state.new_item.set("".to_string());
        }
    })
}

/// Creates a click event handler that removes an item at the given index.
///
/// # Arguments
///
/// - `Signal<Vec<String>>`: The items signal.
/// - `usize`: The index of the item to remove.
///
/// # Returns
///
/// - `NativeEventHandler`: A click handler to remove the item.
pub fn todo_list_on_remove(items: Signal<Vec<String>>, index: usize) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let mut current: Vec<String> = items.get();
        if index < current.len() {
            current.remove(index);
            items.set(current);
        }
    })
}
