use super::*;

// --- HmrState basics ---

#[test]
fn new_is_empty() {
    let state: HmrState = HmrState::new();
    assert!(state.is_empty());
    assert_eq!(state.len(), 0);
}

#[test]
fn default_is_empty() {
    let state: HmrState = HmrState::default();
    assert!(state.is_empty());
}

#[test]
fn set_and_get() {
    let mut state: HmrState = HmrState::new();
    state.set("key1", "value1");
    assert_eq!(state.get("key1"), Some("value1"));
    assert_eq!(state.len(), 1);
}

#[test]
fn set_overwrites_existing_key() {
    let mut state: HmrState = HmrState::new();
    state.set("k", "first");
    state.set("k", "second");
    assert_eq!(state.get("k"), Some("second"));
    assert_eq!(state.len(), 1);
}

#[test]
fn get_missing_key_returns_none() {
    let state: HmrState = HmrState::new();
    assert_eq!(state.get("missing"), None);
}

#[test]
fn contains_key() {
    let mut state: HmrState = HmrState::new();
    state.set("k", "v");
    assert!(state.contains("k"));
    assert!(!state.contains("missing"));
}

#[test]
fn remove_returns_previous_value() {
    let mut state: HmrState = HmrState::new();
    state.set("k", "v");
    assert_eq!(state.remove("k"), Some("v".to_string()));
    assert_eq!(state.get("k"), None);
}

#[test]
fn remove_missing_key_returns_none() {
    let mut state: HmrState = HmrState::new();
    assert_eq!(state.remove("missing"), None);
}

#[test]
fn clear_drops_everything() {
    let mut state: HmrState = HmrState::new();
    state.set("k1", "v1");
    state.set("k2", "v2");
    state.clear();
    assert!(state.is_empty());
}

#[test]
fn iter_yields_all_entries() {
    let mut state: HmrState = HmrState::new();
    state.set("a", "1");
    state.set("b", "2");
    let entries: Vec<(&str, &str)> = state.iter().collect();
    assert_eq!(entries.len(), 2);
    assert!(entries.contains(&("a", "1")));
    assert!(entries.contains(&("b", "2")));
}

#[test]
fn iter_empty_state() {
    let state: HmrState = HmrState::new();
    let entries: Vec<(&str, &str)> = state.iter().collect();
    assert!(entries.is_empty());
}

#[test]
fn from_entries_constructor() {
    let state: HmrState = HmrState::from_entries(vec![
        ("a".to_string(), "1".to_string()),
        ("b".to_string(), "2".to_string()),
    ]);
    assert_eq!(state.len(), 2);
    assert_eq!(state.get("a"), Some("1"));
}

#[test]
fn from_entries_overwrites_duplicates() {
    let state: HmrState = HmrState::from_entries(vec![
        ("k".to_string(), "first".to_string()),
        ("k".to_string(), "second".to_string()),
    ]);
    assert_eq!(state.get("k"), Some("second"));
}

#[test]
fn from_entries_empty() {
    let state: HmrState = HmrState::from_entries(Vec::<(String, String)>::new());
    assert!(state.is_empty());
}

// --- JSON serialization ---

// --- JSON deserialization ---

// --- Round-trip ---

// --- Clone + Debug ---

#[test]
fn clone_preserves_entries() {
    let mut state: HmrState = HmrState::new();
    state.set("k", "v");
    let cloned: HmrState = state.clone();
    assert_eq!(cloned.get("k"), Some("v"));
}

#[test]
fn debug_format_works() {
    let state: HmrState = HmrState::new();
    let s: String = format!("{:?}", state);
    assert!(s.contains("HmrState"));
}
