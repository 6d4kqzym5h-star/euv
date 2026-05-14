use crate::*;

/// The return type of a component function.
///
/// A component may return a virtual node or nothing.
pub type ComponentElement = Option<VirtualNode>;
