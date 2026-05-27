use crate::*;

/// Global mutable storage for function names defined in the user's project.
///
/// Populated by scanning source files before `html!` macro expansion.
///
/// Uses `MaybeUninit` for single-initialization in single-threaded proc-macro context.
pub(crate) static mut USER_FN_NAMES: MaybeUninit<HashSet<String>> = MaybeUninit::uninit();
