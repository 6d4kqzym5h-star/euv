/// Tracks the position and nesting level of a macro invocation
/// found during source scanning.
pub(crate) struct MacroSpan {
    /// Byte offset of the opening `{` after the macro name and `!`.
    pub(crate) start: usize,
    /// Byte offset of the closing `}` (exclusive — points one past it).
    pub(crate) end: usize,
}
