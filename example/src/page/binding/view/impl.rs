use crate::*;

/// Implementation of strongly-typed props extraction for `ChildDisplayProps`.
///
/// Maps `VirtualNode` attributes to the typed struct fields.
impl From<VirtualNode> for ChildDisplayProps {
    /// Extracts typed props from a `VirtualNode`.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The virtual node containing attributes.
    ///
    /// # Returns
    ///
    /// - `Self` - The strongly-typed `ChildDisplayProps`.
    fn from(node: VirtualNode) -> Self {
        ChildDisplayProps {
            message: node.try_get_prop("message").unwrap_or_default(),
            on_respond: node.try_get_event("onclick"),
        }
    }
}

/// Implementation of strongly-typed props extraction for `LimitedCounterProps`.
///
/// Demonstrates extracting `bool` and `i32` props via `try_get_typed_prop`
/// and custom callbacks via `try_get_callback`.
impl From<VirtualNode> for LimitedCounterProps {
    /// Extracts typed props from a `VirtualNode`.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The virtual node containing attributes.
    ///
    /// # Returns
    ///
    /// - `Self` - The strongly-typed `LimitedCounterProps`.
    fn from(node: VirtualNode) -> Self {
        LimitedCounterProps {
            disabled: node.try_get_typed_prop("disabled").unwrap_or(false),
            max_count: node.try_get_typed_prop("max_count").unwrap_or(10),
            on_increment: node.try_get_callback("on-increment"),
            on_reset: node.try_get_callback("on-reset"),
        }
    }
}

/// Implementation of strongly-typed props extraction for `CallbackInputProps`.
///
/// Demonstrates extracting custom callbacks via `try_get_callback`.
impl From<VirtualNode> for CallbackInputProps {
    /// Extracts typed props from a `VirtualNode`.
    ///
    /// # Arguments
    ///
    /// - `VirtualNode` - The virtual node containing attributes.
    ///
    /// # Returns
    ///
    /// - `Self` - The strongly-typed `CallbackInputProps`.
    fn from(node: VirtualNode) -> Self {
        CallbackInputProps {
            on_change: node.try_get_callback("on-change"),
            on_submit: node.try_get_callback("on-submit"),
            on_reset: node.try_get_callback("on-reset"),
        }
    }
}
