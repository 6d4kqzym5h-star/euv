use super::*;

/// Global mutable storage for component function names and their metadata.
///
/// Maps function name → `ComponentInfo` (e.g., "primary_button" → ComponentInfo { props_type: "PrimaryButtonProps", props_fields: [...] }).
/// Populated by scanning source files before `html!` macro expansion.
///
/// Uses `MaybeUninit` for single-initialization in single-threaded proc-macro context.
pub(crate) static mut USER_FN_NAMES: MaybeUninit<HashMap<String, ComponentInfo>> =
    MaybeUninit::uninit();
