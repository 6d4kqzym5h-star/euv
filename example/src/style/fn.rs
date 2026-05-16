use crate::*;

class! {
    pub c_app_root {
        display: "flex";
        height: "100vh";
        overflow: "hidden";
        font-family: "system-ui, -apple-system, sans-serif";
        background: var!(bg-primary);
        color: var!(text-primary);
        line-height: "1.6";
    }

    pub c_app_nav {
        width: "240px";
        background: var!(bg-nav);
        border-right: format!("1px solid {}", var!(border-nav));
        display: "flex";
        flex-direction: "column";
        height: "100vh";
        overflow-y: "auto";
        flex-shrink: "0";
    }

    pub c_nav_header {
        padding: "24px 20px 20px 20px";
        margin: "0";
        border-bottom: format!("1px solid {}", var!(border-subtle));
        font-size: "18px";
        font-weight: "700";
        color: var!(text-primary);
        letter-spacing: "-0.02em";
        display: "flex";
        align-items: "center";
        gap: "10px";
    }

    pub c_nav_logo {
        display: "flex";
        width: "32px";
        height: "32px";
        background: "linear-gradient(135deg, #4f46e5, #7c3aed)";
        border-radius: "8px";
        align-items: "center";
        justify-content: "center";
        color: "white";
        font-weight: "700";
        font-size: "16px";
        flex-shrink: "0";
    }

    pub c_nav_section_label {
        padding: "10px 20px 4px 20px";
        margin: "0";
        font-size: "11px";
        font-weight: "600";
        color: var!(text-muted);
        text-transform: "uppercase";
        letter-spacing: "0.08em";
    }

    pub c_nav_theme_toggle {
        padding: "12px 20px";
        margin-top: "auto";
    }

    pub c_nav_theme_button {
        width: "100%";
        padding: "8px 16px";
        border-radius: "8px";
        cursor: "pointer";
        font-size: "16px";
        text-align: "center";
        outline: "none";
        background: var!(bg-theme-button);
        color: var!(text-theme-button);
        border: format!("1px solid {}", var!(border-theme-button));
        transition: "all 0.3s ease";
    }

    pub c_nav_footer {
        padding: "16px 20px";
        border-top: format!("1px solid {}", var!(border-subtle));
        font-size: "12px";
        color: var!(text-muted);
    }

    pub c_app_main {
        flex: "1";
        padding: "32px 40px";
        background: var!(bg-primary);
        overflow-y: "auto";
        max-width: "960px";
    }

    pub c_page_container {
        max-width: "800px";
        margin: "0 auto";
    }

    pub c_page_header {
        margin-bottom: "32px";
    }

    pub c_page_title {
        font-size: "28px";
        font-weight: "700";
        color: "inherit";
        margin-bottom: "8px";
        letter-spacing: "-0.02em";
    }

    pub c_page_subtitle {
        font-size: "16px";
        color: "inherit";
        opacity: "0.6";
        line-height: "1.6";
    }

    pub c_card {
        background: var!(bg-card);
        border-radius: "12px";
        padding: "24px";
        margin: "16px 0";
        box-shadow: var!(shadow-card);
        border: format!("1px solid {}", var!(border-card));
        color: var!(text-card);
        transition: "box-shadow 0.2s ease, background 0.4s ease, border-color 0.4s ease, color 0.4s ease";
    }

    pub c_card_title {
        margin-top: "0";
        margin-bottom: "16px";
        color: "inherit";
        font-size: "18px";
        font-weight: "600";
        padding-bottom: "12px";
        border-bottom: format!("2px solid {}", var!(border-card-title));
        letter-spacing: "0.01em";
        transition: "border-color 0.4s ease";
    }

    pub c_primary_button {
        background: "#4f46e5";
        color: "white";
        border: "none";
        padding: "10px 22px";
        border-radius: "8px";
        cursor: "pointer";
        font-size: "14px";
        font-weight: "500";
        letter-spacing: "0.01em";
        transition: "all 0.15s ease";
        outline: "none";
    }

