use crate::*;

/// Global mutable storage for component function names and their Props type names.
///
/// Maps function name → Props type name (e.g., "primary_button" → "PrimaryButtonProps").
/// Populated by scanning source files before `html!` macro expansion.
///
/// Uses `MaybeUninit` for single-initialization in single-threaded proc-macro context.
pub(crate) static mut USER_FN_NAMES: MaybeUninit<HashMap<String, String>> = MaybeUninit::uninit();
