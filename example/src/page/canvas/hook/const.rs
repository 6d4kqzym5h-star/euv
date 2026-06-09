/// The HTML `id` attribute value for the drawing canvas element.
pub(crate) const CANVAS_DRAWING_ID: &str = "drawing-canvas";

/// The CSS selector used to query the drawing canvas element from the DOM.
pub(crate) const CANVAS_DRAWING_SELECTOR: &str = "#drawing-canvas";

/// The HTML `id` attribute value for the canvas container wrapper element.
pub(crate) const CANVAS_CONTAINER_ID: &str = "canvas-container";

/// The HTML `id` attribute value for the fullscreen canvas drawing wrapper.
pub(crate) const CANVAS_FULLSCREEN_WRAPPER_ID: &str = "fullscreen-canvas-wrapper";

/// The CSS selector used to query the fullscreen canvas drawing wrapper.
pub(crate) const CANVAS_FULLSCREEN_WRAPPER_SELECTOR: &str = "#fullscreen-canvas-wrapper";

/// The default stroke color for drawing.
pub(crate) const CANVAS_DEFAULT_STROKE_COLOR: &str = "#000000";

/// The default line width for drawing in pixels.
pub(crate) const CANVAS_DEFAULT_LINE_WIDTH: f64 = 3.0;

/// The canvas background fill color.
pub(crate) const CANVAS_BACKGROUND_COLOR: &str = "#ffffff";

/// The minimum allowed line width for drawing.
pub(crate) const CANVAS_MIN_LINE_WIDTH: f64 = 1.0;

/// The maximum allowed line width for drawing.
pub(crate) const CANVAS_MAX_LINE_WIDTH: f64 = 30.0;

/// The line width step for the range slider.
pub(crate) const CANVAS_LINE_WIDTH_STEP: f64 = 1.0;

/// The label displayed on the draw button.
pub(crate) const CANVAS_DRAW_LABEL: &str = "Draw";

/// The label displayed on the fullscreen exit button.
pub(crate) const CANVAS_FULLSCREEN_EXIT_LABEL: &str = "Exit";
