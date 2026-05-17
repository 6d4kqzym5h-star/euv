use crate::*;

/// Reactive state for an async data fetch feature.
#[derive(Clone, Copy, Data, New)]
pub struct UseFetch {
    /// Whether data is currently being fetched.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) loading: Signal<bool>,
    /// The fetched data content.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) data: Signal<String>,
    /// The error message, empty if no error.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) error: Signal<String>,
}
