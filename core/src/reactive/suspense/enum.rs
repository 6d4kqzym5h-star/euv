/// The phase of a `SuspenseState`.
///
/// - `Pending` — the underlying data is still loading.
///   Render a fallback / spinner.
/// - `Resolved(T)` — the data has loaded.
///   Render the loaded content.
/// - `Failed(String)` — the data load failed with the
///   given message. Render an error boundary.
#[derive(Clone, Debug)]
pub enum SuspensePhase<T> {
    /// The data is still loading.
    Pending,
    /// The data has loaded.
    Resolved(T),
    /// The data load failed; the message is for
    /// debugging.
    Failed(String),
}

impl<T> Default for SuspensePhase<T> {
    fn default() -> Self {
        SuspensePhase::Pending
    }
}

impl<T: PartialEq> PartialEq for SuspensePhase<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SuspensePhase::Pending, SuspensePhase::Pending) => true,
            (SuspensePhase::Resolved(a), SuspensePhase::Resolved(b)) => a == b,
            (SuspensePhase::Failed(a), SuspensePhase::Failed(b)) => a == b,
            _ => false,
        }
    }
}
