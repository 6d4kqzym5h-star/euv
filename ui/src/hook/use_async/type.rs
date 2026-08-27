use super::*;

/// Helper alias for the common case with the empty
/// `LoadingHint` default. Use this unless the future
/// wants to surface a typed loading indicator.
pub type DefaultLoadingHandle<T> = UseAsyncHandle<T, ()>;
