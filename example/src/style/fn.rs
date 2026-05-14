use crate::*;

class! {
    pub c_app_root {
        display: "flex";
        min_height: "100vh";
        font_family: "system-ui, -apple-system, sans-serif";
        background: "#f8f9fb";
        color: "#1a1a2e";
        line_height: "1.6";
    }

    pub c_app_nav {
        width: "240px";
        background: "#ffffff";
        border_right: "1px solid #e5e7eb";
        display: "flex";
        flex_direction: "column";
        height: "100vh";
        position: "sticky";
        top: "0";
        overflow_y: "auto";
    }

    pub c_nav_header {
        padding: "24px 20px 20px 20px";
        margin: "0";
        border_bottom: "1px solid #f0f0f0";
        font_size: "18px";
        font_weight: "700";
        color: "#1a1a2e";
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
        color: "#9ca3af";
        text_transform: "uppercase";
        letter_spacing: "0.08em";
    }

    pub c_nav_footer {
        margin_top: "auto";
        padding: "16px 20px";
        border_top: "1px solid #f0f0f0";
        font_size: "12px";
        color: "#9ca3af";
    }

    pub c_app_main {
        flex: "1";
        padding: "32px 40px";
        background: "#f8f9fb";
        min_height: "100vh";
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
        color: "#1a1a2e";
        margin_bottom: "8px";
        letter_spacing: "-0.02em";
    }

    pub c_page_subtitle {
        font_size: "16px";
        color: "#6b7280";
        line_height: "1.6";
    }

    pub c_card {
        background: "white";
        border_radius: "12px";
        padding: "24px";
        margin: "16px 0";
        box_shadow: "0 1px 3px rgba(0,0,0,0.06), 0 4px 12px rgba(0,0,0,0.04)";
        border: "1px solid #f0f0f0";
        transition: "box-shadow 0.2s ease";
    }

