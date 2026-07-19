use crate::*;

/// Represents the type of an HTML tag or a component.
///
/// Distinguishes between standard HTML elements and user-defined components.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Tag {
    /// A standard HTML element identified by its tag name.
    Element(String),
    /// A custom component type.
    Component(String),
}

/// Represents a node in the virtual DOM tree.
///
/// The core enum representing elements, text, fragments, and empty nodes.
/// The generic parameter `T` carries the component props type for component nodes.
/// For non-component nodes, `T` defaults to `()`.
pub enum VirtualNode<T = ()> {
    /// An element node with a tag, attributes, children, and optional props.
    Element {
        /// The tag type of this element.
        tag: Tag,
        /// The attributes attached to this element.
        attributes: Vec<AttributeEntry>,
        /// The child nodes.
        children: Vec<VirtualNode>,
        /// An optional key for diffing.
        key: Option<String>,
        /// The component props, present only for component nodes.
        props: Option<Box<T>>,
    },
    /// A text node containing string content and an optional reactive signal.
    Text(TextNode),
    /// A fragment of multiple nodes without a wrapper element.
    Fragment(Vec<VirtualNode>),
    /// A dynamic node that re-renders based on signal changes.
    Dynamic(DynamicNode),
    /// An empty placeholder node.
    Empty,
}
