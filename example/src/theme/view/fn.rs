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
        // Elevation: primary(0) < nav(1) = card(2) < input inset
        // Clean warm-neutral palette
        bg-primary: "#f7f8fa";
        bg-nav: "#ffffff";
        bg-card: "#ffffff";
        bg-modal: "#ffffff";
        bg-input: "#f3f4f6";
        bg-overlay: "rgba(0, 0, 0, 0.45)";

        // ─── Text Colors ───
        // Clear hierarchy: primary > secondary > muted
        text-primary: "#111827";
        text-secondary: "#4b5563";
        text-muted: "#9ca3af";
        text-nav-item: "#6b7280";
        text-card: "#111827";
        text-on-accent: "#ffffff";

        // ─── Brand / Accent ───
        // Indigo family: consistent hue across light/dark
        accent: "#4f46e5";
        accent-hover: "#4338ca";
        accent-subtle: "rgba(79, 70, 229, 0.06)";
        accent-muted: "rgba(79, 70, 229, 0.03)";
        accent-gradient: "linear-gradient(135deg, #4f46e5, #6366f1)";
        accent-disabled: "#a5a5cc";
        accent-border: "rgba(79, 70, 229, 0.30)";

        // ─── Semantic Colors (non-status) ───
        text-danger: "#dc2626";
        text-positive: "#059669";
        border-interactive: "rgba(79, 70, 229, 0.35)";

        // ─── Border Colors ───
        // Layered: subtle < card < nav = input (increasing visibility)
        border-nav: "#e5e7eb";
        border-subtle: "#f0f1f3";
        border-card: "#e8e9ec";
        border-card-title: "#ecedf0";
        border-input: "#d1d5db";
        border-accent-light: "rgba(79, 70, 229, 0.15)";

        // ─── Shadows ───
        shadow-card: "0 1px 3px rgba(0,0,0,0.04), 0 4px 12px rgba(0,0,0,0.03)";
        shadow-modal: "0 20px 60px rgba(0,0,0,0.2)";
        shadow-float: "0 4px 14px rgba(79, 70, 229, 0.25)";
        shadow-drawer: "4px 0px 20px rgba(0, 0, 0, 0.1)";
        shadow-accent-sm: "0px 1px 2px rgba(79, 70, 229, 0.2)";
        shadow-accent-lg: "0px 6px 20px rgba(79, 70, 229, 0.35)";

        // ─── Theme Toggle ───
        bg-theme-button: "#f3f4f6";
        text-theme-button: "#111827";
        border-theme-button: "#e5e7eb";

        // ─── Status: Error ───
        bg-error: "rgba(239, 68, 68, 0.06)";
        text-error: "#dc2626";
        border-error: "rgba(239, 68, 68, 0.20)";
        bg-error-icon: "rgba(239, 68, 68, 0.15)";

        // ─── Status: Success ───
        bg-success: "rgba(34, 197, 94, 0.06)";
        text-success: "#16a34a";
        border-success: "rgba(34, 197, 94, 0.20)";

        // ─── Status: Warning ───
        bg-warning: "rgba(245, 158, 11, 0.08)";
        text-warning: "#d97706";
        border-warning: "rgba(245, 158, 11, 0.25)";

        // ─── Status: Info ───
        text-info: "#0284c7";
        border-info: "rgba(14, 165, 233, 0.20)";

        // ─── Status: Pink ───
        text-pink: "#db2777";
        border-pink: "rgba(236, 72, 153, 0.20)";

        // ─── Loading ───
        bg-loading: "rgba(79, 70, 229, 0.05)";
        border-loading: "rgba(79, 70, 229, 0.15)";
        text-loading-title: "#4338ca";
        bg-progress: "#e5e7eb";

        // ─── List ───
        bg-list-even: "#f9fafb";
        bg-list-odd: "#ffffff";

        // ─── Console / VConsole ───
        bg-console: "#ffffff";
        bg-console-header: "#f9fafb";
        bg-console-filter: "#f3f4f6";
        border-console: "#e5e7eb";
        text-console: "#1f2937";
        text-console-title: "#4f46e5";
        border-console-button: "#d1d5db";
        text-console-button: "#4b5563";
        text-console-log-latest: "#059669";
        text-console-warn: "#d97706";
        text-console-warn-latest: "#b45309";
        text-console-error: "#dc2626";
        text-console-error-latest: "#b91c1c";
        text-console-empty: "#9ca3af";
        bg-console-badge: "#ef4444";
        bg-console-badge-log: "rgba(5, 150, 105, 0.08)";
        bg-console-badge-warn: "rgba(217, 119, 6, 0.08)";
        bg-console-badge-error: "rgba(239, 68, 68, 0.08)";
        border-console-accent: "#4f46e5";
        text-console-filter-active: "#4f46e5";
        border-console-filter-active: "#4f46e5";
        bg-console-filter-active: "rgba(79, 70, 229, 0.06)";
        bg-console-button: "#4f46e5";
        text-console-button-text: "white";
        shadow-console-button: "0 4px 14px rgba(79, 70, 229, 0.25)";
        bg-console-button-hover: "#4338ca";
        shadow-console-panel: "0 -8px 30px rgba(0, 0, 0, 0.06)";
        bg-console-close-hover: "rgba(239, 68, 68, 0.06)";
        text-console-close-hover: "#dc2626";
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
        // Elevation: primary(0) < nav(1) < card(2) < input(3)
        // Using neutral-cool base with subtle blue undertone
        bg-primary: "#111118";
        bg-nav: "#18181f";
        bg-card: "#1f1f28";
        bg-modal: "#1f1f28";
        bg-input: "#26262f";
        bg-overlay: "rgba(0, 0, 0, 0.65)";

        // ─── Text Colors ───
        // High contrast: primary ≥ 13:1, secondary ≥ 7:1, muted ≥ 4.5:1
        text-primary: "#ececf0";
        text-secondary: "#a0a0b0";
        text-muted: "#6c6c7a";
        text-nav-item: "#8e8e9e";
        text-card: "#ececf0";
        text-on-accent: "#ffffff";

        // ─── Brand / Accent ───
        // Indigo family: consistent hue, adjusted lightness for dark bg
        accent: "#818cf8";
        accent-hover: "#6366f1";
        accent-subtle: "rgba(129, 140, 248, 0.10)";
        accent-muted: "rgba(129, 140, 248, 0.05)";
        accent-gradient: "linear-gradient(135deg, #6366f1, #818cf8)";
        accent-disabled: "#4a4a6a";
        accent-border: "rgba(129, 140, 248, 0.30)";

        // ─── Semantic Colors (non-status) ───
        text-danger: "#f87171";
        text-positive: "#6ee7b7";
        border-interactive: "rgba(129, 140, 248, 0.35)";

        // ─── Border Colors ───
        // Layered: subtle < card < nav < input (increasing visibility)
        border-nav: "#2a2a35";
        border-subtle: "#232330";
        border-card: "#28283a";
        border-card-title: "#2e2e3c";
        border-input: "#363645";
        border-accent-light: "rgba(129, 140, 248, 0.20)";

        // ─── Shadows ───
        shadow-card: "0 1px 3px rgba(0,0,0,0.3), 0 4px 12px rgba(0,0,0,0.2)";
        shadow-modal: "0 20px 60px rgba(0,0,0,0.6)";
        shadow-float: "0 4px 14px rgba(99, 102, 241, 0.25)";
        shadow-drawer: "4px 0px 20px rgba(0, 0, 0, 0.5)";
        shadow-accent-sm: "0px 1px 2px rgba(99, 102, 241, 0.25)";
        shadow-accent-lg: "0px 6px 20px rgba(99, 102, 241, 0.4)";

        // ─── Theme Toggle ───
        bg-theme-button: "#26262f";
        text-theme-button: "#fbbf24";
        border-theme-button: "#363645";

        // ─── Status: Error ───
        bg-error: "rgba(239, 68, 68, 0.08)";
        text-error: "#fca5a5";
        border-error: "rgba(239, 68, 68, 0.25)";
        bg-error-icon: "rgba(239, 68, 68, 0.20)";

        // ─── Status: Success ───
        bg-success: "rgba(34, 197, 94, 0.08)";
        text-success: "#86efac";
        border-success: "rgba(34, 197, 94, 0.25)";

        // ─── Status: Warning ───
        bg-warning: "rgba(245, 158, 11, 0.08)";
        text-warning: "#fcd34d";
        border-warning: "rgba(245, 158, 11, 0.25)";

        // ─── Status: Info ───
        text-info: "#7dd3fc";
        border-info: "rgba(56, 189, 248, 0.25)";

        // ─── Status: Pink ───
        text-pink: "#f9a8d4";
        border-pink: "rgba(236, 72, 153, 0.25)";

        // ─── Loading ───
        bg-loading: "rgba(99, 102, 241, 0.08)";
        border-loading: "rgba(99, 102, 241, 0.20)";
        text-loading-title: "#a5b4fc";
        bg-progress: "#2a2a35";

        // ─── List ───
        bg-list-even: "#1a1a22";
        bg-list-odd: "#1f1f28";

        // ─── Console / VConsole ───
        bg-console: "#111118";
        bg-console-header: "#18181f";
        bg-console-filter: "#141420";
        border-console: "#28283a";
        text-console: "#b0b0be";
        text-console-title: "#a5b4fc";
        border-console-button: "#2a2a35";
        text-console-button: "#8e8e9e";
        text-console-log-latest: "#6ee7b7";
        text-console-warn: "#fbbf24";
        text-console-warn-latest: "#fcd34d";
        text-console-error: "#f87171";
        text-console-error-latest: "#fca5a5";
        text-console-empty: "#4a4a58";
        bg-console-badge: "#dc2626";
        bg-console-badge-log: "rgba(110, 231, 183, 0.12)";
        bg-console-badge-warn: "rgba(251, 191, 36, 0.12)";
        bg-console-badge-error: "rgba(248, 113, 113, 0.12)";
        border-console-accent: "#818cf8";
        text-console-filter-active: "#a5b4fc";
        border-console-filter-active: "#818cf8";
        bg-console-filter-active: "rgba(129, 140, 248, 0.12)";
        bg-console-button: "#6366f1";
        text-console-button-text: "white";
        shadow-console-button: "0 4px 14px rgba(99, 102, 241, 0.25)";
        bg-console-button-hover: "#818cf8";
        shadow-console-panel: "0 -8px 30px rgba(0, 0, 0, 0.4)";
        bg-console-close-hover: "rgba(239, 68, 68, 0.12)";
        text-console-close-hover: "#f87171";
    }
}
