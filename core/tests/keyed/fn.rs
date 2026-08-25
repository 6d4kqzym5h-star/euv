use super::*;

fn keyed_element(key: &str, tag: &str) -> VirtualNode {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Owned(tag.to_string())),
        attributes: vec![],
        children: vec![],
        key: Some(key.to_string()),
        props: None,
    }
}
fn unkeyed_element(tag: &str) -> VirtualNode {
    VirtualNode::Element {
        tag: Tag::Element(Cow::Owned(tag.to_string())),
        attributes: vec![],
        children: vec![],
        key: None,
        props: None,
    }
}

#[test]
fn node_key_returns_element_key() {
    let node: VirtualNode = keyed_element("a", "div");
    assert_eq!(node.key(), Some("a"));
}

#[test]
fn node_key_returns_none_for_unkeyed_element() {
    let node: VirtualNode = unkeyed_element("div");
    assert_eq!(node.key(), None);
}

#[test]
fn node_key_returns_none_for_text() {
    let node: VirtualNode = VirtualNode::Text(TextNode::new("hello".to_string(), None));
    assert_eq!(node.key(), None);
}

#[test]
fn node_key_returns_none_for_fragment() {
    let node: VirtualNode = VirtualNode::Fragment(vec![]);
    assert_eq!(node.key(), None);
}

#[test]
fn node_key_returns_none_for_empty() {
    let node: VirtualNode = VirtualNode::Empty;
    assert_eq!(node.key(), None);
}

#[test]
fn node_has_key_matches_node_key_some() {
    let node: VirtualNode = keyed_element("a", "div");
    assert!(node.has_key());
}

#[test]
fn node_has_key_matches_node_key_none() {
    let node: VirtualNode = unkeyed_element("div");
    assert!(!node.has_key());
}

#[test]
fn all_have_keys_true_for_all_keyed() {
    let children: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    assert!(all_have_keys(&children));
}

#[test]
fn all_have_keys_false_when_any_unkeyed() {
    let children: Vec<VirtualNode> = vec![keyed_element("a", "div"), unkeyed_element("span")];
    assert!(!all_have_keys(&children));
}

#[test]
fn all_have_keys_false_for_empty() {
    let children: Vec<VirtualNode> = vec![];
    assert!(!all_have_keys(&children));
}

#[test]
fn all_have_keys_false_for_single_unkeyed() {
    let children: Vec<VirtualNode> = vec![unkeyed_element("div")];
    assert!(!all_have_keys(&children));
}

#[test]
fn diff_children_keyed_when_both_have_keys() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let ops: Vec<DiffOp> = diff_children(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Update { index: 0 }, DiffOp::Update { index: 1 },]
    );
}

#[test]
fn diff_children_positional_when_unkeyed() {
    let old: Vec<VirtualNode> = vec![unkeyed_element("div"), unkeyed_element("span")];
    let new: Vec<VirtualNode> = vec![unkeyed_element("div"), unkeyed_element("span")];
    let ops: Vec<DiffOp> = diff_children(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Update { index: 0 }, DiffOp::Update { index: 1 },]
    );
}

#[test]
fn diff_children_keyed_when_old_unkeyed() {
    let old: Vec<VirtualNode> = vec![unkeyed_element("div")];
    let new: Vec<VirtualNode> = vec![keyed_element("a", "div")];
    let ops: Vec<DiffOp> = diff_children(&old, &new);
    assert_eq!(ops, vec![DiffOp::Update { index: 0 }]);
}

#[test]
fn positional_no_change() {
    let old: Vec<VirtualNode> = vec![unkeyed_element("div"), unkeyed_element("span")];
    let new: Vec<VirtualNode> = old.clone();
    assert_eq!(
        diff_positional(&old, &new),
        vec![DiffOp::Update { index: 0 }, DiffOp::Update { index: 1 },]
    );
}

#[test]
fn positional_insert_at_tail() {
    let old: Vec<VirtualNode> = vec![unkeyed_element("div")];
    let new: Vec<VirtualNode> = vec![unkeyed_element("div"), unkeyed_element("span")];
    assert_eq!(
        diff_positional(&old, &new),
        vec![
            DiffOp::Update { index: 0 },
            DiffOp::Insert {
                index: 1,
                node: unkeyed_element("span"),
            },
        ]
    );
}

