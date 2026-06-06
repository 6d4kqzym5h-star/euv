use crate::*;

class! {
    // ═══════════════════════════════════════════════════════════════════════════
    // Layout Shell
    // ═══════════════════════════════════════════════════════════════════════════

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
        transition: format!("background {} {}, color {} {}", var!(duration-slow), var!(ease-in-out), var!(duration-slow), var!(ease-in-out));
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
        padding-top: "env(safe-area-inset-top, 0px)";
        transition: format!("background {} {}, color {} {}", var!(duration-slow), var!(ease-in-out), var!(duration-slow), var!(ease-in-out));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Navigation - Desktop Sidebar
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_app_nav {
        width: var!(nav-width);
        background: var!(bg-nav);
        border-right: format!("1px solid {}", var!(border-nav));
        display: "flex";
        flex-direction: "column";
        height: "100vh";
        flex-shrink: "0";
        transition: format!("background {} {}, border-color {} {}", var!(duration-slow), var!(ease-in-out), var!(duration-slow), var!(ease-in-out));
        media("(max-width: 767px)") {
            display: "none";
        }
    }

    pub(crate) c_nav_header {
        padding: format!("{} {}", var!(space-xl), var!(space-xl));
        margin: "0px";
        border-bottom: format!("1px solid {}", var!(border-subtle));
        font-size: var!(font-xl);
        font-weight: "700";
        color: var!(text-primary);
        letter-spacing: "-0.02em";
        display: "flex";
        align-items: "center";
        gap: format!("{}", var!(space-md));
        flex-shrink: "0";
        text-decoration: "none";
        cursor: "pointer";
        transition: format!("border-color {} {}", var!(duration-slow), var!(ease-in-out));
        media("(max-width: 767px)") {
            border-bottom: "none";
        }
    }

    pub(crate) c_nav_brand_title {
        font-size: var!(font-xl);
        font-weight: "700";
        color: var!(text-primary);
        letter-spacing: "-0.02em";
    }

    pub(crate) c_logo_button {
        display: "flex";
        background: var!(accent-gradient);
        border-radius: var!(radius-md);
        align-items: "center";
        justify-content: "center";
        color: var!(text-on-accent);
        font-weight: "700";
        border: "none";
        cursor: "pointer";
        padding: "0px";
        line-height: "1";
        flex-shrink: "0";
    }

    pub(crate) c_logo_button_nav {
        width: "32px";
        height: "32px";
        font-size: var!(font-lg);
    }

    pub(crate) c_logo_button_fab {
        position: "fixed";
        bottom: "calc(20px + env(safe-area-inset-bottom, 0px))";
        right: var!(space-xl);
        width: "48px";
        height: "48px";
        border-radius: "14px";
        font-size: var!(font-xl);
        z-index: "9999";
        box-shadow: var!(shadow-console-button);
        transition: format!("transform {} {}, box-shadow {} {}", var!(duration-normal), var!(ease-out), var!(duration-normal), var!(ease-out));
        media("(max-width: 767px)") {
            bottom: "calc(16px + env(safe-area-inset-bottom, 0px))";
            right: var!(space-lg);
            width: "44px";
            height: "44px";
            border-radius: var!(radius-lg);
        }
    }

    pub(crate) c_logo_button_fab_hover {
        transform: "scale(1.08)";
        box-shadow: var!(shadow-accent-lg);
    }

    pub(crate) c_nav_section_label {
        padding: format!("{} {} {} {}", var!(space-md), var!(space-xl), var!(space-xs), var!(space-xl));
        margin: "0px";
        font-size: var!(font-xs);
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
        padding: format!("{} {}", var!(space-md), var!(space-xl));
        flex-shrink: "0";
        margin-top: "auto";
    }

    pub(crate) c_nav_theme_button {
        width: "100%";
        height: "36px";
        padding: "0px";
        border-radius: var!(radius-md);
        cursor: "pointer";
        outline: "none";
        background: var!(bg-theme-button);
        border: format!("1px solid {}", var!(border-theme-button));
        transition: format!("all {} {}", var!(duration-normal), var!(ease-out));
        display: "flex";
        align-items: "center";
        justify-content: "center";
    }

    pub(crate) c_theme_icon_sun {
        width: "20px";
        height: "20px";
        background-repeat: "no-repeat";
        background-position: "center";
        background-size: "20px 20px";
        background-image: "url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='20' height='20' viewBox='0 0 24 24' fill='%23fbbf24' stroke='%23fbbf24' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round'%3E%3Ccircle cx='12' cy='12' r='5'/%3E%3Cline x1='12' y1='1' x2='12' y2='4'/%3E%3Cline x1='12' y1='20' x2='12' y2='23'/%3E%3Cline x1='4.22' y1='4.22' x2='6.34' y2='6.34'/%3E%3Cline x1='17.66' y1='17.66' x2='19.78' y2='19.78'/%3E%3Cline x1='1' y1='12' x2='4' y2='12'/%3E%3Cline x1='20' y1='12' x2='23' y2='12'/%3E%3Cline x1='4.22' y1='19.78' x2='6.34' y2='17.66'/%3E%3Cline x1='17.66' y1='6.34' x2='19.78' y2='4.22'/%3E%3C/svg%3E\")";
        transition: format!("transform {} {}", var!(duration-normal), var!(ease-out));
    }

    pub(crate) c_theme_icon_moon {
        width: "20px";
        height: "20px";
        background-repeat: "no-repeat";
        background-position: "center";
        background-size: "20px 20px";
        background-image: "url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='20' height='20' viewBox='0 0 24 24' fill='%23fbbf24' stroke='none' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='M21 13A9 9 0 1 1 11 3a7 7 0 0 0 10 10z'/%3E%3C/svg%3E\")";
        transition: format!("transform {} {}", var!(duration-normal), var!(ease-out));
    }

    pub(crate) c_nav_footer {
        padding: format!("{} {}", var!(space-lg), var!(space-xl));
        border-top: format!("1px solid {}", var!(border-subtle));
        font-size: var!(font-sm);
        color: var!(text-muted);
        flex-shrink: "0";
        text-decoration: "none";
        display: "block";
        cursor: "pointer";
        transition: format!("color {} {}", var!(duration-fast), var!(ease-out));
    }

    pub(crate) c_nav_item_active {
        display: "block";
        padding: format!("11px {}", var!(space-xl));
        text-decoration: "none";
        font-size: var!(font-base);
        transition: format!("all {} {}", var!(duration-fast), var!(ease-out));
        color: var!(accent);
        font-weight: "600";
        border-left: format!("3px solid {}", var!(accent));
        background: var!(accent-subtle);
    }

