//! Tests for pre-existing pure-Rust VDOM types.
//!
//! These tests target the data structures used to
//! represent CSS classes, media queries, pseudo-class
//! rules, and attribute entries. They run natively under
//! `cargo test -p euv-core --lib` and provide coverage
//! for `euv-core`'s VDOM data layer.
//!
//! # What's covered here
//!
//! - `vdom::attribute::Css` (CSS class wrapper)
//! - `vdom::attribute::PseudoRule` (CSS pseudo rule)
//! - `vdom::attribute::MediaRule` (CSS @media rule)
//! - `vdom::attribute::AttributeEntry` (name + value pair)
//! - `vdom::attribute::EventAdapter` (closure/event adapter)
//! - `vdom::attribute::EventNamedAdapter` (named event adapter)
//! - `vdom::attribute::AttrValueAdapter` (attribute value adapter)
//! - `vdom::attribute::CallbackNamedAdapter` (named callback adapter)
//!
//! # What is NOT covered here
//!
//! - `AttributeValue::Signal` / `AttributeValue::Event` /
//!   `AttributeValue::Dynamic` / `AttributeValue::Css`
//!   variants — those contain wasm-only types
//!   (`Signal<String>`, `NativeEventHandler`).
//! - The `cast` module (190 LOC of `From<X> for AttributeValue`
//!   impls that all touch `AttributeValue`'s wasm-only
//!   variants).

use super::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// =====================================================================
// Css
// =====================================================================

#[test]
fn css_default_is_empty() {
    let css: Css = Css::default();
    assert_eq!(css.get_name(), "");
    assert_eq!(css.get_style(), "");
    assert!(css.get_pseudo_rules().is_empty());
    assert!(css.get_media_rules().is_empty());
}

#[test]
fn css_new_with_name_only() {
    let css: Css = Css::new(
        String::from("btn-primary"),
        String::new(),
        Vec::new(),
        Vec::new(),
    );
    assert_eq!(css.get_name(), "btn-primary");
    assert_eq!(css.get_style(), "");
    assert!(css.get_pseudo_rules().is_empty());
    assert!(css.get_media_rules().is_empty());
}

#[test]
fn css_set_name_replaces() {
    let mut css: Css = Css::default();
    css.set_name(String::from("first"));
    assert_eq!(css.get_name(), "first");
    css.set_name(String::from("second"));
    assert_eq!(css.get_name(), "second");
}

#[test]
fn css_set_style_replaces() {
    let mut css: Css = Css::default();
    css.set_style(String::from("color: red;"));
    assert_eq!(css.get_style(), "color: red;");
    css.set_style(String::from("color: blue;"));
    assert_eq!(css.get_style(), "color: blue;");
}

#[test]
fn css_pseudo_rules_mutation() {
    let mut css: Css = Css::default();
    assert!(css.get_pseudo_rules().is_empty());
    css.get_mut_pseudo_rules().push(PseudoRule::new(
        String::from(":hover"),
        String::from("background: blue;"),
    ));
    assert_eq!(css.get_pseudo_rules().len(), 1);
    assert_eq!(css.get_pseudo_rules()[0].get_selector(), ":hover");
    assert_eq!(css.get_pseudo_rules()[0].get_style(), "background: blue;");
}

#[test]
fn css_media_rules_mutation() {
    let mut css: Css = Css::default();
    assert!(css.get_media_rules().is_empty());
    css.get_mut_media_rules().push(MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    ));
    assert_eq!(css.get_media_rules().len(), 1);
    assert_eq!(css.get_media_rules()[0].get_query(), "(max-width: 767px)");
    assert_eq!(css.get_media_rules()[0].get_style(), "font-size: 14px;");
}

#[test]
fn css_clone_shares_field_values() {
    let mut css: Css = Css::default();
    css.set_name(String::from("foo"));
    css.set_style(String::from("color: red;"));
    let cloned: Css = css.clone();
    assert_eq!(cloned.get_name(), "foo");
    assert_eq!(cloned.get_style(), "color: red;");
    assert!(cloned.get_pseudo_rules().is_empty());
    assert!(cloned.get_media_rules().is_empty());
}

#[test]
fn css_debug_format_works() {
    let css: Css = Css::default();
    let formatted: String = format!("{:?}", css);
    assert!(formatted.contains("Css"));
}

