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
