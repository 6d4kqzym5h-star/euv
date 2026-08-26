use super::*;

#[test]
fn unsafe_no_inline_macro_emits_raw_html_from_string_literal() {
    let raw: euv::RawHtml = unsafe_no_inline!("<b>bold</b>");
    assert_eq!(raw.get_content(), "<b>bold</b>");
}

#[test]
fn unsafe_no_inline_macro_preserves_empty_string() {
    let raw: euv::RawHtml = unsafe_no_inline!("");
    assert_eq!(raw.get_content(), "");
}

#[test]
fn unsafe_no_inline_macro_preserves_html_with_attributes() {
    let raw: euv::RawHtml = unsafe_no_inline!(r#"<a href="https://example.com">link</a>"#);
    assert!(raw.get_content().contains("href"));
    assert!(raw.get_content().contains("example.com"));
}

#[test]
fn unsafe_no_inline_macro_default_creates_empty_raw_html() {
    let empty: euv::RawHtml = euv::RawHtml::default();
    assert_eq!(empty.get_content(), "");
}
