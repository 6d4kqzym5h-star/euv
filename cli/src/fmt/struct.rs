/// Result of formatting a single file.
///
/// Indicates whether the file was already formatted or needed changes,
/// and provides the formatted output when applicable.
pub struct FmtResult {
    /// Whether the file content was changed by formatting.
    pub changed: bool,
    /// The formatted file content (identical to input if `changed` is false).
    pub output: String,
}
