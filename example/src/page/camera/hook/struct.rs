use crate::*;

/// Reactive state for the camera page, including camera control,
/// facing mode, and QR code scanning results.
#[derive(Clone, Copy, Data)]
pub(crate) struct UseCamera {
    /// Whether the camera stream is currently open.
    #[get(type(copy))]
    pub(crate) camera_open: Signal<bool>,
    /// Whether the camera is in the process of opening.
    #[get(type(copy))]
    pub(crate) camera_loading: Signal<bool>,
    /// The current error message, empty if no error.
    #[get(type(copy))]
    pub(crate) error_message: Signal<String>,
    /// The current camera facing direction (front or rear).
    #[get(type(copy))]
    pub(crate) facing: Signal<CameraFacing>,
    /// The last QR code scan result text.
    #[get(type(copy))]
    pub(crate) scan_result: Signal<String>,
    /// The interval handle for the periodic QR code scan timer.
    #[get(type(copy))]
    pub(crate) scan_handle: Signal<Option<IntervalHandle>>,
}