// =====================================================================
// PseudoRule
// =====================================================================

#[test]
fn pseudo_rule_default_is_empty() {
    let rule: PseudoRule = PseudoRule::default();
    assert_eq!(rule.get_selector(), "");
    assert_eq!(rule.get_style(), "");
}

#[test]
fn pseudo_rule_new_with_selector_and_style() {
    let rule: PseudoRule =
        PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    assert_eq!(rule.get_selector(), ":hover");
    assert_eq!(rule.get_style(), "background: blue;");
}

#[test]
fn pseudo_rule_set_selector_replaces() {
    let mut rule: PseudoRule = PseudoRule::default();
    rule.set_selector(String::from(":focus"));
    assert_eq!(rule.get_selector(), ":focus");
    rule.set_selector(String::from(":active"));
    assert_eq!(rule.get_selector(), ":active");
}

#[test]
fn pseudo_rule_set_style_replaces() {
    let mut rule: PseudoRule = PseudoRule::default();
    rule.set_style(String::from("color: red;"));
    assert_eq!(rule.get_style(), "color: red;");
    rule.set_style(String::from("color: green;"));
    assert_eq!(rule.get_style(), "color: green;");
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
fn pseudo_rule_clone_preserves_values() {
    let original: PseudoRule =
        PseudoRule::new(String::from(":hover"), String::from("background: blue;"));
    let cloned: PseudoRule = original.clone();
    assert_eq!(original, cloned);
    assert_eq!(cloned.get_selector(), ":hover");
    assert_eq!(cloned.get_style(), "background: blue;");
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

// =====================================================================
// MediaRule
// =====================================================================

#[test]
fn media_rule_default_is_empty() {
    let rule: MediaRule = MediaRule::default();
    assert_eq!(rule.get_query(), "");
    assert_eq!(rule.get_style(), "");
    assert!(rule.get_pseudo_rules().is_empty());
}

#[test]
fn media_rule_new_with_query_and_style() {
    let rule: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        Vec::new(),
    );
    assert_eq!(rule.get_query(), "(max-width: 767px)");
    assert_eq!(rule.get_style(), "font-size: 14px;");
    assert!(rule.get_pseudo_rules().is_empty());
}

#[test]
fn media_rule_new_with_pseudo_rules() {
    let pseudos: Vec<PseudoRule> = vec![PseudoRule::new(
        String::from("::-webkit-scrollbar"),
        String::from("width: 0px;"),
    )];
    let rule: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        pseudos,
    );
    assert_eq!(rule.get_pseudo_rules().len(), 1);
    assert_eq!(
        rule.get_pseudo_rules()[0].get_selector(),
        "::-webkit-scrollbar"
    );
}

#[test]
fn media_rule_set_query_replaces() {
    let mut rule: MediaRule = MediaRule::default();
    rule.set_query(String::from("(min-width: 768px)"));
    assert_eq!(rule.get_query(), "(min-width: 768px)");
}

#[test]
fn media_rule_set_style_replaces() {
    let mut rule: MediaRule = MediaRule::default();
    rule.set_style(String::from("font-size: 18px;"));
    assert_eq!(rule.get_style(), "font-size: 18px;");
}

