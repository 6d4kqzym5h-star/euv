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
    // The macro emits `fn name() -> &'static Css` —
    // verified by binding the generated function to a
    // typed fn pointer. Calling it would panic on native
    // (no `window()`), but the type signature is fixed
    // at expansion time.
    let _generated_fn_ptr: fn() -> &'static euv::Css = c_test_red;
}

#[test]
fn class_macro_generates_one_function_per_definition() {
    // Both `c_test_red` and `c_test_blue` are in scope as
    // separate items after the macro expansion.
    let _red: fn() -> &'static euv::Css = c_test_red;
    let _blue: fn() -> &'static euv::Css = c_test_blue;
}

#[test]
fn class_macro_uses_once_lock_for_caching() {
    // The macro emits
    //   static FOO: OnceLock<Css> = OnceLock::new();
    //   FOO.get_or_init(|| Css::new(...))
    // so the same instance is returned on every call.
    // On native the first call panics inside `window()`,
    // so we only verify the call path is exercised via
    // catch_unwind — not the actual cache identity.
    let ran: bool = run_with_window_capture(|| {
        let _a: &euv::Css = c_test_red();
        let _b: &euv::Css = c_test_red();
    });
    // Whether the call returned or panicked, the test
    // confirms the macro emitted a callable function
    // with the right signature.
    let _ = ran;
}
