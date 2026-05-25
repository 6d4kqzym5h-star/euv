use crate::*;

/// Reactive state for a todo list feature.
#[derive(Clone, Copy, Data, New)]
pub(crate) struct UseTodoList {
    /// The list of todo items.
    #[get(pub, type(copy))]
    pub(crate) items: Signal<Vec<String>>,
    /// The new item input text.
    #[get(pub, type(copy))]
    pub(crate) new_item: Signal<String>,
    /// The validation error message for add operation.
    #[get(pub, type(copy))]
    pub(crate) add_error: Signal<String>,
}
