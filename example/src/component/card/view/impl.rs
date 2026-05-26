use crate::*;

/// Implementation of strongly-typed props extraction for `MyCardProps`.
impl From<VirtualNode> for MyCardProps {
    /// Extracts typed props from a `VirtualNode`.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The virtual node containing attributes.
    ///
    /// # Returns
    ///
    /// - `Self` - The strongly-typed `MyCardProps`.
    fn from(node: VirtualNode) -> Self {
        Self {
            title: node.try_get_prop("title").unwrap_or_default(),
        }
    }
}
