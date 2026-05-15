use crate::*;

class! {
    pub c_app_root {
        display: "flex";
        height: "100vh";
        overflow: "hidden";
        font_family: "system-ui, -apple-system, sans-serif";
        background: "var(--bg-primary)";
        color: "var(--text-primary)";
        line_height: "1.6";
    }

    pub c_app_nav {
        width: "240px";
        background: "var(--bg-nav)";
        border_right: "1px solid var(--border-nav)";
        display: "flex";
        flex_direction: "column";
        height: "100vh";
        overflow_y: "auto";
        flex_shrink: "0";
    }

    pub c_nav_header {
        padding: "24px 20px 20px 20px";
        margin: "0";
        border_bottom: "1px solid var(--border-subtle)";
        font_size: "18px";
        font_weight: "700";
        color: "var(--text-primary)";
        letter_spacing: "-0.02em";
        display: "flex";
        align_items: "center";
        gap: "10px";
    }

    pub c_nav_logo {
        display: "flex";
        width: "32px";
        height: "32px";
        background: "linear-gradient(135deg, #4f46e5, #7c3aed)";
        border_radius: "8px";
        align_items: "center";
        justify_content: "center";
        color: "white";
        font_weight: "700";
        font_size: "16px";
        flex_shrink: "0";
    }

    pub c_nav_section_label {
        padding: "10px 20px 4px 20px";
        margin: "0";
        font_size: "11px";
        font_weight: "600";
        color: "var(--text-muted)";
        text_transform: "uppercase";
        letter_spacing: "0.08em";
    }

    pub c_nav_theme_toggle {
        padding: "12px 20px";
        margin_top: "auto";
    }

    pub c_nav_theme_button {
        width: "100%";
        padding: "8px 16px";
        border_radius: "8px";
        cursor: "pointer";
        font_size: "16px";
        text_align: "center";
        outline: "none";
        background: "var(--bg-theme-button)";
        color: "var(--text-theme-button)";
        border: "1px solid var(--border-theme-button)";
        transition: "all 0.3s ease";
    }

    pub c_nav_footer {
        padding: "16px 20px";
        border_top: "1px solid var(--border-subtle)";
        font_size: "12px";
        color: "var(--text-muted)";
    }

    pub c_app_main {
        flex: "1";
        padding: "32px 40px";
        background: "var(--bg-primary)";
        overflow_y: "auto";
        max_width: "960px";
    }

    pub c_page_container {
        max_width: "800px";
        margin: "0 auto";
    }

    pub c_page_header {
        margin_bottom: "32px";
    }

    pub c_page_title {
        font_size: "28px";
        font_weight: "700";
        color: "inherit";
        margin_bottom: "8px";
        letter_spacing: "-0.02em";
    }

    pub c_page_subtitle {
        font_size: "16px";
        color: "inherit";
        opacity: "0.6";
        line_height: "1.6";
    }

    pub c_card {
        background: "var(--bg-card)";
        border_radius: "12px";
        padding: "24px";
        margin: "16px 0";
        box_shadow: "var(--shadow-card)";
        border: "1px solid var(--border-card)";
        color: "var(--text-card)";
        transition: "box-shadow 0.2s ease, background 0.4s ease, border-color 0.4s ease, color 0.4s ease";
    }

    pub c_card_title {
        margin_top: "0";
        margin_bottom: "16px";
        color: "inherit";
        font_size: "18px";
        font_weight: "600";
        padding_bottom: "12px";
        border_bottom: "2px solid var(--border-card-title)";
        letter_spacing: "0.01em";
        transition: "border-color 0.4s ease";
    }

    pub c_primary_button {
        background: "#4f46e5";
        color: "white";
        border: "none";
        padding: "10px 22px";
        border_radius: "8px";
        cursor: "pointer";
        font_size: "14px";
        font_weight: "500";
        letter_spacing: "0.01em";
        transition: "all 0.15s ease";
        outline: "none";
    }

    pub c_badge {
        display: "inline-block";
        color: "white";
        padding: "5px 14px";
        border_radius: "20px";
        font_size: "12px";
        font_weight: "600";
        cursor: "pointer";
        letter_spacing: "0.03em";
        text_transform: "uppercase";
    }

    pub c_badge_outline {
        display: "inline-block";
        padding: "5px 14px";
        border_radius: "20px";
        font_size: "12px";
        font_weight: "600";
        cursor: "pointer";
        letter_spacing: "0.03em";
        text_transform: "uppercase";
        background: "transparent";
        border: "1.5px solid";
    }

