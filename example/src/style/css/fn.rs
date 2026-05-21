use crate::*;

/// Injects application-level global CSS into the DOM.
///
/// Registers the global reset styles, built-in animation keyframes,
/// and responsive media queries for mobile viewports (max-width: 767px).
/// Must be called once during application initialisation before any
/// rendering occurs.
///
/// # Panics
///
/// Panics if `window()` or `document()` is unavailable on the current platform.
pub(crate) fn inject_app_global_css() {
    let global: &str = "html, body, #app { height: 100%; margin: 0; padding: 0; overflow: hidden; } * { -webkit-tap-highlight-color: transparent; }";
    let keyframes: &str = "@keyframes euv-spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } } @keyframes euv-fade-in { from { opacity: 0; } to { opacity: 1; } } @keyframes euv-scale-in { from { transform: scale(0.9); opacity: 0; } to { transform: scale(1); opacity: 1; } } @keyframes euv-pulse { 0%, 100% { transform: scale(1); } 50% { transform: scale(1.2); } } @keyframes euv-slide-up { from { transform: translateY(100%); } to { transform: translateY(0); } } @keyframes euv-slide-left { from { transform: translateX(-100%); } to { transform: translateX(0); } } @keyframes euv-fade-in-up { from { opacity: 0; transform: translateY(8px); } to { opacity: 1; transform: translateY(0); } }";
    let media_queries: &str = "@media (max-width: 767px) { .c_app_nav { display: none; } .c_app_main { padding: 20px 16px; max-width: 100%; } .c_page_title { font-size: 22px; } .c_page_subtitle { font-size: 14px; } .c_card { padding: 16px; margin: 12px 0; border-radius: 10px; } .c_card_title { font-size: 16px; } .c_form_grid { grid-template-columns: 1fr; } .c_browser_api_row { grid-template-columns: 1fr; } .c_modal_content { max-width: 100%; width: calc(100% - 32px); border-radius: 16px; max-height: 85vh; overflow-y: auto; } .c_modal_overlay { align-items: center; justify-content: center; } .c_event_stats { gap: 12px; flex-wrap: wrap; } .c_event_section_row { gap: 12px; flex-wrap: wrap; } .c_event_section_col { min-width: 100%; } .c_counter_value { font-size: 20px; } .c_timer_value { font-size: 36px; } .c_not_found_code { font-size: 56px; } .c_not_found_container { padding: 40px 20px; } .c_list_input_row { flex-direction: column; } .c_vconsole_button { bottom: 16px; right: 16px; width: 44px; height: 44px; border-radius: 12px; } .c_tab_bar { flex-wrap: wrap; } .c_primary_button { padding: 10px 18px; font-size: 14px; } .c_badge { padding: 4px 10px; font-size: 11px; } .c_badge_outline { padding: 4px 10px; font-size: 11px; } .c_browser_info_grid { grid-template-columns: 1fr; } .c_anim_spin { font-size: 36px; } .c_anim_spin_stopped { font-size: 36px; } .c_anim_pulse { font-size: 36px; } .c_anim_pulse_stopped { font-size: 36px; } }";
    CssClass::inject_css(global);
    CssClass::inject_css(keyframes);
    CssClass::inject_css(media_queries);
}
