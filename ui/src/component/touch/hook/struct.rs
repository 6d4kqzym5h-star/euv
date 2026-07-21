use super::*;

/// Represents a single touch point from a multi-touch event.
///
/// Each `NativeTouchPoint` corresponds to one finger or stylus currently
/// touching the screen. The `identifier` field distinguishes between
/// simultaneous touch points, enabling multi-finger gesture tracking.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub struct NativeTouchPoint {
    /// A unique identifier for this touch point.
    ///
    /// The browser assigns a distinct `identifier` to each active touch.
    /// It remains constant for the duration of the touch (from
    /// `touchstart` to `touchend`), allowing the same finger to be
    /// tracked across `touchmove` events.
    #[get(type(copy))]
    pub identifier: i32,
    /// The X coordinate of the touch relative to the viewport.
    #[get(type(copy))]
    pub client_x: i32,
    /// The Y coordinate of the touch relative to the viewport.
    #[get(type(copy))]
    pub client_y: i32,
    /// The X coordinate of the touch relative to the screen.
    #[get(type(copy))]
    pub screen_x: i32,
    /// The Y coordinate of the touch relative to the screen.
    #[get(type(copy))]
    pub screen_y: i32,
    /// The X coordinate of the touch relative to the target element.
    #[get(type(copy))]
    pub offset_x: i32,
    /// The Y coordinate of the touch relative to the target element.
    #[get(type(copy))]
    pub offset_y: i32,
    /// The X coordinate of the touch relative to the page.
    #[get(type(copy))]
    pub page_x: i32,
    /// The Y coordinate of the touch relative to the page.
    #[get(type(copy))]
    pub page_y: i32,
}

/// Represents a single touch point with high-precision `f64` coordinates.
///
/// Used for pixel-precise interactions such as canvas drawing, where
/// sub-pixel accuracy matters. The `identifier` field distinguishes
/// between simultaneous touch points, enabling multi-finger gesture
/// tracking.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct NativeTouchPointF64 {
    /// A unique identifier for this touch point.
    #[get(type(copy))]
    pub identifier: i32,
    /// The X coordinate of the touch relative to the viewport.
    #[get(type(copy))]
    pub client_x: f64,
    /// The Y coordinate of the touch relative to the viewport.
    #[get(type(copy))]
    pub client_y: f64,
    /// The X coordinate of the touch relative to the screen.
    #[get(type(copy))]
    pub screen_x: f64,
    /// The Y coordinate of the touch relative to the screen.
    #[get(type(copy))]
    pub screen_y: f64,
    /// The X coordinate of the touch relative to the target element.
    #[get(type(copy))]
    pub offset_x: f64,
    /// The Y coordinate of the touch relative to the target element.
    #[get(type(copy))]
    pub offset_y: f64,
    /// The X coordinate of the touch relative to the page.
    #[get(type(copy))]
    pub page_x: f64,
    /// The Y coordinate of the touch relative to the page.
    #[get(type(copy))]
    pub page_y: f64,
}
