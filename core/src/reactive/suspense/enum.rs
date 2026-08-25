/// The phase of a `SuspenseState`.
///
/// - `Pending` — the underlying data is still loading.
///   Render a fallback / spinner.
/// - `Resolved(T)` — the data has loaded.
///   Render the loaded content.
/// - `Failed(String)` — the data load failed with the
///   given message. Render an error boundary.
#[derive(Clone, Debug, Default)]
pub enum SuspensePhase<T> {
    /// The data is still loading.
    #[default]
    Pending,
    /// The data has loaded.
    Resolved(T),
    /// The data load failed; the message is for
    /// debugging.
    Failed(String),
}