    pub c_card_title {
        margin_top: "0";
        margin_bottom: "16px";
        color: "#1a1a2e";
        font_size: "18px";
        font_weight: "600";
        padding_bottom: "12px";
        border_bottom: "2px solid #f0f0f0";
        letter_spacing: "0.01em";
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

    pub c_form_input_wrapper {
        margin: "12px 0";
    }

    pub c_form_label {
        display: "block";
        margin_bottom: "6px";
        color: "#374151";
        font_weight: "500";
        font_size: "14px";
    }

    pub c_form_input {
        width: "100%";
        padding: "10px 14px";
        border: "1px solid #e5e7eb";
        border_radius: "8px";
        font_size: "14px";
        box_sizing: "border-box";
        transition: "border-color 0.15s ease, box-shadow 0.15s ease";
        outline: "none";
        background: "#fafbfc";
    }

    pub c_form_input_no_transition {
        width: "100%";
        padding: "10px 14px";
        border: "1px solid #e5e7eb";
        border_radius: "8px";
        font_size: "14px";
        box_sizing: "border-box";
        background: "#fafbfc";
        outline: "none";
    }

    pub c_form_checkbox {
        cursor: "pointer";
        width: "16px";
        height: "16px";
    }

    pub c_form_checkbox_label {
        font_size: "14px";
        color: "#374151";
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
        background: "#fef2f2";
        color: "#991b1b";
        border_radius: "8px";
        font_size: "14px";
        border: "1px solid #fecaca";
    }

    pub c_success_box {
        margin_top: "16px";
        padding: "14px 16px";
        background: "#f0fdf4";
        color: "#166534";
        border_radius: "8px";
        border: "1px solid #bbf7d0";
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
        color: "#374151";
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
        color: "#6b7280";
        font_size: "14px";
    }

    pub c_form_grid {
        display: "grid";
        grid_template_columns: "1fr 1fr";
        gap: "0 20px";
    }

    pub c_custom_attrs_demo {
        padding: "16px";
        background: "linear-gradient(135deg, #f0f4ff, #faf5ff)";
        border_radius: "8px";
        border: "1px solid #e5e7eb";
    }

    pub c_demo_text {
        color: "#374151";
        margin_bottom: "8px";
    }

    pub c_demo_text_muted {
        color: "#6b7280";
        font_size: "13px";
    }

    pub c_loading_container {
        display: "flex";
        align_items: "center";
        gap: "16px";
        padding: "20px";
        background: "#eef2ff";
        border_radius: "10px";
        border: "1px solid #c7d2fe";
        margin_top: "16px";
    }

    pub c_spinner {
        width: "28px";
        height: "28px";
        border: "3px solid #c7d2fe";
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
        color: "#3730a3";
        font_size: "15px";
        font_weight: "500";
    }

    pub c_loading_subtitle {
        color: "#6b7280";
        font_size: "13px";
    }

    pub c_error_container {
        display: "flex";
        align_items: "center";
        gap: "12px";
        margin_top: "16px";
        padding: "16px";
        background: "#fef2f2";
        border_radius: "10px";
        border: "1px solid #fecaca";
    }

    pub c_error_icon {
        width: "20px";
        height: "20px";
        border_radius: "50%";
        background: "#fca5a5";
        display: "flex";
        align_items: "center";
        justify_content: "center";
        flex_shrink: "0";
        color: "#991b1b";
        font_size: "12px";
        font_weight: "700";
    }

    pub c_error_text {
        color: "#991b1b";
        font_size: "14px";
    }

    pub c_data_box {
        margin_top: "16px";
        padding: "16px";
        background: "#f0fdf4";
        border_radius: "10px";
        border: "1px solid #bbf7d0";
    }

    pub c_data_pre {
        color: "#166534";
        font_size: "13px";
        margin: "0";
        white_space: "pre-wrap";
        word_break: "break-all";
        font_family: "ui-monospace, monospace";
    }

    pub c_fetch_hint {
        color: "#6b7280";
        font_size: "14px";
        margin_bottom: "16px";
    }

    pub c_toggle_content {
        margin_top: "16px";
        padding: "16px";
        background: "linear-gradient(135deg, #f0f4ff, #faf5ff)";
        border_radius: "8px";
        border: "1px solid #e0e7ff";
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
        background: "#fefce8";
        border_radius: "8px";
        border: "1px solid #fde68a";
    }

    pub c_role_guest_text {
        color: "#92400e";
        font_size: "14px";
    }

    pub c_role_user {
        padding: "16px";
        background: "#f0f9ff";
        border_radius: "8px";
        border: "1px solid #bae6fd";
    }

    pub c_role_user_text {
        color: "#075985";
        font_size: "14px";
    }

    pub c_role_admin {
        padding: "16px";
        background: "#f0fdf4";
        border_radius: "8px";
        border: "1px solid #bbf7d0";
    }

    pub c_role_admin_text {
        color: "#166534";
        font_size: "14px";
    }

    pub c_tab_bar {
        display: "flex";
        border_bottom: "1px solid #e5e7eb";
        margin_bottom: "16px";
        gap: "4px";
    }

    pub c_tab_content {
        padding: "4px 0";
    }

    pub c_tab_text {
        color: "#374151";
        font_size: "14px";
        margin_bottom: "8px";
    }

    pub c_tab_text_muted {
        color: "#6b7280";
        font_size: "13px";
    }

    pub c_tab_text_input {
        color: "#374151";
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
        border: "1px solid #e5e7eb";
        border_radius: "8px";
        font_size: "14px";
        background: "#fafbfc";
        outline: "none";
    }

    pub c_list_ul {
        list_style: "none";
        padding: "0";
        margin: "0";
    }

    pub c_list_item {
        display: "flex";
        justify_content: "space-between";
        align_items: "center";
        padding: "14px 16px";
        border_bottom: "1px solid #f0f0f0";
        border_radius: "6px";
        margin_bottom: "4px";
    }

    pub c_list_item_text {
        font_size: "14px";
        color: "#374151";
    }

    pub c_log_container {
        max_height: "320px";
        overflow_y: "auto";
        background: "#1a1a2e";
        padding: "16px";
        border_radius: "8px";
        font_family: "ui-monospace, monospace";
        font_size: "13px";
        line_height: "1.5";
    }

    pub c_log_item {
        padding: "6px 0";
        border_bottom: "1px solid #f0f0f0";
        font_size: "13px";
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
        color: "#e5e7eb";
        line_height: "1";
        margin_bottom: "16px";
        letter_spacing: "-0.04em";
    }

    pub c_not_found_title {
        font_size: "24px";
        font_weight: "600";
        color: "#1a1a2e";
        margin_bottom: "12px";
    }

    pub c_not_found_text {
        color: "#6b7280";
        font_size: "16px";
        margin_bottom: "28px";
        line_height: "1.6";
    }

    pub c_render_count_text {
        font_size: "16px";
        color: "#374151";
        margin_bottom: "16px";
    }

    pub c_inline {
        display: "inline";
    }
}
