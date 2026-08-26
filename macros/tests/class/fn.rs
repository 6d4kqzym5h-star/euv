use super::*;

class! {
    c_test_red {
        color: "red";
    }
    c_test_blue {
        color: "blue";
        background: "white";
    }
}

#[test]
fn class_macro_emits_function_with_static_css_return_type() {
    let _emit: fn() -> &'static Css = c_test_red;
}

#[test]
fn class_macro_generates_one_function_per_definition() {
    let _red: fn() -> &'static Css = c_test_red;
    let _blue: fn() -> &'static Css = c_test_blue;
}

#[test]
fn class_macro_uses_once_lock_for_caching() {
    let panicked: bool = catch_unwind(AssertUnwindSafe(|| {
        let _first: &Css = c_test_red();
        let _second: &Css = c_test_red();
    }))
    .is_err();
    assert!(panicked, "c_test_red() must panic without a window()");
}