    pub c_form_input_wrapper {
        margin: "12px 0";
    }

    pub c_form_label {
        display: "block";
        margin_bottom: "6px";
        color: "inherit";
        font_weight: "500";
        font_size: "14px";
    }

    pub c_form_input {
        width: "100%";
        padding: "10px 14px";
        border: "1px solid var(--border-input)";
        border_radius: "8px";
        font_size: "14px";
        box_sizing: "border-box";
        transition: "border-color 0.15s ease, box-shadow 0.15s ease";
        outline: "none";
        background: "var(--bg-input)";
        color: "var(--text-primary)";
    }

    pub c_form_input_no_transition {
        width: "100%";
        padding: "10px 14px";
        border: "1px solid var(--border-input)";
        border_radius: "8px";
        font_size: "14px";
        box_sizing: "border-box";
        background: "var(--bg-input)";
        outline: "none";
        color: "var(--text-primary)";
    }

    pub c_form_checkbox {
        cursor: "pointer";
        width: "16px";
        height: "16px";
    }

    pub c_form_checkbox_label {
        font_size: "14px";
        color: "inherit";
        cursor: "pointer";
    }

    pub c_form_checkbox_row {
        margin: "16px 0";
        display: "flex";
        align_items: "center";
        gap: "10px";
    }

    pub c_error_box {
        margin: "16px 0";
        padding: "14px 16px";
        background: "var(--bg-error)";
        color: "var(--text-error)";
        border_radius: "8px";
        font_size: "14px";
        border: "1px solid var(--border-error)";
    }

    pub c_success_box {
        margin_top: "16px";
        padding: "14px 16px";
        background: "var(--bg-success)";
        color: "var(--text-success)";
        border_radius: "8px";
        border: "1px solid var(--border-success)";
        font_size: "14px";
    }

    pub c_counter_row {
        display: "flex";
        align_items: "center";
        gap: "20px";
        flex_wrap: "wrap";
    }

    pub c_counter_text {
        font_size: "16px";
        color: "inherit";
    }

    pub c_counter_value {
        font_weight: "700";
        color: "#4f46e5";
        font_size: "24px";
    }

    pub c_badge_row {
        display: "flex";
        gap: "10px";
        flex_wrap: "wrap";
        align_items: "center";
    }

    pub c_badge_hint {
        margin_bottom: "12px";
        color: "inherit";
        opacity: "0.7";
        font_size: "14px";
    }

    pub c_form_grid {
        display: "grid";
        grid_template_columns: "1fr 1fr";
        gap: "0 20px";
    }

    pub c_custom_attrs_demo {
        padding: "16px";
        border_radius: "8px";
        border: "1px solid var(--border-input)";
        color: "inherit";
    }

    pub c_demo_text {
        color: "inherit";
        margin_bottom: "8px";
    }

    pub c_demo_text_muted {
        color: "inherit";
        opacity: "0.6";
        font_size: "13px";
    }

    pub c_loading_container {
        display: "flex";
        align_items: "center";
        gap: "16px";
        padding: "20px";
        background: "var(--bg-loading)";
        border_radius: "10px";
        border: "1px solid var(--border-loading)";
        margin_top: "16px";
    }

    pub c_spinner {
        width: "28px";
        height: "28px";
        border: "3px solid var(--border-loading)";
        border_top: "3px solid #4f46e5";
        border_radius: "50%";
        flex_shrink: "0";
        animation: "euv-spin 0.8s linear infinite";
    }

    pub c_loading_text_col {
        display: "flex";
        flex_direction: "column";
        gap: "4px";
    }

    pub c_loading_title {
        color: "var(--text-loading-title)";
        font_size: "15px";
        font_weight: "500";
    }

    pub c_loading_subtitle {
        color: "var(--text-muted)";
        font_size: "13px";
    }

    pub c_error_container {
        display: "flex";
        align_items: "center";
        gap: "12px";
        margin_top: "16px";
        padding: "16px";
        background: "var(--bg-error)";
        border_radius: "10px";
        border: "1px solid var(--border-error)";
    }

    pub c_error_icon {
        width: "20px";
        height: "20px";
        border_radius: "50%";
        background: "var(--bg-error-icon)";
        display: "flex";
        align_items: "center";
        justify_content: "center";
        flex_shrink: "0";
        color: "var(--text-error)";
        font_size: "12px";
        font_weight: "700";
    }

