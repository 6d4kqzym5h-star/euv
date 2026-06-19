use crate::*;

vars! {
    pub(crate) c_theme_light {
        // ═══════════════════════════════════════════════════════════════════════
        // Monochrome Design Tokens (Black & White only)
        // ═══════════════════════════════════════════════════════════════════════

        // ─── Surface ───
        background: "#ffffff";
        foreground: "#000000";
        card: "#ffffff";
        card-foreground: "#000000";
        popover: "#ffffff";
        popover-foreground: "#000000";

        // ─── Primary Action ───
        primary: "#000000";
        primary-foreground: "#ffffff";

        // ─── Secondary / Muted ───
        secondary: "#ffffff";
        secondary-foreground: "#000000";
        muted: "#ffffff";
        muted-foreground: "#000000";

        // ─── Accent ───
        accent: "#000000";
        accent-foreground: "#ffffff";

        // ─── Destructive ───
        destructive: "#000000";
        destructive-foreground: "#ffffff";

        // ─── Border & Input ───
        border: "rgba(0, 0, 0, 0.12)";
        input: "rgba(0, 0, 0, 0.12)";
        ring: "#000000";

        shadcn-radius: "0.5rem";

        // ═══════════════════════════════════════════════════════════════════════
        // Legacy Surface Aliases
        // ═══════════════════════════════════════════════════════════════════════
        bg-primary: "#ffffff";
        bg-nav: "#ffffff";
        bg-card: "#ffffff";
        bg-modal: "#ffffff";
        bg-input: "#ffffff";
        bg-overlay: "rgba(0, 0, 0, 0.45)";

        // ─── Text Colors ───
        text-primary: "#000000";
        text-secondary: "#000000";
        text-muted: "#000000";
        text-tertiary: "#000000";
        text-nav-item: "#000000";
        text-card: "#000000";
        text-on-accent: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand / Accent (Black for minimalist)
        // ═══════════════════════════════════════════════════════════════════════
        accent: "#000000";
        accent-hover: "#000000";
        accent-subtle: "#ffffff";
        accent-muted: "#ffffff";
        accent-gradient: "linear-gradient(135deg, #000000, #000000)";
        accent-disabled: "#000000";
        accent-border: "#000000";

        // ─── Semantic Colors ───
        text-danger: "#000000";
        text-positive: "#000000";
        border-interactive: "#000000";

        // ─── Border Colors ───
        border-nav: "#000000";
        border-subtle: "#000000";
        border-card: "#000000";
        border-card-title: "#000000";
        border-input: "#000000";
        border-accent-light: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Spacing Scale (shadcn/ui Tailwind spacing)
        // ═══════════════════════════════════════════════════════════════════════
        space-2xs: "2px";
        space-2xs: "2px";
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

        // ═══════════════════════════════════════════════════════════════════════
        // Border Radius Scale (shadcn/ui)
        // ═══════════════════════════════════════════════════════════════════════
        radius-xs: "0.25rem";
        radius-sm: "0.375rem";
        radius-md: "0.5rem";
        radius-lg: "0.75rem";
        radius-xl: "1rem";
        radius-2xl: "1.5rem";
        radius-pill: "9999px";

        // ═══════════════════════════════════════════════════════════════════════
        // Font Size Scale (shadcn/ui)
        // ═══════════════════════════════════════════════════════════════════════
        font-xs: "0.75rem";
        font-sm: "0.875rem";
        font-base: "1rem";
        font-md: "1.125rem";
        font-lg: "1.125rem";
        font-xl: "1.25rem";
        font-2xl: "1.5rem";
        font-3xl: "1.875rem";
        font-4xl: "2.25rem";
        font-5xl: "3rem";
        font-6xl: "3.75rem";

        // ═══════════════════════════════════════════════════════════════════════
        // Transition Durations (shadcn/ui)
        // ═══════════════════════════════════════════════════════════════════════
        duration-instant: "0.05s";
        duration-fast: "0.15s";
        duration-normal: "0.2s";
        duration-slow: "0.3s";
        duration-slower: "0.4s";
        duration-overlay: "0.2s";
        duration-modal-overlay: "0.15s";
        duration-modal-content: "0.3s";

        // ═══════════════════════════════════════════════════════════════════════
        // Easing Functions (shadcn/ui)
        // ═══════════════════════════════════════════════════════════════════════
        ease-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-in: "cubic-bezier(0.4, 0, 1, 1)";
        ease-in-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-bounce: "cubic-bezier(0.34, 1.56, 0.64, 1)";
        ease-bounce-out: "cubic-bezier(0.36, 0, 0.66, -0.56)";

        // ═══════════════════════════════════════════════════════════════════════
        // Layout (shadcn/ui aligned)
        // ═══════════════════════════════════════════════════════════════════════
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

        // ═══════════════════════════════════════════════════════════════════════
        // Component Spacing Scale
        // ═══════════════════════════════════════════════════════════════════════
        gap-section: "16px";
        gap-section-mobile: "12px";
        gap-component: "12px";
        gap-component-mobile: "10px";
        gap-element: "8px";
        gap-inline: "8px";

        // ═══════════════════════════════════════════════════════════════════════
        // Responsive Breakpoints
        // ═══════════════════════════════════════════════════════════════════════
        breakpoint-mobile: "767px";

        // ─── Status Color Palette (monochrome)
        // ═══════════════════════════════════════════════════════════════════════
        color-success: "#000000";
        color-error: "#000000";
        color-warning: "#000000";
        color-info: "#000000";
        color-purple: "#000000";
        color-red-channel: "#000000";
        color-green-channel: "#000000";
        color-blue-channel: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Badge Background Colors (monochrome)
        // ═══════════════════════════════════════════════════════════════════════
        badge-bg-success: "#000000";
        badge-bg-error: "#000000";
        badge-bg-warning: "#000000";
        badge-bg-info: "#000000";
        badge-bg-purple: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Shadows (black alpha only)
        // ═══════════════════════════════════════════════════════════════════════
        shadow-xs: "0 1px 2px rgba(0, 0, 0, 0.04)";
        shadow-sm: "0 1px 3px rgba(0, 0, 0, 0.08), 0 1px 2px rgba(0, 0, 0, 0.04)";
        shadow-md: "0 4px 6px -1px rgba(0, 0, 0, 0.08), 0 2px 4px -2px rgba(0, 0, 0, 0.04)";
        shadow-lg: "0 10px 15px -3px rgba(0, 0, 0, 0.08), 0 4px 6px -4px rgba(0, 0, 0, 0.04)";
        shadow-xl: "0 20px 25px -5px rgba(0, 0, 0, 0.08), 0 8px 10px -6px rgba(0, 0, 0, 0.04)";
        shadow-card: "0 1px 3px rgba(0, 0, 0, 0.08), 0 1px 2px rgba(0, 0, 0, 0.04)";
        shadow-modal: "0 25px 50px -12px rgba(0, 0, 0, 0.18)";
        shadow-float: "0 4px 6px -1px rgba(0, 0, 0, 0.08)";
        shadow-drawer: "4px 0 20px rgba(0, 0, 0, 0.08)";
        shadow-accent-sm: "0 1px 3px rgba(0, 0, 0, 0.08)";
        shadow-accent-md: "0 4px 6px -1px rgba(0, 0, 0, 0.08)";
        shadow-accent-lg: "0 10px 15px -3px rgba(0, 0, 0, 0.12)";
        shadow-success: "0 4px 6px -1px rgba(0, 0, 0, 0.08)";
        shadow-error: "0 4px 6px -1px rgba(0, 0, 0, 0.08)";
        shadow-warning: "0 4px 6px -1px rgba(0, 0, 0, 0.08)";

        // ═══════════════════════════════════════════════════════════════════════
        // Theme Toggle
        // ═══════════════════════════════════════════════════════════════════════
        bg-theme-button: "#ffffff";
        text-theme-button: "#000000";
        border-theme-button: "rgba(0, 0, 0, 0.12)";

        // ═══════════════════════════════════════════════════════════════════════
        // Status: Error
        // ═══════════════════════════════════════════════════════════════════════
        bg-error: "#ffffff";
        text-error: "#000000";
        border-error: "rgba(0, 0, 0, 0.12)";
        bg-error-icon: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Status: Success
        // ═══════════════════════════════════════════════════════════════════════
        bg-success: "#ffffff";
        text-success: "#000000";
        border-success: "rgba(0, 0, 0, 0.12)";

        // ═══════════════════════════════════════════════════════════════════════
        // Status: Warning
        // ═══════════════════════════════════════════════════════════════════════
        bg-warning: "#ffffff";
        text-warning: "#000000";
        border-warning: "rgba(0, 0, 0, 0.12)";

        // ═══════════════════════════════════════════════════════════════════════
        // Status: Info
        // ═══════════════════════════════════════════════════════════════════════
        text-info: "#000000";
        border-info: "rgba(0, 0, 0, 0.14)";

        // ═══════════════════════════════════════════════════════════════════════
        // Status: Pink
        // ═══════════════════════════════════════════════════════════════════════
        text-pink: "#000000";
        border-pink: "rgba(0, 0, 0, 0.14)";

        // ═══════════════════════════════════════════════════════════════════════
        // Loading
        // ═══════════════════════════════════════════════════════════════════════
        bg-loading: "#ffffff";
        border-loading: "rgba(0, 0, 0, 0.12)";
        text-loading-title: "#000000";
        bg-progress: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar
        // ═══════════════════════════════════════════════════════════════════════
        scrollbar-track: "transparent";
        scrollbar-thumb: "rgba(0, 0, 0, 0.15)";
        scrollbar-thumb-hover: "rgba(0, 0, 0, 0.30)";
        scrollbar-thumb-active: "rgba(0, 0, 0, 0.45)";
        scrollbar-corner: "transparent";

        // ═══════════════════════════════════════════════════════════════════════
        // List
        // ═══════════════════════════════════════════════════════════════════════
        bg-list-even: "#ffffff";
        bg-list-odd: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Console / VConsole
        // ═══════════════════════════════════════════════════════════════════════
        bg-console: "#ffffff";
        bg-console-header: "#ffffff";
        bg-console-filter: "#ffffff";
        border-console: "rgba(0, 0, 0, 0.10)";
        text-console: "#000000";
        text-console-title: "#000000";
        text-console-log-latest: "#000000";
        text-console-warn: "#000000";
        text-console-warn-latest: "#000000";
        text-console-error: "#000000";
        text-console-error-latest: "#000000";
        text-console-empty: "#000000";
        bg-console-badge: "#ffffff";
        bg-console-badge-log: "#ffffff";
        bg-console-badge-warn: "#ffffff";
        bg-console-badge-error: "#ffffff";
        border-console-accent: "#000000";
        bg-console-button: "#000000";
        text-console-button-text: "#ffffff";
        shadow-console-button: "0 4px 14px rgba(0, 0, 0, 0.15)";
        bg-console-button-hover: "rgba(0, 0, 0, 0.80)";
        shadow-console-panel: "0 -8px 32px rgba(0, 0, 0, 0.06)";
        bg-console-filter-active: "#000000";
        text-console-filter-active: "#ffffff";
        border-console-filter-inactive: "rgba(0, 0, 0, 0.25)";
        text-console-filter-inactive: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Glass / Surface Effects
        // ═══════════════════════════════════════════════════════════════════════
        bg-glass: "rgba(255, 255, 255, 0.65)";
        bg-glass-heavy: "rgba(255, 255, 255, 0.82)";
        glass-border: "rgba(0, 0, 0, 0.10)";
        glass-blur-sm: "blur(6px)";
        glass-blur-md: "blur(14px)";
        glass-blur-lg: "blur(24px)";

        // ═══════════════════════════════════════════════════════════════════════
        // Surface Gradients
        // ═══════════════════════════════════════════════════════════════════════
        surface-gradient: "linear-gradient(135deg, #ffffff 0%, rgba(0, 0, 0, 0.04) 100%)";
        surface-gradient-subtle: "linear-gradient(135deg, rgba(0, 0, 0, 0.02) 0%, rgba(0, 0, 0, 0.05) 100%)";

        // ═══════════════════════════════════════════════════════════════════════
        // Glass border
        // ═══════════════════════════════════════════════════════════════════════
        border-glass: "rgba(255, 255, 255, 0.22)";
    }

    pub(crate) c_theme_dark {
        // ═══════════════════════════════════════════════════════════════════════
        // Monochrome Design Tokens (Black & White only)
        // ═══════════════════════════════════════════════════════════════════════

        // ─── Surface ───
        background: "#000000";
        foreground: "#ffffff";
        card: "#0a0a0a";
        card-foreground: "#ffffff";
        popover: "#0a0a0a";
        popover-foreground: "#ffffff";

        // ─── Primary Action ───
        primary: "#ffffff";
        primary-foreground: "#000000";

        // ─── Secondary / Muted ───
        secondary: "#000000";
        secondary-foreground: "#ffffff";
        muted: "#000000";
        muted-foreground: "#ffffff";

        // ─── Accent ───
        accent: "#ffffff";
        accent-foreground: "#000000";

        // ─── Destructive ───
        destructive: "#ffffff";
        destructive-foreground: "#000000";

        // ─── Border & Input ───
        border: "rgba(255, 255, 255, 0.14)";
        input: "rgba(255, 255, 255, 0.14)";
        ring: "#ffffff";

        shadcn-radius: "0.5rem";

        // ═══════════════════════════════════════════════════════════════════════
        // Legacy Surface Aliases
        // ═══════════════════════════════════════════════════════════════════════
        bg-primary: "#000000";
        bg-nav: "#000000";
        bg-card: "#0a0a0a";
        bg-modal: "#0a0a0a";
        bg-input: "#000000";
        bg-overlay: "rgba(0, 0, 0, 0.60)";

        // ─── Text Colors ───
        text-primary: "#ffffff";
        text-secondary: "#ffffff";
        text-muted: "#ffffff";
        text-tertiary: "#ffffff";
        text-nav-item: "#ffffff";
        text-card: "#ffffff";
        text-on-accent: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Brand / Accent (White for dark minimalist)
        // ═══════════════════════════════════════════════════════════════════════
        accent: "#ffffff";
        accent-hover: "#ffffff";
        accent-subtle: "#000000";
        accent-muted: "#000000";
        accent-gradient: "linear-gradient(135deg, #ffffff, #ffffff)";
        accent-disabled: "#ffffff";
        accent-border: "#ffffff";

        // ─── Semantic Colors ───
        text-danger: "#ffffff";
        text-positive: "#ffffff";
        border-interactive: "#ffffff";

        // ─── Border Colors ───
        border-nav: "#ffffff";
        border-subtle: "#ffffff";
        border-card: "#ffffff";
        border-card-title: "#ffffff";
        border-input: "#ffffff";
        border-accent-light: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Spacing Scale (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        space-2xs: "2px";
        space-2xs: "2px";
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

        // ═══════════════════════════════════════════════════════════════════════
        // Border Radius Scale (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        radius-xs: "0.25rem";
        radius-sm: "0.375rem";
        radius-md: "0.5rem";
        radius-lg: "0.75rem";
        radius-xl: "1rem";
        radius-2xl: "1.5rem";
        radius-pill: "9999px";

        // ═══════════════════════════════════════════════════════════════════════
        // Font Size Scale (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        font-xs: "0.75rem";
        font-sm: "0.875rem";
        font-base: "1rem";
        font-md: "1.125rem";
        font-lg: "1.125rem";
        font-xl: "1.25rem";
        font-2xl: "1.5rem";
        font-3xl: "1.875rem";
        font-4xl: "2.25rem";
        font-5xl: "3rem";
        font-6xl: "3.75rem";

        // ═══════════════════════════════════════════════════════════════════════
        // Transition Durations (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        duration-instant: "0.05s";
        duration-fast: "0.15s";
        duration-normal: "0.2s";
        duration-slow: "0.3s";
        duration-slower: "0.4s";
        duration-overlay: "0.2s";
        duration-modal-overlay: "0.15s";
        duration-modal-content: "0.3s";

        // ═══════════════════════════════════════════════════════════════════════
        // Easing Functions (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        ease-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-in: "cubic-bezier(0.4, 0, 1, 1)";
        ease-in-out: "cubic-bezier(0.4, 0, 0.2, 1)";
        ease-bounce: "cubic-bezier(0.34, 1.56, 0.64, 1)";
        ease-bounce-out: "cubic-bezier(0.36, 0, 0.66, -0.56)";

        // ═══════════════════════════════════════════════════════════════════════
        // Layout (same as light)
        // ═══════════════════════════════════════════════════════════════════════
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

        // ═══════════════════════════════════════════════════════════════════════
        // Component Spacing Scale (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        gap-section: "16px";
        gap-section-mobile: "12px";
        gap-component: "12px";
        gap-component-mobile: "10px";
        gap-element: "8px";
        gap-inline: "8px";

        // ═══════════════════════════════════════════════════════════════════════
        // Responsive Breakpoints (same as light)
        // ═══════════════════════════════════════════════════════════════════════
        breakpoint-mobile: "767px";

        // ═══════════════════════════════════════════════════════════════════════
        // Status Color Palette (monochrome)
        // ═══════════════════════════════════════════════════════════════════════
        color-success: "#ffffff";
        color-error: "#ffffff";
        color-warning: "#ffffff";
        color-info: "#ffffff";
        color-purple: "#ffffff";
        color-red-channel: "#ffffff";
        color-green-channel: "#ffffff";
        color-blue-channel: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Badge Background Colors (monochrome)
        // ═══════════════════════════════════════════════════════════════════════
        badge-bg-success: "#ffffff";
        badge-bg-error: "#ffffff";
        badge-bg-warning: "#ffffff";
        badge-bg-info: "#ffffff";
        badge-bg-purple: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Shadows (white alpha for dark)
        // ═══════════════════════════════════════════════════════════════════════
        shadow-xs: "0 1px 2px rgba(255, 255, 255, 0.08)";
        shadow-sm: "0 1px 3px rgba(255, 255, 255, 0.12), 0 1px 2px rgba(255, 255, 255, 0.06)";
        shadow-md: "0 4px 6px -1px rgba(255, 255, 255, 0.12), 0 2px 4px -2px rgba(255, 255, 255, 0.06)";
        shadow-lg: "0 10px 15px -3px rgba(255, 255, 255, 0.15), 0 4px 6px -4px rgba(255, 255, 255, 0.08)";
        shadow-xl: "0 20px 25px -5px rgba(255, 255, 255, 0.15), 0 8px 10px -6px rgba(255, 255, 255, 0.08)";
        shadow-card: "0 1px 3px rgba(255, 255, 255, 0.12), 0 1px 2px rgba(255, 255, 255, 0.06)";
        shadow-modal: "0 25px 50px -12px rgba(255, 255, 255, 0.25)";
        shadow-float: "0 4px 6px -1px rgba(255, 255, 255, 0.12)";
        shadow-drawer: "4px 0 20px rgba(255, 255, 255, 0.12)";
        shadow-accent-sm: "0 1px 3px rgba(255, 255, 255, 0.15)";
        shadow-accent-md: "0 4px 6px -1px rgba(255, 255, 255, 0.12)";
        shadow-accent-lg: "0 10px 15px -3px rgba(255, 255, 255, 0.18)";
        shadow-success: "0 4px 6px -1px rgba(255, 255, 255, 0.12)";
        shadow-error: "0 4px 6px -1px rgba(255, 255, 255, 0.12)";
        shadow-warning: "0 4px 6px -1px rgba(255, 255, 255, 0.12)";

        // ═══════════════════════════════════════════════════════════════════════
        // Theme Toggle
        // ═══════════════════════════════════════════════════════════════════════
        bg-theme-button: "#000000";
        text-theme-button: "#ffffff";
        border-theme-button: "rgba(255, 255, 255, 0.14)";

        // ═══════════════════════════════════════════════════════════════════════
        // Status: Error
        // ═══════════════════════════════════════════════════════════════════════
        bg-error: "#000000";
        text-error: "#ffffff";
        border-error: "rgba(255, 255, 255, 0.14)";
        bg-error-icon: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Status: Success
        // ═══════════════════════════════════════════════════════════════════════
        bg-success: "#000000";
        text-success: "#ffffff";
        border-success: "rgba(255, 255, 255, 0.12)";

        // ═══════════════════════════════════════════════════════════════════════
        // Status: Warning
        // ═══════════════════════════════════════════════════════════════════════
        bg-warning: "#000000";
        text-warning: "#ffffff";
        border-warning: "rgba(255, 255, 255, 0.12)";

        // ═══════════════════════════════════════════════════════════════════════
        // Status: Info
        // ═══════════════════════════════════════════════════════════════════════
        text-info: "#ffffff";
        border-info: "rgba(255, 255, 255, 0.14)";

        // ═══════════════════════════════════════════════════════════════════════
        // Status: Pink
        // ═══════════════════════════════════════════════════════════════════════
        text-pink: "#ffffff";
        border-pink: "rgba(255, 255, 255, 0.14)";

        // ═══════════════════════════════════════════════════════════════════════
        // Loading
        // ═══════════════════════════════════════════════════════════════════════
        bg-loading: "#000000";
        border-loading: "rgba(255, 255, 255, 0.12)";
        text-loading-title: "#ffffff";
        bg-progress: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Scrollbar
        // ═══════════════════════════════════════════════════════════════════════
        scrollbar-track: "transparent";
        scrollbar-thumb: "rgba(255, 255, 255, 0.18)";
        scrollbar-thumb-hover: "rgba(255, 255, 255, 0.35)";
        scrollbar-thumb-active: "rgba(255, 255, 255, 0.50)";
        scrollbar-corner: "transparent";

        // ═══════════════════════════════════════════════════════════════════════
        // List
        // ═══════════════════════════════════════════════════════════════════════
        bg-list-even: "#000000";
        bg-list-odd: "#000000";

        // ═══════════════════════════════════════════════════════════════════════
        // Console / VConsole
        // ═══════════════════════════════════════════════════════════════════════
        bg-console: "#000000";
        bg-console-header: "#000000";
        bg-console-filter: "#000000";
        border-console: "rgba(255, 255, 255, 0.12)";
        text-console: "#ffffff";
        text-console-title: "#ffffff";
        text-console-log-latest: "#ffffff";
        text-console-warn: "#ffffff";
        text-console-warn-latest: "#ffffff";
        text-console-error: "#ffffff";
        text-console-error-latest: "#ffffff";
        text-console-empty: "#ffffff";
        bg-console-badge: "#000000";
        bg-console-badge-log: "#000000";
        bg-console-badge-warn: "#000000";
        bg-console-badge-error: "#000000";
        border-console-accent: "#ffffff";
        bg-console-button: "#ffffff";
        text-console-button-text: "#000000";
        shadow-console-button: "0 4px 14px rgba(255, 255, 255, 0.15)";
        bg-console-button-hover: "rgba(255, 255, 255, 0.80)";
        shadow-console-panel: "0 -8px 32px rgba(255, 255, 255, 0.08)";
        bg-console-filter-active: "#ffffff";
        text-console-filter-active: "#000000";
        border-console-filter-inactive: "rgba(255, 255, 255, 0.25)";
        text-console-filter-inactive: "#ffffff";

        // ═══════════════════════════════════════════════════════════════════════
        // Glass / Surface Effects
        // ═══════════════════════════════════════════════════════════════════════
        bg-glass: "rgba(0, 0, 0, 0.65)";
        bg-glass-heavy: "rgba(0, 0, 0, 0.82)";
        glass-border: "rgba(255, 255, 255, 0.12)";
        glass-blur-sm: "blur(6px)";
        glass-blur-md: "blur(14px)";
        glass-blur-lg: "blur(24px)";

        // ═══════════════════════════════════════════════════════════════════════
        // Surface Gradients
        // ═══════════════════════════════════════════════════════════════════════
        surface-gradient: "linear-gradient(135deg, #000000 0%, rgba(255, 255, 255, 0.06) 100%)";
        surface-gradient-subtle: "linear-gradient(135deg, rgba(255, 255, 255, 0.03) 0%, rgba(255, 255, 255, 0.07) 100%)";

        // ═══════════════════════════════════════════════════════════════════════
        // Glass border
        // ═══════════════════════════════════════════════════════════════════════
        border-glass: "rgba(0, 0, 0, 0.05)";
    }
}
