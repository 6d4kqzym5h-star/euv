/// The CSS selector used to query the camera video element from the DOM.
pub(crate) const CAMERA_VIDEO_SELECTOR: &str = "#camera-video";

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

/// The hostname that represents the local loopback device.
pub(crate) const CAMERA_LOCALHOST_HOSTNAME: &str = "localhost";
