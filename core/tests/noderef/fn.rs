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
