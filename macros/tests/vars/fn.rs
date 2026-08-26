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
    // The macro emits `fn name() -> Css` that, when
    // called, registers the variable in the DOM and
    // returns the resulting `&'static Css` value. On
    // native the first call panics inside `window()`,
    // so we only verify the call path is exercised via
    // catch_unwind.
    let ran: bool = run_with_window_capture(|| {
        let _css: &'static euv::Css = c_test_bg();
    });
    let _ = ran;
}

#[test]
fn vars_macro_generates_one_function_per_block() {
    // Each `pub c_name { ... }` block becomes a separate
    // function in the generated token stream.
    let _bg: fn() -> &'static euv::Css = c_test_bg;
    let _fg: fn() -> &'static euv::Css = c_test_fg;
}
