use super::*;

/// Reactive state for an async data fetch feature.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub(crate) struct UseFetch {
    /// Whether data is currently being fetched.
    #[get(type(copy))]
    pub(crate) loading: Signal<bool>,
    /// The fetched data content.
    #[get(type(copy))]
    pub(crate) data: Signal<String>,
    /// The error message, empty if no error.
    #[get(type(copy))]
    pub(crate) error: Signal<String>,
}
