use super::*;
#[test]
fn new_stores_content() {
    let raw: RawHtml = RawHtml::new("<svg></svg>".to_string());
    assert_eq!(raw.content(), "<svg></svg>");
}

#[test]
fn new_accepts_empty_string() {
    let raw: RawHtml = RawHtml::new(String::new());
    assert!(raw.is_empty());
    assert_eq!(raw.len(), 0);
}

#[test]
fn new_accepts_plain_text() {
    let raw: RawHtml = RawHtml::new("hello".to_string());
    assert_eq!(raw.content(), "hello");
    assert!(!raw.contains_tag());
}

#[test]
fn content_returns_str_ref() {
    let raw: RawHtml = RawHtml::new("test".to_string());
    let s: &str = raw.content();
    assert_eq!(s, "test");
}

#[test]
fn into_content_consumes() {
    let raw: RawHtml = RawHtml::new("owned".to_string());
    let s: String = raw.into_content();
    assert_eq!(s, "owned");
}

#[test]
fn is_empty_true_for_empty() {
    let raw: RawHtml = RawHtml::new(String::new());
    assert!(raw.is_empty());
}

#[test]
fn is_empty_false_for_non_empty() {
    let raw: RawHtml = RawHtml::new("x".to_string());
    assert!(!raw.is_empty());
}

#[test]
fn len_returns_byte_length() {
    let raw: RawHtml = RawHtml::new("hello".to_string());
    assert_eq!(raw.len(), 5);
}

#[test]
fn len_returns_byte_length_unicode() {
    let raw: RawHtml = RawHtml::new("你好".to_string());
    assert_eq!(raw.len(), 6);
}

#[test]
fn contains_tag_true_for_html() {
    let raw: RawHtml = RawHtml::new("<svg></svg>".to_string());
    assert!(raw.contains_tag());
}

#[test]
fn contains_tag_true_for_self_closing() {
    let raw: RawHtml = RawHtml::new("text<br/>more".to_string());
    assert!(raw.contains_tag());
}

#[test]
fn contains_tag_false_for_plain_text() {
    let raw: RawHtml = RawHtml::new("hello world".to_string());
    assert!(!raw.contains_tag());
}

#[test]
fn contains_tag_false_for_lt_only() {
    let raw: RawHtml = RawHtml::new("a < b".to_string());
    assert!(!raw.contains_tag());
}

#[test]
fn contains_tag_false_for_unclosed_lt() {
    let raw: RawHtml = RawHtml::new("a < b".to_string());
    assert!(!raw.contains_tag());
}

#[test]
fn contains_tag_true_for_html_with_attrs() {
    let raw: RawHtml = RawHtml::new(r#"<a href="https://x.com">link</a>"#.to_string());
    assert!(raw.contains_tag());
}

#[test]
fn default_is_empty() {
    let raw: RawHtml = RawHtml::default();
    assert!(raw.is_empty());
    assert_eq!(raw.content(), "");
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
fn from_string() {
    let raw: RawHtml = RawHtml::from("<p>".to_string());
    assert_eq!(raw.content(), "<p>");
}

#[test]
fn from_str_slice() {
    let raw: RawHtml = RawHtml::from("<p>");
    assert_eq!(raw.content(), "<p>");
}

#[test]
fn as_ref_str() {
    let raw: RawHtml = RawHtml::new("<p>".to_string());
    let s: &str = raw.as_ref();
    assert_eq!(s, "<p>");
}

#[test]
fn clone_copies_content() {
    let raw: RawHtml = RawHtml::new("<svg/>".to_string());
    let cloned: RawHtml = raw.clone();
    assert_eq!(cloned.content(), "<svg/>");
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

#[test]
fn raw_html_can_be_constructed_inside_html_macro_pattern() {
    let raw: RawHtml = RawHtml::new("<svg viewBox=\"0 0 10 10\"/>".to_string());
    let node: VirtualNode = VirtualNode::Text(TextNode::new(raw.into_content(), None));
    let s: String = format!("{:?}", node);
    assert!(s.contains("svg"));
}
