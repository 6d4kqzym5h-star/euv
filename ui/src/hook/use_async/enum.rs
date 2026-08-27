/// Discriminant for the three states an `use_async` future can be in.
///
/// Re-exported as `UseAsyncState` via [`super`]. Kept as a separate
/// enum (rather than a `Result`-shaped type) so the `match` arms
/// produced by users can name the `Loading` case separately and
/// reach for `LoadingHint` (the previous "in flight" payload) when
/// available.
///
/// The default initial state is `Loading(LoadingHint::DEFAULT)`,
/// because until the first `.await` resolves there is no `T` to
/// report. Users that need a distinct pre-fetch state can supply
/// `LoadingHint` via `UseAsyncHandle::loading_hint()`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AsyncState<T, L = ()> {
    /// The future has not yet produced a value (initial state).
    Loading(L),
    /// The future completed successfully with `T`.
    Ok(T),
    /// The future rejected with an error of type `E`.
    ///
    /// We intentionally use a free-form `String` for the error
    /// payload so the same enum works whether the future's error
    /// type is `JsValue`, `String`, `serde_json::Value`, or a custom
    /// domain type — `use_async` normalises everything into
    /// `String::to_string` at the await boundary. Callers that need
    /// typed errors should `String::parse` or use a richer future
    /// combinator on top.
    Err(String),
}
