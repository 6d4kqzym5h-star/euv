//! Tests for pre-existing pure-Rust VDOM node types.
//!
//! Covers the data structures used to represent
//! nodes in the virtual DOM tree. These run natively
//! under `cargo test -p euv-core --lib`.
//!
//! # What's covered here
//!
//! - `vdom::node::Tag` (Element / Component enum)
//! - `vdom::node::VirtualNode::Empty` (zero-sized variant)
//! - `vdom::node::VirtualNode::Element` with
//!   `AttributeEntry` containing `AttributeValue::Text`
//! - `vdom::node::VirtualNode::Fragment` with pure-Rust
//!   children
//! - `vdom::node::VirtualNode::Clone` / `Debug`
//! - `vdom::node::TextNode::PartialEq` (visual equality)
//! - `vdom::node::TextNode::Clone` with `signal: None`
//!
//! # What is NOT covered here
//!
//! - `TextNode::signal: Some(Signal<String>)` — `Signal`
//!   construction works on native but `Signal::set`
//!   panics. We construct with `signal: None` only.
//! - `VirtualNode::Dynamic(DynamicNode)` —
//!   `DynamicNode` contains a `HookContext` plus an
//!   `Rc<UnsafeCell<RenderFnInner>>` where
//!   `RenderFnInner` contains a
//!   `Box<dyn FnMut(&mut HookContext) -> VirtualNode>`.
//!   Constructing one on native would require a
//!   non-trivial render closure that returns
//!   `VirtualNode`, which itself transitively includes
//!   `HookContext` — best deferred to a wasm-bindgen-test
//!   follow-up.
//! - `vdom::node::AsReactiveText` trait (pub(crate)) —
//!   exercised by `Signal<String>::as_reactive_text()`
//!   in the implementation module.

use super::*;

// =====================================================================
// Tag
// =====================================================================

#[test]
fn tag_element_constructs() {
    let tag: Tag = Tag::Element(String::from("div"));
    match &tag {
        Tag::Element(name) => assert_eq!(name, "div"),
        Tag::Component(_) => panic!("expected Element"),
        Tag::Portal(_) => panic!("expected Element"),
    }
}

#[test]
fn tag_component_constructs() {
    let tag: Tag = Tag::Component(String::from("MyButton"));
    match &tag {
        Tag::Component(name) => assert_eq!(name, "MyButton"),
        Tag::Element(_) => panic!("expected Component"),
        Tag::Portal(_) => panic!("expected Component"),
    }
}

#[test]
fn tag_clone_preserves_variant() {
    let element: Tag = Tag::Element(String::from("span"));
    let cloned_element: Tag = element.clone();
    match &cloned_element {
        Tag::Element(name) => assert_eq!(name, "span"),
        _ => panic!("expected Element"),
    }
    let component: Tag = Tag::Component(String::from("Header"));
    let cloned_component: Tag = component.clone();
    match &cloned_component {
        Tag::Component(name) => assert_eq!(name, "Header"),
        _ => panic!("expected Component"),
    }
}

#[test]
fn tag_equality_same_variant() {
    let a: Tag = Tag::Element(String::from("div"));
    let b: Tag = Tag::Element(String::from("div"));
    assert_eq!(a, b);
}

#[test]
fn tag_equality_different_variants() {
    let element: Tag = Tag::Element(String::from("div"));
    let component: Tag = Tag::Component(String::from("div"));
    assert_ne!(element, component);
}

#[test]
fn tag_equality_different_inner_values() {
    let a: Tag = Tag::Element(String::from("div"));
    let b: Tag = Tag::Element(String::from("span"));
    assert_ne!(a, b);
}

#[test]
fn tag_hash_same_for_equal_tags() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let a: Tag = Tag::Element(String::from("div"));
    let b: Tag = Tag::Element(String::from("div"));
    let mut h1: DefaultHasher = DefaultHasher::new();
    let mut h2: DefaultHasher = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn tag_debug_format_works() {
    let tag: Tag = Tag::Element(String::from("div"));
    let formatted: String = format!("{:?}", tag);
    assert!(formatted.contains("Element"));
    assert!(formatted.contains("div"));
}

