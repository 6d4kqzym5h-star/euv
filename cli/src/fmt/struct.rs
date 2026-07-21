use super::*;

/// Result of formatting a single file.
///
/// Indicates whether the file was already formatted or needed changes,
/// and provides the formatted output when applicable.
#[derive(Data, New)]
pub(crate) struct FmtResult {
    /// Whether the file content was changed by formatting.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) changed: bool,
    /// The formatted file content (identical to input if `changed` is false).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) output: String,
}
