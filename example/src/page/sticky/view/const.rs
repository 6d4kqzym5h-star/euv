/// The minimum value for the backdrop blur slider.
pub(crate) const STICKY_BLUR_MIN: &str = "0";

/// The maximum value for the backdrop blur slider.
pub(crate) const STICKY_BLUR_MAX: &str = "30";

/// The step value for the backdrop blur slider.
pub(crate) const STICKY_BLUR_STEP: &str = "1";

/// The minimum value for the gradient angle slider.
pub(crate) const STICKY_ANGLE_MIN: &str = "0";

/// The maximum value for the gradient angle slider.
pub(crate) const STICKY_ANGLE_MAX: &str = "360";

/// The step value for the gradient angle slider.
pub(crate) const STICKY_ANGLE_STEP: &str = "1";

/// The text displayed in the sticky header.
pub(crate) const STICKY_HEADER_TEXT: &str = "I am a sticky header!";

/// The label for the blur slider control.
pub(crate) const STICKY_BLUR_LABEL: &str = "Backdrop Blur";

/// The label for the angle slider control.
pub(crate) const STICKY_ANGLE_LABEL: &str = "Gradient Angle";

/// The placeholder text content for scrollable areas.
pub(crate) const STICKY_PLACEHOLDER_TEXT: &str = "Scroll down to see the sticky header stick to the top. This is placeholder content to make the page scrollable.";

/// The CSS selector for the blur overlay element used in direct DOM manipulation.
pub(crate) const STICKY_BLUR_OVERLAY_SELECTOR: &str = ".c_sticky_blur_overlay";

/// The CSS selector for the blur label element used in direct DOM manipulation.
pub(crate) const STICKY_BLUR_LABEL_SELECTOR: &str = ".c_sticky_slider_blur_label";

/// The CSS selector for the gradient wheel element used in direct DOM manipulation.
pub(crate) const STICKY_GRADIENT_WHEEL_SELECTOR: &str = ".c_sticky_gradient_wheel";

/// The CSS selector for the angle label element used in direct DOM manipulation.
pub(crate) const STICKY_ANGLE_LABEL_SELECTOR: &str = ".c_sticky_slider_angle_label";

/// The initial backdrop blur value in pixels.
pub(crate) const STICKY_BLUR_INITIAL: i32 = 10;

/// The initial gradient angle in degrees.
pub(crate) const STICKY_ANGLE_INITIAL: i32 = 135;