    pub(crate) c_nav_item_inactive {
        display: "block";
        padding: format!("11px {}", var!(space-xl));
        text-decoration: "none";
        font-size: var!(font-base);
        transition: format!("all {} {}", var!(duration-fast), var!(ease-out));
        color: var!(text-nav-item);
        font-weight: "400";
        border-left: "3px solid transparent";
        background: "transparent";
        hover {
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Main Content Area
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_app_main {
        flex: "1";
        width: "0px";
        padding: format!("{} {}", var!(padding-main-top), var!("space-4xl"));
        background: var!(bg-primary);
        overflow-y: "auto";
        transition: format!("background {} {}", var!(duration-slow), var!(ease-in-out));
        media("(max-width: 767px)") {
            padding: format!("{} {}", var!(space-md), var!(space-lg));
            max-width: "100%";
        }
    }

    pub(crate) c_page_container {
        max-width: var!(content-max-width);
        margin: "0px auto";
    }

    pub(crate) c_page_container_wide {
        max-width: var!(content-max-width);
        margin: "0px auto";
        display: "flex";
        flex-direction: "column";
        height: "100%";
    }

    pub(crate) c_page_header {
        margin-bottom: var!(gap-page-header);
    }

    pub(crate) c_page_title {
        font-size: var!("font-3xl");
        font-weight: "700";
        color: "inherit";
        margin-bottom: var!(gap-page-title);
        letter-spacing: "-0.02em";
        media("(max-width: 767px)") {
            font-size: var!("font-2xl");
        }
    }

    pub(crate) c_page_subtitle {
        font-size: var!(font-lg);
        color: "inherit";
        opacity: "0.6";
        line-height: "1.6";
        media("(max-width: 767px)") {
            font-size: var!(font-base);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Card Component
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_card {
        background: var!(bg-card);
        border-radius: var!(radius-lg);
        padding: var!(space-xl);
        margin-top: "0px";
        margin-bottom: var!(gap-section);
        box-shadow: var!(shadow-card);
        border: format!("1px solid {}", var!(border-card));
        color: var!(text-card);
        transition: format!("box-shadow {} {}", var!(duration-normal), var!(ease-out));
        media("(max-width: 767px)") {
            padding: var!(space-lg);
            margin-bottom: var!(gap-section-mobile);
            border-radius: "10px";
        }
    }

    pub(crate) c_card_title {
        margin-top: "0px";
        margin-bottom: var!(gap-component);
        color: "inherit";
        font-size: var!(font-xl);
        font-weight: "600";
        padding-bottom: var!(space-sm);
        border-bottom: format!("2px solid {}", var!(border-card-title));
        letter-spacing: "0.01em";
        transition: format!("border-color {} {}", var!(duration-slow), var!(ease-in-out));
        media("(max-width: 767px)") {
            font-size: var!(font-lg);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Buttons
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_primary_button {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        gap: var!(space-sm);
        width: "100%";
        flex: "1";
        background: var!(accent);
        color: var!(text-on-accent);
        border: "none";
        padding: format!("10px {}", var!("space-2xl"));
        border-radius: var!(radius-md);
        cursor: "pointer";
        font-size: var!(font-base);
        font-weight: "500";
        letter-spacing: "0.02em";
        text-align: "center";
        line-height: "1.4";
        transition: format!("box-shadow {} {}, transform 0.1s ease", var!(duration-fast), var!(ease-out));
        outline: "none";
        box-shadow: var!(shadow-accent-sm);
        white-space: "nowrap";
        overflow: "hidden";
        text-overflow: "ellipsis";
        user-select: "none";
        -webkit-user-select: "none";
        media("(max-width: 767px)") {
            max-width: "100%";
            padding: format!("{} {}", var!(space-md), var!(space-xl));
            font-size: var!(font-md);
            border-radius: "10px";
            box-shadow: "none";
        }
    }

    pub(crate) c_primary_button_disabled {
        c_primary_button();
        background: var!(accent-disabled);
        cursor: "not-allowed";
        opacity: "0.55";
        box-shadow: "none";
    }

    pub(crate) c_modal_primary_button {
        display: "inline-flex";
        justify-content: "center";
        align-items: "center";
        gap: var!(space-sm);
        width: "100%";
        flex: "1";
        background: var!(accent);
        color: var!(text-on-accent);
        border: "none";
        padding: format!("10px {}", var!("space-2xl"));
        border-radius: var!(radius-md);
        cursor: "pointer";
        font-size: var!(font-base);
        font-weight: "500";
        letter-spacing: "0.02em";
        text-align: "center";
        line-height: "1.4";
        transition: format!("box-shadow {} {}, transform 0.1s ease", var!(duration-fast), var!(ease-out));
        outline: "none";
        box-shadow: var!(shadow-accent-sm);
        white-space: "nowrap";
        overflow: "hidden";
        text-overflow: "ellipsis";
        user-select: "none";
        -webkit-user-select: "none";
    }

    pub(crate) c_modal_primary_button_disabled {
        c_modal_primary_button();
        background: var!(accent-disabled);
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
        padding: format!("{} 10px", var!(space-sm));
        border-radius: var!(radius-sm);
        cursor: "pointer";
        font-size: "20px";
        font-weight: "400";
        line-height: "1";
        transition: format!("background {} {}", var!(duration-fast), var!(ease-out));
        outline: "none";
        opacity: "0.6";
        flex-shrink: "0";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Badges
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_badge {
        display: "inline-block";
        color: var!(text-on-accent);
        padding: "5px 14px";
        border-radius: var!(radius-pill);
        font-size: var!(font-sm);
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.03em";
        text-transform: "uppercase";
        transition: format!("transform {} {}, box-shadow {} {}", var!(duration-fast), var!(ease-out), var!(duration-fast), var!(ease-out));
        media("(max-width: 767px)") {
            padding: format!("{} 10px", var!(space-xs));
            font-size: var!(font-xs);
        }
    }

    pub(crate) c_badge_outline {
        display: "inline-block";
        padding: "5px 14px";
        border-radius: var!(radius-pill);
        font-size: var!(font-sm);
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.03em";
        text-transform: "uppercase";
        background: "transparent";
        border: "1.5px solid";
        transition: format!("transform {} {}, box-shadow {} {}", var!(duration-fast), var!(ease-out), var!(duration-fast), var!(ease-out));
        media("(max-width: 767px)") {
            padding: format!("{} 10px", var!(space-xs));
            font-size: var!(font-xs);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Form Elements
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_form_input_wrapper {
        margin: format!("{} 0px", var!(gap-element));
    }

    pub(crate) c_form_label {
        display: "block";
        margin-bottom: var!(space-sm);
        color: "inherit";
        font-weight: "500";
        font-size: var!(font-base);
    }

    pub(crate) c_form_input {
        width: "100%";
        padding: "10px 14px";
        border: format!("1px solid {}", var!(border-input));
        border-radius: var!(radius-md);
        font-size: var!(font-base);
        box-sizing: "border-box";
        transition: format!("border-color {} {}, box-shadow {} {}", var!(duration-fast), var!(ease-out), var!(duration-fast), var!(ease-out));
        outline: "none";
        background: var!(bg-input);
        color: var!(text-primary);
    }

    pub(crate) c_form_input_no_transition {
        width: "100%";
        padding: "10px 14px";
        border: format!("1px solid {}", var!(border-input));
        border-radius: var!(radius-md);
        font-size: var!(font-base);
        box-sizing: "border-box";
        background: var!(bg-input);
        outline: "none";
        color: var!(text-primary);
    }

    pub(crate) c_form_input_error {
        width: "100%";
        padding: "10px 14px";
        border: format!("1px solid {}", var!(text-error));
        border-radius: var!(radius-md);
        font-size: var!(font-base);
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

    pub(crate) c_form_checkbox {
        cursor: "pointer";
        width: var!(space-lg);
        height: var!(space-lg);
    }

    pub(crate) c_form_checkbox_label {
        font-size: var!(font-base);
        color: "inherit";
        cursor: "pointer";
    }

    pub(crate) c_form_checkbox_row {
        margin: format!("{} 0px", var!(gap-component));
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
    }

    pub(crate) c_form_grid {
        display: "grid";
        grid-template-columns: "1fr 1fr";
        gap: format!("0px {}", var!(gap-component));
        media("(max-width: 767px)") {
            grid-template-columns: "1fr";
        }
    }

    pub(crate) c_select_input {
        width: "100%";
        padding: "10px 14px";
        border: format!("1px solid {}", var!(border-input));
        border-radius: var!(radius-md);
        font-size: var!(font-base);
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
        border-radius: var!(radius-md);
        font-size: var!(font-base);
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
        margin-top: var!(space-xs);
    }

    pub(crate) c_textarea_counter_text {
        font-size: var!(font-sm);
        color: "inherit";
        opacity: "0.5";
    }

    pub(crate) c_field_error_text {
        color: var!(text-error);
        font-size: "13px";
        margin-top: var!(space-xs);
        margin-bottom: var!(space-sm);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Status Boxes
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_error_box {
        margin: format!("{} 0px", var!(gap-component));
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        background: var!(bg-error);
        color: var!(text-error);
        border-radius: var!(radius-md);
        font-size: var!(font-base);
        border: format!("1px solid {}", var!(border-error));
    }

    pub(crate) c_success_box {
        margin-top: var!(gap-component);
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        background: var!(bg-success);
        color: var!(text-success);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-success));
        font-size: var!(font-base);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Counter / Signals Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_counter_row {
        display: "flex";
        align-items: "center";
        gap: var!(gap-component);
        flex-wrap: "wrap";
    }

    pub(crate) c_counter_text {
        font-size: var!(font-lg);
        color: "inherit";
    }

    pub(crate) c_counter_value {
        font-weight: "700";
        color: var!(accent);
        font-size: var!("font-2xl");
        transition: format!("color {} {}", var!(duration-slow), var!(ease-in-out));
        media("(max-width: 767px)") {
            font-size: var!(font-xl);
        }
    }

    pub(crate) c_badge_row {
        display: "flex";
        gap: var!(gap-inline);
        flex-wrap: "wrap";
        align-items: "center";
    }

    pub(crate) c_badge_hint {
        margin-bottom: var!(gap-component);
        color: "inherit";
        opacity: "0.7";
        font-size: var!(font-base);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Home Page
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_text_ellipsis {
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub(crate) c_info_row {
        display: "flex";
        align-items: "center";
        gap: var!(space-md);
        padding: format!("{} 0", var!(space-sm));
        overflow: "hidden";
    }

    pub(crate) c_info_label {
        font-size: var!(font-base);
        font-weight: "600";
        color: var!(text-secondary);
        min-width: "80px";
        flex-shrink: "0";
    }

    pub(crate) c_info_value {
        c_text_ellipsis();
        font-size: var!(font-lg);
        font-weight: "700";
        color: var!(accent);
        font-family: "ui-monospace, monospace";
        min-width: "0px";
        flex: "1";
    }

    pub(crate) c_info_link {
        c_text_ellipsis();
        font-size: var!(font-lg);
        font-weight: "700";
        color: var!(accent);
        font-family: "ui-monospace, monospace";
        min-width: "0px";
        flex: "1";
        text-decoration: "none";
        transition: format!("opacity {} {}", var!(duration-fast), var!(ease-out));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Multimedia Page
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_multimedia_image_section {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        padding: format!("{} 0", var!(space-lg));
    }

    pub(crate) c_multimedia_qr_code {
        width: "200px";
        height: "200px";
        object-fit: "contain";
        border-radius: var!(radius-lg);
        pointer-events: "none";
    }

    pub(crate) c_multimedia_url_text {
        text-align: "center";
        font-size: var!(font-sm);
        color: "inherit";
        opacity: "0.6";
        font-family: "ui-monospace, monospace";
        word-break: "break-all";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Camera Page
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_camera_video_container {
        margin: format!("{} 0", var!(space-lg));
    }

    pub(crate) c_camera_video_active {
        width: "100%";
        border-radius: var!(radius-lg);
        background: "#000";
        aspect-ratio: "16 / 9";
        object-fit: "cover";
        border: format!("2px solid {}", var!(accent));
        box-shadow: var!(shadow-accent-sm);
    }

    pub(crate) c_camera_video_hidden {
        width: "100%";
        border-radius: var!(radius-lg);
        aspect-ratio: "16 / 9";
        display: "none";
    }

    pub(crate) c_camera_video_placeholder {
        width: "100%";
        border-radius: var!(radius-lg);
        background: var!(bg-input);
        aspect-ratio: "16 / 9";
        border: format!("2px dashed {}", var!(border-input));
        display: "flex";
        align-items: "center";
        justify-content: "center";
        transition: format!("border-color {} {}", var!(duration-fast), var!(ease-out));
    }

    pub(crate) c_camera_placeholder_content {
        display: "flex";
        flex-direction: "column";
        align-items: "center";
        gap: var!(space-md);
    }

    pub(crate) c_camera_placeholder_icon {
        font-size: var!("font-4xl");
        opacity: "0.4";
    }

    pub(crate) c_camera_placeholder_text {
        font-size: var!(font-base);
        color: "inherit";
        opacity: "0.5";
    }

    pub(crate) c_camera_controls {
        display: "flex";
        gap: var!(gap-element);
        flex-wrap: "wrap";
        margin-top: var!(gap-component);
    }

    pub(crate) c_camera_error_box {
        margin: format!("{} 0px", var!(gap-component));
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        background: var!(bg-error);
        color: var!(text-error);
        border-radius: var!(radius-md);
        font-size: var!(font-base);
        border: format!("1px solid {}", var!(border-error));
        word-break: "break-all";
        overflow-wrap: "break-word";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Custom Attrs Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_custom_attrs_demo {
        padding: var!(space-lg);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-input));
        color: "inherit";
    }

    pub(crate) c_demo_text {
        color: "inherit";
        margin-bottom: var!(gap-component);
    }

    pub(crate) c_demo_text_muted {
        color: "inherit";
        opacity: "0.6";
        font-size: "13px";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Async / Loading States
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_loading_container {
        display: "flex";
        align-items: "center";
        gap: var!(gap-component);
        padding: var!(space-xl);
        background: var!(bg-loading);
        border-radius: "10px";
        border: format!("1px solid {}", var!(border-loading));
        margin-top: var!(gap-component);
        transition: format!("background {} {}, border-color {} {}", var!(duration-slow), var!(ease-in-out), var!(duration-slow), var!(ease-in-out));
    }

    pub(crate) c_spinner {
        width: "28px";
        height: "28px";
        border: format!("3px solid {}", var!(border-loading));
        border-top: format!("3px solid {}", var!(accent));
        border-radius: "50%";
        flex-shrink: "0";
        animation: "euv-spin 0.8s linear infinite";
    }

    pub(crate) c_loading_text_col {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
    }

    pub(crate) c_loading_title {
        color: var!(text-loading-title);
        font-size: var!(font-md);
        font-weight: "500";
    }

    pub(crate) c_loading_subtitle {
        color: var!(text-muted);
        font-size: "13px";
    }

    pub(crate) c_error_container {
        display: "flex";
        align-items: "center";
        gap: var!(gap-element);
        margin-top: var!(gap-component);
        padding: var!(space-lg);
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
        font-size: var!(font-sm);
        font-weight: "700";
    }

    pub(crate) c_error_text {
        color: var!(text-error);
        font-size: var!(font-base);
    }

    pub(crate) c_data_box {
        margin-top: var!(gap-component);
        padding: var!(space-lg);
        background: var!(bg-success);
        border-radius: "10px";
        border: format!("1px solid {}", var!(border-success));
    }

    pub(crate) c_data_pre {
        color: var!(text-success);
        font-size: "13px";
        margin: "0px";
        white-space: "pre-wrap";
        word-break: "break-all";
        font-family: "ui-monospace, monospace";
    }

    pub(crate) c_fetch_hint {
        color: "inherit";
        opacity: "0.7";
        font-size: var!(font-base);
        margin-bottom: var!(gap-component);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Conditional Rendering Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_toggle_content {
        margin-top: var!(gap-component);
        padding: var!(space-lg);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-accent-light));
        color: "inherit";
    }

    pub(crate) c_toggle_title {
        margin-top: "0px";
        color: var!(accent);
        font-size: var!(font-md);
    }

    pub(crate) c_role_button_row {
        display: "flex";
        gap: var!(gap-element);
        margin-bottom: var!(gap-component);
        media("(max-width: 767px)") {
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_role_guest {
        padding: var!(space-lg);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-warning));
    }

    pub(crate) c_role_guest_text {
        color: var!(text-warning);
        font-size: var!(font-base);
    }

    pub(crate) c_role_user {
        padding: var!(space-lg);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-info));
    }

    pub(crate) c_role_user_text {
        color: var!(text-info);
        font-size: var!(font-base);
    }

    pub(crate) c_role_admin {
        padding: var!(space-lg);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-success));
    }

    pub(crate) c_role_admin_text {
        color: var!(text-success);
        font-size: var!(font-base);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Tabs
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_tab_bar {
        display: "flex";
        border-bottom: format!("1px solid {}", var!(border-input));
        margin-bottom: var!(gap-component);
        gap: var!(gap-element);
        media("(max-width: 767px)") {
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_tab_item_active {
        padding: "10px 20px";
        cursor: "pointer";
        border-bottom: format!("2px solid {}", var!(accent));
        color: var!(accent);
        background: var!(accent-subtle);
        border-radius: "6px 6px 0px 0px";
        font-size: "14px";
        font-weight: "500";
    }

    pub(crate) c_tab_item_inactive {
        padding: "10px 20px";
        cursor: "pointer";
        border-bottom: "2px solid transparent";
        color: "inherit";
        background: "transparent";
        border-radius: "6px 6px 0px 0px";
        font-size: "14px";
        font-weight: "500";
        hover {
            background: var!(accent-muted);
            color: var!(accent);
        }
    }

    pub(crate) c_tab_content {
        padding: format!("{} 0px", var!(gap-element));
    }

    pub(crate) c_tab_text {
        color: "inherit";
        font-size: var!(font-base);
        margin-bottom: var!(gap-element);
    }

    pub(crate) c_tab_text_muted {
        color: "inherit";
        opacity: "0.6";
        font-size: "13px";
    }

    pub(crate) c_tab_text_input {
        color: "inherit";
        font-size: var!(font-base);
        margin-bottom: var!(gap-component);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // List
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_list_input_row {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        padding: format!("{} {}", var!(gap-component), var!(space-lg));
        border-bottom: format!("1px solid {}", var!(border-subtle));
        border-radius: var!(radius-sm);
        margin-bottom: var!(gap-element);
        gap: var!(gap-component);
        background: var!(bg-list-even);
        media("(max-width: 767px)") {
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_list_input {
        flex: "1";
        min-width: "0px";
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border: format!("1px solid {}", var!(border-input));
        border-radius: var!(radius-sm);
        font-size: var!(font-base);
        line-height: "1.4";
        box-sizing: "border-box";
        background: var!(bg-input);
        outline: "none";
        color: var!(text-primary);
    }

    pub(crate) c_list_input_error {
        flex: "1";
        min-width: "0px";
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border: format!("1px solid {}", var!(text-error));
        border-radius: var!(radius-sm);
        font-size: var!(font-base);
        line-height: "1.4";
        box-sizing: "border-box";
        background: var!(bg-input);
        outline: "none";
        color: var!(text-primary);
    }

    pub(crate) c_list_error_text {
        color: var!(text-error);
        font-size: "13px";
        margin-top: var!(gap-element);
        margin-bottom: var!(gap-element);
        padding-left: var!(space-lg);
    }

    pub(crate) c_list_ul {
        list-style: "none";
        padding: "0px";
        margin: "0px";
    }

    pub(crate) c_list_item_even {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        padding: format!("{} {}", var!(gap-component), var!(space-lg));
        border-bottom: format!("1px solid {}", var!(border-subtle));
        border-radius: var!(radius-sm);
        margin-bottom: var!(gap-element);
        gap: var!(gap-component);
        background: var!(bg-list-even);
    }

    pub(crate) c_list_item_odd {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        padding: format!("{} {}", var!(gap-component), var!(space-lg));
        border-bottom: format!("1px solid {}", var!(border-subtle));
        border-radius: var!(radius-sm);
        margin-bottom: var!(gap-element);
        gap: var!(gap-component);
        background: var!(bg-list-odd);
    }

    pub(crate) c_list_item_text {
        flex: "1";
        min-width: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        font-size: var!(font-base);
        color: "inherit";
    }

    pub(crate) c_list_item_button {
        min-width: "0px";
        max-width: "120px";
        padding: format!("{} {}", var!(space-sm), var!(space-lg));
        flex-shrink: "0";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Log / Console Display
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_log_container {
        background: var!(bg-console);
        padding: format!("0px {}", var!(space-sm));
        border-radius: var!(radius-md);
        font-family: "ui-monospace, monospace";
        font-size: "13px";
        line-height: "1.5";
        margin-top: "0px";
        color: var!(text-console);
    }

    pub(crate) c_log_item {
        padding: format!("{} 0px", var!(space-sm));
        border-bottom: format!("1px solid {}", var!(border-console));
        font-size: "13px";
        color: var!(text-console);
    }

    pub(crate) c_log_empty {
        color: var!(text-muted);
        font-size: "13px";
        text-align: "center";
        padding: var!(space-lg);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // 404 Not Found
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_not_found_container {
        text-align: "center";
        padding: format!("{} {}", var!("space-7xl"), var!(space-xl));
        max-width: "480px";
        margin: "0px auto";
        media("(max-width: 767px)") {
            padding: format!("{} {}", var!("space-4xl"), var!(space-xl));
        }
    }

    pub(crate) c_not_found_code {
        font-size: var!("font-6xl");
        font-weight: "800";
        color: var!(text-muted);
        line-height: "1";
        margin-bottom: var!(space-md);
        letter-spacing: "-0.04em";
        media("(max-width: 767px)") {
            font-size: "56px";
        }
    }

    pub(crate) c_not_found_title {
        font-size: var!("font-2xl");
        font-weight: "600";
        color: "inherit";
        margin-bottom: var!(space-md);
    }

    pub(crate) c_not_found_text {
        color: "inherit";
        opacity: "0.7";
        font-size: var!(font-lg);
        margin-bottom: var!("space-3xl");
        line-height: "1.6";
    }

    pub(crate) c_render_count_text {
        font-size: var!(font-lg);
        color: "inherit";
        margin-bottom: var!(space-md);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Event Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_event_result {
        font-size: var!(font-base);
        color: "inherit";
        margin-top: var!(gap-element);
    }

    pub(crate) c_event_highlight {
        font-weight: "600";
        color: var!(accent);
    }

    pub(crate) c_event_stats {
        display: "flex";
        gap: var!(gap-component);
        margin-top: var!(gap-component);
        media("(max-width: 767px)") {
            gap: var!(gap-component-mobile);
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_event_mouse_area {
        min-height: "120px";
        padding: var!(space-xl);
        border-radius: var!(radius-md);
        border: format!("2px dashed {}", var!(border-accent-light));
        cursor: "crosshair";
        text-align: "center";
        user-select: "none";
        color: "inherit";
        transition: format!("border-color {} {}", var!(duration-fast), var!(ease-out));
    }

    pub(crate) c_event_drag_zone {
        min-height: "100px";
        padding: var!(space-xl);
        border-radius: var!(radius-md);
        border: format!("2px dashed {}", var!(border-warning));
        text-align: "center";
        user-select: "none";
        cursor: "grab";
        color: var!(text-warning);
        transition: format!("border-color {} {}, color {} {}", var!(duration-fast), var!(ease-out), var!(duration-fast), var!(ease-out));
    }

    pub(crate) c_event_drag_zone_active {
        min-height: "100px";
        padding: var!(space-xl);
        border-radius: var!(radius-md);
        border: format!("2px dashed {}", var!(border-success));
        text-align: "center";
        user-select: "none";
        color: var!(text-success);
    }

    pub(crate) c_event_drag_item {
        display: "inline-block";
        padding: format!("{} {}", var!(space-sm), var!(space-xl));
        background: var!(accent);
        color: var!(text-on-accent);
        border-radius: var!(radius-sm);
        font-size: var!(font-base);
        font-weight: "500";
        cursor: "grab";
        margin: var!(space-sm);
    }

    pub(crate) c_event_wheel_zone {
        min-height: "120px";
        padding: var!(space-xl);
        border-radius: var!(radius-md);
        border: format!("2px dashed {}", var!(border-success));
        text-align: "center";
        overflow: "auto";
        color: var!(text-success);
    }

    pub(crate) c_event_clipboard_area {
        padding: var!(space-lg);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-pink));
        color: var!(text-pink);
    }

    pub(crate) c_event_touch_zone {
        min-height: "120px";
        padding: var!(space-xl);
        border-radius: var!(radius-md);
        border: format!("2px dashed {}", var!(accent));
        text-align: "center";
        touch-action: "none";
        user-select: "none";
        color: var!(accent);
    }

    pub(crate) c_event_form_area {
        padding: var!(space-lg);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-info));
        color: var!(text-info);
    }

    pub(crate) c_event_media_area {
        padding: var!(space-lg);
        border-radius: var!(radius-md);
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
        gap: var!(gap-component);
        flex-wrap: "wrap";
        margin-top: var!(gap-component);
        media("(max-width: 767px)") {
            gap: var!(gap-component-mobile);
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
        margin-bottom: var!(space-xs);
    }

    pub(crate) c_event_section_value {
        font-size: var!(font-base);
        color: "inherit";
        font-weight: "500";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Timer Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_timer_display {
        text-align: "center";
        padding: var!(space-xl);
        margin: format!("{} 0px", var!(gap-component));
        border-radius: var!(radius-lg);
        border: format!("1px solid {}", var!(border-accent-light));
        color: "inherit";
        transition: format!("border-color {} {}", var!(duration-slow), var!(ease-in-out));
    }

    pub(crate) c_timer_value {
        font-size: var!("font-5xl");
        font-weight: "700";
        color: var!(accent);
        letter-spacing: "0.04em";
        font-family: "ui-monospace, monospace";
        transition: format!("color {} {}", var!(duration-slow), var!(ease-in-out));
        media("(max-width: 767px)") {
            font-size: var!("font-4xl");
        }
    }

    pub(crate) c_timer_controls {
        display: "flex";
        gap: var!(gap-element);
        justify-content: "center";
        media("(max-width: 767px)") {
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_timer_done {
        text-align: "center";
        padding: var!(space-md);
        margin-top: var!(gap-component);
        background: var!(bg-warning);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-warning));
        font-size: var!(font-lg);
        font-weight: "600";
        color: var!(text-warning);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Modal
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_modal_overlay {
        position: "fixed";
        top: "0px";
        left: "0px";
        width: "100%";
        height: "100%";
        background: var!(bg-overlay);
        display: "flex";
        align-items: "center";
        justify-content: "center";
        z-index: "1000";
        animation: format!("euv-fade-in {} {}", var!(duration-normal), var!(ease-out));
        media("(max-width: 767px)") {
            align-items: "center";
            justify-content: "center";
        }
    }

    pub(crate) c_modal_content {
        background: var!(bg-modal);
        border-radius: var!(radius-xl);
        padding: "0px";
        max-width: "480px";
        width: "90%";
        box-shadow: var!(shadow-modal);
        animation: format!("euv-scale-in {} {}", var!(duration-normal), var!(ease-out));
        overflow: "hidden";
        color: var!(text-card);
        media("(max-width: 767px)") {
            max-width: "100%";
            width: "calc(100% - 32px)";
            border-radius: var!(radius-xl);
            max-height: "85vh";
            overflow-y: "auto";
        }
    }

    pub(crate) c_modal_header {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        padding: format!("{} {}", var!(space-lg), var!(space-xl));
        border-bottom: format!("1px solid {}", var!(border-card-title));
    }

    pub(crate) c_modal_title {
        margin: "0px";
        font-size: var!(font-xl);
        font-weight: "600";
        color: "inherit";
    }

    pub(crate) c_modal_body {
        padding: format!("0px {}", var!(space-xl));
        color: "inherit";
    }

    pub(crate) c_modal_actions {
        display: "flex";
        justify-content: "center";
        gap: var!(space-md);
        margin: format!("{} 0px", var!(space-lg));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Animation Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_anim_fade_in {
        margin-top: var!(gap-component);
        padding: var!(space-xl);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-accent-light));
        animation: format!("euv-fade-in 0.5s {}", var!(ease-out));
        font-size: var!(font-base);
        color: "inherit";
    }

    pub(crate) c_anim_spin_container {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        margin: format!("{} 0px", var!(space-lg));
        min-height: "80px";
    }

    pub(crate) c_anim_spin {
        font-size: var!("font-5xl");
        animation: "euv-spin 1.5s linear infinite";
        display: "inline-block";
        media("(max-width: 767px)") {
            font-size: var!("font-4xl");
        }
    }

    pub(crate) c_anim_spin_stopped {
        font-size: var!("font-5xl");
        display: "inline-block";
        opacity: "0.4";
        media("(max-width: 767px)") {
            font-size: var!("font-4xl");
        }
    }

    pub(crate) c_anim_pulse_container {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        margin: format!("{} 0px", var!(space-lg));
        min-height: "80px";
    }

    pub(crate) c_anim_pulse {
        font-size: var!("font-5xl");
        animation: "euv-pulse 1.5s ease-in-out infinite";
        display: "inline-block";
        color: var!(text-danger);
        media("(max-width: 767px)") {
            font-size: var!("font-4xl");
        }
    }

    pub(crate) c_anim_pulse_stopped {
        font-size: var!("font-5xl");
        display: "inline-block";
        opacity: "0.4";
        color: var!(text-danger);
        media("(max-width: 767px)") {
            font-size: var!("font-4xl");
        }
    }

    pub(crate) c_progress_container {
        width: "100%";
        height: "12px";
        background: var!(bg-progress);
        border-radius: var!(radius-pill);
        margin: format!("{} 0px", var!(space-lg));
        overflow: "hidden";
        transition: format!("background {} {}", var!(duration-slow), var!(ease-in-out));
    }

    pub(crate) c_progress_bar_running {
        height: "100%";
        border-radius: var!(radius-pill);
        background: var!(accent);
        animation: format!("euv-progress 1.6s {} forwards", var!(ease-in-out));
    }

    pub(crate) c_progress_bar_stopped {
        height: "100%";
        border-radius: var!(radius-pill);
        background: var!(accent);
        width: "0%";
    }

    pub(crate) c_anim_color_box {
        margin: format!("{} 0px", var!(gap-component));
        padding: var!(space-xl);
        border-radius: var!(radius-lg);
        color: var!(text-on-accent);
        text-align: "center";
        font-weight: "600";
        font-size: var!(font-lg);
        cursor: "pointer";
        transition: format!("background 0.5s {}, transform {} {}", var!(ease-in-out), var!(duration-normal), var!(ease-out));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Browser API Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_browser_api_row {
        display: "grid";
        grid-template-columns: "1fr 1fr";
        gap: format!("0px {}", var!(gap-component));
        margin-bottom: var!(gap-component);
        media("(max-width: 767px)") {
            grid-template-columns: "1fr";
        }
    }

    pub(crate) c_browser_api_actions {
        display: "flex";
        gap: var!(gap-element);
        flex-wrap: "wrap";
        margin: format!("{} 0px", var!(gap-component));
    }

    pub(crate) c_browser_result_box {
        margin-top: var!(gap-component);
        padding: format!("{} {}", "14px", var!(space-lg));
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-accent-light));
        font-size: var!(font-base);
        word-break: "break-all";
        color: "inherit";
    }

    pub(crate) c_browser_result_label {
        font-weight: "600";
        color: var!(accent);
    }

    pub(crate) c_browser_result_value {
        color: "inherit";
    }

    pub(crate) c_browser_info_grid {
        display: "grid";
        gap: var!(gap-component);
        margin-top: var!(gap-component);
        media("(max-width: 767px)") {
            grid-template-columns: "1fr";
        }
    }

    pub(crate) c_browser_info_item {
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-input));
        color: "inherit";
    }

    pub(crate) c_browser_info_label {
        font-size: var!(font-sm);
        font-weight: "600";
        color: "inherit";
        opacity: "0.6";
        text-transform: "uppercase";
        letter-spacing: "0.05em";
    }

    pub(crate) c_browser_info_value {
        font-size: var!(font-base);
        color: "inherit";
        word-break: "break-all";
        font-family: "ui-monospace, monospace";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // VConsole Panel
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_vconsole_badge {
        position: "absolute";
        top: "-6px";
        right: "-8px";
        min-width: "18px";
        height: "18px";
        border-radius: "9px";
        background: var!(bg-console-badge);
        color: var!(text-on-accent);
        font-size: "10px";
        font-weight: "600";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        padding: format!("0px {}", var!(space-xs));
        line-height: "1";
        box-shadow: "0px 2px 6px rgba(239, 68, 68, 0.3)";
        pointer-events: "none";
    }

    pub(crate) c_vconsole_overlay {
        position: "fixed";
        top: "0px";
        left: "0px";
        right: "0px";
        bottom: "0px";
        z-index: "10000";
        background: "rgba(0, 0, 0, 0.15)";
        transition: format!("opacity {} {}", var!(duration-normal), var!(ease-out));
    }

    pub(crate) c_vconsole_overlay_hidden {
        opacity: "0";
        pointer-events: "none";
    }

    pub(crate) c_vconsole_panel {
        position: "fixed";
        bottom: "0px";
        left: "0px";
        right: "0px";
        height: "66vh";
        background: var!(bg-console);
        z-index: "10001";
        display: "flex";
        flex-direction: "column";
        border-top: format!("3px solid {}", var!(border-console-accent));
        border-radius: format!("{} {} 0px 0px", var!(radius-xl), var!(radius-xl));
        box-shadow: var!(shadow-console-panel);
        padding-bottom: "env(safe-area-inset-bottom, 0px)";
        transition: format!("transform {} {}", var!(duration-normal), var!(ease-out));
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
        padding: format!("{} {}", "14px", var!(space-xl));
        background: var!(bg-console-header);
        border-bottom: format!("1px solid {}", var!(border-console));
        flex-shrink: "0";
    }

    pub(crate) c_vconsole_title {
        color: var!(text-console-title);
        font-size: var!(font-md);
        font-weight: "600";
        margin: "0px";
        letter-spacing: "0.03em";
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
    }

    pub(crate) c_vconsole_title_dot {
        width: "8px";
        height: "8px";
        border-radius: var!(space-xs);
        background: var!(text-console-log-latest);
        display: "inline-block";
        animation: "euv-pulse 2s ease-in-out infinite";
    }

    pub(crate) c_vconsole_header_actions {
        display: "flex";
        gap: var!(space-sm);
        align-items: "center";
    }

    pub(crate) c_vconsole_clear_button {
        background: "transparent";
        border: format!("1px solid {}", var!(border-console-button));
        color: var!(text-console-button);
        padding: "5px 14px";
        border-radius: var!(radius-sm);
        cursor: "pointer";
        font-size: var!(font-sm);
        font-weight: "500";
        transition: format!("all {} {}", var!(duration-fast), var!(ease-out));
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
        padding: format!("5px {}", var!(space-md));
        border-radius: var!(radius-sm);
        cursor: "pointer";
        font-size: var!(font-lg);
        line-height: "1";
        transition: format!("all {} {}", var!(duration-fast), var!(ease-out));
    }

    pub(crate) c_vconsole_close_button_hover {
        border-color: var!(text-console-close-hover);
        color: var!(text-console-close-hover);
        background: var!(bg-console-close-hover);
    }

    pub(crate) c_vconsole_body {
        flex: "1";
        overflow-y: "auto";
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        font-family: "ui-monospace, monospace";
        font-size: "13px";
        line-height: "1.6";
    }

    pub(crate) c_vconsole_log_item {
        padding: format!("{} 0px", var!(space-xs));
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
        padding: format!("{} 0px", var!("space-4xl"));
    }

    pub(crate) c_vconsole_count {
        color: var!(text-console-empty);
        font-size: var!(font-sm);
        font-weight: "400";
    }

    pub(crate) c_vconsole_filter_bar {
        display: "flex";
        gap: var!(space-sm);
        padding: format!("10px {}", var!(space-xl));
        background: var!(bg-console-filter);
        border-bottom: format!("1px solid {}", var!(border-console));
        flex-shrink: "0";
    }

    pub(crate) c_vconsole_filter_button {
        padding: format!("{} {}", var!(space-xs), var!(space-md));
        border-radius: var!(radius-sm);
        background: "transparent";
        border: format!("1px solid {}", var!(border-console-button));
        color: var!(text-console-button);
        font-size: var!(font-xs);
        font-weight: "500";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
        transition: format!("all {} {}", var!(duration-fast), var!(ease-out));
    }

    pub(crate) c_vconsole_filter_active {
        padding: format!("{} {}", var!(space-xs), var!(space-md));
        border-radius: var!(radius-sm);
        background: var!(bg-console-filter-active);
        border: format!("1px solid {}", var!(border-console-filter-active));
        color: var!(text-console-filter-active);
        font-size: var!(font-xs);
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
    }

    pub(crate) c_vconsole_filter_active_log {
        padding: format!("{} {}", var!(space-xs), var!(space-md));
        border-radius: var!(radius-sm);
        background: var!(bg-console-badge-log);
        border: format!("1px solid {}", var!(text-console-log-latest));
        color: var!(text-console-log-latest);
        font-size: var!(font-xs);
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
    }

    pub(crate) c_vconsole_filter_active_warn {
        padding: format!("{} {}", var!(space-xs), var!(space-md));
        border-radius: var!(radius-sm);
        background: var!(bg-console-badge-warn);
        border: format!("1px solid {}", var!(text-console-warn-latest));
        color: var!(text-console-warn-latest);
        font-size: var!(font-xs);
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
    }

    pub(crate) c_vconsole_filter_active_error {
        padding: format!("{} {}", var!(space-xs), var!(space-md));
        border-radius: var!(radius-sm);
        background: var!(bg-console-badge-error);
        border: format!("1px solid {}", var!(text-console-error-latest));
        color: var!(text-console-error-latest);
        font-size: var!(font-xs);
        font-weight: "600";
        cursor: "pointer";
        letter-spacing: "0.05em";
        text-transform: "uppercase";
    }

    pub(crate) c_vconsole_level_badge {
        display: "inline-block";
        padding: format!("2px 7px");
        border-radius: var!(space-xs);
        font-size: "10px";
        font-weight: "600";
        margin-right: var!(space-sm);
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

    // ═══════════════════════════════════════════════════════════════════════════
    // Mobile Layout
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_mobile_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        padding: format!("0px {}", var!(space-lg));
        height: var!(mobile-header-height);
        background: var!(bg-nav);
        flex-shrink: "0";
        position: "sticky";
        top: "0px";
        z-index: "100";
        transition: format!("background {} {}", var!(duration-slow), var!(ease-in-out));
    }

    pub(crate) c_mobile_header_left {
        display: "flex";
        align-items: "center";
        gap: format!("{}", var!(space-md));
    }

    pub(crate) c_mobile_header_logo {
        display: "flex";
        align-items: "center";
        gap: format!("{}", var!(space-sm));
        text-decoration: "none";
        color: var!(text-primary);
    }

    pub(crate) c_mobile_menu_button {
        width: "40px";
        height: "40px";
        border-radius: var!(radius-md);
        background: "transparent";
        border: "none";
        cursor: "pointer";
        font-size: "22px";
        font-weight: "700";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        color: var!(text-primary);
        transition: format!("background {} {}", var!(duration-fast), var!(ease-out));
        padding: "0px";
    }

    pub(crate) c_mobile_theme_button {
        width: "40px";
        height: "40px";
        border-radius: var!(radius-md);
        background: "transparent";
        border: "none";
        cursor: "pointer";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        transition: format!("all {} {}", var!(duration-normal), var!(ease-out));
        padding: "0px";
    }

    pub(crate) c_mobile_menu_button_active {
        width: "40px";
        height: "40px";
        border-radius: var!(radius-md);
        background: var!(accent-subtle);
        border: "none";
        cursor: "pointer";
        font-size: "22px";
        font-weight: "700";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        color: var!(accent);
        transition: format!("background {} {}", var!(duration-fast), var!(ease-out));
        padding: "0px";
    }

    pub(crate) c_mobile_overlay {
        position: "fixed";
        top: "0px";
        left: "0px";
        width: "100%";
        height: "100%";
        background: "rgba(0, 0, 0, 0.4)";
        z-index: "200";
        animation: format!("euv-fade-in {} {}", var!(duration-fast), var!(ease-out));
    }

    pub(crate) c_mobile_nav_drawer {
        position: "fixed";
        top: "0px";
        left: "0px";
        width: "280px";
        height: "100%";
        background: var!(bg-nav);
        z-index: "201";
        display: "flex";
        flex-direction: "column";
        padding-top: "env(safe-area-inset-top, 0px)";
        padding-bottom: "env(safe-area-inset-bottom, 0px)";
        animation: format!("euv-slide-left {} {}", var!(duration-normal), var!(ease-out));
        box-shadow: var!(shadow-drawer);
    }

    pub(crate) c_mobile_nav_drawer_header {
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        padding: format!("0px {}", var!(space-lg));
        height: var!(mobile-header-height);
        flex-shrink: "0";
    }

    pub(crate) c_mobile_main {
        flex: "1";
        padding: format!("{} {} {} {}", var!(space-md), var!(space-lg), "calc(16px + env(safe-area-inset-bottom, 0px))", var!(space-lg));
        background: var!(bg-primary);
        overflow-y: "auto";
        transition: format!("background {} {}", var!(duration-slow), var!(ease-in-out));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Keep-Alive Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_keep_alive_tab_bar {
        display: "flex";
        border-bottom: format!("1px solid {}", var!(border-input));
        margin-bottom: var!(gap-component);
        gap: var!(gap-element);
    }

    pub(crate) c_keep_alive_tab_panel {
        padding: format!("{} 0px", var!(gap-element));
    }

    pub(crate) c_keep_alive_panel_title {
        margin-top: "0px";
        color: var!(accent);
        font-size: var!(font-md);
        margin-bottom: var!(gap-element);
    }

    pub(crate) c_keep_alive_demo_text {
        color: "inherit";
        font-size: var!(font-base);
        margin-bottom: var!(gap-component);
    }

    pub(crate) c_keep_alive_counter_display {
        display: "flex";
        justify-content: "center";
        align-items: "center";
        margin: format!("{} 0px", var!(space-xl));
    }

    pub(crate) c_keep_alive_counter_value {
        font-size: var!("font-5xl");
        font-weight: "700";
        font-variant-numeric: "tabular-nums";
        color: var!(accent);
        min-width: "120px";
        text-align: "center";
        transition: format!("color {} {}", var!(duration-slow), var!(ease-in-out));
    }

    pub(crate) c_keep_alive_counter_controls {
        display: "flex";
        gap: format!("{}", var!(space-md));
        justify-content: "center";
        media("(max-width: 767px)") {
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_keep_alive_form_group {
        margin-bottom: var!(gap-component);
    }

    pub(crate) c_keep_alive_form_preview {
        margin-top: var!(gap-component);
        padding: format!("{} {}", var!(gap-component), var!(space-lg));
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-accent-light));
        background: var!(accent-muted);
    }

    pub(crate) c_keep_alive_preview_label {
        font-size: "13px";
        font-weight: "600";
        color: var!(accent);
        margin: format!("0px 0px {} 0px", var!(space-xs));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // File Upload
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_file_upload_input {
        width: "100%";
        padding: "10px 14px";
        border: format!("1px solid {}", var!(border-input));
        border-radius: var!(radius-md);
        font-size: var!(font-base);
        box-sizing: "border-box";
        background: var!(bg-input);
        color: var!(text-primary);
        cursor: "pointer";
    }

    pub(crate) c_file_upload_input_hidden {
        position: "absolute";
        width: "1px";
        height: "1px";
        padding: "0px";
        margin: "-1px";
        overflow: "hidden";
        clip: "rect(0, 0, 0, 0)";
        white-space: "nowrap";
        border: "0px";
    }

    pub(crate) c_file_upload_options {
        margin: format!("{} 0px", var!(gap-component));
        padding: format!("{} 0px", var!(gap-component));
        border-top: format!("1px solid {}", var!(border-subtle));
        border-bottom: format!("1px solid {}", var!(border-subtle));
    }

    pub(crate) c_file_upload_item {
        display: "flex";
        align-items: "center";
        gap: format!("{}", var!(space-md));
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border-radius: var!(radius-sm);
        background: var!(bg-input);
        margin: format!("{} 0px", var!(space-sm));
        font-size: var!(font-base);
        color: "inherit";
    }

    pub(crate) c_file_upload_item_index {
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(accent);
        min-width: "24px";
    }

    pub(crate) c_file_upload_item_name {
        color: "inherit";
        word-break: "break-all";
    }

    pub(crate) c_file_upload_drop_zone {
        border: format!("2px dashed {}", var!(border-input));
        border-radius: var!(radius-lg);
        padding: format!("{} {}", var!("space-4xl"), var!(space-xl));
        text-align: "center";
        cursor: "pointer";
        transition: format!("all {} {}", var!(duration-normal), var!(ease-out));
        background: "transparent";
    }

    pub(crate) c_file_upload_drop_zone_active {
        border: format!("2px dashed {}", var!(accent));
        border-radius: var!(radius-lg);
        padding: format!("{} {}", var!("space-4xl"), var!(space-xl));
        text-align: "center";
        cursor: "pointer";
        transition: format!("all {} {}", var!(duration-normal), var!(ease-out));
        background: var!(accent-muted);
    }

    pub(crate) c_file_upload_drop_icon {
        font-size: var!("font-5xl");
        display: "block";
        margin-bottom: var!(space-md);
    }

    pub(crate) c_file_upload_drop_text {
        font-size: var!(font-lg);
        font-weight: "500";
        color: "inherit";
        margin: format!("0px 0px {} 0px", var!(space-sm));
    }

    pub(crate) c_file_upload_drop_hint {
        font-size: "13px";
        color: "inherit";
        opacity: "0.5";
        margin: "0px";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Binding Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_binding_child_box {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-component);
        margin-top: var!(gap-component);
        padding: format!("{} {} {} {}", var!(space-lg), var!(space-lg), var!(gap-component), var!(space-lg));
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-accent-light));
        background: var!(accent-muted);
    }

    pub(crate) c_binding_child_label {
        font-size: "13px";
        font-weight: "600";
        color: var!(accent);
        text-transform: "uppercase";
        letter-spacing: "0.05em";
    }

    pub(crate) c_binding_child_message {
        color: "inherit";
        font-size: var!(font-md);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border-radius: var!(radius-sm);
        background: var!(accent-subtle);
        font-weight: "500";
    }

    pub(crate) c_binding_parent_box {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-component);
        padding: format!("{} {} {} {}", var!(space-lg), var!(space-lg), var!(gap-component), var!(space-lg));
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-success));
        background: var!(bg-success);
    }

    pub(crate) c_binding_section_title {
        margin-top: var!(gap-component);
        margin-bottom: var!(gap-element);
        color: var!(accent);
        font-size: var!(font-md);
        font-weight: "600";
    }

    pub(crate) c_binding_temp_converter {
        display: "flex";
        align-items: "flex-end";
        gap: var!(gap-component);
        flex-wrap: "wrap";
        margin-top: var!(gap-component);
    }

    pub(crate) c_binding_temp_field {
        flex: "1";
        min-width: "120px";
    }

    pub(crate) c_binding_temp_arrow {
        font-size: "20px";
        font-weight: "700";
        color: var!(accent);
        padding-bottom: "10px";
    }

    pub(crate) c_binding_color_mixer {
        margin-top: var!(gap-component);
    }

    pub(crate) c_binding_color_preview {
        width: "100%";
        height: "80px";
        border-radius: "10px";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        margin-bottom: var!(gap-component);
        border: format!("1px solid {}", var!(border-subtle));
        transition: format!("background {} {}", var!(duration-normal), var!(ease-out));
    }

    pub(crate) c_binding_color_hex {
        font-family: "ui-monospace, monospace";
        font-size: var!(font-xl);
        font-weight: "700";
        color: var!(text-on-accent);
        text-shadow: "0px 1px 3px rgba(0, 0, 0, 0.5)";
        letter-spacing: "0.02em";
    }

    pub(crate) c_binding_slider_row {
        display: "flex";
        align-items: "center";
        gap: format!("{}", var!(space-md));
        margin-bottom: var!(space-sm);
    }

    pub(crate) c_binding_slider_label {
        font-size: var!(font-base);
        font-weight: "700";
        min-width: "16px";
    }

    pub(crate) c_binding_slider {
        flex: "1";
        height: "6px";
        cursor: "pointer";
        accent-color: var!(accent);
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
        color: var!(accent);
        padding: format!("1px {}", var!(space-sm));
        border-radius: var!(space-xs);
        background: var!(accent-subtle);
    }

    pub(crate) c_binding_typed_prop_group {
        display: "flex";
        align-items: "center";
        gap: var!(space-sm);
    }

    pub(crate) c_binding_typed_warning {
        font-size: "13px";
        font-weight: "500";
        color: var!(text-error);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        border-radius: var!(radius-sm);
        background: var!(bg-error);
        border: format!("1px solid {}", var!(border-error));
        margin-bottom: "0px";
    }

    pub(crate) c_binding_callback_form {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-component);
    }

    pub(crate) c_binding_form_label {
        display: "block";
        color: "inherit";
        font-weight: "500";
        font-size: var!(font-base);
    }

    pub(crate) c_binding_demo_text {
        color: "inherit";
        margin: "0px"
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Custom Attrs - Dynamic Style
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_attrs_dynamic_demo(prop_key: &str, prop_value: &str) {
        { prop_key }: prop_value;
        padding: var!(space-lg);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-input));
        color: "inherit";
        margin-top: var!(space-md);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Dynamic Component Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_dynamic_component_tab_bar {
        display: "flex";
        gap: var!(gap-element);
        margin-bottom: var!(gap-component);
        flex-wrap: "wrap";
    }

    pub(crate) c_dynamic_component_panel {
        display: "block";
        padding: var!(space-lg);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-subtle));
        background: var!(bg-primary);
        min-height: "48px";
        margin-top: var!(gap-component);
    }

    pub(crate) c_dynamic_component_panel_title {
        margin: format!("0px 0px {} 0px", var!(gap-component));
        font-size: var!(font-lg);
        font-weight: "600";
        color: var!(text-primary);
    }

    pub(crate) c_dynamic_component_inline {
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        border-radius: var!(radius-sm);
        background: var!(accent-subtle);
        border: format!("1px solid {}", var!(accent-border));
        color: var!(accent);
        font-weight: "500";
        font-size: var!(font-base);
    }

    pub(crate) c_dynamic_component_inline_alt {
        padding: format!("{} {}", var!(space-md), var!(space-lg));
        border-radius: var!(radius-sm);
        background: var!(bg-success);
        border: format!("1px solid {}", var!(border-success));
        color: var!(text-positive);
        font-weight: "500";
        font-size: var!(font-base);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Virtual List
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_virtual_list_status {
        display: "flex";
        gap: var!(gap-component);
        padding: format!("{} 0px", var!(gap-component));
        margin-bottom: var!(gap-component);
        border-bottom: format!("1px solid {}", var!(border-subtle));
        flex-wrap: "nowrap";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        media("(max-width: 767px)") {
            gap: var!(gap-component-mobile);
        }
    }

    pub(crate) c_virtual_list_status_item {
        font-size: var!(font-base);
        color: "inherit";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        min-width: "0px";
    }

    pub(crate) c_virtual_list_status_value {
        font-weight: "700";
        color: var!(accent);
        font-family: "ui-monospace, monospace";
    }

    pub(crate) c_virtual_list_container {
        flex: "1";
        min-height: "0px";
        overflow-y: "auto";
        border: format!("1px solid {}", var!(border-input));
        border-radius: var!(radius-md);
        background: var!(bg-input);
    }

    pub(crate) c_virtual_list_card {
        background: var!(bg-card);
        border-radius: var!(radius-lg);
        padding: var!(space-xl);
        margin-top: "0px";
        margin-bottom: var!(gap-section);
        box-shadow: var!(shadow-card);
        border: format!("1px solid {}", var!(border-card));
        color: var!(text-card);
        transition: format!("box-shadow {} {}", var!(duration-normal), var!(ease-out));
        display: "flex";
        flex-direction: "column";
        flex: "1";
        min-height: "0px";
        media("(max-width: 767px)") {
            padding: var!(space-lg);
            margin-bottom: var!(gap-section-mobile);
            border-radius: "10px";
        }
    }

    pub(crate) c_virtual_list_row_even {
        display: "flex";
        align-items: "center";
        padding: format!("0px {}", var!(space-lg));
        gap: var!(space-lg);
        border-bottom: format!("1px solid {}", var!(border-subtle));
        background: var!(bg-list-even);
    }

    pub(crate) c_virtual_list_row_odd {
        display: "flex";
        align-items: "center";
        padding: format!("0px {}", var!(space-lg));
        gap: var!(space-lg);
        border-bottom: format!("1px solid {}", var!(border-subtle));
        background: var!(bg-list-odd);
    }

    pub(crate) c_virtual_list_row_index {
        min-width: "48px";
        font-size: var!(font-sm);
        font-weight: "600";
        color: var!(accent);
        font-family: "ui-monospace, monospace";
        flex-shrink: "0";
    }

    pub(crate) c_virtual_list_row_label {
        font-size: var!(font-base);
        font-weight: "500";
        color: "inherit";
        min-width: "120px";
        flex-shrink: "0";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
    }

    pub(crate) c_virtual_list_row_description {
        font-size: "13px";
        color: "inherit";
        opacity: "0.6";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        flex: "1";
        min-width: "0px";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Unified Layout Primitives
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_section_stack {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-section);
        media("(max-width: 767px)") {
            gap: var!(gap-section-mobile);
        }
    }

    pub(crate) c_component_stack {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-component);
        media("(max-width: 767px)") {
            gap: var!(gap-component-mobile);
        }
    }

    pub(crate) c_element_stack {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
    }

    pub(crate) c_cluster {
        display: "flex";
        gap: var!(gap-element);
        flex-wrap: "wrap";
        align-items: "center";
        media("(max-width: 767px)") {
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_cluster_center {
        display: "flex";
        gap: var!(gap-element);
        flex-wrap: "wrap";
        align-items: "center";
        justify-content: "center";
        media("(max-width: 767px)") {
            flex-wrap: "wrap";
        }
    }

    pub(crate) c_inline_cluster {
        display: "flex";
        gap: var!(gap-inline);
        flex-wrap: "wrap";
        align-items: "center";
    }

    pub(crate) c_grid_2col {
        display: "grid";
        grid-template-columns: "1fr 1fr";
        gap: format!("0px {}", var!(gap-component));
        media("(max-width: 767px)") {
            grid-template-columns: "1fr";
        }
    }

    pub(crate) c_switcher {
        display: "flex";
        gap: var!(gap-component);
        flex-wrap: "wrap";
        media("(max-width: 767px)") {
            flex-direction: "column";
            gap: var!(gap-component-mobile);
        }
    }

    pub(crate) c_switcher_col {
        flex: "1";
        min-width: "200px";
        media("(max-width: 767px)") {
            min-width: "100%";
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Unified Description / Hint Text
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_description {
        color: "inherit";
        margin-bottom: var!(gap-element);
    }

    pub(crate) c_hint {
        color: "inherit";
        opacity: "0.6";
        font-size: "13px";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SSE Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_sse_url_row {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        gap: var!(gap-component);
        margin-bottom: var!(gap-component);
    }

    pub(crate) c_sse_url_input {
        flex: "1";
        min-width: "0px";
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        background: var!(bg-input);
        color: var!(text-primary);
        border: format!("1px solid {}", var!(border-input));
        border-radius: var!(radius-md);
        font-size: var!(font-base);
        outline: "none";
        transition: format!("border-color {} {}", var!(duration-fast), var!(ease-out));
    }

    pub(crate) c_sse_disconnect_button {
        c_primary_button();
        background: var!(bg-error);
        color: var!(text-error);
        border: format!("1px solid {}", var!(border-error));
        flex-shrink: "0";
        max-width: "120px";
    }

    pub(crate) c_sse_action_button {
        flex-shrink: "0";
        max-width: "120px";
    }

    pub(crate) c_sse_messages_header {
        display: "flex";
        flex-direction: "column";
        gap: var!(gap-element);
        margin-bottom: var!(gap-component);
    }

    pub(crate) c_sse_message_count {
        color: "inherit";
        opacity: "0.6";
        font-size: var!(font-sm);
    }

    pub(crate) c_sse_messages_empty {
        color: "inherit";
        opacity: "0.5";
        padding: var!(space-xl);
        text-align: "center";
        font-size: var!(font-base);
    }

    pub(crate) c_sse_messages_list {
        max-height: "400px";
        overflow-y: "auto";
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
    }

    pub(crate) c_sse_message_item {
        display: "flex";
        align-items: "baseline";
        gap: var!(gap-element);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        background: var!(bg-loading);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-loading));
    }

    pub(crate) c_sse_message_index {
        color: var!(accent);
        font-size: var!(font-sm);
        font-weight: "600";
        white-space: "nowrap";
        flex-shrink: "0";
    }

    pub(crate) c_sse_message_data {
        color: "inherit";
        font-size: var!(font-base);
        word-break: "break-all";
        overflow-wrap: "break-word";
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // WebSocket Demo
    // ═══════════════════════════════════════════════════════════════════════════

    pub(crate) c_ws_send_row {
        display: "flex";
        justify-content: "space-between";
        align-items: "center";
        gap: var!(gap-component);
    }

    pub(crate) c_ws_message_input {
        flex: "1";
        min-width: "0px";
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        background: var!(bg-input);
        color: var!(text-primary);
        border: format!("1px solid {}", var!(border-input));
        border-radius: var!(radius-md);
        font-size: var!(font-base);
        outline: "none";
        transition: format!("border-color {} {}", var!(duration-fast), var!(ease-out));
    }

    pub(crate) c_ws_messages_list {
        max-height: "400px";
        overflow-y: "auto";
        display: "flex";
        flex-direction: "column";
        gap: var!(space-xs);
    }

    pub(crate) c_ws_message_item {
        display: "flex";
        align-items: "baseline";
        gap: var!(gap-element);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        background: var!(bg-loading);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-loading));
        min-width: "0px";
    }

    pub(crate) c_ws_message_data {
        color: "inherit";
        font-size: var!(font-base);
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        flex: "1";
        min-width: "0px";
    }

    pub(crate) c_ws_message_time {
        color: var!(text-tertiary);
        font-size: var!(font-sm);
        white-space: "nowrap";
        flex-shrink: "0";
        margin-left: "auto";
    }

    pub(crate) c_ws_message_sent {
        display: "flex";
        align-items: "flex-start";
        gap: var!(gap-element);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        background: var!(bg-success);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-success));
        margin-left: "auto";
        max-width: "80%";
    }

    pub(crate) c_ws_message_received {
        display: "flex";
        align-items: "flex-start";
        gap: var!(gap-element);
        padding: format!("{} {}", var!(space-sm), var!(space-md));
        background: var!(bg-loading);
        border-radius: var!(radius-md);
        border: format!("1px solid {}", var!(border-loading));
        margin-right: "auto";
        max-width: "80%";
    }

    pub(crate) c_ws_message_direction {
        font-size: var!(font-sm);
        font-weight: "600";
        white-space: "nowrap";
        flex-shrink: "0";
        opacity: "0.7";
    }

    pub(crate) c_ws_message_content {
        color: "inherit";
        font-size: var!(font-base);
        word-break: "break-all";
        overflow-wrap: "break-word";
    }
}
