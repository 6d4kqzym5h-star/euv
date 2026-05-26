use crate::*;

/// Global mutable storage for function names defined in the user's project.
/// Populated by scanning source files before `html!` macro expansion.
pub(crate) static mut USER_FN_NAMES: MaybeUninit<HashSet<String>> = MaybeUninit::uninit();
