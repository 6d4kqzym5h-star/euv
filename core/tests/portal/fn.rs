use super::*;

#[test]
fn portal_tag_is_clone() {
    let tag: Tag = Tag::Portal(Cow::Borrowed("#modal-root"));
    let cloned: Tag = tag.clone();
    assert_eq!(tag, cloned);
}

#[test]
fn portal_tag_partial_eq_distinguishes_selectors() {
    assert_eq!(
        Tag::Portal(Cow::Borrowed("#a")),
        Tag::Portal(Cow::Borrowed("#a")),
    );
    assert_ne!(
        Tag::Portal(Cow::Borrowed("#a")),
        Tag::Portal(Cow::Borrowed("#b")),
    );
    assert_ne!(
        Tag::Portal(Cow::Borrowed("body")),
        Tag::Element(Cow::Borrowed("body")),
        "Portal(\"body\") must not equal Element(\"body\")",
    );
}

#[test]
fn portal_tag_is_hashable() {
    let mut set: HashSet<Tag> = HashSet::new();
    set.insert(Tag::Portal(Cow::Borrowed("#a")));
    set.insert(Tag::Portal(Cow::Borrowed("#a")));
    set.insert(Tag::Portal(Cow::Borrowed("#b")));
    assert_eq!(set.len(), 2, "selectors collapse by equality, not identity");
}

#[test]
fn portal_tag_debug_names_variant() {
    let formatted: String = format!("{:?}", Tag::Portal(Cow::Borrowed("#x")));
    assert!(
        formatted.contains("Portal"),
        "Debug output must name the variant, got: {formatted}",
    );
    assert!(
        formatted.contains("#x"),
        "Debug output must include the selector for traceability, got: {formatted}",
    );
}

#[test]
fn virtual_node_try_get_tag_name_returns_none_for_portal() {
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Portal(Cow::Borrowed("#toast-host")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    assert_eq!(
        node.try_get_tag_name(),
        None,
        "Portal must not expose a tag name through the public API",
    );
}

#[test]
fn virtual_node_try_get_tag_name_still_works_for_element() {
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Element(Cow::Borrowed("div")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    assert_eq!(
        node.try_get_tag_name(),
        Some(String::from("div")),
        "Element tag name must still resolve",
    );
}

#[test]
fn virtual_node_try_get_tag_name_still_works_for_component() {
    let node: VirtualNode = VirtualNode::Element {
        tag: Tag::Component(Cow::Borrowed("euv_button")),
        attributes: Vec::new(),
        children: Vec::new(),
        key: None,
        props: None,
    };
    assert_eq!(
        node.try_get_tag_name(),
        Some(String::from("euv_button")),
        "Component tag name must still resolve",
    );
}

#[test]
fn portal_tag_partial_eq_same_selector_equals() {
    let a: Tag = Tag::Portal(Cow::Borrowed("#modal-root"));
    let b: Tag = Tag::Portal(Cow::Borrowed("#modal-root"));
    assert!(a == b, "same-selector portals must compare equal");
}

#[test]
fn portal_tag_supports_empty_selector() {
    let tag: Tag = Tag::Portal(Cow::Owned(String::new()));
    assert_eq!(tag.clone(), tag, "empty selector must round-trip");
}