    pub c_error_text {
        color: "var(--text-error)";
        font_size: "14px";
    }

    pub c_data_box {
        margin_top: "16px";
        padding: "16px";
        background: "var(--bg-success)";
        border_radius: "10px";
        border: "1px solid var(--border-success)";
    }

    pub c_data_pre {
        color: "var(--text-success)";
        font_size: "13px";
        margin: "0";
        white_space: "pre-wrap";
        word_break: "break-all";
        font_family: "ui-monospace, monospace";
    }

    pub c_fetch_hint {
        color: "inherit";
        opacity: "0.7";
        font_size: "14px";
        margin_bottom: "16px";
    }

    pub c_toggle_content {
        margin_top: "16px";
        padding: "16px";
        border_radius: "8px";
        border: "1px solid var(--border-accent-light)";
        color: "inherit";
    }

    pub c_toggle_title {
        margin_top: "0";
        color: "#4f46e5";
        font_size: "15px";
    }

    pub c_role_button_row {
        display: "flex";
        gap: "10px";
        margin_bottom: "16px";
    }

    pub c_role_guest {
        padding: "16px";
        border_radius: "8px";
        border: "1px solid var(--border-warning)";
    }

    pub c_role_guest_text {
        color: "var(--text-warning)";
        font_size: "14px";
    }

    pub c_role_user {
        padding: "16px";
        border_radius: "8px";
        border: "1px solid var(--border-info)";
    }

    pub c_role_user_text {
        color: "var(--text-info)";
        font_size: "14px";
    }

    pub c_role_admin {
        padding: "16px";
        border_radius: "8px";
        border: "1px solid var(--border-success)";
    }

    pub c_role_admin_text {
        color: "var(--text-success)";
        font_size: "14px";
    }

    pub c_tab_bar {
        display: "flex";
        border_bottom: "1px solid var(--border-input)";
        margin_bottom: "16px";
        gap: "4px";
    }

    pub c_tab_content {
        padding: "4px 0";
    }

    pub c_tab_text {
        color: "inherit";
        font_size: "14px";
        margin_bottom: "8px";
    }

    pub c_tab_text_muted {
        color: "inherit";
        opacity: "0.6";
        font_size: "13px";
    }

    pub c_tab_text_input {
        color: "inherit";
        font_size: "14px";
        margin_bottom: "12px";
    }

    pub c_list_input_row {
        display: "flex";
        gap: "12px";
        margin_bottom: "20px";
    }

    pub c_list_input {
        flex: "1";
        padding: "10px 14px";
        border: "1px solid var(--border-input)";
        border_radius: "8px";
        font_size: "14px";
        background: "var(--bg-input)";
        outline: "none";
        color: "var(--text-primary)";
    }

    pub c_list_ul {
        list_style: "none";
        padding: "0";
        margin: "0";
    }

    pub c_list_item_even {
        display: "flex";
        justify_content: "space-between";
        align_items: "center";
        padding: "12px 16px";
        border_bottom: "1px solid var(--border-subtle)";
        border_radius: "6px";
        margin_bottom: "8px";
        gap: "16px";
        background: "var(--bg-list-even)";
    }

    pub c_list_item_odd {
        display: "flex";
        justify_content: "space-between";
        align_items: "center";
        padding: "12px 16px";
        border_bottom: "1px solid var(--border-subtle)";
        border_radius: "6px";
        margin_bottom: "8px";
        gap: "16px";
        background: "var(--bg-list-odd)";
    }

    pub c_list_item_text {
        flex: "1";
        font_size: "14px";
        color: "inherit";
    }

    pub c_log_container {
        max_height: "320px";
        overflow_y: "auto";
        background: "var(--bg-console)";
        padding: "16px";
        border_radius: "8px";
        font_family: "ui-monospace, monospace";
        font_size: "13px";
        line_height: "1.5";
        margin_top: "16px";
        color: "var(--text-console)";
    }

    pub c_log_item {
        padding: "6px 0";
        border_bottom: "1px solid var(--border-console)";
        font_size: "13px";
        color: "var(--text-console)";
    }

    pub c_not_found_container {
        text_align: "center";
        padding: "80px 20px";
        max_width: "480px";
        margin: "0 auto";
    }

