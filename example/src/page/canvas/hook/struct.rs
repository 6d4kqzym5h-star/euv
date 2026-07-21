use super::*;

/// Reactive state for the canvas drawing board page.
///
/// Aggregates all signals needed for the drawing board including
/// drawing status, stroke configuration, fullscreen mode, and
/// a snapshot of the current canvas content as a data URL.
#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub(crate) struct UseCanvas {
    /// Whether the user is currently drawing on the canvas.
    #[get(type(copy))]
    pub(crate) drawing: Signal<bool>,
    /// The current stroke color as a CSS color string.
    #[get(type(copy))]
    pub(crate) stroke_color: Signal<String>,
    /// The current line width in pixels.
    #[get(type(copy))]
    pub(crate) line_width: Signal<f64>,
    /// Whether the canvas is currently in fullscreen mode.
    #[get(type(copy))]
    pub(crate) fullscreen: Signal<bool>,
    /// The data URL of the current canvas snapshot for preview.
    #[get(type(copy))]
    pub(crate) snapshot_data_url: Signal<String>,
    /// The x coordinate of the last pointer position during drawing.
    #[get(type(copy))]
    pub(crate) last_x: Signal<f64>,
    /// The y coordinate of the last pointer position during drawing.
    #[get(type(copy))]
    pub(crate) last_y: Signal<f64>,
    /// A map from touch identifier to its last known canvas coordinates,
    /// used to draw continuous line segments for each touch point.
    #[get(type(copy))]
    pub(crate) touch_last_points: Signal<HashMap<i32, (f64, f64)>>,
}
