/// The state of a `LazyComponent`.
///
/// - `Pending` — the factory has not been called yet.
/// - `Loading` — `prefetch()` has been called (or the
///   factory is running synchronously); UI should show a
///   spinner.
/// - `Loaded(T)` — the factory has produced a value;
///   `T::clone()` returns it.
/// - `Failed(String)` — the factory panicked or returned
///   an error message; UI should show an error state.
#[derive(Clone, Debug)]
pub enum LoadState<T> {
    /// The factory has not been called yet.
    Pending,
    /// The factory is running (set by `prefetch()`).
    Loading,
    /// The factory has produced a value.
    Loaded(T),
    /// The factory failed; the message is for debugging.
    Failed(String),
}
