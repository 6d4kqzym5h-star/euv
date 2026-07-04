use crate::*;

use std::rc::Rc;

/// Implementation of default configuration for virtual list.
impl Default for EuvVirtualListConfig {
    /// Returns a default configuration with sensible defaults.
    ///
    /// The default id is "virtual-list-default". For multiple instances,
    /// always provide unique ids.
    ///
    /// # Returns
    ///
    /// - `EuvVirtualListConfig` - Default configuration with 1000 items, 44px height, and 5 overscan items.
    fn default() -> Self {
        Self {
            id: String::from("virtual-list-default"),
            total_count: 1000,
            item_height: 44,
            overscan_count: 5,
        }
    }
}

/// Implementation of default props for virtual list.
impl Default for EuvVirtualListProps {
    /// Returns default props with empty callbacks.
    ///
    /// # Returns
    ///
    /// - `EuvVirtualListProps` - Default props with default config and empty item renderer.
    fn default() -> Self {
        Self {
            config: EuvVirtualListConfig::default(),
            item_renderer: Rc::new(|_: usize| VirtualNode::Empty),
            on_scroll: None,
            on_visible_range_change: None,
        }
    }
}
