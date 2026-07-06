/// The canvas 2D rendering context type identifier.
pub(crate) const RENDERER_CONTEXT_TYPE_2D: &str = "2d";

/// The default font family used for text rendering.
pub(crate) const RENDERER_DEFAULT_FONT_FAMILY: &str = "sans-serif";

/// The default font size in pixels.
pub(crate) const RENDERER_DEFAULT_FONT_SIZE: f64 = 16.0;

/// The canvas context property name for fill style.
pub(crate) const RENDERER_PROPERTY_FILL_STYLE: &str = "fillStyle";

/// The canvas context property name for stroke style.
pub(crate) const RENDERER_PROPERTY_STROKE_STYLE: &str = "strokeStyle";

/// The canvas context property name for line width.
pub(crate) const RENDERER_PROPERTY_LINE_WIDTH: &str = "lineWidth";

/// The canvas context property name for font.
pub(crate) const RENDERER_PROPERTY_FONT: &str = "font";

/// The canvas context property name for global alpha.
pub(crate) const RENDERER_PROPERTY_GLOBAL_ALPHA: &str = "globalAlpha";

/// The canvas context property name for global composite operation (blend mode).
pub(crate) const RENDERER_PROPERTY_GLOBAL_COMPOSITE_OPERATION: &str = "globalCompositeOperation";

/// The canvas context property name for shadow color.
pub(crate) const RENDERER_PROPERTY_SHADOW_COLOR: &str = "shadowColor";

/// The canvas context property name for shadow blur.
pub(crate) const RENDERER_PROPERTY_SHADOW_BLUR: &str = "shadowBlur";

/// The canvas context property name for shadow offset x.
pub(crate) const RENDERER_PROPERTY_SHADOW_OFFSET_X: &str = "shadowOffsetX";

/// The canvas context property name for shadow offset y.
pub(crate) const RENDERER_PROPERTY_SHADOW_OFFSET_Y: &str = "shadowOffsetY";

/// The default camera zoom level.
pub(crate) const RENDERER_DEFAULT_CAMERA_ZOOM: f64 = 1.0;

/// The default camera rotation in radians.
pub(crate) const RENDERER_DEFAULT_CAMERA_ROTATION: f64 = 0.0;

/// The canvas context property name for text rendering.
pub(crate) const RENDERER_PROPERTY_TEXT_RENDERING: &str = "textRendering";

/// The high-quality text rendering value for geometric precision.
pub(crate) const RENDERER_TEXT_RENDERING_GEOMETRIC_PRECISION: &str = "geometricPrecision";

/// The canvas context property name for image smoothing enabled.
pub(crate) const RENDERER_PROPERTY_IMAGE_SMOOTHING_ENABLED: &str = "imageSmoothingEnabled";

/// The canvas context property name for image smoothing quality.
pub(crate) const RENDERER_PROPERTY_IMAGE_SMOOTHING_QUALITY: &str = "imageSmoothingQuality";

/// The high-quality image smoothing value.
pub(crate) const RENDERER_IMAGE_SMOOTHING_QUALITY_HIGH: &str = "high";

/// The HTML element tag name for creating a canvas element.
pub(crate) const RENDERER_ELEMENT_CANVAS: &str = "canvas";

/// The default SSAA scale factor (2.0 means 4x supersampling).
pub(crate) const RENDERER_DEFAULT_SSAA_SCALE_FACTOR: f64 = 2.0;

/// The CSS composite operation string for the `Normal` blend mode.
pub(crate) const BLEND_MODE_NORMAL: &str = "source-over";

/// The CSS composite operation string for the `Multiply` blend mode.
pub(crate) const BLEND_MODE_MULTIPLY: &str = "multiply";

/// The CSS composite operation string for the `Screen` blend mode.
pub(crate) const BLEND_MODE_SCREEN: &str = "screen";

/// The CSS composite operation string for the `Lighter` blend mode.
pub(crate) const BLEND_MODE_LIGHTER: &str = "lighter";

/// The CSS composite operation string for the `Overlay` blend mode.
pub(crate) const BLEND_MODE_OVERLAY: &str = "overlay";

/// The CSS composite operation string for the `Darken` blend mode.
pub(crate) const BLEND_MODE_DARKEN: &str = "darken";

/// The CSS composite operation string for the `Lighten` blend mode.
pub(crate) const BLEND_MODE_LIGHTEN: &str = "lighten";

/// The CSS composite operation string for the `ColorDodge` blend mode.
pub(crate) const BLEND_MODE_COLOR_DODGE: &str = "color-dodge";

/// The CSS composite operation string for the `ColorBurn` blend mode.
pub(crate) const BLEND_MODE_COLOR_BURN: &str = "color-burn";

/// The CSS composite operation string for the `HardLight` blend mode.
pub(crate) const BLEND_MODE_HARD_LIGHT: &str = "hard-light";

/// The CSS composite operation string for the `SoftLight` blend mode.
pub(crate) const BLEND_MODE_SOFT_LIGHT: &str = "soft-light";

/// The CSS composite operation string for the `Difference` blend mode.
pub(crate) const BLEND_MODE_DIFFERENCE: &str = "difference";

/// The CSS composite operation string for the `Exclusion` blend mode.
pub(crate) const BLEND_MODE_EXCLUSION: &str = "exclusion";

/// The CSS composite operation string for the `Hue` blend mode.
pub(crate) const BLEND_MODE_HUE: &str = "hue";

/// The CSS composite operation string for the `Saturation` blend mode.
pub(crate) const BLEND_MODE_SATURATION: &str = "saturation";

/// The CSS composite operation string for the `Color` blend mode.
pub(crate) const BLEND_MODE_COLOR: &str = "color";

/// The CSS composite operation string for the `Luminosity` blend mode.
pub(crate) const BLEND_MODE_LUMINOSITY: &str = "luminosity";

/// The default shadow color used when no explicit color is provided.
pub(crate) const RENDERER_DEFAULT_SHADOW_COLOR: &str = "rgba(0, 0, 0, 0.5)";

/// The default shadow blur radius in pixels.
pub(crate) const RENDERER_DEFAULT_SHADOW_BLUR: f64 = 4.0;

/// The default render layer z-index for background elements.
pub(crate) const RENDERER_LAYER_BACKGROUND: i32 = 0;

/// The default render layer z-index for foreground game objects.
pub(crate) const RENDERER_LAYER_FOREGROUND: i32 = 100;

/// The default render layer z-index for UI overlay elements.
pub(crate) const RENDERER_LAYER_UI: i32 = 1000;
