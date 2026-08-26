use super::*;

vars! {
    pub c_test_bg {
        background: "white";
        foreground: "black";
    }
    pub c_test_fg {
        color: "blue";
    }
}

#[test]
fn vars_macro_emits_function_returning_static_css_ref() {
    let _emit: fn() -> &'static Css = c_test_bg;
}

#[test]
fn vars_macro_generates_one_function_per_block() {
    let _bg: fn() -> &'static Css = c_test_bg;
    let _fg: fn() -> &'static Css = c_test_fg;
}

#[test]
fn vars_macro_uses_once_lock_for_caching() {
    let panicked: bool = catch_unwind(AssertUnwindSafe(|| {
        let _first: &Css = c_test_bg();
        let _second: &Css = c_test_bg();
    }))
    .is_err();
    assert!(panicked, "c_test_bg() must panic without a window()");
}
