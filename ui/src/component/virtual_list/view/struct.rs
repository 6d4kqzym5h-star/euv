use crate::*;

/// Configuration for a virtual list instance.
///
/// Defines the total item count, item height, overscan buffer size,
/// and a unique container id for a virtual list. This struct allows
/// multiple virtual list instances with different configurations
/// on the same page.
#[derive(Clone, CustomDebug, Data, New)]
pub struct EuvVirtualListConfig {
    /// The unique identifier for this virtual list container.
    /// Used to reference the scroll container element.
    pub id: String,
    /// The total number of items in the list.
    #[get(type(copy))]
    pub total_count: usize,
    /// The fixed height of each item in pixels.
    #[get(type(copy))]
    pub item_height: i32,
    /// The number of extra items to render above and below the visible viewport.
    #[get(type(copy))]
    pub overscan_count: usize,
}

/// Props for the `euv_virtual_list` component.
///
/// Encapsulates the configuration, item renderer callback, and optional
/// event callbacks for the virtual list.
#[derive(Clone, CustomDebug, Data, New)]
pub struct EuvVirtualListProps {
    pub config: EuvVirtualListConfig,
    #[debug(skip)]
    pub item_renderer: VirtualListItemRenderer,
    #[debug(skip)]
    pub on_scroll: Option<VirtualListScrollHandler>,
    #[debug(skip)]
    pub on_visible_range_change: Option<VirtualListRangeHandler>,
}
