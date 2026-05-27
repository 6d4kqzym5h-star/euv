use crate::*;

/// Reactive state for a todo list feature.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseTodoList {
    /// The list of todo items.
    #[get(type(copy))]
    pub(crate) items: Signal<Vec<String>>,
    /// The new item input text.
    #[get(type(copy))]
    pub(crate) new_item: Signal<String>,
    /// The validation error message for add operation.
    #[get(type(copy))]
    pub(crate) add_error: Signal<String>,
}
