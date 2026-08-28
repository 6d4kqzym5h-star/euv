use super::*;

/// Props for the [`euv_drawer`] component.
#[derive(Clone, CustomDebug, Default)]
pub struct EuvDrawerProps {
    /// The open state signal; the drawer slides in while set and clicking the
    /// overlay closes it.
    pub open: Signal<bool>,
}
