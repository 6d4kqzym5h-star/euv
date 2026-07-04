/// The formatting mode for the CLI.
///
/// - `Check` — report files that need formatting without modifying them.
/// - `Write` — format files in-place.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FmtMode {
    /// Only check if formatting is needed; do not modify files.
    #[default]
    Check,
    /// Format files in-place.
    Write,
}