    pub c_not_found_code {
        font_size: "72px";
        font_weight: "800";
        color: "var(--text-muted)";
        line_height: "1";
        margin_bottom: "16px";
        letter_spacing: "-0.04em";
    }

    pub c_not_found_title {
        font_size: "24px";
        font_weight: "600";
        color: "inherit";
        margin_bottom: "12px";
    }

    pub c_not_found_text {
        color: "inherit";
        opacity: "0.7";
        font_size: "16px";
        margin_bottom: "28px";
        line_height: "1.6";
    }

    pub c_render_count_text {
        font_size: "16px";
        color: "inherit";
        margin_bottom: "16px";
    }

    pub c_inline {
        display: "inline";
    }

    pub c_event_result {
        font_size: "14px";
        color: "inherit";
        margin_top: "12px";
    }

    pub c_event_highlight {
        font_weight: "600";
        color: "#4f46e5";
    }

    pub c_event_stats {
        display: "flex";
        gap: "24px";
        margin_top: "12px";
    }

    pub c_event_mouse_area {
        min_height: "120px";
        padding: "20px";
        border_radius: "8px";
        border: "2px dashed var(--border-accent-light)";
        cursor: "crosshair";
        text_align: "center";
        user_select: "none";
        color: "inherit";
    }

    pub c_log_empty {
        color: "var(--text-muted)";
        font_size: "13px";
        text_align: "center";
        padding: "16px";
    }

    pub c_timer_display {
        text_align: "center";
        padding: "20px";
        margin: "16px 0";
        border_radius: "12px";
        border: "1px solid var(--border-accent-light)";
        color: "inherit";
    }

    pub c_timer_value {
        font_size: "48px";
        font_weight: "700";
        color: "#4f46e5";
        letter_spacing: "0.04em";
        font_family: "ui-monospace, monospace";
    }

    pub c_timer_controls {
        display: "flex";
        gap: "12px";
        justify_content: "center";
    }

    pub c_timer_done {
        text_align: "center";
        padding: "12px";
        margin_top: "12px";
        background: "var(--bg-warning)";
        border_radius: "8px";
        border: "1px solid var(--border-warning)";
        font_size: "16px";
        font_weight: "600";
        color: "var(--text-warning)";
    }

    pub c_modal_overlay {
        position: "fixed";
        top: "0";
        left: "0";
        width: "100%";
        height: "100%";
        background: "rgba(0, 0, 0, 0.5)";
        display: "flex";
        align_items: "center";
        justify_content: "center";
        z_index: "1000";
        animation: "euv-fade-in 0.2s ease";
    }

    pub c_modal_content {
        background: "var(--bg-modal)";
        border_radius: "16px";
        padding: "0";
        max_width: "480px";
        width: "90%";
        box_shadow: "var(--shadow-modal)";
        animation: "euv-scale-in 0.2s ease";
        overflow: "hidden";
        color: "var(--text-card)";
    }

    pub c_modal_header {
        display: "flex";
        justify_content: "space-between";
        align_items: "center";
        padding: "20px 24px";
        border_bottom: "1px solid var(--border-subtle)";
    }

    pub c_modal_title {
        margin: "0";
        font_size: "18px";
        font_weight: "600";
        color: "inherit";
    }

    pub c_modal_body {
        padding: "24px";
        color: "inherit";
    }

    pub c_modal_actions {
        display: "flex";
        gap: "12px";
        justify_content: "flex-end";
        margin_top: "20px";
    }

    pub c_select_input {
        width: "100%";
        padding: "10px 14px";
        border: "1px solid var(--border-input)";
        border_radius: "8px";
        font_size: "14px";
        box_sizing: "border-box";
        background: "var(--bg-input)";
        outline: "none";
        cursor: "pointer";
        appearance: "none";
        color: "var(--text-primary)";
        background_image: "url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%236b7280' d='M6 8L1 3h10z'/%3E%3C/svg%3E\")";
        background_repeat: "no-repeat";
        background_position: "right 14px center";
    }

    pub c_textarea_input {
        width: "100%";
        padding: "10px 14px";
        border: "1px solid var(--border-input)";
        border_radius: "8px";
        font_size: "14px";
        box_sizing: "border-box";
        background: "var(--bg-input)";
        outline: "none";
        resize: "vertical";
        font_family: "inherit";
        line_height: "1.5";
        color: "var(--text-primary)";
    }

