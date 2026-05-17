use crate::*;

#[test]
fn test_element_creation() {
    let node: VirtualNode = VirtualNode::get_element_node("div");
    match node {
        VirtualNode::Element {
            tag,
            attributes,
            children,
            key,
        } => {
            assert!(matches!(tag, Tag::Element(ref name) if name == "div"));
            assert!(attributes.is_empty());
            assert!(children.is_empty());
            assert!(key.is_none());
        }
        _ => panic!("expected element node"),
    }
}

#[test]
fn test_text_creation() {
    let node: VirtualNode = VirtualNode::get_text_node("hello");
    match node {
        VirtualNode::Text(TextNode { content, .. }) => {
            assert_eq!(content, "hello");
        }
        _ => panic!("expected text node"),
    }
}

#[test]
fn test_with_attribute() {
    let node: VirtualNode = VirtualNode::get_element_node("div")
        .with_attribute("class", AttributeValue::Text("app".to_string()));
    match node {
        VirtualNode::Element { attributes, .. } => {
            assert_eq!(attributes.len(), 1);
            assert_eq!(attributes[0].get_name(), "class");
            match attributes[0].get_value() {
                AttributeValue::Text(value) => assert_eq!(value, "app"),
                _ => panic!("expected text attribute value"),
            }
        }
        _ => panic!("expected element node"),
    }
}

#[test]
fn test_with_child() {
    let child: VirtualNode = VirtualNode::get_text_node("hello");
    let node: VirtualNode = VirtualNode::get_element_node("div").with_child(child);
    match node {
        VirtualNode::Element { children, .. } => {
            assert_eq!(children.len(), 1);
        }
        _ => panic!("expected element node"),
    }
}

#[test]
fn test_virtual_node_default() {
    let node: VirtualNode = VirtualNode::default();
    match node {
        VirtualNode::Empty => {}
        _ => panic!("expected empty node"),
    }
}