    pub c_badge {
        display: "inline-block";
        color: "white";
        padding: "5px 14px";
        border-radius: "20px";
        font-size: "12px";
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.03em";
        text-transform: "uppercase";
    }

    pub c_badge_outline {
        display: "inline-block";
        padding: "5px 14px";
        border-radius: "20px";
        font-size: "12px";
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.03em";
        text-transform: "uppercase";
        background: "transparent";
        border: "1.5px solid";
    }

    pub c_form_input_wrapper {
        margin: "12px 0";
    }

    pub c_form_label {
        display: "block";
        margin-bottom: "6px";
        color: "inherit";
        font-weight: "500";
        font-size: "14px";
    }

    pub c_form_input {
        width: "100%";
        padding: "10px 14px";
        border: format!("1px solid {}", var!(border-input));
        border-radius: "8px";
        font-size: "14px";
        box-sizing: "border-box";
        transition: "border-color 0.15s ease, box-shadow 0.15s ease";
        outline: "none";
        background: var!(bg-input);
        color: var!(text-primary);
    }

    pub c_form_input_no_transition {
        width: "100%";
        padding: "10px 14px";
        border: format!("1px solid {}", var!(border-input));
        border-radius: "8px";
        font-size: "14px";
        box-sizing: "border-box";
        background: var!(bg-input);
        outline: "none";
        color: var!(text-primary);
    }

    pub c_form_checkbox {
        cursor: "pointer";
        width: "16px";
        height: "16px";
    }

    pub c_form_checkbox_label {
        font-size: "14px";
        color: "inherit";
        cursor: "pointer";
    }

    pub c_form_checkbox_row {
        margin: "16px 0";
        display: "flex";
        align-items: "center";
        gap: "10px";
    }

    pub c_error_box {
        margin: "16px 0";
        padding: "14px 16px";
        background: var!(bg-error);
        color: var!(text-error);
        border-radius: "8px";
        font-size: "14px";
        border: format!("1px solid {}", var!(border-error));
    }

