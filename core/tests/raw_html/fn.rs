use super::*;
#[test]
fn new_stores_content() {
    let raw: RawHtml = RawHtml::new("<svg></svg>".to_string());
    assert_eq!(raw.get_content(), "<svg></svg>");
}

#[test]
fn new_accepts_empty_string() {
    let raw: RawHtml = RawHtml::new(String::new());
    assert_eq!(raw.get_content(), "");
}

#[test]
fn new_accepts_plain_text() {
    let raw: RawHtml = RawHtml::new("hello".to_string());
    assert_eq!(raw.get_content(), "hello");
}

#[test]
fn get_content_returns_string_ref() {
    let raw: RawHtml = RawHtml::new("test".to_string());
    let s: &String = raw.get_content();
    assert_eq!(s, "test");
}

#[test]
fn default_is_empty_string() {
    let raw: RawHtml = RawHtml::default();
    assert_eq!(raw.get_content(), "");
}

#[test]
fn display_writes_content() {
    let raw: RawHtml = RawHtml::new("<b>bold</b>".to_string());
    let s: String = format!("{}", raw);
    assert_eq!(s, "<b>bold</b>");
}

#[test]
fn display_empty() {
    let raw: RawHtml = RawHtml::new(String::new());
    assert_eq!(format!("{}", raw), "");
}

#[test]
fn clone_copies_content() {
    let raw: RawHtml = RawHtml::new("<svg/>".to_string());
    let cloned: RawHtml = raw.clone();
    assert_eq!(cloned.get_content(), "<svg/>");
}

#[test]
fn eq_same_content() {
    let a: RawHtml = RawHtml::new("<p>".to_string());
    let b: RawHtml = RawHtml::new("<p>".to_string());
    assert_eq!(a, b);
}

#[test]
fn eq_different_content() {
    let a: RawHtml = RawHtml::new("<p>".to_string());
    let b: RawHtml = RawHtml::new("<span>".to_string());
    assert_ne!(a, b);
}

#[test]
fn hash_consistent_with_eq() {
    let mut h1: DefaultHasher = DefaultHasher::new();
    let mut h2: DefaultHasher = DefaultHasher::new();
    RawHtml::new("<p>".to_string()).hash(&mut h1);
    RawHtml::new("<p>".to_string()).hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn debug_format() {
    let raw: RawHtml = RawHtml::new("<svg>".to_string());
    let s: String = format!("{:?}", raw);
    assert!(s.contains("RawHtml"));
    assert!(s.contains("<svg>"));
}
