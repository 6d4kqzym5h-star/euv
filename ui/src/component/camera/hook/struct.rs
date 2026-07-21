use super::*;

/// Reactive state for the camera, including camera control,
/// facing mode, and QR code scanning results.
#[derive(Clone, Copy, Data, Debug, Eq, New, PartialEq)]
pub struct UseEuvCamera {
    /// Whether the camera stream is currently open.
    pub camera_open: Signal<bool>,
    /// Whether the camera is in the process of opening.
    pub camera_loading: Signal<bool>,
    /// The current error message, empty if no error.
    pub error_message: Signal<String>,
    /// The current camera facing direction (front or rear).
    pub facing: Signal<EuvCameraFacing>,
    /// The last QR code scan result text.
    pub scan_result: Signal<String>,
    /// The interval handle for the periodic QR code scan timer.
    pub scan_handle: Signal<Option<IntervalHandle>>,
}

/// Configuration for camera initialization.
#[derive(Clone, CustomDebug)]
pub struct EuvCameraConfig {
    /// The CSS selector for the video element.
    pub video_selector: &'static str,
    /// The scan interval in milliseconds.
    pub scan_interval_millis: i32,
    /// Whether to auto-start QR scanning when camera opens.
    pub auto_scan: bool,
    /// Callback when a QR code is detected.
    #[debug(skip)]
    pub on_qr_detected: Option<QrDetectedCallback>,
    /// Callback when camera error occurs.
    #[debug(skip)]
    pub on_error: Option<CameraErrorCallback>,
}
