use crate::*;

#[test]
fn test_simple_if() {
    let input = "if{a}{b}";
    let expected = "if { a } {\n    b\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_if_with_else() {
    let input = "if{a}{b}else{c}";
    let expected = "if { a } {\n    b\n} else {\n    c\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_if_else_if_else() {
    let input = "if{a}{b}else if{c}{d}else{e}";
    let expected = "if { a } {\n    b\n} else if { c } {\n    d\n} else {\n    e\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_nested_if() {
    let input = "if{a}{if{b}{c}}";
    let expected = "if { a } {\n    if { b } {\n        c\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_match_single_arm() {
    let input = "match{a}{\"b\"=>{c}}";
    let expected = "match { a } {\n    \"b\" => {\n        c\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_match_multiple_arms() {
    let input = "match{a}{\"b\"=>{c}\"d\"=>{e}}";
    let expected =
        "match { a } {\n    \"b\" => {\n        c\n    }\n    \"d\" => {\n        e\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_for_loop_bare_expr() {
    let input = "for i in a.iter(){b}";
    let expected = "for i in a.iter() {\n    b\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_for_loop_bare_expr_chain() {
    let input = "for (i, v) in a.iter().enumerate(){b}";
    let expected = "for (i, v) in a.iter().enumerate() {\n    b\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_for_loop_braced_expr_unchanged() {
    let input = "for i in {a}{b}";
    let expected = "for i in { a } {\n    b\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_simple_div() {
    let input = "div{a}";
    let expected = "div {\n    a\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_nested_div() {
    let input = "div{a{b}c}";
    let expected = "div {\n    a {\n        b\n    }\n    c\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_empty_block() {
    let input = "div{}";
    let expected = "div {}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_bare_empty_block() {
    let input = "{}";
    let expected = "{}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_string_in_body() {
    let input = "div{\"hello\"}";
    let expected = "div {\n    \"hello\"\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_braces_in_string() {
    let input = "div{\"hello { world }\"}";
    let expected = "div {\n    \"hello { world }\"\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_comment_in_body() {
    let input = "div{// comment\n    a\n}";
    let expected = "div {\n    // comment\n    a\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_class_macro() {
    let input = "class!{c_app_root{display:\"flex\"}}";
    let expected = "class!{\n    c_app_root {\n        display: \"flex\"\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_vars_macro() {
    let input = "vars!{a:\"1\";b:\"2\"}";
    let expected = "vars!{\n    a: \"1\";b: \"2\"\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_deeply_nested() {
    let input = "div{a{div{b{div{c}}}}}";
    let expected = concat!(
        "div {\n",
        "    a {\n",
        "        div {\n",
        "            b {\n",
        "                div {\n",
        "                    c\n",
        "                }\n",
        "            }\n",
        "        }\n",
        "    }\n",
        "}"
    );
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_multiple_siblings() {
    let input = "div{a\n    b\n    c}";
    let expected = "div {\n    a\n    b\n    c\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_if_with_complex_condition() {
    let input = "if{a>0}{b}";
    let expected = "if { a>0 } {\n    b\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_watch_macro() {
    let input = "watch!{signal=>{callback}}";
    let expected = "watch!{\n    signal => {\n        callback\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_pseudo_selector() {
    let input = "div{:hover{color:\"red\"}}";
    let expected = "div {\n    :hover {\n        color: \"red\"\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_media_query() {
    let input = "div{@media{(max-width: 767px){display:\"none\"}}}";
    let expected = "div {\n    @media {\n        (max-width: 767px){\n            display: \"none\"\n        }\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_block_comment() {
    let input = "div{/* comment */a}";
    let expected = "div {\n    /* comment */a\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_escape_in_string() {
    let input = "div{\"hello \\\"world\\\"\"}";
    let expected = "div {\n    \"hello \\\"world\\\"\"\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_attribute_with_if() {
    let input = "class:if{a}{b}else{c}";
    let expected = "class: if { a } {\n    b\n} else {\n    c\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_nested_match_in_if() {
    let input = "if{a}{match{b}{\"c\"=>{d}}}";
    let expected =
        "if { a } {\n    match { b } {\n        \"c\" => {\n            d\n        }\n    }\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_empty_line_preservation() {
    let input = "div{a\n\n    b}";
    let expected = "div {\n    a\n\n    b\n}";
    assert_eq!(format_macro_body(input), expected);
}

#[test]
fn test_format_euv_macros_html() {
    let input = "html! {if{a}{b}}";
    let expected = "html! {\n    if { a } {\n        b\n    }\n}";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_format_euv_macros_class() {
    let input = "class! {c{display:\"flex\"}}";
    let expected = "class! {\n    c {\n        display: \"flex\"\n    }\n}";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_format_euv_macros_vars() {
    let input = "vars! {a:\"1\"}";
    let expected = "vars! {\n    a: \"1\"\n}";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_format_euv_macros_watch() {
    let input = "watch! {s => {f}}";
    let expected = "watch! {\n    s => {\n        f\n    }\n}";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_format_euv_macros_no_leading_blank_lines() {
    let input = "    html! {\n            div {\n                class: c_euv_input_wrapper()\n                label {\n                    for: id\n                    class: c_form_label()\n                    label_string\n                }\n                input {\n                    id: id\n                    name: id\n                    type: \"text\"\n                    placeholder: placeholder\n                    value: value\n                    autocomplete: autocomplete\n                    class: c_euv_input()\n                    onfocus: on_focus_scroll_into_view()\n                }\n            }\n        }";
    let expected = "    html! {\n            div {\n                class: c_euv_input_wrapper()\n                label {\n                    for: id\n                    class: c_form_label()\n                    label_string\n                }\n                input {\n                    id: id\n                    name: id\n                    type: \"text\"\n                    placeholder: placeholder\n                    value: value\n                    autocomplete: autocomplete\n                    class: c_euv_input()\n                    onfocus: on_focus_scroll_into_view()\n                }\n            }\n    }";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_sibling_elements() {
    let input = "html! {\n    vconsole_fab {\n        panel_open: panel_open\n        console_signal: console_signal\n    }\n    vconsole_drawer {\n        console_signal: console_signal\n        panel_open: panel_open\n    }\n}";
    let expected = "html! {\n    vconsole_fab {\n        panel_open: panel_open\n        console_signal: console_signal\n    }\n    vconsole_drawer {\n        console_signal: console_signal\n        panel_open: panel_open\n    }\n}";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_sibling_elements_with_indent() {
    let input = "    html! {\n            vconsole_fab {\n                panel_open: panel_open\n                console_signal: console_signal\n            }\n            vconsole_drawer {\n                console_signal: console_signal\n                panel_open: panel_open\n            }\n        }";
    let expected = "    html! {\n            vconsole_fab {\n                panel_open: panel_open\n                console_signal: console_signal\n            }\n            vconsole_drawer {\n                console_signal: console_signal\n                panel_open: panel_open\n            }\n    }";
    assert_eq!(format_euv_macros(input), expected);
}

#[test]
fn test_sibling_elements_compressed() {
    let input = "html!{vconsole_fab{panel_open:panel_open console_signal:console_signal}vconsole_drawer{console_signal:console_signal panel_open:panel_open}}";
    let expected = "html! {\n    vconsole_fab {\n        panel_open: panel_open\n        console_signal: console_signal\n    }\n    vconsole_drawer {\n        console_signal: console_signal\n        panel_open: panel_open\n    }\n}";
    assert_eq!(format_euv_macros(input), expected);
}
