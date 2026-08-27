use super::*;

#[test]
fn var_macro_emits_css_var_string_for_unquoted_identifier() {
    let css: &str = var!(bg_primary);
    assert_eq!(css, "var(--bg_primary)");
}

#[test]
fn var_macro_emits_css_var_string_for_kebab_identifier() {
    let css: &str = var!("bg-primary");
    assert_eq!(css, "var(--bg-primary)");
}

#[test]
fn var_macro_emits_css_var_string_for_quoted_literal() {
    let css: &str = var!("bg-primary");
    assert_eq!(css, "var(--bg-primary)");
}
