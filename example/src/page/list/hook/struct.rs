use crate::*;

/// Reactive state for a todo list feature.
#[derive(Clone, Copy, Data, New)]
pub struct UseTodoList {
    /// The list of todo items.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub items: Signal<Vec<String>>,
    /// The new item input text.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub new_item: Signal<String>,
    /// The validation error message for add operation.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub add_error: Signal<String>,
}