    pub c_textarea_counter {
        text_align: "right";
        margin_top: "4px";
    }

    pub c_textarea_counter_text {
        font_size: "12px";
        color: "inherit";
        opacity: "0.5";
    }

    pub c_anim_fade_in {
        margin_top: "16px";
        padding: "20px";
        border_radius: "8px";
        border: "1px solid var(--border-accent-light)";
        animation: "euv-fade-in 0.5s ease";
        font_size: "14px";
        color: "inherit";
    }

    pub c_anim_spin_container {
        display: "flex";
        justify_content: "center";
        align_items: "center";
        margin: "24px 0";
        min_height: "80px";
    }

    pub c_anim_spin {
        font_size: "48px";
        animation: "euv-spin 1.5s linear infinite";
        display: "inline-block";
    }

    pub c_anim_spin_stopped {
        font_size: "48px";
        display: "inline-block";
        opacity: "0.4";
    }

    pub c_anim_pulse_container {
        display: "flex";
        justify_content: "center";
        align_items: "center";
        margin: "24px 0";
        min_height: "80px";
    }

    pub c_anim_pulse {
        font_size: "48px";
        animation: "euv-pulse 1.5s ease-in-out infinite";
        display: "inline-block";
        color: "#dc2626";
    }

    pub c_anim_pulse_stopped {
        font_size: "48px";
        display: "inline-block";
        opacity: "0.4";
        color: "#dc2626";
    }

    pub c_progress_container {
        width: "100%";
        height: "12px";
        background: "var(--bg-progress)";
        border_radius: "999px";
        margin: "16px 0 8px 0";
        overflow: "hidden";
    }

    pub c_progress_bar {
        height: "100%";
        border_radius: "999px";
        transition: "width 0.1s ease";
    }

    pub c_progress_text {
        text_align: "center";
        font_size: "14px";
        color: "inherit";
        font_weight: "500";
    }

    pub c_anim_color_box {
        margin: "16px 0";
        padding: "24px";
        border_radius: "12px";
        color: "white";
        text_align: "center";
        font_weight: "600";
        font_size: "16px";
        cursor: "pointer";
        transition: "background 0.5s ease, transform 0.3s ease";
    }

    pub c_event_drag_zone {
        min_height: "100px";
        padding: "20px";
        border_radius: "8px";
        border: "2px dashed var(--border-warning)";
        text_align: "center";
        user_select: "none";
        cursor: "grab";
        color: "var(--text-warning)";
    }

    pub c_event_drag_zone_active {
        min_height: "100px";
        padding: "20px";
        border_radius: "8px";
        border: "2px dashed var(--border-success)";
        text_align: "center";
        user_select: "none";
        color: "var(--text-success)";
    }

    pub c_event_drag_item {
        display: "inline-block";
        padding: "8px 20px";
        background: "#4f46e5";
        color: "white";
        border_radius: "6px";
        font_size: "14px";
        font_weight: "500";
        cursor: "grab";
        margin: "8px";
    }

    pub c_event_wheel_zone {
        min_height: "120px";
        padding: "20px";
        border_radius: "8px";
        border: "2px dashed var(--border-success)";
        text_align: "center";
        overflow: "auto";
        color: "var(--text-success)";
    }

    pub c_event_clipboard_area {
        padding: "16px";
        border_radius: "8px";
        border: "1px solid var(--border-pink)";
        color: "var(--text-pink)";
    }

    pub c_event_touch_zone {
        min_height: "120px";
        padding: "20px";
        border_radius: "8px";
        border: "2px dashed #8b5cf6";
        text_align: "center";
        touch_action: "none";
        user_select: "none";
        color: "#5b21b6";
    }

    pub c_event_form_area {
        padding: "16px";
        border_radius: "8px";
        border: "1px solid var(--border-info)";
        color: "var(--text-info)";
    }

    pub c_event_media_area {
        padding: "16px";
        border_radius: "8px";
        border: "1px solid var(--border-success)";
        color: "var(--text-success)";
    }

    pub c_event_section_row {
        display: "flex";
        gap: "24px";
        flex_wrap: "wrap";
        margin_top: "12px";
    }

    pub c_event_section_col {
        flex: "1";
        min_width: "200px";
    }

    pub c_event_section_label {
        font_size: "13px";
        color: "inherit";
        opacity: "0.6";
        margin_bottom: "4px";
    }

