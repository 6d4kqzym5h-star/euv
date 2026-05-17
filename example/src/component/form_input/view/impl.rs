use crate::*;

/// Implementation of strongly-typed props extraction for `FormInputProps`.
impl From<VirtualNode> for FormInputProps {
    /// Extracts typed props from a `VirtualNode`.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The virtual node containing attributes.
    ///
    /// # Returns
    ///
    /// - `Self` - The strongly-typed `FormInputProps`.
    fn from(node: VirtualNode) -> Self {
        FormInputProps {
            label: node.try_get_prop("label").unwrap_or_default(),
            placeholder: node.try_get_prop("placeholder").unwrap_or_default(),
            value: node.try_get_prop("value").unwrap_or_default(),
        }
    }
}
