use super::*;

/// Type alias for QR code detection callback.
pub type QrDetectedCallback = Rc<dyn Fn(&str)>;

/// Type alias for camera error callback.
pub type CameraErrorCallback = Rc<dyn Fn(String)>;