#[test]
fn positional_remove_from_tail() {
    let old: Vec<VirtualNode> = vec![unkeyed_element("div"), unkeyed_element("span")];
    let new: Vec<VirtualNode> = vec![unkeyed_element("div")];
    let ops: Vec<DiffOp> = diff_positional(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Update { index: 0 }, DiffOp::Remove { index: 1 },]
    );
}

#[test]
fn positional_empty_to_non_empty() {
    let old: Vec<VirtualNode> = vec![];
    let new: Vec<VirtualNode> = vec![unkeyed_element("div"), unkeyed_element("span")];
    let ops: Vec<DiffOp> = diff_positional(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Insert {
                index: 0,
                node: unkeyed_element("div"),
            },
            DiffOp::Insert {
                index: 1,
                node: unkeyed_element("span"),
            },
        ]
    );
}

#[test]
fn positional_non_empty_to_empty() {
    let old: Vec<VirtualNode> = vec![unkeyed_element("div"), unkeyed_element("span")];
    let new: Vec<VirtualNode> = vec![];
    let ops: Vec<DiffOp> = diff_positional(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Remove { index: 1 }, DiffOp::Remove { index: 0 },]
    );
}

#[test]
fn positional_empty_to_empty() {
    let old: Vec<VirtualNode> = vec![];
    let new: Vec<VirtualNode> = vec![];
    assert!(diff_positional(&old, &new).is_empty());
}

#[test]
fn keyed_no_change() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = old.clone();
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Update { index: 0 }, DiffOp::Update { index: 1 },]
    );
}

#[test]
fn keyed_insert_at_tail() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![
        keyed_element("a", "div"),
        keyed_element("b", "span"),
        keyed_element("c", "p"),
    ];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Update { index: 0 },
            DiffOp::Update { index: 1 },
            DiffOp::Insert {
                index: 2,
                node: keyed_element("c", "p"),
            },
        ]
    );
}

#[test]
fn keyed_insert_at_head() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![
        keyed_element("z", "h1"),
        keyed_element("a", "div"),
        keyed_element("b", "span"),
    ];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Insert {
                index: 0,
                node: keyed_element("z", "h1"),
            },
            DiffOp::Update { index: 1 },
            DiffOp::Update { index: 2 },
        ]
    );
}

#[test]
fn keyed_remove_from_tail() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![keyed_element("a", "div")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Update { index: 0 }, DiffOp::Remove { index: 1 },]
    );
}

#[test]
fn keyed_remove_from_head() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![keyed_element("b", "span")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Update { index: 0 }, DiffOp::Remove { index: 0 },]
    );
}

#[test]
fn keyed_swap_two() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![keyed_element("b", "span"), keyed_element("a", "div")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Update { index: 0 }, DiffOp::Update { index: 1 },]
    );
}

#[test]
fn keyed_reorder_three() {
    let old: Vec<VirtualNode> = vec![
        keyed_element("a", "div"),
        keyed_element("b", "span"),
        keyed_element("c", "p"),
    ];
    let new: Vec<VirtualNode> = vec![
        keyed_element("c", "p"),
        keyed_element("a", "div"),
        keyed_element("b", "span"),
    ];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Update { index: 0 },
            DiffOp::Update { index: 1 },
            DiffOp::Update { index: 2 },
        ]
    );
}

#[test]
fn keyed_insert_and_remove() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("c", "p")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Update { index: 0 },
            DiffOp::Insert {
                index: 1,
                node: keyed_element("c", "p"),
            },
            DiffOp::Remove { index: 1 },
        ]
    );
}

#[test]
fn keyed_replace_all() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![keyed_element("x", "h1"), keyed_element("y", "p")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Insert {
                index: 0,
                node: keyed_element("x", "h1"),
            },
            DiffOp::Insert {
                index: 1,
                node: keyed_element("y", "p"),
            },
            DiffOp::Remove { index: 1 },
            DiffOp::Remove { index: 0 },
        ]
    );
}