    pub c_event_section_value {
        font_size: "14px";
        color: "inherit";
        font_weight: "500";
    }

    pub c_browser_api_row {
        display: "grid";
        grid_template_columns: "1fr 1fr";
        gap: "0 20px";
        margin_bottom: "12px";
    }

    pub c_browser_api_actions {
        display: "flex";
        gap: "10px";
        flex_wrap: "wrap";
        margin: "12px 0";
    }

    pub c_browser_result_box {
        margin_top: "12px";
        padding: "14px 16px";
        border_radius: "8px";
        border: "1px solid var(--border-accent-light)";
        font_size: "14px";
        word_break: "break-all";
        color: "inherit";
    }

    pub c_browser_result_label {
        font_weight: "600";
        color: "#4f46e5";
    }

    pub c_browser_result_value {
        color: "inherit";
    }

    pub c_browser_info_grid {
        display: "grid";
        gap: "12px";
        margin_top: "12px";
    }

    pub c_browser_info_item {
        display: "flex";
        flex_direction: "column";
        gap: "4px";
        padding: "12px 16px";
        border_radius: "8px";
        border: "1px solid var(--border-input)";
        color: "inherit";
    }

    pub c_browser_info_label {
        font_size: "12px";
        font_weight: "600";
        color: "inherit";
        opacity: "0.6";
        text_transform: "uppercase";
        letter_spacing: "0.05em";
    }

    pub c_browser_info_value {
        font_size: "14px";
        color: "inherit";
        word_break: "break-all";
        font_family: "ui-monospace, monospace";
    }

    pub c_vconsole_button {
        position: "fixed";
        bottom: "20px";
        right: "20px";
        width: "44px";
        height: "44px";
        border_radius: "50%";
        background: "#4f46e5";
        color: "white";
        border: "none";
        cursor: "pointer";
        font_size: "18px";
        font_weight: "700";
        display: "flex";
        align_items: "center";
        justify_content: "center";
        z_index: "9999";
        box_shadow: "0 4px 14px rgba(79, 70, 229, 0.4)";
        transition: "transform 0.2s ease, box-shadow 0.2s ease";
        padding: "0";
        line_height: "1";
    }

    pub c_vconsole_badge {
        position: "absolute";
        top: "-4px";
        right: "-4px";
        min_width: "18px";
        height: "18px";
        border_radius: "9px";
        background: "#ef4444";
        color: "white";
        font_size: "11px";
        font_weight: "600";
        display: "flex";
        align_items: "center";
        justify_content: "center";
        padding: "0 5px";
        line_height: "1";
    }

    pub c_vconsole_overlay {
        position: "fixed";
        top: "0";
        left: "0";
        right: "0";
        bottom: "50vh";
        z_index: "10000";
    }

    pub c_vconsole_panel {
        position: "fixed";
        bottom: "0";
        left: "0";
        right: "0";
        height: "50vh";
        background: "var(--bg-console)";
        z_index: "10000";
        display: "flex";
        flex_direction: "column";
        border_top: "3px solid #4f46e5";
        border_radius: "12px 12px 0 0";
        animation: "euv-slide-up 0.25s ease";
        overflow: "hidden";
    }

    pub c_vconsole_header {
        display: "flex";
        justify_content: "space-between";
        align_items: "center";
        padding: "12px 16px";
        background: "var(--bg-console-header)";
        border_bottom: "1px solid var(--border-console)";
        flex_shrink: "0";
    }

    pub c_vconsole_title {
        color: "#a5b4fc";
        font_size: "14px";
        font_weight: "600";
        margin: "0";
        letter_spacing: "0.03em";
    }

    pub c_vconsole_header_actions {
        display: "flex";
        gap: "8px";
        align_items: "center";
    }

    pub c_vconsole_clear_button {
        background: "transparent";
        border: "1px solid #4b5563";
        color: "#9ca3af";
        padding: "4px 12px";
        border_radius: "4px";
        cursor: "pointer";
        font_size: "12px";
        transition: "all 0.15s ease";
    }

    pub c_vconsole_close_button {
        background: "transparent";
        border: "1px solid #4b5563";
        color: "#9ca3af";
        padding: "4px 10px";
        border_radius: "4px";
        cursor: "pointer";
        font_size: "16px";
        line_height: "1";
        transition: "all 0.15s ease";
    }

