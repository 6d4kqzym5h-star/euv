/// The total number of items in the virtual list demo.
pub(crate) const VIRTUAL_LIST_TOTAL_COUNT: usize = 10000;

/// The fixed height in pixels for each virtual list item row.
pub(crate) const VIRTUAL_LIST_ITEM_HEIGHT: i32 = 44;

/// The number of extra items rendered above and below the visible viewport
/// to reduce flicker during fast scrolling.
pub(crate) const VIRTUAL_LIST_OVERSCAN_COUNT: usize = 5;

/// The HTML id for the virtual list scroll container element.
pub(crate) const VIRTUAL_LIST_CONTAINER_ID: &str = "virtual-list-container";
