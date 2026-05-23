use crate::*;

/// Represents the type of an HTML tag or a component.
///
/// Distinguishes between standard HTML elements and user-defined components.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tag {
    /// A standard HTML element identified by its tag name.
    Element(String),
    /// A custom component type.
    Component(String),
}

/// Represents a node in the virtual DOM tree.
///
/// The core enum representing elements, text, fragments, and empty nodes.
#[derive(Clone, CustomDebug, Default)]
pub enum VirtualNode {
    /// An element node with a tag, attributes, and children.
    Element {
        /// The tag type of this element.
        tag: Tag,
        /// The attributes attached to this element.
        attributes: Vec<AttributeEntry>,
        /// The child nodes.
        children: Vec<VirtualNode>,
        /// An optional key for diffing.
        key: Option<String>,
    },
    /// A text node containing string content and an optional reactive signal.
    Text(TextNode),
    /// A fragment of multiple nodes without a wrapper element.
    Fragment(Vec<VirtualNode>),
    /// A dynamic node that re-renders based on signal changes.
    #[debug(skip)]
    Dynamic(DynamicNode),
    /// An empty placeholder node.
    #[default]
    Empty,
}
