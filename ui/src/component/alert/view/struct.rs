use super::*;

/// Props for the `euv_alert` component.
///
/// Defines the strongly-typed interface for an alert with variant styling.
#[derive(Clone, CustomDebug, Data, Default, New)]
pub struct EuvAlertProps {
    /// The visual variant of the alert (success or error).
    #[get(type(copy))]
    pub variant: AlertVariant,
}