    pub c_vconsole_body {
        flex: "1";
        overflow_y: "auto";
        padding: "12px 16px";
        font_family: "ui-monospace, monospace";
        font_size: "13px";
        line_height: "1.6";
    }

    pub c_vconsole_log_item {
        padding: "4px 0";
        border_bottom: "1px solid var(--border-console)";
        color: "#9ca3af";
        font_size: "13px";
        word_break: "break-all";
    }

    pub c_vconsole_log_latest {
        color: "#34d399";
        font_weight: "500";
    }

    pub c_vconsole_empty {
        color: "#6b7280";
        font_size: "13px";
        text_align: "center";
        padding: "40px 0";
    }

    pub c_vconsole_count {
        color: "#6b7280";
        font_size: "12px";
    }

    pub c_vconsole_filter_bar {
        display: "flex";
        gap: "6px";
        padding: "8px 16px";
        background: "var(--bg-console-filter)";
        border_bottom: "1px solid var(--border-console)";
        flex_shrink: "0";
    }

    pub c_vconsole_filter_button {
        padding: "3px 10px";
        border_radius: "4px";
        background: "transparent";
        border: "1px solid #4b5563";
        color: "#9ca3af";
        font_size: "11px";
        font_weight: "500";
        cursor: "pointer";
        letter_spacing: "0.05em";
        text_transform: "uppercase";
    }

    pub c_vconsole_filter_active {
        padding: "3px 10px";
        border_radius: "4px";
        background: "rgba(167, 139, 250, 0.15)";
        border: "1px solid #7c3aed";
        color: "#a78bfa";
        font_size: "11px";
        font_weight: "600";
        cursor: "pointer";
        letter_spacing: "0.05em";
        text_transform: "uppercase";
    }

    pub c_vconsole_filter_active_log {
        padding: "3px 10px";
        border_radius: "4px";
        background: "rgba(52, 211, 153, 0.15)";
        border: "1px solid #34d399";
        color: "#34d399";
        font_size: "11px";
        font_weight: "600";
        cursor: "pointer";
        letter_spacing: "0.05em";
        text_transform: "uppercase";
    }

    pub c_vconsole_filter_active_warn {
        padding: "3px 10px";
        border_radius: "4px";
        background: "rgba(251, 191, 36, 0.15)";
        border: "1px solid #fbbf24";
        color: "#fbbf24";
        font_size: "11px";
        font_weight: "600";
        cursor: "pointer";
        letter_spacing: "0.05em";
        text_transform: "uppercase";
    }

    pub c_vconsole_filter_active_error {
        padding: "3px 10px";
        border_radius: "4px";
        background: "rgba(248, 113, 113, 0.15)";
        border: "1px solid #f87171";
        color: "#f87171";
        font_size: "11px";
        font_weight: "600";
        cursor: "pointer";
        letter_spacing: "0.05em";
        text_transform: "uppercase";
    }

    pub c_vconsole_level_badge {
        display: "inline-block";
        padding: "1px 6px";
        border_radius: "3px";
        font_size: "10px";
        font_weight: "600";
        margin_right: "6px";
        letter_spacing: "0.05em";
        color: "inherit";
    }

    pub c_nav_item_active {
        display: "block";
        padding: "11px 20px";
        text_decoration: "none";
        font_size: "14px";
        transition: "all 0.15s ease";
        color: "#4f46e5";
        font_weight: "600";
        border_left: "3px solid #4f46e5";
        background: "rgba(79, 70, 229, 0.08)";
    }

    pub c_nav_item_inactive {
        display: "block";
        padding: "11px 20px";
        text_decoration: "none";
        font_size: "14px";
        transition: "all 0.15s ease";
        color: "var(--text-nav-item)";
        font_weight: "400";
        border_left: "3px solid transparent";
        background: "transparent";
    }
}

