/// The phase of an `ErrorBoundary`.
///
/// - `Healthy` — no child render has thrown yet.
///   Render the child normally.
/// - `Caught(String)` — a child render threw; the
///   message is for debugging. Render the fallback.
#[derive(Clone, Debug, Default)]
pub enum ErrorBoundaryPhase {
    /// No child render has thrown.
    #[default]
    Healthy,
    /// A child render threw; the message is for
    /// debugging.
    Caught(String),
}
