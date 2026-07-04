use crate::*;

/// Router functionality for managing browser history, overlays, and navigation.
///
/// Provides methods for scroll-to-top behavior, hash change handling,
/// overlay history management, and drawer scroll positioning.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Router;

/// A single entry in the unified overlay history stack.
///
/// Holds the close callback for an overlay (modal, panel, or drawer) so that
/// a system back gesture can dismiss the most recently opened overlay first,
/// regardless of its type.
pub(crate) struct OverlayEntry {
    /// The callback that closes the overlay.
    pub(crate) closer: Rc<dyn Fn()>,
}
