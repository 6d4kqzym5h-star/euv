/// The duration of the modal exit animation in milliseconds.
///
/// Must match the `duration-modal-content` CSS variable value (0.32s = 320ms)
/// so that `setTimeout` fires after the CSS animation completes.
pub(crate) const MODAL_CLOSE_DURATION_MS: i32 = 320;
