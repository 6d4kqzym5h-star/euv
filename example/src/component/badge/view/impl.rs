use crate::*;

/// Implementation of strongly-typed props extraction for `MyBadgeProps`.
impl From<VirtualNode> for MyBadgeProps {
    /// Extracts typed props from a `VirtualNode`.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The virtual node containing attributes.
    ///
    /// # Returns
    ///
    /// - `Self` - The strongly-typed `MyBadgeProps`.
    fn from(node: VirtualNode) -> Self {
        MyBadgeProps {
            color: node
                .try_get_prop("color")
                .unwrap_or_else(|| "#4f46e5".to_string()),
            text: node.try_get_prop("text").unwrap_or_default(),
            outline: node.try_get_prop("outline").is_some(),
            on_click: node.try_get_callback("on-click"),
        }
    }
}
