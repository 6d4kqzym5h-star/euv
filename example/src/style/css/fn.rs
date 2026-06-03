use crate::*;

/// Injects application-level global CSS into the DOM.
///
/// Registers the global reset styles and built-in animation keyframes.
/// Responsive media queries are now handled directly within `class!` macro
/// definitions via the `media()` block syntax.
/// Must be called once during application initialisation before any
/// rendering occurs.
///
/// # Panics
///
/// Panics if `window()` or `document()` is unavailable on the current platform.
pub(crate) fn inject_app_global_css() {
    let global: &str = "html, body, #app { height: 100%; margin: 0; padding: 0; overflow: hidden; } * { -webkit-tap-highlight-color: transparent; box-sizing: border-box; }";
    let keyframes: &str = concat!(
        "@keyframes euv-spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } } ",
        "@keyframes euv-fade-in { from { opacity: 0; transform: translateY(4px); } to { opacity: 1; transform: translateY(0); } } ",
        "@keyframes euv-scale-in { from { transform: scale(0.95); opacity: 0; } to { transform: scale(1); opacity: 1; } } ",
        "@keyframes euv-pulse { 0%, 100% { transform: scale(1); } 50% { transform: scale(1.15); } } ",
        "@keyframes euv-slide-up { from { transform: translateY(100%); } to { transform: translateY(0); } } ",
        "@keyframes euv-slide-left { from { transform: translateX(-100%); } to { transform: translateX(0); } } ",
        "@keyframes euv-fade-in-up { from { opacity: 0; transform: translateY(8px); } to { opacity: 1; transform: translateY(0); } } ",
        "@keyframes euv-progress { from { width: 0%; } to { width: 100%; } } ",
        "@keyframes euv-shimmer { from { background-position: -200% 0; } to { background-position: 200% 0; } }",
    );
    Css::inject_css(global);
    Css::inject_css(keyframes);
}
