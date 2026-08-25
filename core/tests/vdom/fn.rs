use super::*;
#[test]
fn css_debug_format_works() {
    let css: Css = Css::default();
    let formatted: String = format!("{:?}", css);
    assert!(formatted.contains("Css"));
}

#[test]
fn pseudo_rule_equality_same_values() {
    let a: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let b: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    assert_eq!(a, b);
}

#[test]
fn pseudo_rule_equality_different_selectors() {
    let a: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let b: PseudoRule = PseudoRule::new(String::from(":focus"), String::from("background: blue;"));
    assert_ne!(a, b);
}

#[test]
fn pseudo_rule_equality_different_styles() {
    let a: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let b: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: red;"));
    assert_ne!(a, b);
}

#[test]
fn pseudo_rule_hash_same_for_equal_values() {
    let a: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let b: PseudoRule = PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let mut h1: DefaultHasher = DefaultHasher::new();
    let mut h2: DefaultHasher = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn pseudo_rule_debug_format_works() {
    let rule: PseudoRule = PseudoRule::default();
    let formatted: String = format!("{:?}", rule);
    assert!(formatted.contains("PseudoRule"));
}

#[test]
fn media_rule_equality_same_values() {
    let a: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    let b: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    assert_eq!(a, b);
}

#[test]
fn media_rule_equality_different_queries() {
    let a: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    let b: MediaRule = MediaRule::new(
        String::from("(min-width: 768px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    assert_ne!(a, b);
}

#[test]
fn media_rule_hash_same_for_equal_values() {
    let a: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    let b: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    let mut h1: DefaultHasher = DefaultHasher::new();
    let mut h2: DefaultHasher = DefaultHasher::new();
    a.hash(&mut h1);
    b.hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn media_rule_debug_format_works() {
    let rule: MediaRule = MediaRule::default();
    let formatted: String = format!("{:?}", rule);
    assert!(formatted.contains("MediaRule"));
}

#[test]
fn attribute_entry_debug_format_works() {
    let entry: AttributeEntry = AttributeEntry::new(
        Cow::Borrowed("class"),
        AttributeValue::Text(String::from("btn")),
    );
    let formatted: String = format!("{:?}", entry);
    assert!(formatted.contains("AttributeEntry"));
}

#[test]
fn native_css_construct_does_not_panic() {
    let result: Result<(), String> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: Css = Css::default();
        let _: PseudoRule = PseudoRule::default();
        let _: MediaRule = MediaRule::default();
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn native_pseudo_rule_clone_does_not_panic() {
    let result: Result<(), String> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let rule: PseudoRule = PseudoRule::default();
        let cloned: PseudoRule = rule.clone();
        assert_eq!(rule, cloned);
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}

#[test]
fn native_media_rule_clone_does_not_panic() {
    let result: Result<(), String> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let rule: MediaRule = MediaRule::default();
        let cloned: MediaRule = rule.clone();
        assert_eq!(rule, cloned);
    }))
    .map_err(|_| "panic".to_string());
    assert!(result.is_ok());
}
