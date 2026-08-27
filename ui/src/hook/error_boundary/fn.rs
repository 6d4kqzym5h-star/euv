use super::*;

/// Best-effort conversion of a `Box<dyn Any + Send>`
/// panic payload to a string.
///
/// # Arguments
///
/// - `&Box<dyn Any + Send>` - Shared reference to a `Box<dyn Any + Send>`.
///
/// # Returns
///
/// - `String` - A `String` value.
pub(crate) fn extract_message(payload: &Box<dyn Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<&'static str>() {
        (*message).to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "<non-string panic payload>".to_string()
    }
}