#[test]
fn keyed_empty_to_non_empty() {
    let old: Vec<VirtualNode> = vec![];
    let new: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Insert {
                index: 0,
                node: keyed_element("a", "div"),
            },
            DiffOp::Insert {
                index: 1,
                node: keyed_element("b", "span"),
            },
        ]
    );
}

#[test]
fn keyed_non_empty_to_empty() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![DiffOp::Remove { index: 1 }, DiffOp::Remove { index: 0 },]
    );
}

#[test]
fn keyed_single_element_no_change() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div")];
    let new: Vec<VirtualNode> = vec![keyed_element("a", "div")];
    assert_eq!(diff_keyed(&old, &new), vec![DiffOp::Update { index: 0 }]);
}

#[test]
fn keyed_single_element_replaced() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div")];
    let new: Vec<VirtualNode> = vec![keyed_element("b", "span")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Insert {
                index: 0,
                node: keyed_element("b", "span"),
            },
            DiffOp::Remove { index: 0 },
        ]
    );
}

#[test]
fn keyed_move_from_tail_to_head() {
    let old: Vec<VirtualNode> = vec![
        keyed_element("a", "div"),
        keyed_element("b", "span"),
        keyed_element("c", "p"),
    ];
    let new: Vec<VirtualNode> = vec![
        keyed_element("c", "p"),
        keyed_element("a", "div"),
        keyed_element("b", "span"),
    ];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Update { index: 0 },
            DiffOp::Update { index: 1 },
            DiffOp::Update { index: 2 },
        ]
    );
}

