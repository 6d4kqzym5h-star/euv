use super::*;

/// Trait for types that can be converted into a reactive text `VirtualNode`.
///
/// Signals implement this to produce a text node that auto-updates.
pub(crate) trait AsReactiveText {
    /// Converts this value into a `VirtualNode::Text` with reactive signal binding.
    fn as_reactive_text(&self) -> VirtualNode;
}
