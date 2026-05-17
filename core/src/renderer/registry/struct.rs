use crate::*;

/// A wrapper around `Option<NativeEventHandler>` that enables `From<usize>` conversions.
///
/// Used as the value type in the handler registry. Allocated via `Box::leak`
/// and lives for the remainder of the program.
pub(crate) struct HandlerSlot {
    /// The optional event handler stored in this slot.
    pub(crate) handler: Option<NativeEventHandler>,
}
