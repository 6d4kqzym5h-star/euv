/// Maximum number of recursive flush cycles before giving up.
/// Prevents infinite loops caused by effect callbacks that
/// continuously re-trigger themselves.
pub(crate) const MAX_FLUSH_ITERATIONS: usize = 16;
