use crate::*;

/// Trait for types that can be converted into a reactive text `VirtualNode`.
///
/// Signals implement this to produce a text node that auto-updates.
pub(crate) trait AsReactiveText {
    /// Converts this value into a `VirtualNode::Text` with reactive signal binding.
    fn as_reactive_text(&self) -> VirtualNode;
}

/// Trait for converting a value into a `VirtualNode` by consuming it.
///
/// Unlike `AsNode` which borrows `&self`, this trait takes ownership,
/// enabling closures to be wrapped into `DynamicNode` for reactive re-rendering.
pub trait IntoNode {
    /// Converts this value into a `VirtualNode` by consuming it.
    fn into_node(self) -> VirtualNode;
}