#[test]
fn media_rule_pseudo_rules_mutation() {
    let mut rule: MediaRule = MediaRule::default();
    rule.get_mut_pseudo_rules().push(PseudoRule::new(
        String::from("::-webkit-scrollbar"),
        String::from("width: 0px;"),
    ));
    assert_eq!(rule.get_pseudo_rules().len(), 1);
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
fn media_rule_clone_preserves_values() {
    let original: MediaRule = MediaRule::new(
        String::from("(max-width: 767px)"),
        String::from("font-size: 14px;"),
        vec![PseudoRule::new(
            String::from("::-webkit-scrollbar"),
            String::from("width: 0px;"),
        )],
    );
    let cloned: MediaRule = original.clone();
    assert_eq!(original, cloned);
    assert_eq!(cloned.get_pseudo_rules().len(), 1);
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

// =====================================================================
// AttributeEntry (limited coverage; value type is wasm-only)
// =====================================================================

#[test]
fn attribute_entry_with_text_value() {
    // The Text variant of AttributeValue is the only
    // wasm-portable one; we use it to verify
    // AttributeEntry construction.
    let entry: AttributeEntry = AttributeEntry::new(
        String::from("class"),
        AttributeValue::Text(String::from("btn")),
    );
    assert_eq!(entry.get_name(), "class");
}

#[test]
fn attribute_entry_clone_preserves_name() {
    let entry: AttributeEntry = AttributeEntry::new(
        String::from("id"),
        AttributeValue::Text(String::from("main")),
    );
    let cloned: AttributeEntry = entry.clone();
    assert_eq!(cloned.get_name(), "id");
}

#[test]
fn attribute_entry_set_name_replaces() {
    let mut entry: AttributeEntry =
        AttributeEntry::new(String::from("first"), AttributeValue::Text(String::new()));
    entry.set_name(String::from("second"));
    assert_eq!(entry.get_name(), "second");
}

#[test]
fn attribute_entry_debug_format_works() {
    let entry: AttributeEntry = AttributeEntry::new(
        String::from("class"),
        AttributeValue::Text(String::from("btn")),
    );
    let formatted: String = format!("{:?}", entry);
    assert!(formatted.contains("AttributeEntry"));
}

// =====================================================================
// EventAdapter / EventNamedAdapter / AttrValueAdapter /
// CallbackNamedAdapter (pure-Rust newtype constructors).
// =====================================================================

#[test]
fn event_adapter_new_with_i32() {
    let adapter: EventAdapter<i32> = EventAdapter::new(42);
    assert_eq!(adapter.get_inner(), &42);
}

#[test]
fn event_adapter_new_with_string() {
    let adapter: EventAdapter<String> = EventAdapter::new(String::from("hello"));
    assert_eq!(adapter.get_inner(), "hello");
}

#[test]
fn event_adapter_set_inner_replaces() {
    let mut adapter: EventAdapter<i32> = EventAdapter::new(1);
    adapter.set_inner(2);
    assert_eq!(adapter.get_inner(), &2);
}

#[test]
fn event_named_adapter_new_constructor() {
    let adapter: EventNamedAdapter<i32> = EventNamedAdapter::new(1, "click");
    assert_eq!(adapter.get_inner(), &1);
    assert_eq!(adapter.get_event_name(), "click");
}

#[test]
fn event_named_adapter_set_event_name_replaces() {
    let mut adapter: EventNamedAdapter<i32> = EventNamedAdapter::new(0, "click");
    adapter.set_event_name("mouseover");
    assert_eq!(adapter.get_event_name(), "mouseover");
}

#[test]
fn event_named_adapter_set_inner_replaces() {
    let mut adapter: EventNamedAdapter<i32> = EventNamedAdapter::new(0, "click");
    adapter.set_inner(42);
    assert_eq!(adapter.get_inner(), &42);
}

#[test]
fn attr_value_adapter_new_with_string() {
    let adapter: AttrValueAdapter<String> = AttrValueAdapter::new(String::from("value"));
    assert_eq!(adapter.get_inner(), "value");
}

#[test]
fn attr_value_adapter_set_inner_replaces() {
    let mut adapter: AttrValueAdapter<String> = AttrValueAdapter::new(String::from("a"));
    adapter.set_inner(String::from("b"));
    assert_eq!(adapter.get_inner(), "b");
}

#[test]
fn callback_named_adapter_new_constructor() {
    let adapter: CallbackNamedAdapter<i32> = CallbackNamedAdapter::new(0, "on-increment");
    assert_eq!(adapter.get_inner(), &0);
    assert_eq!(adapter.get_name(), "on-increment");
}

#[test]
fn callback_named_adapter_set_name_replaces() {
    let mut adapter: CallbackNamedAdapter<i32> = CallbackNamedAdapter::new(0, "old-name");
    adapter.set_name("new-name");
    assert_eq!(adapter.get_name(), "new-name");
}

// =====================================================================
// Regression: Css + PseudoRule + MediaRule constructible on native
// =====================================================================

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
fn native_css_with_pseudos_does_not_panic() {
    let result: Result<(), String> = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut css: Css = Css::default();
        css.set_name(String::from("x"));
        css.set_style(String::from("y"));
        css.get_mut_pseudo_rules().push(PseudoRule::default());
        css.get_mut_media_rules().push(MediaRule::default());
        let _: Css = css.clone();
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