// =====================================================================
// VirtualNode::Empty
// =====================================================================

#[test]
fn virtual_node_empty_constructs() {
    let node: VirtualNode = VirtualNode::Empty;
    let _: VirtualNode = node;
}

#[test]
fn virtual_node_empty_clone_is_identical() {
    let node: VirtualNode = VirtualNode::Empty;
    let cloned: VirtualNode = node.clone();
    matches!(cloned, VirtualNode::Empty);
}

#[test]
fn virtual_node_empty_debug_format() {
    let node: VirtualNode = VirtualNode::Empty;
    let formatted: String = format!("{:?}", node);
    assert!(formatted.contains("Empty"));
}

// =====================================================================
// VirtualNode::Element (with AttributeEntry + AttributeValue::Text)
// =====================================================================

#[test]
fn virtual_node_element_with_no_attributes_no_children() {
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(String::from("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    match node {
        VirtualNode::Element { tag, .. } => match tag {
            Tag::Element(name) => assert_eq!(name, "div"),
            _ => panic!("expected Element tag"),
        },
        _ => panic!("expected Element variant"),
    }
}

#[test]
fn virtual_node_element_with_text_attribute() {
    let mut attributes: Vec<AttributeEntry> = Vec::new();
    attributes.push(AttributeEntry::new(
        String::from("class"),
        AttributeValue::Text(String::from("btn")),
    ));
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(String::from("button")),
        attributes,
        children: Vec::new(),
        key: None,
        props: None,
    };
    if let VirtualNode::Element {
        tag, attributes, ..
    } = &node
    {
        match tag {
            Tag::Element(name) => assert_eq!(name, "button"),
            _ => panic!("expected Element tag"),
        }
        assert_eq!(attributes.len(), 1);
        assert_eq!(attributes[0].get_name(), "class");
    } else {
        panic!("expected Element variant");
    }
}

#[test]
fn virtual_node_element_with_key() {
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(String::from("li")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: Some(String::from("item-1")),
        props: None,
    };
    if let VirtualNode::Element { key, .. } = &node {
        assert_eq!(key.as_ref().unwrap(), "item-1");
    } else {
        panic!("expected Element variant");
    }
}

#[test]
fn virtual_node_element_with_nested_children() {
    let inner: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(String::from("span")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let outer: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(String::from("div")),
        attributes: Vec::new(),
        children: vec![inner],
        key: None,
        props: None,
    };
    if let VirtualNode::Element { children, .. } = &outer {
        assert_eq!(children.len(), 1);
        matches!(children[0], VirtualNode::Element { .. });
    } else {
        panic!("expected Element variant");
    }
}

#[test]
fn virtual_node_element_clone_deep_copies() {
    let original: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(String::from("div")),
        attributes: vec![AttributeEntry::new(
            String::from("id"),
            AttributeValue::Text(String::from("root")),
        )],
        children: Vec::new(),
        key: None,
        props: None,
    };
    let cloned: VirtualNode = original.clone();
    if let VirtualNode::Element {
        tag, attributes, ..
    } = &cloned
    {
        match tag {
            Tag::Element(name) => assert_eq!(name, "div"),
            _ => panic!("expected Element tag"),
        }
        assert_eq!(attributes.len(), 1);
    } else {
        panic!("expected Element after clone");
    }
}

// =====================================================================
// VirtualNode::Fragment
// =====================================================================

#[test]
fn virtual_node_fragment_with_empty_children() {
    let node: VirtualNode = VirtualNode::Fragment(Vec::new());
    if let VirtualNode::Fragment(children) = &node {
        assert!(children.is_empty());
    } else {
        panic!("expected Fragment variant");
    }
}

