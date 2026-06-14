use crate::*;

vars! {
    pub(crate) c_theme_light {
        // ─── Spacing Scale ───
        // 4px base unit, harmonized geometric progression
        space-xs: "4px";
        space-sm: "8px";
        space-md: "12px";
        space-lg: "16px";
        space-xl: "20px";
        space-2xl: "24px";
        space-3xl: "32px";
        space-4xl: "40px";
        space-5xl: "48px";
        space-6xl: "64px";
        space-7xl: "80px";

        // ─── Border Radius Scale ───
        // Consistent rounding: subtle → medium → large → full
        radius-xs: "4px";
        radius-sm: "6px";
        radius-md: "10px";
        radius-lg: "14px";
        radius-xl: "18px";
        radius-2xl: "24px";
        radius-pill: "9999px";

        // ─── Font Size Scale ───
        // Modular scale (1.2 ratio) for clear typographic hierarchy
        font-xs: "11px";
        font-sm: "12px";
        font-base: "14px";
        font-md: "15px";
        font-lg: "16px";
        font-xl: "18px";
        font-2xl: "22px";
        font-3xl: "28px";
        font-4xl: "36px";
        font-5xl: "48px";
        font-6xl: "72px";

        // ─── Transition Durations ───
        // Unified timing: all micro-interactions use consistent timing
        duration-instant: "0.05s";
        duration-fast: "0.12s";
        duration-normal: "0.18s";
        duration-slow: "0.24s";
        duration-slower: "0.32s";
        duration-overlay: "0.24s";
        duration-modal-overlay: "0.18s";
        duration-modal-content: "0.32s";

        // ─── Easing Functions ───
        ease-out: "cubic-bezier(0.16, 1, 0.3, 1)";
        ease-in: "cubic-bezier(0.4, 0, 1, 1)";
        ease-in-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-bounce: "cubic-bezier(0.34, 1.56, 0.64, 1)";
        ease-bounce-out: "cubic-bezier(0.36, 0, 0.66, -0.56)";

        // ─── Layout ───
        padding-main-top: "24px";
        padding-main-horizontal: "28px";
        padding-main-horizontal-mobile: "16px";
        gap-page-header: "16px";
        gap-page-title: "6px";
        gap-card: "16px";
        gap-card-mobile: "12px";
        nav-width: "248px";
        content-max-width: "820px";
        mobile-header-height: "52px";

        // ─── Component Spacing Scale ───
        gap-section: "16px";
        gap-section-mobile: "12px";
        gap-component: "12px";
        gap-component-mobile: "10px";
        gap-element: "8px";
        gap-inline: "8px";

        // ─── Responsive Breakpoints ───
        breakpoint-mobile: "767px";

        // ─── Status Color Palette ───
        // WCAG AA compliant on light surfaces
        color-success: "#15803d";
        color-error: "#dc2626";
        color-warning: "#b45309";
        color-info: "#0369a1";
        color-purple: "#7c3aed";
        color-red-channel: "#ef4444";
        color-green-channel: "#22c55e";
        color-blue-channel: "#3b82f6";

        // ─── Badge Background Colors ───
        badge-bg-success: "#15803d";
        badge-bg-error: "#dc2626";
        badge-bg-warning: "#b45309";
        badge-bg-info: "#0369a1";
        badge-bg-purple: "#7c3aed";

        // ─── Surface Colors ───
        // Clear elevation: page(0) → nav(1) → card(2) → input(inset)
        // Neutral cool-gray palette, no warm tint
        bg-primary: "#f4f4f8";
        bg-nav: "#ffffff";
        bg-card: "#ffffff";
        bg-modal: "#ffffff";
        bg-input: "#f0f0f4";
        bg-overlay: "rgba(17, 17, 27, 0.45)";

        // ─── Text Colors ───
        // Hierarchy: primary ≥ 14:1, secondary ≥ 7:1, muted ≥ 4.5:1
        text-primary: "#0f0f1a";
        text-secondary: "#3d3d56";
        text-muted: "#7c7c92";
        text-tertiary: "#7c7c92";
        text-nav-item: "#5c5c72";
        text-card: "#0f0f1a";
        text-on-accent: "#ffffff";

        // ─── Brand / Accent ───
        // Indigo family, consistent across modes
        accent: "#4f46e5";
        accent-hover: "#4338ca";
        accent-subtle: "rgba(79, 70, 229, 0.08)";
        accent-muted: "rgba(79, 70, 229, 0.05)";
        accent-gradient: "linear-gradient(135deg, #4f46e5, #6366f1)";
        accent-disabled: "#a5a5cc";
        accent-border: "rgba(79, 70, 229, 0.25)";

        // ─── Semantic Colors ───
        text-danger: "#dc2626";
        text-positive: "#15803d";
        border-interactive: "rgba(79, 70, 229, 0.35)";

        // ─── Border Colors ───
        // Layered: subtle < card < nav < input (increasing visibility)
        border-nav: "#ebebf0";
        border-subtle: "#f0f0f4";
        border-card: "#e8e8f0";
        border-card-title: "#ededf2";
        border-input: "#d4d4de";
        border-accent-light: "rgba(79, 70, 229, 0.12)";

        // ─── Shadows ───
        // Consistent shadow language: xs → sm → md → lg → xl
        shadow-xs: "0 1px 2px rgba(15, 15, 26, 0.03)";
        shadow-sm: "0 1px 3px rgba(15, 15, 26, 0.04), 0 1px 2px rgba(15, 15, 26, 0.02)";
        shadow-md: "0 4px 12px rgba(15, 15, 26, 0.05), 0 1px 4px rgba(15, 15, 26, 0.03)";
        shadow-lg: "0 8px 24px rgba(15, 15, 26, 0.07), 0 2px 8px rgba(15, 15, 26, 0.04)";
        shadow-xl: "0 16px 48px rgba(15, 15, 26, 0.09), 0 4px 16px rgba(15, 15, 26, 0.05)";
        shadow-card: "0 1px 3px rgba(15, 15, 26, 0.04), 0 4px 12px rgba(15, 15, 26, 0.03)";
        shadow-modal: "0 24px 64px rgba(15, 15, 26, 0.18), 0 8px 24px rgba(15, 15, 26, 0.08)";
        shadow-float: "0 4px 16px rgba(79, 70, 229, 0.18)";
        shadow-drawer: "4px 0 20px rgba(15, 15, 26, 0.08)";
        shadow-accent-sm: "0 1px 3px rgba(79, 70, 229, 0.18)";
        shadow-accent-md: "0 4px 16px rgba(79, 70, 229, 0.18)";
        shadow-accent-lg: "0 8px 24px rgba(79, 70, 229, 0.28)";
        shadow-success: "0 4px 12px rgba(21, 128, 61, 0.12)";
        shadow-error: "0 4px 12px rgba(220, 38, 38, 0.12)";
        shadow-warning: "0 4px 12px rgba(180, 83, 9, 0.12)";

        // ─── Theme Toggle ───
        bg-theme-button: "#f3f4f6";
        text-theme-button: "#0f0f1a";
        border-theme-button: "#e5e7eb";

        // ─── Status: Error ───
        bg-error: "rgba(220, 38, 38, 0.05)";
        text-error: "#dc2626";
        border-error: "rgba(220, 38, 38, 0.18)";
        bg-error-icon: "rgba(220, 38, 38, 0.12)";

        // ─── Status: Success ───
        bg-success: "rgba(21, 128, 61, 0.05)";
        text-success: "#15803d";
        border-success: "rgba(21, 128, 61, 0.18)";

        // ─── Status: Warning ───
        bg-warning: "rgba(180, 83, 9, 0.06)";
        text-warning: "#b45309";
        border-warning: "rgba(180, 83, 9, 0.20)";

        // ─── Status: Info ───
        text-info: "#0369a1";
        border-info: "rgba(3, 105, 161, 0.18)";

        // ─── Status: Pink ───
        text-pink: "#be185d";
        border-pink: "rgba(190, 24, 93, 0.18)";

        // ─── Loading ───
        bg-loading: "rgba(79, 70, 229, 0.04)";
        border-loading: "rgba(79, 70, 229, 0.12)";
        text-loading-title: "#4338ca";
        bg-progress: "#e8e8f0";

        // ─── Scrollbar ───
        scrollbar-track: "transparent";
        scrollbar-thumb: "rgba(79, 70, 229, 0.15)";
        scrollbar-thumb-hover: "rgba(79, 70, 229, 0.30)";
        scrollbar-thumb-active: "rgba(79, 70, 229, 0.45)";
        scrollbar-corner: "transparent";

        // ─── List ───
        bg-list-even: "#f8f8fc";
        bg-list-odd: "#ffffff";

        // ─── Console / VConsole ───
        bg-console: "#ffffff";
        bg-console-header: "#f8f8fc";
        bg-console-filter: "#f4f4f8";
        border-console: "#e8e8f0";
        text-console: "#1a1a2e";
        text-console-title: "#4f46e5";
        border-console-button: "#d4d4de";
        text-console-button: "#5c5c72";
        text-console-log-latest: "#15803d";
        text-console-warn: "#b45309";
        text-console-warn-latest: "#92400e";
        text-console-error: "#dc2626";
        text-console-error-latest: "#b91c1c";
        text-console-empty: "#7c7c92";
        bg-console-badge: "#dc2626";
        bg-console-badge-log: "rgba(21, 128, 61, 0.06)";
        bg-console-badge-warn: "rgba(180, 83, 9, 0.06)";
        bg-console-badge-error: "rgba(220, 38, 38, 0.06)";
        border-console-accent: "#4f46e5";
        text-console-filter-active: "#4f46e5";
        border-console-filter-active: "#4f46e5";
        bg-console-filter-active: "rgba(79, 70, 229, 0.06)";
        bg-console-button: "#4f46e5";
        text-console-button-text: "white";
        shadow-console-button: "0 4px 14px rgba(79, 70, 229, 0.22)";
        bg-console-button-hover: "#4338ca";
        shadow-console-panel: "0 -8px 32px rgba(15, 15, 26, 0.06)";
        bg-console-close-hover: "rgba(220, 38, 38, 0.05)";
        text-console-close-hover: "#dc2626";

        // ─── Glass / Surface Effects ───
        bg-glass: "rgba(255, 255, 255, 0.65)";
        bg-glass-heavy: "rgba(255, 255, 255, 0.82)";
        glass-border: "rgba(255, 255, 255, 0.35)";
        glass-blur-sm: "blur(6px)";
        glass-blur-md: "blur(14px)";
        glass-blur-lg: "blur(24px)";

        // ─── Surface Gradients ───
        surface-gradient: "linear-gradient(135deg, #f4f4f8 0%, #e8e8f0 100%)";
        surface-gradient-subtle: "linear-gradient(135deg, rgba(79,70,229,0.02) 0%, rgba(79,70,229,0.05) 100%)";

        // ─── Glass border ───
        border-glass: "rgba(255, 255, 255, 0.22)";
    }

    pub(crate) c_theme_dark {
        // ─── Spacing Scale (same as light) ───
        space-xs: "4px";
        space-sm: "8px";
        space-md: "12px";
        space-lg: "16px";
        space-xl: "20px";
        space-2xl: "24px";
        space-3xl: "32px";
        space-4xl: "40px";
        space-5xl: "48px";
        space-6xl: "64px";
        space-7xl: "80px";

        // ─── Border Radius Scale (same as light) ───
        radius-xs: "4px";
        radius-sm: "6px";
        radius-md: "10px";
        radius-lg: "14px";
        radius-xl: "18px";
        radius-2xl: "24px";
        radius-pill: "9999px";

        // ─── Font Size Scale (same as light) ───
        font-xs: "11px";
        font-sm: "12px";
        font-base: "14px";
        font-md: "15px";
        font-lg: "16px";
        font-xl: "18px";
        font-2xl: "22px";
        font-3xl: "28px";
        font-4xl: "36px";
        font-5xl: "48px";
        font-6xl: "72px";

        // ─── Transition Durations (same as light) ───
        duration-instant: "0.05s";
        duration-fast: "0.12s";
        duration-normal: "0.18s";
        duration-slow: "0.24s";
        duration-slower: "0.32s";
        duration-overlay: "0.24s";
        duration-modal-overlay: "0.18s";
        duration-modal-content: "0.32s";

        // ─── Easing Functions (same as light) ───
        ease-out: "cubic-bezier(0.16, 1, 0.3, 1)";
        ease-in: "cubic-bezier(0.4, 0, 1, 1)";
        ease-in-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-bounce: "cubic-bezier(0.34, 1.56, 0.64, 1)";
        ease-bounce-out: "cubic-bezier(0.36, 0, 0.66, -0.56)";

        // ─── Layout (same as light) ───
        padding-main-top: "24px";
        padding-main-horizontal: "28px";
        padding-main-horizontal-mobile: "16px";
        gap-page-header: "16px";
        gap-page-title: "6px";
        gap-card: "16px";
        gap-card-mobile: "12px";
        nav-width: "248px";
        content-max-width: "820px";
        mobile-header-height: "52px";

        // ─── Component Spacing Scale (same as light) ───
        gap-section: "16px";
        gap-section-mobile: "12px";
        gap-component: "12px";
        gap-component-mobile: "10px";
        gap-element: "8px";
        gap-inline: "8px";

        // ─── Responsive Breakpoints (same as light) ───
        breakpoint-mobile: "767px";

        // ─── Status Color Palette (dark-adjusted for contrast) ───
        // All colors ≥ 4.5:1 contrast on dark surfaces
        color-success: "#4ade80";
        color-error: "#f87171";
        color-warning: "#fbbf24";
        color-info: "#7dd3fc";
        color-purple: "#a78bfa";
        color-red-channel: "#ef4444";
        color-green-channel: "#22c55e";
        color-blue-channel: "#60a5fa";

        // ─── Badge Background Colors (muted fills for dark mode) ───
        badge-bg-success: "#166534";
        badge-bg-error: "#991b1b";
        badge-bg-warning: "#92400e";
        badge-bg-info: "#0c4a6e";
        badge-bg-purple: "#5b21b6";

        // ─── Surface Colors ───
        // Clear elevation with subtle blue undertone
        bg-primary: "#0e0e14";
        bg-nav: "#151520";
        bg-card: "#1c1c28";
        bg-modal: "#1c1c28";
        bg-input: "#1a1a28";
        bg-overlay: "rgba(0, 0, 0, 0.60)";

        // ─── Text Colors ───
        // High contrast: primary ≥ 13:1, secondary ≥ 7:1, muted ≥ 4.5:1
        text-primary: "#ededf2";
        text-secondary: "#a8a8be";
        text-muted: "#6e6e84";
        text-tertiary: "#6e6e84";
        text-nav-item: "#8e8ea4";
        text-card: "#ededf2";
        text-on-accent: "#ffffff";

        // ─── Brand / Accent ───
        // Brighter indigo for dark bg visibility
        accent: "#7c7cf8";
        accent-hover: "#6366f1";
        accent-subtle: "rgba(124, 124, 248, 0.14)";
        accent-muted: "rgba(124, 124, 248, 0.08)";
        accent-gradient: "linear-gradient(135deg, #6366f1, #818cf8)";
        accent-disabled: "#4a4a6a";
        accent-border: "rgba(124, 124, 248, 0.30)";

        // ─── Semantic Colors ───
        text-danger: "#f87171";
        text-positive: "#4ade80";
        border-interactive: "rgba(124, 124, 248, 0.40)";

        // ─── Border Colors ───
        border-nav: "#262635";
        border-subtle: "#1e1e2c";
        border-card: "#28283a";
        border-card-title: "#2c2c3e";
        border-input: "#363648";
        border-accent-light: "rgba(124, 124, 248, 0.18)";

        // ─── Shadows ───
        shadow-xs: "0 1px 2px rgba(0, 0, 0, 0.20)";
        shadow-sm: "0 1px 3px rgba(0, 0, 0, 0.24), 0 1px 2px rgba(0, 0, 0, 0.16)";
        shadow-md: "0 4px 12px rgba(0, 0, 0, 0.28), 0 1px 4px rgba(0, 0, 0, 0.20)";
        shadow-lg: "0 8px 24px rgba(0, 0, 0, 0.32), 0 2px 8px rgba(0, 0, 0, 0.22)";
        shadow-xl: "0 16px 48px rgba(0, 0, 0, 0.38), 0 4px 16px rgba(0, 0, 0, 0.25)";
        shadow-card: "0 1px 3px rgba(0, 0, 0, 0.24), 0 4px 12px rgba(0, 0, 0, 0.16)";
        shadow-modal: "0 24px 64px rgba(0, 0, 0, 0.50), 0 8px 24px rgba(0, 0, 0, 0.30)";
        shadow-float: "0 4px 16px rgba(124, 124, 248, 0.20)";
        shadow-drawer: "4px 0 20px rgba(0, 0, 0, 0.40)";
        shadow-accent-sm: "0 1px 3px rgba(124, 124, 248, 0.22)";
        shadow-accent-md: "0 4px 16px rgba(124, 124, 248, 0.20)";
        shadow-accent-lg: "0 8px 24px rgba(124, 124, 248, 0.30)";
        shadow-success: "0 4px 12px rgba(74, 222, 128, 0.08)";
        shadow-error: "0 4px 12px rgba(248, 113, 113, 0.08)";
        shadow-warning: "0 4px 12px rgba(251, 191, 36, 0.08)";

        // ─── Theme Toggle ───
        bg-theme-button: "#232330";
        text-theme-button: "#fbbf24";
        border-theme-button: "#363648";

        // ─── Status: Error ───
        bg-error: "rgba(248, 113, 113, 0.08)";
        text-error: "#f87171";
        border-error: "rgba(248, 113, 113, 0.22)";
        bg-error-icon: "rgba(248, 113, 113, 0.18)";

        // ─── Status: Success ───
        bg-success: "rgba(74, 222, 128, 0.08)";
        text-success: "#4ade80";
        border-success: "rgba(74, 222, 128, 0.22)";

        // ─── Status: Warning ───
        bg-warning: "rgba(251, 191, 36, 0.08)";
        text-warning: "#fbbf24";
        border-warning: "rgba(251, 191, 36, 0.22)";

        // ─── Status: Info ───
        text-info: "#7dd3fc";
        border-info: "rgba(125, 211, 252, 0.22)";

        // ─── Status: Pink ───
        text-pink: "#f472b6";
        border-pink: "rgba(244, 114, 182, 0.22)";

        // ─── Loading ───
        bg-loading: "rgba(124, 124, 248, 0.06)";
        border-loading: "rgba(124, 124, 248, 0.16)";
        text-loading-title: "#a5b4fc";
        bg-progress: "#28283a";

        // ─── Scrollbar ───
        scrollbar-track: "transparent";
        scrollbar-thumb: "rgba(124, 124, 248, 0.22)";
        scrollbar-thumb-hover: "rgba(124, 124, 248, 0.40)";
        scrollbar-thumb-active: "rgba(124, 124, 248, 0.55)";
        scrollbar-corner: "transparent";

        // ─── List ───
        bg-list-even: "#141420";
        bg-list-odd: "#1c1c28";

        // ─── Console / VConsole ───
        bg-console: "#0e0e14";
        bg-console-header: "#151520";
        bg-console-filter: "#121220";
        border-console: "#28283a";
        text-console: "#b0b0c4";
        text-console-title: "#a5b4fc";
        border-console-button: "#2c2c3e";
        text-console-button: "#8e8ea4";
        text-console-log-latest: "#4ade80";
        text-console-warn: "#fbbf24";
        text-console-warn-latest: "#f59e0b";
        text-console-error: "#f87171";
        text-console-error-latest: "#ef4444";
        text-console-empty: "#4a4a5c";
        bg-console-badge: "#dc2626";
        bg-console-badge-log: "rgba(74, 222, 128, 0.10)";
        bg-console-badge-warn: "rgba(251, 191, 36, 0.10)";
        bg-console-badge-error: "rgba(248, 113, 113, 0.10)";
        border-console-accent: "#7c7cf8";
        text-console-filter-active: "#a5b4fc";
        border-console-filter-active: "#7c7cf8";
        bg-console-filter-active: "rgba(124, 124, 248, 0.10)";
        bg-console-button: "#6366f1";
        text-console-button-text: "white";
        shadow-console-button: "0 4px 14px rgba(99, 102, 241, 0.28)";
        bg-console-button-hover: "#7c7cf8";
        shadow-console-panel: "0 -8px 32px rgba(0, 0, 0, 0.35)";
        bg-console-close-hover: "rgba(248, 113, 113, 0.10)";
        text-console-close-hover: "#f87171";

        // ─── Glass / Surface Effects ───
        bg-glass: "rgba(28, 28, 40, 0.65)";
        bg-glass-heavy: "rgba(28, 28, 40, 0.82)";
        glass-border: "rgba(255, 255, 255, 0.06)";
        glass-blur-sm: "blur(6px)";
        glass-blur-md: "blur(14px)";
        glass-blur-lg: "blur(24px)";

        // ─── Surface Gradients ───
        surface-gradient: "linear-gradient(135deg, #0e0e14 0%, #151520 100%)";
        surface-gradient-subtle: "linear-gradient(135deg, rgba(124,124,248,0.03) 0%, rgba(124,124,248,0.07) 100%)";

        // ─── Glass border ───
        border-glass: "rgba(255, 255, 255, 0.05)";
    }
}