#[test]
fn keyed_keeps_ops_in_walk_order() {
    let old: Vec<VirtualNode> = vec![keyed_element("a", "div"), keyed_element("b", "span")];
    let new: Vec<VirtualNode> = vec![keyed_element("b", "span"), keyed_element("c", "p")];
    let ops: Vec<DiffOp> = diff_keyed(&old, &new);
    assert_eq!(
        ops,
        vec![
            DiffOp::Update { index: 0 },
            DiffOp::Insert {
                index: 1,
                node: keyed_element("c", "p"),
            },
            DiffOp::Remove { index: 0 },
        ]
    );
}
#[cfg(all(target_arch = "wasm32", test))]
mod wasm_only {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    use web_sys::{Document, Element};
    fn make_root(suffix: &str) -> Element {
        let document: Document = cached_document().expect("document must exist in wasm test");
        let div: Element = document
            .create_element("div")
            .expect("create_element must succeed");
        let _: Result<(), JsValue> = div.set_attribute("id", suffix);
        let body: web_sys::HtmlElement = document.body().expect("body");
        let body_node: web_sys::Node = body.into();
        let _: Result<web_sys::Node, JsValue> = body_node.append_child(&div);
        div
    }
    fn cleanup(root: &Element) {
        root.remove();
    }
    fn keyed_with_attrs(
        key: &str,
        tag: &str,
        id: Option<&str>,
        class: Option<&str>,
    ) -> VirtualNode {
        let mut attributes: Vec<AttributeEntry> = Vec::new();
        if let Some(id_val) = id {
            attributes.push(AttributeEntry::new(
                Cow::Borrowed("id"),
                AttributeValue::Text(id_val.to_string()),
            ));
        }
        if let Some(class_val) = class {
            attributes.push(AttributeEntry::new(
                Cow::Borrowed("class"),
                AttributeValue::Text(class_val.to_string()),
            ));
        }
        VirtualNode::Element {
            tag: Tag::Element(Cow::Owned(tag.to_string())),
            attributes,
            children: Vec::new(),
            key: Some(key.to_string()),
            props: None,
        }
    }
    fn wrap_parent(children: Vec<VirtualNode>) -> VirtualNode {
        VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("div")),
            attributes: Vec::new(),
            children,
            key: None,
            props: None,
        }
    }
    fn child_ids(root: &Element) -> Vec<String> {
        let list: web_sys::NodeList = root.child_nodes();
        let mut ids: Vec<String> = Vec::new();
        for i in 0..list.length() {
            if let Some(node) = list.get(i)
                && let Some(el) = node.dyn_ref::<Element>()
            {
                ids.push(el.get_attribute("id").unwrap_or_default());
            }
        }
        ids
    }
    #[wasm_bindgen_test]
    fn keyed_reorder_preserves_dom_node_identity() {
        let root: Element = make_root("test-reorder");
        let mut renderer: Renderer = Renderer::new(root.clone());
        let initial: Vec<VirtualNode> = vec![
            keyed_with_attrs("a", "span", Some("a"), Some("first")),
            keyed_with_attrs("b", "span", Some("b"), Some("second")),
            keyed_with_attrs("c", "span", Some("c"), Some("third")),
        ];
        renderer.render(wrap_parent(initial));
        let list: web_sys::NodeList = root.child_nodes();
        for i in 0..list.length() {
            if let Some(node) = list.get(i)
                && let Some(el) = node.dyn_ref::<Element>()
            {
                let _: Result<(), JsValue> = el.set_attribute("data-stamp", &format!("stamp-{i}"));
            }
        }
        let reordered: Vec<VirtualNode> = vec![
            keyed_with_attrs("c", "span", Some("c"), Some("third")),
            keyed_with_attrs("a", "span", Some("a"), Some("first")),
            keyed_with_attrs("b", "span", Some("b"), Some("second")),
        ];
        renderer.render(wrap_parent(reordered));
        assert_eq!(
            root.child_nodes().length(),
            3,
            "reorder must not add or remove DOM children"
        );
        let ids: Vec<String> = child_ids(&root);
        assert_eq!(ids, vec!["c", "a", "b"]);
        let after: web_sys::NodeList = root.child_nodes();
        let stamps: Vec<String> = (0..after.length())
            .map(|i| {
                after
                    .get(i)
                    .and_then(|n| n.dyn_ref::<Element>().cloned())
                    .and_then(|el| el.get_attribute("data-stamp"))
                    .unwrap_or_default()
            })
            .collect();
        assert_eq!(
            stamps,
            vec!["stamp-2", "stamp-0", "stamp-1"],
            "keyed reorder must move DOM nodes, not recreate them"
        );
        cleanup(&root);
    }
    #[wasm_bindgen_test]
    fn keyed_insert_at_tail_appends_dom_node() {
        let root: Element = make_root("test-insert-tail");
        let mut renderer: Renderer = Renderer::new(root.clone());
        let initial: Vec<VirtualNode> = vec![
            keyed_with_attrs("a", "span", Some("a"), None),
            keyed_with_attrs("b", "span", Some("b"), None),
        ];
        renderer.render(wrap_parent(initial));
        assert_eq!(root.child_nodes().length(), 2);
        let mut updated: Vec<VirtualNode> = vec![
            keyed_with_attrs("a", "span", Some("a"), None),
            keyed_with_attrs("b", "span", Some("b"), None),
        ];
        updated.push(keyed_with_attrs("c", "span", Some("c"), None));
        renderer.render(wrap_parent(updated));
        assert_eq!(root.child_nodes().length(), 3);
        let ids: Vec<String> = child_ids(&root);
        assert_eq!(ids, vec!["a", "b", "c"]);
        cleanup(&root);
    }
    #[wasm_bindgen_test]
    fn keyed_remove_in_middle_drops_dom_node() {
        let root: Element = make_root("test-remove-mid");
        let mut renderer: Renderer = Renderer::new(root.clone());
        let initial: Vec<VirtualNode> = vec![
            keyed_with_attrs("a", "span", Some("a"), None),
            keyed_with_attrs("b", "span", Some("b"), None),
            keyed_with_attrs("c", "span", Some("c"), None),
        ];
        renderer.render(wrap_parent(initial));
        assert_eq!(root.child_nodes().length(), 3);
        let updated: Vec<VirtualNode> = vec![
            keyed_with_attrs("a", "span", Some("a"), None),
            keyed_with_attrs("c", "span", Some("c"), None),
        ];
        renderer.render(wrap_parent(updated));
        assert_eq!(root.child_nodes().length(), 2);
        let ids: Vec<String> = child_ids(&root);
        assert_eq!(ids, vec!["a", "c"]);
        let document: Document = cached_document().unwrap();
        let b_node: Option<Element> = document.get_element_by_id("b");
        assert!(
            b_node.is_none(),
            "removed keyed child must not remain anywhere in the DOM"
        );
        cleanup(&root);
    }
    #[wasm_bindgen_test]
    fn attribute_alignment_updates_dom_setattribute() {
        let root: Element = make_root("test-attr-set");
        let mut renderer: Renderer = Renderer::new(root.clone());
        let initial: VirtualNode = VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("span")),
            attributes: vec![AttributeEntry::new(
                Cow::Borrowed("class"),
                AttributeValue::Text(String::from("initial")),
            )],
            children: Vec::new(),
            key: None,
            props: None,
        };
        renderer.render(initial);
        let list: web_sys::NodeList = root.child_nodes();
        let span: Element = list
            .get(0)
            .and_then(|n| n.dyn_ref::<Element>().cloned())
            .expect("first child must be the span");
        assert_eq!(
            span.get_attribute("class").as_deref(),
            Some("initial"),
            "initial render must call setAttribute"
        );
        let updated: VirtualNode = VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("span")),
            attributes: vec![AttributeEntry::new(
                Cow::Borrowed("class"),
                AttributeValue::Text(String::from("updated")),
            )],
            children: Vec::new(),
            key: None,
            props: None,
        };
        renderer.render(updated);
        assert_eq!(
            span.get_attribute("class").as_deref(),
            Some("updated"),
            "attribute value change must propagate to the DOM"
        );
        cleanup(&root);
    }
    #[wasm_bindgen_test]
    fn attribute_alignment_removes_dom_attribute() {
        let root: Element = make_root("test-attr-remove");
        let mut renderer: Renderer = Renderer::new(root.clone());
        let initial: VirtualNode = VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("span")),
            attributes: vec![
                AttributeEntry::new(
                    Cow::Borrowed("class"),
                    AttributeValue::Text(String::from("keep-me")),
                ),
                AttributeEntry::new(
                    Cow::Borrowed("title"),
                    AttributeValue::Text(String::from("drop-me")),
                ),
            ],
            children: Vec::new(),
            key: None,
            props: None,
        };
        renderer.render(initial);
        let list: web_sys::NodeList = root.child_nodes();
        let span: Element = list
            .get(0)
            .and_then(|n| n.dyn_ref::<Element>().cloned())
            .expect("first child must be the span");
        assert_eq!(span.get_attribute("class").as_deref(), Some("keep-me"));
        assert_eq!(span.get_attribute("title").as_deref(), Some("drop-me"));
        let updated: VirtualNode = VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("span")),
            attributes: vec![AttributeEntry::new(
                Cow::Borrowed("class"),
                AttributeValue::Text(String::from("keep-me")),
            )],
            children: Vec::new(),
            key: None,
            props: None,
        };
        renderer.render(updated);
        assert_eq!(span.get_attribute("class").as_deref(), Some("keep-me"));
        assert_eq!(
            span.get_attribute("title"),
            None,
            "removed attribute must call removeAttribute"
        );
        cleanup(&root);
    }
    #[wasm_bindgen_test]
    fn attribute_alignment_stable_value_is_noop() {
        let root: Element = make_root("test-attr-stable");
        let mut renderer: Renderer = Renderer::new(root.clone());
        let make_node = || VirtualNode::Element {
            tag: Tag::Element(Cow::Borrowed("span")),
            attributes: vec![AttributeEntry::new(
                Cow::Borrowed("class"),
                AttributeValue::Text(String::from("same")),
            )],
            children: Vec::new(),
            key: None,
            props: None,
        };
        renderer.render(make_node());
        let list: web_sys::NodeList = root.child_nodes();
        let span: Element = list
            .get(0)
            .and_then(|n| n.dyn_ref::<Element>().cloned())
            .expect("first child must be the span");
        let _: Result<(), JsValue> = span.set_attribute("data-sentinel", "alive");
        renderer.render(make_node());
        assert_eq!(
            span.get_attribute("data-sentinel").as_deref(),
            Some("alive"),
            "stable attribute value must not trigger a re-write"
        );
        assert_eq!(span.get_attribute("class").as_deref(), Some("same"));
        cleanup(&root);
    }
}
