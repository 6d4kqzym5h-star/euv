use crate::*;

/// Props for the `euv_alert` component.
///
/// Defines the strongly-typed interface for an alert with variant styling.
#[derive(Clone, Default)]
pub(crate) struct EuvAlertProps {
    /// The visual variant of the alert (success or error).
    pub(crate) variant: AlertVariant,
}
