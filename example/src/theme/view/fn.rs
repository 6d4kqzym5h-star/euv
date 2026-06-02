use crate::*;

css_vars! {
    pub(crate) c_theme_light {
        // ─── Spacing Scale ───
        space-xs: "4px";
        space-sm: "8px";
        space-md: "12px";
        space-lg: "16px";
        space-xl: "20px";
        "space-2xl": "24px";
        "space-3xl": "32px";
        "space-4xl": "40px";
        "space-5xl": "48px";
        "space-6xl": "64px";
        "space-7xl": "80px";

        // ─── Border Radius Scale ───
        radius-sm: "6px";
        radius-md: "8px";
        radius-lg: "12px";
        radius-xl: "16px";
        radius-pill: "9999px";

        // ─── Font Size Scale ───
        font-xs: "11px";
        font-sm: "12px";
        font-base: "14px";
        font-md: "15px";
        font-lg: "16px";
        font-xl: "18px";
        "font-2xl": "22px";
        "font-3xl": "28px";
        "font-4xl": "36px";
        "font-5xl": "48px";
        "font-6xl": "72px";

        // ─── Transition Durations ───
        duration-fast: "0.15s";
        duration-normal: "0.25s";
        duration-slow: "0.4s";

        // ─── Easing Functions ───
        ease-out: "cubic-bezier(0.16, 1, 0.3, 1)";
        ease-in-out: "cubic-bezier(0.4, 0, 0.2, 1)";

        // ─── Layout ───
        padding-main-top: "20px";
        gap-page-header: "12px";
        gap-page-title: "4px";
        gap-card: "12px";
        gap-card-mobile: "8px";
        nav-width: "240px";
        content-max-width: "800px";
        mobile-header-height: "56px";

        // ─── Surface Colors ───
        bg-primary: "#f8f9fb";
        bg-nav: "#ffffff";
        bg-card: "#ffffff";
        bg-modal: "#ffffff";
        bg-input: "#fafbfc";
        bg-overlay: "rgba(0, 0, 0, 0.5)";

        // ─── Text Colors ───
        text-primary: "#1a1a2e";
        text-secondary: "#4b5563";
        text-muted: "#9ca3af";
        text-nav-item: "#6b7280";
        text-card: "#1a1a2e";
        text-on-accent: "#ffffff";

        // ─── Brand / Accent ───
        accent: "#4f46e5";
        accent-hover: "#4338ca";
        accent-subtle: "rgba(79, 70, 229, 0.08)";
        accent-muted: "rgba(79, 70, 229, 0.04)";
        accent-gradient: "linear-gradient(135deg, #4f46e5, #7c3aed)";

        // ─── Border Colors ───
        border-nav: "#e5e7eb";
        border-subtle: "#f0f0f0";
        border-card: "#f0f0f0";
        border-card-title: "#f0f0f0";
        border-input: "#e5e7eb";
        border-accent-light: "#e0e7ff";

        // ─── Shadows ───
        shadow-card: "0 1px 3px rgba(0,0,0,0.06), 0 4px 12px rgba(0,0,0,0.04)";
        shadow-modal: "0 20px 60px rgba(0,0,0,0.3)";
        shadow-float: "0 4px 14px rgba(79, 70, 229, 0.35)";
        shadow-drawer: "4px 0px 20px rgba(0, 0, 0, 0.15)";

        // ─── Theme Toggle ───
        bg-theme-button: "#f3f4f6";
        text-theme-button: "#1a1a2e";
        border-theme-button: "#e5e7eb";

        // ─── Status: Error ───
        bg-error: "#fef2f2";
        text-error: "#991b1b";
        border-error: "#fecaca";
        bg-error-icon: "#fca5a5";

        // ─── Status: Success ───
        bg-success: "#f0fdf4";
        text-success: "#166534";
        border-success: "#bbf7d0";

        // ─── Status: Warning ───
        bg-warning: "#fef3c7";
        text-warning: "#92400e";
        border-warning: "#fde68a";

        // ─── Status: Info ───
        text-info: "#075985";
        border-info: "#bae6fd";

        // ─── Status: Pink ───
        text-pink: "#9d174d";
        border-pink: "#f9a8d4";

        // ─── Loading ───
        bg-loading: "#eef2ff";
        border-loading: "#c7d2fe";
        text-loading-title: "#3730a3";
        bg-progress: "#e5e7eb";

        // ─── List ───
        bg-list-even: "#fafbfc";
        bg-list-odd: "#ffffff";

        // ─── Console / VConsole ───
        bg-console: "#ffffff";
        bg-console-header: "#eef0f6";
        bg-console-filter: "#eaecf4";
        border-console: "#dde0ea";
        text-console: "#2c3048";
        text-console-title: "#4f46e5";
        border-console-button: "#cdd1de";
        text-console-button: "#5b6078";
        text-console-log-latest: "#059669";
        text-console-warn: "#b45309";
        text-console-warn-latest: "#d97706";
        text-console-error: "#dc2626";
        text-console-error-latest: "#ef4444";
        text-console-empty: "#9ca3af";
        bg-console-badge: "#ef4444";
        bg-console-badge-log: "rgba(5, 150, 105, 0.1)";
        bg-console-badge-warn: "rgba(217, 119, 6, 0.1)";
        bg-console-badge-error: "rgba(239, 68, 68, 0.1)";
        border-console-accent: "#4f46e5";
        text-console-filter-active: "#4f46e5";
        border-console-filter-active: "#4f46e5";
        bg-console-filter-active: "rgba(79, 70, 229, 0.08)";
        bg-console-button: "#4f46e5";
        text-console-button-text: "white";
        shadow-console-button: "0 4px 14px rgba(79, 70, 229, 0.35)";
        bg-console-button-hover: "#4338ca";
        shadow-console-panel: "0 -8px 30px rgba(0, 0, 0, 0.08)";
        bg-console-close-hover: "rgba(239, 68, 68, 0.08)";
        text-console-close-hover: "#ef4444";
    }

    pub(crate) c_theme_dark {
        // ─── Spacing Scale (same as light) ───
        space-xs: "4px";
        space-sm: "8px";
        space-md: "12px";
        space-lg: "16px";
        space-xl: "20px";
        "space-2xl": "24px";
        "space-3xl": "32px";
        "space-4xl": "40px";
        "space-5xl": "48px";
        "space-6xl": "64px";
        "space-7xl": "80px";

        // ─── Border Radius Scale (same as light) ───
        radius-sm: "6px";
        radius-md: "8px";
        radius-lg: "12px";
        radius-xl: "16px";
        radius-pill: "9999px";

        // ─── Font Size Scale (same as light) ───
        font-xs: "11px";
        font-sm: "12px";
        font-base: "14px";
        font-md: "15px";
        font-lg: "16px";
        font-xl: "18px";
        "font-2xl": "22px";
        "font-3xl": "28px";
        "font-4xl": "36px";
        "font-5xl": "48px";
        "font-6xl": "72px";

        // ─── Transition Durations (same as light) ───
        duration-fast: "0.15s";
        duration-normal: "0.25s";
        duration-slow: "0.4s";

        // ─── Easing Functions (same as light) ───
        ease-out: "cubic-bezier(0.16, 1, 0.3, 1)";
        ease-in-out: "cubic-bezier(0.4, 0, 0.2, 1)";

        // ─── Layout (same as light) ───
        padding-main-top: "20px";
        gap-page-header: "12px";
        gap-page-title: "4px";
        gap-card: "12px";
        gap-card-mobile: "8px";
        nav-width: "240px";
        content-max-width: "800px";
        mobile-header-height: "56px";

        // ─── Surface Colors ───
        bg-primary: "#0f0f1a";
        bg-nav: "#16162a";
        bg-card: "#1e1e36";
        bg-modal: "#1e1e36";
        bg-input: "#252540";
        bg-overlay: "rgba(0, 0, 0, 0.6)";

        // ─── Text Colors ───
        text-primary: "#e0e0e0";
        text-secondary: "#b0b0c0";
        text-muted: "#6b7280";
        text-nav-item: "#9ca3af";
        text-card: "#e0e0e0";
        text-on-accent: "#ffffff";

        // ─── Brand / Accent ───
        accent: "#818cf8";
        accent-hover: "#a5b4fc";
        accent-subtle: "rgba(129, 140, 248, 0.12)";
        accent-muted: "rgba(129, 140, 248, 0.06)";
        accent-gradient: "linear-gradient(135deg, #6366f1, #8b5cf6)";

        // ─── Border Colors ───
        border-nav: "#2d2d4a";
        border-subtle: "#2d2d4a";
        border-card: "#2d2d4a";
        border-card-title: "#2d2d4a";
        border-input: "#3d3d5a";
        border-accent-light: "#3d3d5a";

        // ─── Shadows ───
        shadow-card: "0 1px 3px rgba(0,0,0,0.2), 0 4px 12px rgba(0,0,0,0.15)";
        shadow-modal: "0 20px 60px rgba(0,0,0,0.5)";
        shadow-float: "0 4px 14px rgba(124, 58, 237, 0.3)";
        shadow-drawer: "4px 0px 20px rgba(0, 0, 0, 0.4)";

        // ─── Theme Toggle ───
        bg-theme-button: "#2d2d4a";
        text-theme-button: "#fbbf24";
        border-theme-button: "#3d3d5a";

        // ─── Status: Error ───
        bg-error: "#451a1a";
        text-error: "#fca5a5";
        border-error: "#7f1d1d";
        bg-error-icon: "#7f1d1d";

        // ─── Status: Success ───
        bg-success: "#052e16";
        text-success: "#86efac";
        border-success: "#166534";

        // ─── Status: Warning ───
        bg-warning: "#451a03";
        text-warning: "#fbbf24";
        border-warning: "#92400e";

        // ─── Status: Info ───
        text-info: "#7dd3fc";
        border-info: "#075985";

        // ─── Status: Pink ───
        text-pink: "#f9a8d4";
        border-pink: "#9d174d";

        // ─── Loading ───
        bg-loading: "#1e1b4b";
        border-loading: "#312e81";
        text-loading-title: "#a5b4fc";
        bg-progress: "#3d3d5a";

        // ─── List ───
        bg-list-even: "#252540";
        bg-list-odd: "#1e1e36";

        // ─── Console / VConsole ───
        bg-console: "#0a0a18";
        bg-console-header: "#08081a";
        bg-console-filter: "#060616";
        border-console: "#1e1e3a";
        text-console: "#9a9ab0";
        text-console-title: "#c4b5fd";
        border-console-button: "#2d2d4a";
        text-console-button: "#8a8aa0";
        text-console-log-latest: "#6ee7b7";
        text-console-warn: "#fbbf24";
        text-console-warn-latest: "#fcd34d";
        text-console-error: "#f87171";
        text-console-error-latest: "#fca5a5";
        text-console-empty: "#52525b";
        bg-console-badge: "#dc2626";
        bg-console-badge-log: "rgba(110, 231, 183, 0.15)";
        bg-console-badge-warn: "rgba(251, 191, 36, 0.15)";
        bg-console-badge-error: "rgba(248, 113, 113, 0.15)";
        border-console-accent: "#a78bfa";
        text-console-filter-active: "#c4b5fd";
        border-console-filter-active: "#a78bfa";
        bg-console-filter-active: "rgba(196, 181, 253, 0.15)";
        bg-console-button: "#7c3aed";
        text-console-button-text: "white";
        shadow-console-button: "0 4px 14px rgba(124, 58, 237, 0.3)";
        bg-console-button-hover: "#8b5cf6";
        shadow-console-panel: "0 -8px 30px rgba(0, 0, 0, 0.5)";
        bg-console-close-hover: "rgba(239, 68, 68, 0.15)";
        text-console-close-hover: "#f87171";
    }
}
