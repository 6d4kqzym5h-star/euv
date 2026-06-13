/// The duration, in milliseconds, of the modal closing (exit) animation.
///
/// Must stay in sync with the `--duration-modal-content` CSS variable
/// (`0.32s`) defined in the theme, since the modal is removed from the DOM
/// only after this delay so the `euv-scale-out-modal` / `euv-fade-out`
/// keyframes have time to finish playing.
pub(crate) const MODAL_CLOSE_ANIMATION_MILLIS: i32 = 320;

/// The HTML id for the modal name input element.
pub(crate) const MODAL_NAME_ID: &str = "modal-name";

/// The HTML id for the modal email input element.
pub(crate) const MODAL_EMAIL_ID: &str = "modal-email";

/// The HTML name attribute for the modal name input element.
pub(crate) const MODAL_NAME_NAME: &str = "name";

/// The HTML name attribute for the modal email input element.
pub(crate) const MODAL_EMAIL_NAME: &str = "email";

/// The HTML input type for the modal name input element.
pub(crate) const MODAL_TEXT_TYPE: &str = "text";

/// The HTML input type for the modal email input element.
pub(crate) const MODAL_EMAIL_TYPE: &str = "email";

/// The HTML autocomplete attribute value for name.
pub(crate) const MODAL_AUTOCOMPLETE_NAME: &str = "name";

/// The HTML autocomplete attribute value for email.
pub(crate) const MODAL_AUTOCOMPLETE_EMAIL: &str = "email";

/// The HTML placeholder for the modal name input element.
pub(crate) const MODAL_NAME_PLACEHOLDER: &str = "Enter your name";

/// The HTML placeholder for the modal email input element.
pub(crate) const MODAL_EMAIL_PLACEHOLDER: &str = "Enter your email";
