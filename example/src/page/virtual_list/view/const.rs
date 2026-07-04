/// The unique identifier for the virtual list demo container.
pub(crate) const VIRTUAL_LIST_DEMO_ID: &str = "virtual-list-demo";

/// The total number of items in the virtual list demo.
pub(crate) const VIRTUAL_LIST_DEMO_TOTAL_COUNT: usize = 10000;

/// The fixed height of each item in the virtual list demo (pixels).
pub(crate) const VIRTUAL_LIST_DEMO_ITEM_HEIGHT: i32 = 44;

/// The number of overscan items in the virtual list demo.
pub(crate) const VIRTUAL_LIST_DEMO_OVERSCAN_COUNT: usize = 5;

/// The subtitle text for the virtual list demo page.
/// Note: This must match VIRTUAL_LIST_DEMO_TOTAL_COUNT. When updating the count,
/// update this string accordingly.
pub(crate) const VIRTUAL_LIST_DEMO_SUBTITLE: &str = "High-performance windowed list rendering 10000 items with minimal DOM nodes. Only visible rows are rendered.";
