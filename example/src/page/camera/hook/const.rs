/// The HTML `id` attribute value for the camera video element.
pub(crate) const CAMERA_VIDEO_ID: &str = "camera-video";

/// The CSS selector used to query the camera video element from the DOM.
pub(crate) const CAMERA_VIDEO_SELECTOR: &str = "#camera-video";

/// The `autoplay` attribute value for the camera video element.
pub(crate) const CAMERA_VIDEO_AUTOPLAY: bool = true;

/// The `playsinline` attribute value for the camera video element.
pub(crate) const CAMERA_VIDEO_PLAYSINLINE: bool = true;

/// The icon displayed in the camera placeholder when the camera is closed.
pub(crate) const CAMERA_PLACEHOLDER_ICON: &str = "📷";

/// The facing mode constraint value for the user-facing (front) camera.
pub(crate) const CAMERA_FACING_MODE_USER: &str = "user";

/// The facing mode constraint value for the environment-facing (rear) camera.
pub(crate) const CAMERA_FACING_MODE_ENVIRONMENT: &str = "environment";

/// The interval in milliseconds between QR code scan attempts.
pub(crate) const CAMERA_SCAN_INTERVAL_MILLIS: i32 = 500;

/// The prefix used to detect URL strings in QR code scan results.
pub(crate) const CAMERA_URL_PREFIX_HTTP: &str = "http://";

/// The prefix used to detect secure URL strings in QR code scan results.
pub(crate) const CAMERA_URL_PREFIX_HTTPS: &str = "https://";

/// The label displayed on the switch camera button when the front camera is active.
pub(crate) const CAMERA_SWITCH_TO_REAR_LABEL: &str = "🔄 Rear";

/// The label displayed on the switch camera button when the rear camera is active.
pub(crate) const CAMERA_SWITCH_TO_FRONT_LABEL: &str = "🔄 Front";