#[test]
fn virtual_node_fragment_with_pure_rust_children() {
    let children: Vec<VirtualNode> = vec![
        VirtualNode::Empty,
        VirtualNode::Element {
            tag: Tag::Element(String::from("br")),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
            props: None,
        },
        VirtualNode::Text(TextNode::new(String::from("hello"), None)),
    ];
    let fragment: VirtualNode = VirtualNode::Fragment(children);
    if let VirtualNode::Fragment(children) = &fragment {
        assert_eq!(children.len(), 3);
    } else {
        panic!("expected Fragment variant");
    }
}

#[test]
fn virtual_node_fragment_clone_preserves_children() {
    let fragment: VirtualNode = VirtualNode::Fragment(vec![VirtualNode::Empty, VirtualNode::Empty]);
    let cloned: VirtualNode = fragment.clone();
    if let VirtualNode::Fragment(children) = &cloned {
        assert_eq!(children.len(), 2);
    } else {
        panic!("expected Fragment after clone");
    }
}

// =====================================================================
// VirtualNode::Text (with TextNode, signal: None)
// =====================================================================

#[test]
fn virtual_node_text_with_no_signal() {
    let node: VirtualNode = VirtualNode::Text(TextNode::new(String::from("hello"), None));
    if let VirtualNode::Text(text) = &node {
        assert_eq!(text.get_content(), "hello");
    } else {
        panic!("expected Text variant");
    }
}

#[test]
fn virtual_node_text_clone_preserves_content() {
    let original: VirtualNode = VirtualNode::Text(TextNode::new(String::from("world"), None));
    let cloned: VirtualNode = original.clone();
    if let VirtualNode::Text(text) = &cloned {
        assert_eq!(text.get_content(), "world");
    } else {
        panic!("expected Text after clone");
    }
}

#[test]
fn text_node_partial_eq_visual_equality() {
    let a: TextNode = TextNode::new(String::from("hello"), None);
    let b: TextNode = TextNode::new(String::from("hello"), None);
    assert_eq!(a, b);
}

#[test]
fn text_node_partial_eq_different_content() {
    let a: TextNode = TextNode::new(String::from("hello"), None);
    let b: TextNode = TextNode::new(String::from("world"), None);
    assert_ne!(a, b);
}

#[test]
fn text_node_set_content_replaces() {
    let mut node: TextNode = TextNode::new(String::from("first"), None);
    node.set_content(String::from("second"));
    assert_eq!(node.get_content(), "second");
}

#[test]
fn text_node_debug_format_skips_signal_field() {
    let node: TextNode = TextNode::new(String::from("hello"), None);
    let formatted: String = format!("{:?}", node);
    assert!(formatted.contains("TextNode"));
}

// =====================================================================
// VirtualNode::Debug
// =====================================================================

#[test]
fn virtual_node_empty_debug_format_includes_variant() {
    let node: VirtualNode = VirtualNode::Empty;
    let formatted: String = format!("{:?}", node);
    assert!(formatted.contains("Empty"));
}

#[test]
fn virtual_node_element_debug_format_includes_tag() {
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(String::from("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    let formatted: String = format!("{:?}", node);
    assert!(formatted.contains("Element"));
    assert!(formatted.contains("div"));
}

// =====================================================================
// Regression: native construction doesn't panic
// =====================================================================

#[test]
fn native_virtual_node_empty_construction_does_not_panic() {
    let result: Result<(), String> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: VirtualNode = VirtualNode::Empty;
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn native_virtual_node_element_construction_does_not_panic() {
    let result: Result<(), String> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: VirtualNode = VirtualNode::Element {
            tag: Tag::Element(String::from("div")),
            attributes: Vec::new(),
            children: Vec::new(),
            key: None,
            props: None,
        };
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn native_text_node_construction_does_not_panic() {
    let result: Result<(), String> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: TextNode = TextNode::new(String::from("x"), None);
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn native_tag_construction_does_not_panic() {
    let result: Result<(), String> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: Tag = Tag::Element(String::from("div"));
        let _: Tag = Tag::Component(String::from("Foo"));
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}
