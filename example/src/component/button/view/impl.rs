use crate::*;

/// Implementation of strongly-typed props extraction for `PrimaryButtonProps`.
impl From<VirtualNode> for PrimaryButtonProps {
    /// Extracts typed props from a `VirtualNode`.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The virtual node containing attributes.
    ///
    /// # Returns
    ///
    /// - `Self` - The strongly-typed `PrimaryButtonProps`.
    fn from(node: VirtualNode) -> Self {
        Self {
            label: node
                .try_get_prop("label")
                .unwrap_or_else(|| "Button".to_string()),
            onclick: node.try_get_event("onclick"),
            disabled: node.try_get_typed_prop("disabled").unwrap_or(false),
        }
    }
}
