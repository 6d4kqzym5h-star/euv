use crate::*;

/// Type alias for the virtual list item renderer callback.
pub type VirtualListItemRenderer = Rc<dyn Fn(usize) -> VirtualNode>;

/// Type alias for the scroll offset change callback.
pub type VirtualListScrollHandler = Rc<dyn Fn(i32)>;

/// Type alias for the visible range change callback.
pub type VirtualListRangeHandler = Rc<dyn Fn((usize, usize))>;

/// Type alias for backward compatibility.
pub type VirtualListConfig = EuvVirtualListConfig;

/// Type alias for backward compatibility.
pub type VirtualListProps = EuvVirtualListProps;
