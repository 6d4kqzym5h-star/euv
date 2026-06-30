use crate::*;

/// Result of formatting a single file.
///
/// Indicates whether the file was already formatted or needed changes,
/// and provides the formatted output when applicable.
#[derive(Data, New)]
pub struct FmtResult {
    /// Whether the file content was changed by formatting.
    #[get(pub, type(copy))]
    #[get_mut(pub)]
    #[set(pub)]
    pub changed: bool,
    /// The formatted file content (identical to input if `changed` is false).
    #[get(pub)]
    #[get_mut(pub)]
    #[set(pub)]
    pub output: String,
}
