use super::*;

/// Extension trait that exposes the hook-context factory
/// used by `App::use_form`.
///
/// Splitting the factory into a trait (rather than a bare
/// `impl HookContext`) keeps the public surface compact
/// and lets future hooks slot into the same trait.
pub trait HookContextFormExt {
    /// Returns the `FormState` for the current hook
    /// context slot.
    ///
    /// # Returns
    ///
    /// - `FormState` - A `FormState` value.
    fn form() -> FormState;
}
