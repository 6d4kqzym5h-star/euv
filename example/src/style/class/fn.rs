use crate::*;

class! {
    pub(crate) c_app_root {
        display: "flex";
        height: "100vh";
        overflow: "hidden";
        font-family: "system-ui, -apple-system, sans-serif";
        background: var!(bg-primary);
        color: var!(text-primary);
        line-height: "1.6";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_app_nav {
        width: "240px";
        background: var!(bg-nav);
        border-right: format!("1px solid {}", var!(border-nav));
        display: "flex";
        flex-direction: "column";
        height: "100vh";
        flex-shrink: "0";
        media("(max-width: 767px)") {
            display: "none";
        }
    }

    pub(crate) c_nav_header {
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
        flex-shrink: "0";
        text-decoration: "none";
        cursor: "pointer";
        media("(max-width: 767px)") {
            border-bottom: "none";
        }
    }

    pub(crate) c_nav_logo {
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

    pub(crate) c_nav_section_label {
        padding: "10px 20px 4px 20px";
        margin: "0";
        font-size: "11px";
        font-weight: "600";
        color: var!(text-muted);
        text-transform: "uppercase";
        letter-spacing: "0.08em";
        flex-shrink: "0";
    }

    pub(crate) c_nav_items_scroll {
        flex: "1";
        overflow-y: "auto";
    }

    pub(crate) c_nav_theme_toggle {
        padding: "12px 20px";
        flex-shrink: "0";
        margin-top: "auto";
    }

    pub(crate) c_nav_theme_button {
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

    pub(crate) c_nav_footer {
        padding: "16px 20px";
        border-top: format!("1px solid {}", var!(border-subtle));
        font-size: "12px";
        color: var!(text-muted);
        flex-shrink: "0";
        text-decoration: "none";
        display: "block";
        cursor: "pointer";
    }

    pub(crate) c_app_main {
        flex: "1";
        width: "0";
        padding: "32px 40px";
        background: var!(bg-primary);
        overflow-y: "auto";
        media("(max-width: 767px)") {
            padding: "20px 16px";
            max-width: "100%";
        }
    }

    pub(crate) c_page_container {
        max-width: "800px";
        margin: "0 auto";
    }

    pub(crate) c_page_header {
        margin-bottom: "32px";
    }

    pub(crate) c_page_title {
        font-size: "28px";
        font-weight: "700";
        color: "inherit";
        margin-bottom: "8px";
        letter-spacing: "-0.02em";
        media("(max-width: 767px)") {
            font-size: "22px";
        }
    }

    pub(crate) c_page_subtitle {
        font-size: "16px";
        color: "inherit";
        opacity: "0.6";
        line-height: "1.6";
        media("(max-width: 767px)") {
            font-size: "14px";
        }
    }

    pub(crate) c_card {
        background: var!(bg-card);
        border-radius: "12px";
        padding: "24px";
        margin: "16px 0";
        box-shadow: var!(shadow-card);
        border: format!("1px solid {}", var!(border-card));
        color: var!(text-card);
        transition: "box-shadow 0.2s ease, background 0.4s ease, border-color 0.4s ease, color 0.4s ease";
        media("(max-width: 767px)") {
            padding: "16px";
            margin: "12px 0";
            border-radius: "10px";
        }
    }

    pub(crate) c_card_title {
        margin-top: "0";
        margin-bottom: "16px";
        color: "inherit";
        font-size: "18px";
        font-weight: "600";
        padding-bottom: "12px";
        border-bottom: format!("2px solid {}", var!(border-card-title));
        letter-spacing: "0.01em";
        transition: "border-color 0.4s ease";
        media("(max-width: 767px)") {
            font-size: "16px";
        }
    }

    pub(crate) c_primary_button {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        gap: "8px";
        background: "#4f46e5";
        color: "white";
        border: "none";
        padding: "10px 24px";
        border-radius: "8px";
        cursor: "pointer";
        font-size: "14px";
        font-weight: "500";
        letter-spacing: "0.02em";
        text-align: "center";
        line-height: "1.4";
        transition: "background 0.2s ease, box-shadow 0.2s ease, transform 0.1s ease";
        outline: "none";
        box-shadow: "0 1px 2px rgba(79, 70, 229, 0.2)";
        white-space: "nowrap";
        user-select: "none";
        -webkit-user-select: "none";
        media("(max-width: 767px)") {
            width: "100%";
            padding: "12px 20px";
            font-size: "15px";
            border-radius: "10px";
            box-shadow: "none";
        }
    }

    pub(crate) c_primary_button_disabled {
        c_primary_button();
        background: "#9b9bd4";
        cursor: "not-allowed";
        opacity: "0.55";
        box-shadow: "none";
        media("(max-width: 767px)") {
            width: "100%";
            padding: "12px 20px";
            font-size: "15px";
            border-radius: "10px";
        }
    }

    pub(crate) c_modal_primary_button {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        gap: "8px";
        flex: "1";
        min-width: "0";
        background: "#4f46e5";
        color: "white";
        border: "none";
        padding: "10px 24px";
        border-radius: "8px";
        cursor: "pointer";
        font-size: "14px";
        font-weight: "500";
        letter-spacing: "0.02em";
        text-align: "center";
        line-height: "1.4";
        transition: "background 0.2s ease, box-shadow 0.2s ease, transform 0.1s ease";
        outline: "none";
        box-shadow: "0 1px 2px rgba(79, 70, 229, 0.2)";
        white-space: "nowrap";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_modal_primary_button_disabled {
        c_modal_primary_button();
        background: "#9b9bd4";
        cursor: "not-allowed";
        opacity: "0.55";
        box-shadow: "none";
    }

    pub(crate) c_modal_close_button {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        background: "transparent";
        color: "inherit";
        border: "none";
        padding: "6px 10px";
        border-radius: "6px";
        cursor: "pointer";
        font-size: "20px";
        font-weight: "400";
        line-height: "1";
        transition: "background 0.15s ease";
        outline: "none";
        opacity: "0.6";
        flex-shrink: "0";
    }

    pub(crate) c_badge {
        display: "inline-block";
        color: "white";
        padding: "5px 14px";
        border-radius: "20px";
        font-size: "12px";
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.03em";
        text-transform: "uppercase";
        media("(max-width: 767px)") {
            padding: "4px 10px";
            font-size: "11px";
        }
    }

    pub(crate) c_badge_outline {
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
        media("(max-width: 767px)") {
            padding: "4px 10px";
            font-size: "11px";
        }
    }

    pub(crate) c_form_input_wrapper {
        margin: "12px 0";
    }

    pub(crate) c_form_label {
        display: "block";
        margin-bottom: "6px";
        color: "inherit";
        font-weight: "500";
        font-size: "14px";
    }

    pub(crate) c_form_input {
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

    pub(crate) c_form_input_no_transition {
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

    pub(crate) c_form_checkbox {
        cursor: "pointer";
        width: "16px";
        height: "16px";
    }

    pub(crate) c_form_checkbox_label {
        font-size: "14px";
        color: "inherit";
        cursor: "pointer";
    }

    pub(crate) c_form_checkbox_row {
        margin: "16px 0";
        display: "flex";
        align-items: "center";
        gap: "10px";
    }

    pub(crate) c_error_box {
        margin: "16px 0";
        padding: "14px 16px";
        background: var!(bg-error);
        color: var!(text-error);
        border-radius: "8px";
        font-size: "14px";
        border: format!("1px solid {}", var!(border-error));
    }

    pub(crate) c_success_box {
        margin-top: "16px";
        padding: "14px 16px";
        background: var!(bg-success);
        color: var!(text-success);
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-success));
        font-size: "14px";
    }

    pub(crate) c_counter_row {
        display: "flex";
        align-items: "center";
        gap: "20px";
        flex-wrap: "wrap";
    }

    pub(crate) c_counter_text {
        font-size: "16px";
        color: "inherit";
    }

    pub(crate) c_counter_value {
        font-weight: "700";
        color: "#4f46e5";
        font-size: "24px";
        media("(max-width: 767px)") {
            font-size: "20px";
        }
    }

    pub(crate) c_badge_row {
        display: "flex";
        gap: "10px";
        flex-wrap: "wrap";
        align-items: "center";
    }

    pub(crate) c_badge_hint {
        margin-bottom: "12px";
        color: "inherit";
        opacity: "0.7";
        font-size: "14px";
    }

    pub(crate) c_form_grid {
        display: "grid";
        grid-template-columns: "1fr 1fr";
        gap: "0 20px";
        media("(max-width: 767px)") {
            grid-template-columns: "1fr";
        }
    }

    pub(crate) c_custom_attrs_demo {
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-input));
        color: "inherit";
    }

    pub(crate) c_demo_text {
        color: "inherit";
        margin-bottom: "8px";
    }

    pub(crate) c_demo_text_muted {
        color: "inherit";
        opacity: "0.6";
        font-size: "13px";
    }

    pub(crate) c_loading_container {
        display: "flex";
        align-items: "center";
        gap: "16px";
        padding: "20px";
        background: var!(bg-loading);
        border-radius: "10px";
        border: format!("1px solid {}", var!(border-loading));
        margin-top: "16px";
    }

    pub(crate) c_spinner {
        width: "28px";
        height: "28px";
        border: format!("3px solid {}", var!(border-loading));
        border-top: "3px solid #4f46e5";
        border-radius: "50%";
        flex-shrink: "0";
        animation: "euv-spin 0.8s linear infinite";
    }

    pub(crate) c_loading_text_col {
        display: "flex";
        flex-direction: "column";
        gap: "4px";
    }

    pub(crate) c_loading_title {
        color: var!(text-loading-title);
        font-size: "15px";
        font-weight: "500";
    }

    pub(crate) c_loading_subtitle {
        color: var!(text-muted);
        font-size: "13px";
    }

    pub(crate) c_error_container {
        display: "flex";
        align-items: "center";
        gap: "12px";
        margin-top: "16px";
        padding: "16px";
        background: var!(bg-error);
        border-radius: "10px";
        border: format!("1px solid {}", var!(border-error));
    }

    pub(crate) c_error_icon {
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

    pub(crate) c_error_text {
        color: var!(text-error);
        font-size: "14px";
    }

    pub(crate) c_data_box {
        margin-top: "16px";
        padding: "16px";
        background: var!(bg-success);
        border-radius: "10px";
        border: format!("1px solid {}", var!(border-success));
    }

    pub(crate) c_data_pre {
        color: var!(text-success);
        font-size: "13px";
        margin: "0";
        white-space: "pre-wrap";
        word-break: "break-all";
        font-family: "ui-monospace, monospace";
    }

    pub(crate) c_fetch_hint {
        color: "inherit";
        opacity: "0.7";
        font-size: "14px";
        margin-bottom: "16px";
    }

    pub(crate) c_toggle_content {
        margin-top: "16px";
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-accent-light));
        color: "inherit";
    }

    pub(crate) c_toggle_title {
        margin-top: "0";
        color: "#4f46e5";
        font-size: "15px";
    }

    pub(crate) c_role_button_row {
        display: "flex";
        gap: "10px";
        margin-bottom: "16px";
        media("(max-width: 767px)") {
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_role_guest {
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-warning));
    }

    pub(crate) c_role_guest_text {
        color: var!(text-warning);
        font-size: "14px";
    }

    pub(crate) c_role_user {
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-info));
    }

    pub(crate) c_role_user_text {
        color: var!(text-info);
        font-size: "14px";
    }

    pub(crate) c_role_admin {
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-success));
    }

    pub(crate) c_role_admin_text {
        color: var!(text-success);
        font-size: "14px";
    }

    pub(crate) c_tab_bar {
        display: "flex";
        border-bottom: format!("1px solid {}", var!(border-input));
        margin-bottom: "16px";
        gap: "4px";
        media("(max-width: 767px)") {
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_tab_content {
        padding: "4px 0";
    }

    pub(crate) c_tab_text {
        color: "inherit";
        font-size: "14px";
        margin-bottom: "8px";
    }

    pub(crate) c_tab_text_muted {
        color: "inherit";
        opacity: "0.6";
        font-size: "13px";
    }

    pub(crate) c_tab_text_input {
        color: "inherit";
        font-size: "14px";
        margin-bottom: "12px";
    }

    pub(crate) c_list_input_row {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        padding: "12px 16px";
        border-bottom: format!("1px solid {}", var!(border-subtle));
        border-radius: "6px";
        margin-bottom: "8px";
        gap: "16px";
        background: var!(bg-list-even);
        media("(max-width: 767px)") {
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_list_input {
        flex: "1";
        min-width: "0";
        padding: "8px 12px";
        border: format!("1px solid {}", var!(border-input));
        border-radius: "6px";
        font-size: "14px";
        line-height: "1.4";
        box-sizing: "border-box";
        background: var!(bg-input);
        outline: "none";
        color: var!(text-primary);
    }

    pub(crate) c_list_ul {
        list-style: "none";
        padding: "0";
        margin: "0";
    }

    pub(crate) c_list_item_even {
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

    pub(crate) c_list_item_odd {
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

    pub(crate) c_list_item_text {
        flex: "1";
        min-width: "0";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        font-size: "14px";
        color: "inherit";
    }

    pub(crate) c_list_item_button {
        width: "auto";
        padding: "8px 16px";
        flex-shrink: "0";
    }

    pub(crate) c_log_container {
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

    pub(crate) c_log_item {
        padding: "6px 0";
        border-bottom: format!("1px solid {}", var!(border-console));
        font-size: "13px";
        color: var!(text-console);
    }

    pub(crate) c_not_found_container {
        text-align: "center";
        padding: "80px 20px";
        max-width: "480px";
        margin: "0 auto";
        media("(max-width: 767px)") {
            padding: "40px 20px";
        }
    }

    pub(crate) c_not_found_code {
        font-size: "72px";
        font-weight: "800";
        color: var!(text-muted);
        line-height: "1";
        margin-bottom: "16px";
        letter-spacing: "-0.04em";
        media("(max-width: 767px)") {
            font-size: "56px";
        }
    }

    pub(crate) c_not_found_title {
        font-size: "24px";
        font-weight: "600";
        color: "inherit";
        margin-bottom: "12px";
    }

    pub(crate) c_not_found_text {
        color: "inherit";
        opacity: "0.7";
        font-size: "16px";
        margin-bottom: "28px";
        line-height: "1.6";
    }

    pub(crate) c_render_count_text {
        font-size: "16px";
        color: "inherit";
        margin-bottom: "16px";
    }


    pub(crate) c_event_result {
        font-size: "14px";
        color: "inherit";
        margin-top: "12px";
    }

    pub(crate) c_event_highlight {
        font-weight: "600";
        color: "#4f46e5";
    }

    pub(crate) c_event_stats {
        display: "flex";
        gap: "24px";
        margin-top: "12px";
        media("(max-width: 767px)") {
            gap: "12px";
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_event_mouse_area {
        min-height: "120px";
        padding: "20px";
        border-radius: "8px";
        border: format!("2px dashed {}", var!(border-accent-light));
        cursor: "crosshair";
        text-align: "center";
        user-select: "none";
        color: "inherit";
    }

    pub(crate) c_log_empty {
        color: var!(text-muted);
        font-size: "13px";
        text-align: "center";
        padding: "16px";
    }

    pub(crate) c_timer_display {
        text-align: "center";
        padding: "20px";
        margin: "16px 0";
        border-radius: "12px";
        border: format!("1px solid {}", var!(border-accent-light));
        color: "inherit";
    }

    pub(crate) c_timer_value {
        font-size: "48px";
        font-weight: "700";
        color: "#4f46e5";
        letter-spacing: "0.04em";
        font-family: "ui-monospace, monospace";
        media("(max-width: 767px)") {
            font-size: "36px";
        }
    }

    pub(crate) c_timer_controls {
        display: "flex";
        gap: "12px";
        justify-content: "center";
        media("(max-width: 767px)") {
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_timer_done {
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

    pub(crate) c_modal_overlay {
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
        media("(max-width: 767px)") {
            align-items: "center";
            justify-content: "center";
        }
    }

    pub(crate) c_modal_content {
        background: var!(bg-modal);
        border-radius: "16px";
        padding: "0";
        max-width: "480px";
        width: "90%";
        box-shadow: var!(shadow-modal);
        animation: "euv-scale-in 0.2s ease";
        overflow: "hidden";
        color: var!(text-card);
        media("(max-width: 767px)") {
            max-width: "100%";
            width: "calc(100% - 32px)";
            border-radius: "16px";
            max-height: "85vh";
            overflow-y: "auto";
        }
    }

    pub(crate) c_modal_header {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        padding: "20px 24px";
        border-bottom: format!("1px solid {}", var!(border-subtle));
    }

    pub(crate) c_modal_title {
        margin: "0";
        font-size: "18px";
        font-weight: "600";
        color: "inherit";
    }

    pub(crate) c_modal_body {
        padding: "24px";
        color: "inherit";
    }

    pub(crate) c_modal_actions {
        display: "flex";
        gap: "12px";
        margin-top: "20px";
    }

    pub(crate) c_select_input {
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

    pub(crate) c_textarea_input {
        width: "100%";
        max-width: "100%";
        padding: "10px 14px";
        border: format!("1px solid {}", var!(border-input));
        border-radius: "8px";
        font-size: "14px";
        box-sizing: "border-box";
        background: var!(bg-input);
        outline: "none";
        resize: "vertical";
        overflow-x: "hidden";
        word-wrap: "break-word";
        font-family: "inherit";
        line-height: "1.5";
        color: var!(text-primary);
    }

    pub(crate) c_textarea_counter {
        text-align: "right";
        margin-top: "4px";
    }

    pub(crate) c_textarea_counter_text {
        font-size: "12px";
        color: "inherit";
        opacity: "0.5";
    }

    pub(crate) c_anim_fade_in {
        margin-top: "16px";
        padding: "20px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-accent-light));
        animation: "euv-fade-in 0.5s ease";
        font-size: "14px";
        color: "inherit";
    }

    pub(crate) c_anim_spin_container {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        margin: "24px 0";
        min-height: "80px";
    }

    pub(crate) c_anim_spin {
        font-size: "48px";
        animation: "euv-spin 1.5s linear infinite";
        display: "inline-block";
        media("(max-width: 767px)") {
            font-size: "36px";
        }
    }

    pub(crate) c_anim_spin_stopped {
        font-size: "48px";
        display: "inline-block";
        opacity: "0.4";
        media("(max-width: 767px)") {
            font-size: "36px";
        }
    }

    pub(crate) c_anim_pulse_container {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        margin: "24px 0";
        min-height: "80px";
    }

    pub(crate) c_anim_pulse {
        font-size: "48px";
        animation: "euv-pulse 1.5s ease-in-out infinite";
        display: "inline-block";
        color: "#dc2626";
        media("(max-width: 767px)") {
            font-size: "36px";
        }
    }

    pub(crate) c_anim_pulse_stopped {
        font-size: "48px";
        display: "inline-block";
        opacity: "0.4";
        color: "#dc2626";
        media("(max-width: 767px)") {
            font-size: "36px";
        }
    }

    pub(crate) c_progress_container {
        width: "100%";
        height: "12px";
        background: var!(bg-progress);
        border-radius: "999px";
        margin: "16px 0 8px 0";
        overflow: "hidden";
    }

    pub(crate) c_progress_bar {
        height: "100%";
        border-radius: "999px";
        transition: "width 0.05s linear";
    }

    pub(crate) c_progress_text {
        text-align: "center";
        font-size: "14px";
        color: "inherit";
        font-weight: "500";
    }

    pub(crate) c_anim_color_box {
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

    pub(crate) c_event_drag_zone {
        min-height: "100px";
        padding: "20px";
        border-radius: "8px";
        border: format!("2px dashed {}", var!(border-warning));
        text-align: "center";
        user-select: "none";
        cursor: "grab";
        color: var!(text-warning);
    }

    pub(crate) c_event_drag_zone_active {
        min-height: "100px";
        padding: "20px";
        border-radius: "8px";
        border: format!("2px dashed {}", var!(border-success));
        text-align: "center";
        user-select: "none";
        color: var!(text-success);
    }

    pub(crate) c_event_drag_item {
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

    pub(crate) c_event_wheel_zone {
        min-height: "120px";
        padding: "20px";
        border-radius: "8px";
        border: format!("2px dashed {}", var!(border-success));
        text-align: "center";
        overflow: "auto";
        color: var!(text-success);
    }

    pub(crate) c_event_clipboard_area {
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-pink));
        color: var!(text-pink);
    }

    pub(crate) c_event_touch_zone {
        min-height: "120px";
        padding: "20px";
        border-radius: "8px";
        border: "2px dashed #8b5cf6";
        text-align: "center";
        touch-action: "none";
        user-select: "none";
        color: "#5b21b6";
    }

    pub(crate) c_event_form_area {
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-info));
        color: var!(text-info);
    }

    pub(crate) c_event_media_area {
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-success));
        color: var!(text-success);
        max-width: "100%";
        overflow: "hidden";
    }

    pub(crate) c_event_audio {
        width: "100%";
        max-width: "100%";
    }

    pub(crate) c_event_section_row {
        display: "flex";
        gap: "24px";
        flex-wrap: "wrap";
        margin-top: "12px";
        media("(max-width: 767px)") {
            gap: "12px";
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_event_section_col {
        flex: "1";
        min-width: "200px";
        media("(max-width: 767px)") {
            min-width: "100%";
        }
    }

    pub(crate) c_event_section_label {
        font-size: "13px";
        color: "inherit";
        opacity: "0.6";
        margin-bottom: "4px";
    }

    pub(crate) c_event_section_value {
        font-size: "14px";
        color: "inherit";
        font-weight: "500";
    }

    pub(crate) c_browser_api_row {
        display: "grid";
        grid-template-columns: "1fr 1fr";
        gap: "0 20px";
        margin-bottom: "12px";
        media("(max-width: 767px)") {
            grid-template-columns: "1fr";
        }
    }

    pub(crate) c_browser_api_actions {
        display: "flex";
        gap: "10px";
        flex-wrap: "wrap";
        margin: "12px 0";
    }

    pub(crate) c_browser_result_box {
        margin-top: "12px";
        padding: "14px 16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-accent-light));
        font-size: "14px";
        word-break: "break-all";
        color: "inherit";
    }

    pub(crate) c_browser_result_label {
        font-weight: "600";
        color: "#4f46e5";
    }

    pub(crate) c_browser_result_value {
        color: "inherit";
    }

    pub(crate) c_browser_info_grid {
        display: "grid";
        gap: "12px";
        margin-top: "12px";
        media("(max-width: 767px)") {
            grid-template-columns: "1fr";
        }
    }

    pub(crate) c_browser_info_item {
        display: "flex";
        flex-direction: "column";
        gap: "4px";
        padding: "12px 16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-input));
        color: "inherit";
    }

    pub(crate) c_browser_info_label {
        font-size: "12px";
        font-weight: "600";
        color: "inherit";
        opacity: "0.6";
        text-transform: "uppercase";
        letter-spacing: "0.05em";
    }

    pub(crate) c_browser_info_value {
        font-size: "14px";
        color: "inherit";
        word-break: "break-all";
        font-family: "ui-monospace, monospace";
    }

    pub(crate) c_vconsole_button {
        position: "fixed";
        bottom: "20px";
        right: "20px";
        width: "48px";
        height: "48px";
        border-radius: "14px";
        background: var!(bg-console-button);
        color: var!(text-console-button-text);
        border: "none";
        cursor: "pointer";
        font-size: "18px";
        font-weight: "700";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        z-index: "9999";
        box-shadow: var!(shadow-console-button);
        transition: "transform 0.2s ease, box-shadow 0.2s ease, background 0.2s ease";
        padding: "0";
        line-height: "1";
        letter-spacing: "-0.02em";
        media("(max-width: 767px)") {
            bottom: "16px";
            right: "16px";
            width: "44px";
            height: "44px";
            border-radius: "12px";
        }
    }

    pub(crate) c_vconsole_button_hover {
        transform: "scale(1.08)";
        background: var!(bg-console-button-hover);
        box-shadow: "0 6px 20px rgba(99, 102, 241, 0.4)";
    }

    pub(crate) c_vconsole_badge {
        position: "absolute";
        top: "-6px";
        right: "-8px";
        min-width: "18px";
        height: "18px";
        border-radius: "9px";
        background: var!(bg-console-badge);
        color: "white";
        font-size: "10px";
        font-weight: "600";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        padding: "0 4px";
        line-height: "1";
        box-shadow: "0 2px 6px rgba(239, 68, 68, 0.3)";
        pointer-events: "none";
    }

    pub(crate) c_vconsole_overlay {
        position: "fixed";
        top: "0";
        left: "0";
        right: "0";
        bottom: "0";
        z-index: "10000";
        background: "rgba(0, 0, 0, 0.15)";
        transition: "opacity 0.25s ease";
    }

    pub(crate) c_vconsole_overlay_hidden {
        opacity: "0";
        pointer-events: "none";
    }

    pub(crate) c_vconsole_panel {
        position: "fixed";
        bottom: "0";
        left: "0";
        right: "0";
        height: "60vh";
        background: var!(bg-console);
        z-index: "10001";
        display: "flex";
        flex-direction: "column";
        border-top: format!("3px solid {}", var!(border-console-accent));
        border-radius: "16px 16px 0 0";
        box-shadow: var!(shadow-console-panel);
        transition: "transform 0.25s ease";
        overflow: "hidden";
    }

    pub(crate) c_vconsole_panel_closed {
        transform: "translateY(100%)";
    }

    pub(crate) c_vconsole_fab_hidden {
        display: "none";
    }

    pub(crate) c_vconsole_header {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        padding: "14px 20px";
        background: var!(bg-console-header);
        border-bottom: format!("1px solid {}", var!(border-console));
        flex-shrink: "0";
    }

    pub(crate) c_vconsole_title {
        color: var!(text-console-title);
        font-size: "15px";
        font-weight: "600";
        margin: "0";
        letter-spacing: "0.03em";
        display: "flex";
        align-items: "center";
        gap: "6px";
    }

    pub(crate) c_vconsole_title_dot {
        width: "8px";
        height: "8px";
        border-radius: "4px";
        background: var!(text-console-log-latest);
        display: "inline-block";
        animation: "euv-pulse 2s ease-in-out infinite";
    }

    pub(crate) c_vconsole_header_actions {
        display: "flex";
        gap: "8px";
        align-items: "center";
    }

    pub(crate) c_vconsole_clear_button {
        background: "transparent";
        border: format!("1px solid {}", var!(border-console-button));
        color: var!(text-console-button);
        padding: "5px 14px";
        border-radius: "6px";
        cursor: "pointer";
        font-size: "12px";
        font-weight: "500";
        transition: "all 0.15s ease";
        letter-spacing: "0.02em";
    }

    pub(crate) c_vconsole_clear_button_hover {
        border-color: var!(text-console-warn);
        color: var!(text-console-warn);
        background: "rgba(251, 191, 36, 0.08)";
    }

    pub(crate) c_vconsole_close_button {
        background: "transparent";
        border: format!("1px solid {}", var!(border-console-button));
        color: var!(text-console-button);
        padding: "5px 12px";
        border-radius: "6px";
        cursor: "pointer";
        font-size: "16px";
        line-height: "1";
        transition: "all 0.15s ease";
    }

    pub(crate) c_vconsole_close_button_hover {
        border-color: var!(text-console-close-hover);
        color: var!(text-console-close-hover);
        background: var!(bg-console-close-hover);
    }

    pub(crate) c_vconsole_body {
        flex: "1";
        overflow-y: "auto";
        padding: "12px 16px";
        font-family: "ui-monospace, monospace";
        font-size: "13px";
        line-height: "1.6";
    }

    pub(crate) c_vconsole_log_item {
        padding: "4px 0";
        border-bottom: format!("1px solid {}", var!(border-console));
        color: var!(text-console);
        font-size: "13px";
        word-break: "break-all";
    }

    pub(crate) c_vconsole_log_latest {
        color: var!(text-console-log-latest);
        font-weight: "500";
    }

    pub(crate) c_vconsole_log_warn {
        color: var!(text-console-warn);
    }

    pub(crate) c_vconsole_log_warn_latest {
        color: var!(text-console-warn-latest);
        font-weight: "500";
    }

    pub(crate) c_vconsole_log_error {
        color: var!(text-console-error);
    }

    pub(crate) c_vconsole_log_error_latest {
        color: var!(text-console-error-latest);
        font-weight: "500";
    }

    pub(crate) c_vconsole_empty {
        color: var!(text-console-empty);
        font-size: "13px";
        text-align: "center";
        padding: "40px 0";
    }

    pub(crate) c_vconsole_count {
        color: var!(text-console-empty);
        font-size: "12px";
        font-weight: "400";
    }

    pub(crate) c_vconsole_filter_bar {
        display: "flex";
        gap: "6px";
        padding: "10px 20px";
        background: var!(bg-console-filter);
        border-bottom: format!("1px solid {}", var!(border-console));
        flex-shrink: "0";
    }

    pub(crate) c_vconsole_filter_button {
        padding: "4px 12px";
        border-radius: "6px";
        background: "transparent";
        border: format!("1px solid {}", var!(border-console-button));
        color: var!(text-console-button);
        font-size: "11px";
        font-weight: "500";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
        transition: "all 0.15s ease";
    }

    pub(crate) c_vconsole_filter_active {
        padding: "4px 12px";
        border-radius: "6px";
        background: var!(bg-console-filter-active);
        border: format!("1px solid {}", var!(border-console-filter-active));
        color: var!(text-console-filter-active);
        font-size: "11px";
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
    }

    pub(crate) c_vconsole_filter_active_log {
        padding: "4px 12px";
        border-radius: "6px";
        background: var!(bg-console-badge-log);
        border: format!("1px solid {}", var!(text-console-log-latest));
        color: var!(text-console-log-latest);
        font-size: "11px";
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
    }

    pub(crate) c_vconsole_filter_active_warn {
        padding: "4px 12px";
        border-radius: "6px";
        background: var!(bg-console-badge-warn);
        border: format!("1px solid {}", var!(text-console-warn-latest));
        color: var!(text-console-warn-latest);
        font-size: "11px";
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
    }

    pub(crate) c_vconsole_filter_active_error {
        padding: "4px 12px";
        border-radius: "6px";
        background: var!(bg-console-badge-error);
        border: format!("1px solid {}", var!(text-console-error-latest));
        color: var!(text-console-error-latest);
        font-size: "11px";
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
    }

    pub(crate) c_vconsole_level_badge {
        display: "inline-block";
        padding: "2px 7px";
        border-radius: "4px";
        font-size: "10px";
        font-weight: "600";
        margin-right: "8px";
        letter-spacing: "0.05em";
        color: "inherit";
    }

    pub(crate) c_vconsole_badge_log {
        background: var!(bg-console-badge-log);
    }

    pub(crate) c_vconsole_badge_warn {
        background: var!(bg-console-badge-warn);
    }

    pub(crate) c_vconsole_badge_error {
        background: var!(bg-console-badge-error);
    }

    pub(crate) c_nav_item_active {
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

    pub(crate) c_nav_item_inactive {
        display: "block";
        padding: "11px 20px";
        text-decoration: "none";
        font-size: "14px";
        transition: "all 0.15s ease";
        color: var!(text-nav-item);
        font-weight: "400";
        border-left: "3px solid transparent";
        background: "transparent";
        hover {
            background: "rgba(79, 70, 229, 0.04)";
            color: "#4f46e5";
        }
    }

    pub(crate) c_mobile_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        padding: "0 16px";
        height: "56px";
        background: var!(bg-nav);
        flex-shrink: "0";
        position: "sticky";
        top: "0";
        z-index: "100";
    }

    pub(crate) c_mobile_header_left {
        display: "flex";
        align-items: "center";
        gap: "10px";
    }

    pub(crate) c_mobile_menu_button {
        width: "40px";
        height: "40px";
        border-radius: "8px";
        background: "transparent";
        border: "none";
        cursor: "pointer";
        font-size: "22px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        color: var!(text-primary);
        transition: "background 0.15s ease";
        padding: "0";
    }

    pub(crate) c_mobile_menu_button_active {
        width: "40px";
        height: "40px";
        border-radius: "8px";
        background: "rgba(79, 70, 229, 0.08)";
        border: "none";
        cursor: "pointer";
        font-size: "22px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        color: "#4f46e5";
        transition: "background 0.15s ease";
        padding: "0";
    }

    pub(crate) c_mobile_overlay {
        position: "fixed";
        top: "0";
        left: "0";
        width: "100%";
        height: "100%";
        background: "rgba(0, 0, 0, 0.4)";
        z-index: "200";
        animation: "euv-fade-in 0.15s ease";
    }

    pub(crate) c_mobile_nav_drawer {
        position: "fixed";
        top: "0";
        left: "0";
        width: "280px";
        height: "100%";
        background: var!(bg-nav);
        z-index: "201";
        display: "flex";
        flex-direction: "column";
        animation: "euv-slide-left 0.25s ease";
        box-shadow: "4px 0 20px rgba(0, 0, 0, 0.15)";
    }

    pub(crate) c_mobile_nav_drawer_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        padding: "16px 20px";
        border-bottom: format!("1px solid {}", var!(border-subtle));
    }

    pub(crate) c_mobile_nav_drawer_close {
        width: "32px";
        height: "32px";
        border-radius: "6px";
        background: "transparent";
        border: "none";
        cursor: "pointer";
        font-size: "18px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        color: var!(text-muted);
        transition: "background 0.15s ease";
        padding: "0";
    }

    pub(crate) c_mobile_app_root {
        display: "flex";
        flex-direction: "column";
        height: "100vh";
        overflow: "hidden";
        font-family: "system-ui, -apple-system, sans-serif";
        background: var!(bg-primary);
        color: var!(text-primary);
        line-height: "1.6";
    }

    pub(crate) c_mobile_main {
        flex: "1";
        padding: "20px 16px";
        background: var!(bg-primary);
        overflow-y: "auto";
    }

    pub(crate) c_keep_alive_tab_bar {
        display: "flex";
        border-bottom: format!("1px solid {}", var!(border-input));
        margin-bottom: "16px";
        gap: "4px";
    }

    pub(crate) c_keep_alive_tab_panel {
        padding: "4px 0";
    }

    pub(crate) c_keep_alive_panel_title {
        margin-top: "0";
        color: "#4f46e5";
        font-size: "15px";
        margin-bottom: "8px";
    }

    pub(crate) c_keep_alive_demo_text {
        color: "inherit";
        font-size: "14px";
        margin-bottom: "12px";
    }

    pub(crate) c_keep_alive_counter_display {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        margin: "20px 0";
    }

    pub(crate) c_keep_alive_counter_value {
        font-size: "48px";
        font-weight: "700";
        font-variant-numeric: "tabular-nums";
        color: "#4f46e5";
        min-width: "120px";
        text-align: "center";
    }

    pub(crate) c_keep_alive_counter_controls {
        display: "flex";
        gap: "10px";
        justify-content: "center";
        media("(max-width: 767px)") {
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_keep_alive_form_group {
        margin-bottom: "16px";
    }

    pub(crate) c_keep_alive_form_preview {
        margin-top: "16px";
        padding: "12px 16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-accent-light));
        background: "rgba(79, 70, 229, 0.04)";
    }

    pub(crate) c_keep_alive_preview_label {
        font-size: "13px";
        font-weight: "600";
        color: "#4f46e5";
        margin: "0 0 4px 0";
    }

    pub(crate) c_file_upload_input {
        width: "100%";
        padding: "10px 14px";
        border: format!("1px solid {}", var!(border-input));
        border-radius: "8px";
        font-size: "14px";
        box-sizing: "border-box";
        background: var!(bg-input);
        color: var!(text-primary);
        cursor: "pointer";
    }

    pub(crate) c_file_upload_input_hidden {
        position: "absolute";
        width: "1px";
        height: "1px";
        padding: "0";
        margin: "-1px";
        overflow: "hidden";
        clip: "rect(0, 0, 0, 0)";
        white-space: "nowrap";
        border: "0";
    }

    pub(crate) c_file_upload_options {
        margin: "12px 0";
        padding: "12px 0";
        border-top: format!("1px solid {}", var!(border-subtle));
        border-bottom: format!("1px solid {}", var!(border-subtle));
    }

    pub(crate) c_file_upload_item {
        display: "flex";
        align-items: "center";
        gap: "10px";
        padding: "8px 12px";
        border-radius: "6px";
        background: var!(bg-input);
        margin: "6px 0";
        font-size: "14px";
        color: "inherit";
    }

    pub(crate) c_file_upload_item_index {
        font-size: "12px";
        font-weight: "600";
        color: "#4f46e5";
        min-width: "24px";
    }

    pub(crate) c_file_upload_item_name {
        color: "inherit";
        word-break: "break-all";
    }

    pub(crate) c_file_upload_drop_zone {
        border: format!("2px dashed {}", var!(border-input));
        border-radius: "12px";
        padding: "40px 20px";
        text-align: "center";
        cursor: "pointer";
        transition: "all 0.2s ease";
        background: "transparent";
    }

    pub(crate) c_file_upload_drop_zone_active {
        border: "2px dashed #4f46e5";
        border-radius: "12px";
        padding: "40px 20px";
        text-align: "center";
        cursor: "pointer";
        transition: "all 0.2s ease";
        background: "rgba(79, 70, 229, 0.06)";
    }

    pub(crate) c_file_upload_drop_icon {
        font-size: "48px";
        display: "block";
        margin-bottom: "12px";
    }

    pub(crate) c_file_upload_drop_text {
        font-size: "16px";
        font-weight: "500";
        color: "inherit";
        margin: "0 0 6px 0";
    }

    pub(crate) c_file_upload_drop_hint {
        font-size: "13px";
        color: "inherit";
        opacity: "0.5";
        margin: "0";
    }

    pub(crate) c_form_input_error {
        width: "100%";
        padding: "10px 14px";
        border: "1px solid #ef4444";
        border-radius: "8px";
        font-size: "14px";
        box-sizing: "border-box";
        background: var!(bg-input);
        outline: "none";
        resize: "vertical";
        overflow-x: "hidden";
        word-wrap: "break-word";
        font-family: "inherit";
        line-height: "1.5";
        color: var!(text-primary);
    }

    pub(crate) c_list_input_error {
        flex: "1";
        padding: "10px 14px";
        border: "1px solid #ef4444";
        border-radius: "8px";
        font-size: "14px";
        background: var!(bg-input);
        outline: "none";
        color: var!(text-primary);
    }

    pub(crate) c_field_error_text {
        color: var!(text-error);
        font-size: "13px";
        margin-top: "4px";
        margin-bottom: "8px";
    }

    pub(crate) c_binding_child_box {
        margin-top: "16px";
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-accent-light));
        background: "rgba(79, 70, 229, 0.04)";
    }

    pub(crate) c_binding_child_label {
        font-size: "13px";
        font-weight: "600";
        color: "#4f46e5";
        margin-bottom: "8px";
        text-transform: "uppercase";
        letter-spacing: "0.05em";
    }

    pub(crate) c_binding_child_message {
        color: "inherit";
        font-size: "15px";
        margin-bottom: "12px";
        padding: "8px 12px";
        border-radius: "6px";
        background: "rgba(79, 70, 229, 0.08)";
        font-weight: "500";
    }

    pub(crate) c_binding_parent_box {
        margin-bottom: "16px";
        padding: "16px";
        border-radius: "8px";
        border: format!("1px solid {}", var!(border-success));
        background: "rgba(22, 101, 52, 0.04)";
    }

    pub(crate) c_binding_section_title {
        margin-top: "20px";
        margin-bottom: "8px";
        color: "#4f46e5";
        font-size: "15px";
        font-weight: "600";
    }

    pub(crate) c_binding_temp_converter {
        display: "flex";
        align-items: "flex-end";
        gap: "12px";
        flex-wrap: "wrap";
        margin-top: "12px";
    }

    pub(crate) c_binding_temp_field {
        flex: "1";
        min-width: "120px";
    }

    pub(crate) c_binding_temp_arrow {
        font-size: "20px";
        font-weight: "700";
        color: "#4f46e5";
        padding-bottom: "10px";
    }

    pub(crate) c_binding_color_mixer {
        margin-top: "12px";
    }

    pub(crate) c_binding_color_preview {
        width: "100%";
        height: "80px";
        border-radius: "10px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        margin-bottom: "16px";
        border: format!("1px solid {}", var!(border-subtle));
    }

    pub(crate) c_binding_color_hex {
        font-family: "ui-monospace, monospace";
        font-size: "18px";
        font-weight: "700";
        color: "white";
        text-shadow: "0 1px 3px rgba(0, 0, 0, 0.5)";
        letter-spacing: "0.02em";
    }

    pub(crate) c_binding_slider_row {
        display: "flex";
        align-items: "center";
        gap: "10px";
        margin-bottom: "8px";
    }

    pub(crate) c_binding_slider_label {
        font-size: "14px";
        font-weight: "700";
        min-width: "16px";
    }

    pub(crate) c_binding_slider {
        flex: "1";
        height: "6px";
        cursor: "pointer";
        accent-color: "#4f46e5";
        will-change: "auto";
        touch-action: "pan-x";
    }

    pub(crate) c_binding_slider_value {
        font-size: "13px";
        font-weight: "500";
        color: "inherit";
        min-width: "32px";
        text-align: "right";
        font-family: "ui-monospace, monospace";
    }

    pub(crate) c_binding_typed_prop_value {
        font-family: "ui-monospace, monospace";
        font-size: "13px";
        font-weight: "600";
        color: "#4f46e5";
        padding: "1px 6px";
        border-radius: "4px";
        background: "rgba(79, 70, 229, 0.08)";
    }

    pub(crate) c_binding_typed_prop_group {
        display: "flex";
        align-items: "center";
        gap: "8px";
    }

    pub(crate) c_binding_typed_warning {
        font-size: "13px";
        font-weight: "500";
        color: "#dc2626";
        margin-top: "10px";
        padding: "6px 10px";
        border-radius: "6px";
        background: "rgba(220, 38, 38, 0.08)";
        border: "1px solid rgba(220, 38, 38, 0.2)";
        margin-bottom: "0px";
    }

    pub(crate) c_binding_callback_form {
        margin-top: "8px";
        display: "flex";
        flex-direction: "column";
        gap: "10px";
    }
}
