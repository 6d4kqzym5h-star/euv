//! Stable diagnostic banners shown by the WebGPU demo tabs.
//!
//! Centralizing the strings here makes localization, copy review, and
//! future renames a single-file edit. The codes referenced by these
//! banners are produced by `euv_engine::WebGpuInitError::code()` and
//! by `euv_engine::WebGpuRenderer::is_available()`; keep them in
//! sync if either side renames an error variant.

/// Banner shown while `WebGpuRenderer::init` is still in flight.
pub(crate) const WEBGPU_STATUS_INITIALIZING: &str = "Initializing...";

/// Banner shown once the renderer is fully active and frames are
/// flowing through the swap chain.
pub(crate) const WEBGPU_STATUS_ACTIVE: &str = "WebGPU Active";

/// Banner shown when `navigator.gpu` is missing - the current origin
/// is not a secure context (HTTPS / localhost), or the browser
/// itself lacks the WebGPU feature.
pub(crate) const WEBGPU_STATUS_NEEDS_HTTPS_OR_LOCALHOST: &str = "WebGPU needs HTTPS or localhost";

/// Banner shown when `requestAdapter()` resolves to `null` or its
/// promise rejects/times out - typically software rendering,
/// headless GPU, or a driver/GPU blacklist.
pub(crate) const WEBGPU_STATUS_ADAPTER_UNAVAILABLE: &str = "WebGPU adapter unavailable";

/// Banner shown when `requestAdapter()` succeeded but
/// `requestDevice()` returned `null` or its promise rejected/timed
/// out - usually a `device-lost` state on the adapter.
pub(crate) const WEBGPU_STATUS_DEVICE_UNAVAILABLE: &str = "WebGPU device unavailable (driver?)";

/// Banner shown when the configured canvas selector did not resolve
/// to a DOM element.
pub(crate) const WEBGPU_STATUS_CANVAS_NOT_FOUND: &str = "Canvas element missing in DOM";

/// Banner shown when the canvas already has a non-WebGPU context
/// (Canvas 2D `2d`, webgl, etc.) bound to it.
pub(crate) const WEBGPU_STATUS_CANVAS_CONTEXT_UNAVAILABLE: &str =
    "Canvas already using another context";

/// Banner shown for `WebGpuInitError` variants that indicate the
/// browser API itself threw or rejected (reflect errors, sync
/// `requestAdapter()`/`requestDevice()`/`getPreferredCanvasFormat()`
/// exceptions, `configure`/`queue` lookup failures).
pub(crate) const WEBGPU_STATUS_INIT_FAILED_BROWSER_API: &str = "WebGPU init failed (browser API)";

/// Fallback banner shown when no specific error variant matches.
pub(crate) const WEBGPU_STATUS_NOT_SUPPORTED: &str = "WebGPU Not Supported";

/// Engine error code produced by `WebGpuInitError::NavigatorGpuMissing`.
pub(crate) const WEBGPU_CODE_NAVIGATOR_GPU_MISSING: &str = "WEBGPU_NAVIGATOR_GPU_MISSING";

/// Engine error code produced by `WebGpuInitError::AdapterUnavailable`
/// or `WebGpuInitError::AdapterPromise`.
pub(crate) const WEBGPU_CODE_ADAPTER_UNAVAILABLE: &str = "WEBGPU_ADAPTER_UNAVAILABLE";
pub(crate) const WEBGPU_CODE_ADAPTER_PROMISE: &str = "WEBGPU_ADAPTER_PROMISE";

/// Engine error code produced by `WebGpuInitError::DeviceUnavailable`
/// or `WebGpuInitError::DevicePromise`.
pub(crate) const WEBGPU_CODE_DEVICE_UNAVAILABLE: &str = "WEBGPU_DEVICE_UNAVAILABLE";
pub(crate) const WEBGPU_CODE_DEVICE_PROMISE: &str = "WEBGPU_DEVICE_PROMISE";

/// Engine error code produced by `WebGpuInitError::CanvasNotFound`.
pub(crate) const WEBGPU_CODE_CANVAS_NOT_FOUND: &str = "WEBGPU_CANVAS_NOT_FOUND";

/// Engine error code produced by `WebGpuInitError::CanvasContextUnavailable`.
pub(crate) const WEBGPU_CODE_CANVAS_CONTEXT_UNAVAILABLE: &str = "WEBGPU_CANVAS_CONTEXT_UNAVAILABLE";

/// Engine error codes that indicate a browser-side API failure
/// (Reflect lookup, sync throw, configure/queue/format errors).
pub(crate) const WEBGPU_CODE_API_FAILURE: &[&str] = &[
    "WEBGPU_REQUEST_ADAPTER_LOOKUP",
    "WEBGPU_REQUEST_ADAPTER_CALL",
    "WEBGPU_REQUEST_DEVICE_LOOKUP",
    "WEBGPU_REQUEST_DEVICE_CALL",
    "WEBGPU_PREFERRED_FORMAT_LOOKUP",
    "WEBGPU_PREFERRED_FORMAT_CALL",
    "WEBGPU_PREFERRED_FORMAT_TYPE",
    "WEBGPU_CONFIGURE_LOOKUP",
    "WEBGPU_QUEUE_LOOKUP",
    "WEBGPU_CANVAS_QUERY",
    "WEBGPU_NAVIGATOR_LOOKUP",
];
