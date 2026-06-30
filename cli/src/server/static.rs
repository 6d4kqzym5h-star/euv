use crate::*;

/// Global application state singleton.
///
/// Initialized once at server startup and shared across all request handlers.
pub static APP_STATE: OnceLock<Arc<AppState>> = OnceLock::new();