    pub c_success_box {
        margin-top: "16px";
        padding: "14px 16px";
        background: var!(bg-success);
        color: var!(text-success);
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-success));
        font-size: "14px";
    }

    pub c_counter_row {
        display: "flex";
        align-items: "center";
        gap: "20px";
        flex-wrap: "wrap";
    }

    pub c_counter_text {
        font-size: "16px";
        color: "inherit";
    }

    pub c_counter_value {
        font-weight: "700";
        color: "#4f46e5";
        font-size: "24px";
    }

    pub c_badge_row {
        display: "flex";
        gap: "10px";
        flex-wrap: "wrap";
        align-items: "center";
    }

    pub c_badge_hint {
        margin-bottom: "12px";
        color: "inherit";
        opacity: "0.7";
        font-size: "14px";
    }

    pub c_form_grid {
        display: "grid";
        grid-template-columns: "1fr 1fr";
        gap: "0 20px";
    }

    pub c_custom_attrs_demo {
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-input));
        color: "inherit";
    }

    pub c_demo_text {
        color: "inherit";
        margin-bottom: "8px";
    }

    pub c_demo_text_muted {
        color: "inherit";
        opacity: "0.6";
        font-size: "13px";
    }

    pub c_loading_container {
        display: "flex";
        align-items: "center";
        gap: "16px";
        padding: "20px";
        background: var!(bg-loading);
        border-radius: "10px";
        border: format!("1px solid {}", var!(border-loading));
        margin-top: "16px";
    }

    pub c_spinner {
        width: "28px";
        height: "28px";
        border: format!("3px solid {}", var!(border-loading));
        border-top: "3px solid #4f46e5";
        border-radius: "50%";
        flex-shrink: "0";
        animation: "euv-spin 0.8s linear infinite";
    }

    pub c_loading_text_col {
        display: "flex";
        flex-direction: "column";
        gap: "4px";
    }

    pub c_loading_title {
        color: var!(text-loading-title);
        font-size: "15px";
        font-weight: "500";
    }

    pub c_loading_subtitle {
        color: var!(text-muted);
        font-size: "13px";
    }

    pub c_error_container {
        display: "flex";
        align-items: "center";
        gap: "12px";
        margin-top: "16px";
        padding: "16px";
        background: var!(bg-error);
        border-radius: "10px";
        border: format!("1px solid {}", var!(border-error));
    }

    pub c_error_icon {
        width: "20px";
        height: "20px";
        border-radius: "50%";
        background: var!(bg-error-icon);
        display: "flex";
        align-items: "center";
        justify-content: "center";
        flex-shrink: "0";
        color: var!(text-error);
        font-size: "12px";
        font-weight: "700";
    }

    pub c_error_text {
        color: var!(text-error);
        font-size: "14px";
    }

    pub c_data_box {
        margin-top: "16px";
        padding: "16px";
        background: var!(bg-success);
        border-radius: "10px";
        border: format!("1px solid {}", var!(border-success));
    }

    pub c_data_pre {
        color: var!(text-success);
        font-size: "13px";
        margin: "0";
        white-space: "pre-wrap";
        word-break: "break-all";
        font-family: "ui-monospace, monospace";
    }

    pub c_fetch_hint {
        color: "inherit";
        opacity: "0.7";
        font-size: "14px";
        margin-bottom: "16px";
    }

    pub c_toggle_content {
        margin-top: "16px";
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-accent-light));
        color: "inherit";
    }

    pub c_toggle_title {
        margin-top: "0";
        color: "#4f46e5";
        font-size: "15px";
    }

    pub c_role_button_row {
        display: "flex";
        gap: "10px";
        margin-bottom: "16px";
    }

    pub c_role_guest {
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-warning));
    }

    pub c_role_guest_text {
        color: var!(text-warning);
        font-size: "14px";
    }

    pub c_role_user {
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-info));
    }

    pub c_role_user_text {
        color: var!(text-info);
        font-size: "14px";
    }

    pub c_role_admin {
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-success));
    }

    pub c_role_admin_text {
        color: var!(text-success);
        font-size: "14px";
    }

    pub c_tab_bar {
        display: "flex";
        border-bottom: format!("1px solid {}", var!(border-input));
        margin-bottom: "16px";
        gap: "4px";
    }

    pub c_tab_content {
        padding: "4px 0";
    }

    pub c_tab_text {
        color: "inherit";
        font-size: "14px";
        margin-bottom: "8px";
    }

    pub c_tab_text_muted {
        color: "inherit";
        opacity: "0.6";
        font-size: "13px";
    }

    pub c_tab_text_input {
        color: "inherit";
        font-size: "14px";
        margin-bottom: "12px";
    }

    pub c_list_input_row {
        display: "flex";
        gap: "12px";
        margin-bottom: "20px";
    }

    pub c_list_input {
        flex: "1";
        padding: "10px 14px";
        border: format!("1px solid {}", var!(border-input));
        border-radius: "8px";
        font-size: "14px";
        background: var!(bg-input);
        outline: "none";
        color: var!(text-primary);
    }

    pub c_list_ul {
        list-style: "none";
        padding: "0";
        margin: "0";
    }

    pub c_list_item_even {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        padding: "12px 16px";
        border-bottom: format!("1px solid {}", var!(border-subtle));
        border-radius: "6px";
        margin-bottom: "8px";
        gap: "16px";
        background: var!(bg-list-even);
    }

    pub c_list_item_odd {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        padding: "12px 16px";
        border-bottom: format!("1px solid {}", var!(border-subtle));
        border-radius: "6px";
        margin-bottom: "8px";
        gap: "16px";
        background: var!(bg-list-odd);
    }

    pub c_list_item_text {
        flex: "1";
        font-size: "14px";
        color: "inherit";
    }

    pub c_log_container {
        max-height: "320px";
        overflow-y: "auto";
        background: var!(bg-console);
        padding: "16px";
        border-radius: "8px";
        font-family: "ui-monospace, monospace";
        font-size: "13px";
        line-height: "1.5";
        margin-top: "16px";
        color: var!(text-console);
    }

    pub c_log_item {
        padding: "6px 0";
        border-bottom: format!("1px solid {}", var!(border-console));
        font-size: "13px";
        color: var!(text-console);
    }

    pub c_not_found_container {
        text-align: "center";
        padding: "80px 20px";
        max-width: "480px";
        margin: "0 auto";
    }

    pub c_not_found_code {
        font-size: "72px";
        font-weight: "800";
        color: var!(text-muted);
        line-height: "1";
        margin-bottom: "16px";
        letter-spacing: "-0.04em";
    }

    pub c_not_found_title {
        font-size: "24px";
        font-weight: "600";
        color: "inherit";
        margin-bottom: "12px";
    }

    pub c_not_found_text {
        color: "inherit";
        opacity: "0.7";
        font-size: "16px";
        margin-bottom: "28px";
        line-height: "1.6";
    }

    pub c_render_count_text {
        font-size: "16px";
        color: "inherit";
        margin-bottom: "16px";
    }

    pub c_inline {
        display: "inline";
    }

    pub c_event_result {
        font-size: "14px";
        color: "inherit";
        margin-top: "12px";
    }

    pub c_event_highlight {
        font-weight: "600";
        color: "#4f46e5";
    }

    pub c_event_stats {
        display: "flex";
        gap: "24px";
        margin-top: "12px";
    }

    pub c_event_mouse_area {
        min-height: "120px";
        padding: "20px";
        border-radius: "8px";
        border: format!("2px dashed {}", var!(border-accent-light));
        cursor: "crosshair";
        text-align: "center";
        user-select: "none";
        color: "inherit";
    }

    pub c_log_empty {
        color: var!(text-muted);
        font-size: "13px";
        text-align: "center";
        padding: "16px";
    }

    pub c_timer_display {
        text-align: "center";
        padding: "20px";
        margin: "16px 0";
        border-radius: "12px";
        border: format!("1px solid {}", var!(border-accent-light));
        color: "inherit";
    }

    pub c_timer_value {
        font-size: "48px";
        font-weight: "700";
        color: "#4f46e5";
        letter-spacing: "0.04em";
        font-family: "ui-monospace, monospace";
    }

    pub c_timer_controls {
        display: "flex";
        gap: "12px";
        justify-content: "center";
    }

    pub c_timer_done {
        text-align: "center";
        padding: "12px";
        margin-top: "12px";
        background: var!(bg-warning);
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-warning));
        font-size: "16px";
        font-weight: "600";
        color: var!(text-warning);
    }

    pub c_modal_overlay {
        position: "fixed";
        top: "0";
        left: "0";
        width: "100%";
        height: "100%";
        background: "rgba(0, 0, 0, 0.5)";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        z-index: "1000";
        animation: "euv-fade-in 0.2s ease";
    }

    pub c_modal_content {
        background: var!(bg-modal);
        border-radius: "16px";
        padding: "0";
        max-width: "480px";
        width: "90%";
        box-shadow: var!(shadow-modal);
        animation: "euv-scale-in 0.2s ease";
        overflow: "hidden";
        color: var!(text-card);
    }

    pub c_modal_header {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        padding: "20px 24px";
        border-bottom: format!("1px solid {}", var!(border-subtle));
    }

    pub c_modal_title {
        margin: "0";
        font-size: "18px";
        font-weight: "600";
        color: "inherit";
    }

    pub c_modal_body {
        padding: "24px";
        color: "inherit";
    }

    pub c_modal_actions {
        display: "flex";
        gap: "12px";
        justify-content: "flex-end";
        margin-top: "20px";
    }

    pub c_select_input {
        width: "100%";
        padding: "10px 14px";
        border: format!("1px solid {}", var!(border-input));
        border-radius: "8px";
        font-size: "14px";
        box-sizing: "border-box";
        background: var!(bg-input);
        outline: "none";
        cursor: "pointer";
        appearance: "none";
        color: var!(text-primary);
        background-image: "url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%236b7280' d='M6 8L1 3h10z'/%3E%3C/svg%3E\")";
        background-repeat: "no-repeat";
        background-position: "right 14px center";
    }

    pub c_textarea_input {
        width: "100%";
        padding: "10px 14px";
        border: format!("1px solid {}", var!(border-input));
        border-radius: "8px";
        font-size: "14px";
        box-sizing: "border-box";
        background: var!(bg-input);
        outline: "none";
        resize: "vertical";
        font-family: "inherit";
        line-height: "1.5";
        color: var!(text-primary);
    }

    pub c_textarea_counter {
        text-align: "right";
        margin-top: "4px";
    }

    pub c_textarea_counter_text {
        font-size: "12px";
        color: "inherit";
        opacity: "0.5";
    }

    pub c_anim_fade_in {
        margin-top: "16px";
        padding: "20px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-accent-light));
        animation: "euv-fade-in 0.5s ease";
        font-size: "14px";
        color: "inherit";
    }

    pub c_anim_spin_container {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        margin: "24px 0";
        min-height: "80px";
    }

    pub c_anim_spin {
        font-size: "48px";
        animation: "euv-spin 1.5s linear infinite";
        display: "inline-block";
    }

    pub c_anim_spin_stopped {
        font-size: "48px";
        display: "inline-block";
        opacity: "0.4";
    }

    pub c_anim_pulse_container {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        margin: "24px 0";
        min-height: "80px";
    }

    pub c_anim_pulse {
        font-size: "48px";
        animation: "euv-pulse 1.5s ease-in-out infinite";
        display: "inline-block";
        color: "#dc2626";
    }

    pub c_anim_pulse_stopped {
        font-size: "48px";
        display: "inline-block";
        opacity: "0.4";
        color: "#dc2626";
    }

    pub c_progress_container {
        width: "100%";
        height: "12px";
        background: var!(bg-progress);
        border-radius: "999px";
        margin: "16px 0 8px 0";
        overflow: "hidden";
    }

    pub c_progress_bar {
        height: "100%";
        border-radius: "999px";
        transition: "width 0.1s ease";
    }

    pub c_progress_text {
        text-align: "center";
        font-size: "14px";
        color: "inherit";
        font-weight: "500";
    }

    pub c_anim_color_box {
        margin: "16px 0";
        padding: "24px";
        border-radius: "12px";
        color: "white";
        text-align: "center";
        font-weight: "600";
        font-size: "16px";
        cursor: "pointer";
        transition: "background 0.5s ease, transform 0.3s ease";
    }

    pub c_event_drag_zone {
        min-height: "100px";
        padding: "20px";
        border-radius: "8px";
        border: format!("2px dashed {}", var!(border-warning));
        text-align: "center";
        user-select: "none";
        cursor: "grab";
        color: var!(text-warning);
    }

    pub c_event_drag_zone_active {
        min-height: "100px";
        padding: "20px";
        border-radius: "8px";
        border: format!("2px dashed {}", var!(border-success));
        text-align: "center";
        user-select: "none";
        color: var!(text-success);
    }

    pub c_event_drag_item {
        display: "inline-block";
        padding: "8px 20px";
        background: "#4f46e5";
        color: "white";
        border-radius: "6px";
        font-size: "14px";
        font-weight: "500";
        cursor: "grab";
        margin: "8px";
    }

    pub c_event_wheel_zone {
        min-height: "120px";
        padding: "20px";
        border-radius: "8px";
        border: format!("2px dashed {}", var!(border-success));
        text-align: "center";
        overflow: "auto";
        color: var!(text-success);
    }

    pub c_event_clipboard_area {
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-pink));
        color: var!(text-pink);
    }

    pub c_event_touch_zone {
        min-height: "120px";
        padding: "20px";
        border-radius: "8px";
        border: "2px dashed #8b5cf6";
        text-align: "center";
        touch-action: "none";
        user-select: "none";
        color: "#5b21b6";
    }

    pub c_event_form_area {
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-info));
        color: var!(text-info);
    }

    pub c_event_media_area {
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-success));
        color: var!(text-success);
    }

    pub c_event_section_row {
        display: "flex";
        gap: "24px";
        flex-wrap: "wrap";
        margin-top: "12px";
    }

    pub c_event_section_col {
        flex: "1";
        min-width: "200px";
    }

    pub c_event_section_label {
        font-size: "13px";
        color: "inherit";
        opacity: "0.6";
        margin-bottom: "4px";
    }

    pub c_event_section_value {
        font-size: "14px";
        color: "inherit";
        font-weight: "500";
    }

    pub c_browser_api_row {
        display: "grid";
        grid-template-columns: "1fr 1fr";
        gap: "0 20px";
        margin-bottom: "12px";
    }

    pub c_browser_api_actions {
        display: "flex";
        gap: "10px";
        flex-wrap: "wrap";
        margin: "12px 0";
    }

    pub c_browser_result_box {
        margin-top: "12px";
        padding: "14px 16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-accent-light));
        font-size: "14px";
        word-break: "break-all";
        color: "inherit";
    }

    pub c_browser_result_label {
        font-weight: "600";
        color: "#4f46e5";
    }

    pub c_browser_result_value {
        color: "inherit";
    }

    pub c_browser_info_grid {
        display: "grid";
        gap: "12px";
        margin-top: "12px";
    }

    pub c_browser_info_item {
        display: "flex";
        flex-direction: "column";
        gap: "4px";
        padding: "12px 16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-input));
        color: "inherit";
    }

    pub c_browser_info_label {
        font-size: "12px";
        font-weight: "600";
        color: "inherit";
        opacity: "0.6";
        text-transform: "uppercase";
        letter-spacing: "0.05em";
    }

    pub c_browser_info_value {
        font-size: "14px";
        color: "inherit";
        word-break: "break-all";
        font-family: "ui-monospace, monospace";
    }

    pub c_vconsole_button {
        position: "fixed";
        bottom: "20px";
        right: "20px";
        width: "44px";
        height: "44px";
        border-radius: "50%";
        background: var!(border-console-accent);
        color: "white";
        border: "none";
        cursor: "pointer";
        font-size: "18px";
        font-weight: "700";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        z-index: "9999";
        box-shadow: "0 4px 14px rgba(79, 70, 229, 0.4)";
        transition: "transform 0.2s ease, box-shadow 0.2s ease";
        padding: "0";
        line-height: "1";
    }

    pub c_vconsole_badge {
        position: "absolute";
        top: "-4px";
        right: "-4px";
        min-width: "18px";
        height: "18px";
        border-radius: "9px";
        background: var!(bg-console-badge);
        color: "white";
        font-size: "11px";
        font-weight: "600";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        padding: "0 5px";
        line-height: "1";
    }

    pub c_vconsole_overlay {
        position: "fixed";
        top: "0";
        left: "0";
        right: "0";
        bottom: "50vh";
        z-index: "10000";
    }

    pub c_vconsole_panel {
        position: "fixed";
        bottom: "0";
        left: "0";
        right: "0";
        height: "50vh";
        background: var!(bg-console);
        z-index: "10000";
        display: "flex";
        flex-direction: "column";
        border-top: format!("3px solid {}", var!(border-console-accent));
        border-radius: "12px 12px 0 0";
        animation: "euv-slide-up 0.25s ease";
        overflow: "hidden";
    }

    pub c_vconsole_header {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        padding: "12px 16px";
        background: var!(bg-console-header);
        border-bottom: format!("1px solid {}", var!(border-console));
        flex-shrink: "0";
    }

    pub c_vconsole_title {
        color: var!(text-console-title);
        font-size: "14px";
        font-weight: "600";
        margin: "0";
        letter-spacing: "0.03em";
    }

    pub c_vconsole_header_actions {
        display: "flex";
        gap: "8px";
        align-items: "center";
    }

    pub c_vconsole_clear_button {
        background: "transparent";
        border: format!("1px solid {}", var!(border-console-button));
        color: var!(text-console-button);
        padding: "4px 12px";
        border-radius: "4px";
        cursor: "pointer";
        font-size: "12px";
        transition: "all 0.15s ease";
    }

    pub c_vconsole_close_button {
        background: "transparent";
        border: format!("1px solid {}", var!(border-console-button));
        color: var!(text-console-button);
        padding: "4px 10px";
        border-radius: "4px";
        cursor: "pointer";
        font-size: "16px";
        line-height: "1";
        transition: "all 0.15s ease";
    }

    pub c_vconsole_body {
        flex: "1";
        overflow-y: "auto";
        padding: "12px 16px";
        font-family: "ui-monospace, monospace";
        font-size: "13px";
        line-height: "1.6";
    }

    pub c_vconsole_log_item {
        padding: "4px 0";
        border-bottom: format!("1px solid {}", var!(border-console));
        color: var!(text-console);
        font-size: "13px";
        word-break: "break-all";
    }

    pub c_vconsole_log_latest {
        color: var!(text-console-log-latest);
        font-weight: "500";
    }

    pub c_vconsole_log_warn {
        color: var!(text-console-warn);
    }

    pub c_vconsole_log_warn_latest {
        color: var!(text-console-warn-latest);
        font-weight: "500";
    }

    pub c_vconsole_log_error {
        color: var!(text-console-error);
    }

    pub c_vconsole_log_error_latest {
        color: var!(text-console-error-latest);
        font-weight: "500";
    }

    pub c_vconsole_empty {
        color: var!(text-console-empty);
        font-size: "13px";
        text-align: "center";
        padding: "40px 0";
    }

    pub c_vconsole_count {
        color: var!(text-console-empty);
        font-size: "12px";
    }

    pub c_vconsole_filter_bar {
        display: "flex";
        gap: "6px";
        padding: "8px 16px";
        background: var!(bg-console-filter);
        border-bottom: format!("1px solid {}", var!(border-console));
        flex-shrink: "0";
    }

    pub c_vconsole_filter_button {
        padding: "3px 10px";
        border-radius: "4px";
        background: "transparent";
        border: format!("1px solid {}", var!(border-console-button));
        color: var!(text-console-button);
        font-size: "11px";
        font-weight: "500";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
    }

    pub c_vconsole_filter_active {
        padding: "3px 10px";
        border-radius: "4px";
        background: var!(bg-console-filter-active);
        border: format!("1px solid {}", var!(border-console-filter-active));
        color: var!(text-console-filter-active);
        font-size: "11px";
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
    }

    pub c_vconsole_filter_active_log {
        padding: "3px 10px";
        border-radius: "4px";
        background: var!(bg-console-badge-log);
        border: format!("1px solid {}", var!(text-console-log-latest));
        color: var!(text-console-log-latest);
        font-size: "11px";
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
    }

    pub c_vconsole_filter_active_warn {
        padding: "3px 10px";
        border-radius: "4px";
        background: var!(bg-console-badge-warn);
        border: format!("1px solid {}", var!(text-console-warn-latest));
        color: var!(text-console-warn-latest);
        font-size: "11px";
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
    }

    pub c_vconsole_filter_active_error {
        padding: "3px 10px";
        border-radius: "4px";
        background: var!(bg-console-badge-error);
        border: format!("1px solid {}", var!(text-console-error-latest));
        color: var!(text-console-error-latest);
        font-size: "11px";
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
    }

    pub c_vconsole_level_badge {
        display: "inline-block";
        padding: "1px 6px";
        border-radius: "3px";
        font-size: "10px";
        font-weight: "600";
        margin-right: "6px";
        letter-spacing: "0.05em";
        color: "inherit";
    }

    pub c_vconsole_badge_log {
        background: var!(bg-console-badge-log);
    }

    pub c_vconsole_badge_warn {
        background: var!(bg-console-badge-warn);
    }

    pub c_vconsole_badge_error {
        background: var!(bg-console-badge-error);
    }

    pub c_nav_item_active {
        display: "block";
        padding: "11px 20px";
        text-decoration: "none";
        font-size: "14px";
        transition: "all 0.15s ease";
        color: "#4f46e5";
        font-weight: "600";
        border-left: "3px solid #4f46e5";
        background: "rgba(79, 70, 229, 0.08)";
    }

    pub c_nav_item_inactive {
        display: "block";
        padding: "11px 20px";
        text-decoration: "none";
        font-size: "14px";
        transition: "all 0.15s ease";
        color: var!(text-nav-item);
        font-weight: "400";
        border-left: "3px solid transparent";
        background: "transparent";
    }
}
