use crate::*;

/// Represents a single touch point from a multi-touch event.
///
/// Each `NativeTouchPoint` corresponds to one finger or stylus currently
/// touching the screen. The `identifier` field distinguishes between
/// simultaneous touch points, enabling multi-finger gesture tracking.
#[derive(Clone, Data, Debug, Default, Eq, PartialEq)]
pub(crate) struct NativeTouchPoint {
    /// A unique identifier for this touch point.
    ///
    /// The browser assigns a distinct `identifier` to each active touch.
    /// It remains constant for the duration of the touch (from
    /// `touchstart` to `touchend`), allowing the same finger to be
    /// tracked across `touchmove` events.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) identifier: i32,
    /// The X coordinate of the touch relative to the viewport.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) client_x: i32,
    /// The Y coordinate of the touch relative to the viewport.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) client_y: i32,
    /// The X coordinate of the touch relative to the screen.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) screen_x: i32,
    /// The Y coordinate of the touch relative to the screen.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) screen_y: i32,
    /// The X coordinate of the touch relative to the target element.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) offset_x: i32,
    /// The Y coordinate of the touch relative to the target element.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) offset_y: i32,
    /// The X coordinate of the touch relative to the page.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) page_x: i32,
    /// The Y coordinate of the touch relative to the page.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) page_y: i32,
}

/// Represents a single touch point with high-precision `f64` coordinates.
///
/// Used for pixel-precise interactions such as canvas drawing, where
/// sub-pixel accuracy matters. The `identifier` field distinguishes
/// between simultaneous touch points, enabling multi-finger gesture
/// tracking.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub(crate) struct NativeTouchPointF64 {
    /// A unique identifier for this touch point.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) identifier: i32,
    /// The X coordinate of the touch relative to the viewport.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) client_x: f64,
    /// The Y coordinate of the touch relative to the viewport.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) client_y: f64,
    /// The X coordinate of the touch relative to the screen.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) screen_x: f64,
    /// The Y coordinate of the touch relative to the screen.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) screen_y: f64,
    /// The X coordinate of the touch relative to the target element.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) offset_x: f64,
    /// The Y coordinate of the touch relative to the target element.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) offset_y: f64,
    /// The X coordinate of the touch relative to the page.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) page_x: f64,
    /// The Y coordinate of the touch relative to the page.
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(crate) page_y: f64,
}