/// Returns the CSS custom properties string for the given theme.
///
/// # Arguments
///
/// - `&str`: The theme name ("light" or "dark").
///
/// # Returns
///
/// - `String`: The inline style string containing all CSS variable definitions.
pub fn theme_css_variables(theme: &str) -> String {
    if theme == "dark" {
        "--bg-primary: #0f0f1a; \
         --bg-nav: #16162a; \
         --border-nav: #2d2d4a; \
         --border-subtle: #2d2d4a; \
         --text-primary: #e0e0e0; \
         --text-muted: #6b7280; \
         --text-nav-item: #9ca3af; \
         --bg-theme-button: #2d2d4a; \
         --text-theme-button: #fbbf24; \
         --border-theme-button: #3d3d5a; \
         --bg-card: #1e1e36; \
         --border-card: #2d2d4a; \
         --border-card-title: #2d2d4a; \
         --text-card: #e0e0e0; \
         --shadow-card: 0 1px 3px rgba(0,0,0,0.2), 0 4px 12px rgba(0,0,0,0.15); \
         --bg-modal: #1e1e36; \
         --shadow-modal: 0 20px 60px rgba(0,0,0,0.5); \
         --bg-input: #252540; \
         --border-input: #3d3d5a; \
         --bg-list-even: #252540; \
         --bg-list-odd: #1e1e36; \
         --bg-error: #451a1a; \
         --text-error: #fca5a5; \
         --border-error: #7f1d1d; \
         --bg-error-icon: #7f1d1d; \
         --bg-success: #052e16; \
         --text-success: #86efac; \
         --border-success: #166534; \
         --bg-warning: #451a03; \
         --text-warning: #fbbf24; \
         --border-warning: #92400e; \
         --text-info: #7dd3fc; \
         --border-info: #075985; \
         --text-pink: #f9a8d4; \
         --border-pink: #9d174d; \
         --border-accent-light: #3d3d5a; \
         --bg-loading: #1e1b4b; \
         --border-loading: #312e81; \
         --text-loading-title: #a5b4fc; \
         --bg-progress: #3d3d5a; \
         --bg-console: #1a1a2e; \
         --bg-console-header: #16162a; \
         --bg-console-filter: #12122a; \
         --border-console: #2d2d4a; \
         --text-console: #9ca3af"
            .to_string()
    } else {
        "--bg-primary: #f8f9fb; \
         --bg-nav: #ffffff; \
         --border-nav: #e5e7eb; \
         --border-subtle: #f0f0f0; \
         --text-primary: #1a1a2e; \
         --text-muted: #9ca3af; \
         --text-nav-item: #6b7280; \
         --bg-theme-button: #f3f4f6; \
         --text-theme-button: #1a1a2e; \
         --border-theme-button: #e5e7eb; \
         --bg-card: white; \
         --border-card: #f0f0f0; \
         --border-card-title: #f0f0f0; \
         --text-card: #1a1a2e; \
         --shadow-card: 0 1px 3px rgba(0,0,0,0.06), 0 4px 12px rgba(0,0,0,0.04); \
         --bg-modal: white; \
         --shadow-modal: 0 20px 60px rgba(0,0,0,0.3); \
         --bg-input: #fafbfc; \
         --border-input: #e5e7eb; \
         --bg-list-even: #fafbfc; \
         --bg-list-odd: white; \
         --bg-error: #fef2f2; \
         --text-error: #991b1b; \
         --border-error: #fecaca; \
         --bg-error-icon: #fca5a5; \
         --bg-success: #f0fdf4; \
         --text-success: #166534; \
         --border-success: #bbf7d0; \
         --bg-warning: #fef3c7; \
         --text-warning: #92400e; \
         --border-warning: #fde68a; \
         --text-info: #075985; \
         --border-info: #bae6fd; \
         --text-pink: #9d174d; \
         --border-pink: #f9a8d4; \
         --border-accent-light: #e0e7ff; \
         --bg-loading: #eef2ff; \
         --border-loading: #c7d2fe; \
         --text-loading-title: #3730a3; \
         --bg-progress: #e5e7eb; \
         --bg-console: #1a1a2e; \
         --bg-console-header: #16162a; \
         --bg-console-filter: #12122a; \
         --border-console: #2d2d4a; \
         --text-console: #9ca3af"
            .to_string()
    }
}

/// Creates and returns the reactive theme state for the application.
///
/// Initializes a theme signal (defaulting to "light") and a derived style signal
/// that holds the CSS custom properties string. Uses `watch!` to reactively update
/// the style signal whenever the theme signal changes.
///
/// # Returns
///
/// - `ThemeState`: The reactive theme state containing theme and style signals.
pub fn use_theme() -> ThemeState {
    let theme: Signal<String> = use_signal(|| "light".to_string());
    let style: Signal<String> = use_signal(|| theme_css_variables("light"));
    watch!(theme, |theme_value| {
        style.set(theme_css_variables(&theme_value));
    });
    ThemeState { theme, style }
}
