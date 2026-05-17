use crate::*;

/// Implementation of strongly-typed props extraction for `MyModalProps`.
impl From<VirtualNode> for MyModalProps {
    /// Extracts typed props from a `VirtualNode`.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The virtual node containing attributes.
    ///
    /// # Returns
    ///
    /// - `Self` - The strongly-typed `MyModalProps`.
    fn from(node: VirtualNode) -> Self {
        MyModalProps {
            title: node.try_get_prop("title").unwrap_or_default(),
            on_close: node.try_get_event("onclick"),
        }
    }
}
