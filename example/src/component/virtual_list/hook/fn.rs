use crate::*;

/// Creates virtual list state signals wrapped in a `UseVirtualList` struct.
///
/// # Returns
///
/// - `UseVirtualList` - The virtual list state containing scroll offset and viewport height signals.
pub(crate) fn use_virtual_list() -> UseVirtualList {
    UseVirtualList::new(use_signal(|| 0), use_signal(|| 0))
}
