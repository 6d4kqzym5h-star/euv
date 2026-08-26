use super::*;

#[test]
fn default_ref_is_empty() {
    let node_ref: NodeRef<JsValue> = NodeRef::default();
    assert!(!node_ref.is_set(), "default ref must not be set");
    assert!(node_ref.get().is_none(), "default ref must return None");
}

#[test]
fn debug_reports_is_set_without_leaking_payload() {
    let node_ref: NodeRef<JsValue> = NodeRef::new();
    let formatted: String = format!("{node_ref:?}");
    assert!(
        formatted.contains("is_set: false"),
        "empty ref debug must mention is_set: false, got: {formatted}",
    );
    assert!(
        formatted.contains("NodeRef"),
        "debug output must name the type, got: {formatted}",
    );
}

#[test]
fn use_node_ref_without_hook_context_returns_empty_ref() {
    let node_ref: NodeRef<JsValue> = App::use_node_ref();
    assert!(!node_ref.is_set());
    assert!(node_ref.get().is_none());
}
#[cfg(all(target_arch = "wasm32", test))]
mod wasm_only {
    use super::*;
    #[wasm_bindgen_test]
    fn set_then_get_round_trip() {
        let node_ref: NodeRef<JsValue> = NodeRef::new();
        let value: JsValue = JsValue::from_f64(3.14);
        node_ref.set(value.clone());
        assert!(node_ref.is_set());
        let back: Option<JsValue> = node_ref.get();
        assert_eq!(back.and_then(|v| v.as_f64()), Some(3.14));
    }
    #[wasm_bindgen_test]
    fn set_overwrites_previous_value() {
        let node_ref: NodeRef<JsValue> = NodeRef::new();
        node_ref.set(JsValue::from_f64(1.0));
        let first: Option<JsValue> = node_ref.get();
        node_ref.set(JsValue::from_f64(2.0));
        let second: Option<JsValue> = node_ref.get();
        assert_eq!(first.and_then(|v| v.as_f64()), Some(1.0));
        assert_eq!(second.and_then(|v| v.as_f64()), Some(2.0));
    }
    #[wasm_bindgen_test]
    fn clear_resets_to_none() {
        let node_ref: NodeRef<JsValue> = NodeRef::new();
        node_ref.set(JsValue::from_f64(7.0));
        node_ref.clear();
        assert!(!node_ref.is_set());
        assert!(node_ref.get().is_none());
    }
    #[wasm_bindgen_test]
    fn clones_observe_each_others_writes() {
        let original: NodeRef<JsValue> = NodeRef::new();
        let clone: NodeRef<JsValue> = original.clone();
        clone.set(JsValue::from_f64(11.0));
        assert!(original.is_set());
        assert_eq!(
            original.get().and_then(|v| v.as_f64()),
            Some(11.0),
            "write through clone must be visible through original",
        );
    }
    #[wasm_bindgen_test]
    fn hook_ordering_preserved_across_rerender() {
        let ref_a: NodeRef<JsValue> = App::use_node_ref();
        let ref_b: NodeRef<JsValue> = App::use_node_ref();
        assert!(
            !Rc::ptr_eq(&ref_a.inner, &ref_b.inner),
            "distinct hook indices must produce distinct refs",
        );
        ref_a.set(JsValue::from_f64(1.0));
        ref_b.set(JsValue::from_f64(2.0));
        assert_eq!(ref_a.get().and_then(|v| v.as_f64()), Some(1.0));
        assert_eq!(ref_b.get().and_then(|v| v.as_f64()), Some(2.0));
    }
}
