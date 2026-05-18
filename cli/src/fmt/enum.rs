/// The formatting mode for the CLI.
///
/// - `Check` — report files that need formatting without modifying them.
/// - `Write` — format files in-place.
pub enum FmtMode {
    /// Only check if formatting is needed; do not modify files.
    Check,
    /// Format files in-place.
    Write,
}
