use super::*;

/// Extension trait that exposes the hook-context factory
/// used by `App::use_i18n`.
pub trait HookContextI18nExt {
    /// Returns the `I18n` for the current hook context
    /// slot.
    fn i18n() -> I18n;
}
