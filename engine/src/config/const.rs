/// The default canvas width in CSS pixels.
pub(crate) const CONFIG_DEFAULT_CANVAS_WIDTH: f64 = 800.0;

/// The default canvas height in CSS pixels.
pub(crate) const CONFIG_DEFAULT_CANVAS_HEIGHT: f64 = 600.0;

/// The default CSS selector for the canvas element.
pub(crate) const CONFIG_DEFAULT_CANVAS_SELECTOR: &str = "#engine-canvas";

/// The default SSAA supersampling scale factor (2.0 means 4x supersampling).
pub(crate) const CONFIG_DEFAULT_SSAA_SCALE_FACTOR: f64 = 2.0;

/// Whether anti-aliasing (MSAA) is enabled by default for the WebGPU backend.
pub(crate) const CONFIG_DEFAULT_ANTIALIAS: bool = true;

/// The WebGPU `powerPreference` value that hints "prefer low-power GPU".
///
/// Per the WebGPU spec (`GpuPowerPreference`), the W3C-defined string is
/// `"low-power"`. Centralized here so any future change to the
/// spec value is a single-file edit.
pub(crate) const GPU_POWER_PREFERENCE_LOW_POWER: &str = "low-power";

/// The WebGPU `powerPreference` value that hints "prefer high-performance GPU".
///
/// Per the WebGPU spec (`GpuPowerPreference`), the W3C-defined string is
/// `"high-performance"`. Centralized here so any future change to the
/// spec value is a single-file edit.
pub(crate) const GPU_POWER_PREFERENCE_HIGH_PERFORMANCE: &str = "high-performance";
