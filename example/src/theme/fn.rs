use crate::*;

css_vars! {
    pub c_theme_light {
        bg-primary: "#f8f9fb";
        bg-nav: "#ffffff";
        border-nav: "#e5e7eb";
        border-subtle: "#f0f0f0";
        text-primary: "#1a1a2e";
        text-muted: "#9ca3af";
        text-nav-item: "#6b7280";
        bg-theme-button: "#f3f4f6";
        text-theme-button: "#1a1a2e";
        border-theme-button: "#e5e7eb";
        bg-card: "white";
        border-card: "#f0f0f0";
        border-card-title: "#f0f0f0";
        text-card: "#1a1a2e";
        shadow-card: "0 1px 3px rgba(0,0,0,0.06), 0 4px 12px rgba(0,0,0,0.04)";
        bg-modal: "white";
        shadow-modal: "0 20px 60px rgba(0,0,0,0.3)";
        bg-input: "#fafbfc";
        border-input: "#e5e7eb";
        bg-list-even: "#fafbfc";
        bg-list-odd: "white";
        bg-error: "#fef2f2";
        text-error: "#991b1b";
        border-error: "#fecaca";
        bg-error-icon: "#fca5a5";
        bg-success: "#f0fdf4";
        text-success: "#166534";
        border-success: "#bbf7d0";
        bg-warning: "#fef3c7";
        text-warning: "#92400e";
        border-warning: "#fde68a";
        text-info: "#075985";
        border-info: "#bae6fd";
        text-pink: "#9d174d";
        border-pink: "#f9a8d4";
        border-accent-light: "#e0e7ff";
        bg-loading: "#eef2ff";
        border-loading: "#c7d2fe";
        text-loading-title: "#3730a3";
        bg-progress: "#e5e7eb";
        bg-console: "#1a1a2e";
        bg-console-header: "#16162a";
        bg-console-filter: "#12122a";
        border-console: "#2d2d4a";
        text-console: "#9ca3af";
        text-console-title: "#a5b4fc";
        border-console-button: "#4b5563";
        text-console-button: "#9ca3af";
        text-console-log-latest: "#34d399";
        text-console-warn: "#d97706";
        text-console-warn-latest: "#fbbf24";
        text-console-error: "#ef4444";
        text-console-error-latest: "#f87171";
        text-console-empty: "#6b7280";
        bg-console-badge: "#ef4444";
        bg-console-badge-log: "rgba(52, 211, 153, 0.15)";
        bg-console-badge-warn: "rgba(251, 191, 36, 0.15)";
        bg-console-badge-error: "rgba(248, 113, 113, 0.15)";
        border-console-accent: "#4f46e5";
        text-console-filter-active: "#a78bfa";
        border-console-filter-active: "#7c3aed";
        bg-console-filter-active: "rgba(167, 139, 250, 0.15)";
    }

    pub c_theme_dark {
        bg-primary: "#0f0f1a";
        bg-nav: "#16162a";
        border-nav: "#2d2d4a";
        border-subtle: "#2d2d4a";
        text-primary: "#e0e0e0";
        text-muted: "#6b7280";
        text-nav-item: "#9ca3af";
        bg-theme-button: "#2d2d4a";
        text-theme-button: "#fbbf24";
        border-theme-button: "#3d3d5a";
        bg-card: "#1e1e36";
        border-card: "#2d2d4a";
        border-card-title: "#2d2d4a";
        text-card: "#e0e0e0";
        shadow-card: "0 1px 3px rgba(0,0,0,0.2), 0 4px 12px rgba(0,0,0,0.15)";
        bg-modal: "#1e1e36";
        shadow-modal: "0 20px 60px rgba(0,0,0,0.5)";
        bg-input: "#252540";
        border-input: "#3d3d5a";
        bg-list-even: "#252540";
        bg-list-odd: "#1e1e36";
        bg-error: "#451a1a";
        text-error: "#fca5a5";
        border-error: "#7f1d1d";
        bg-error-icon: "#7f1d1d";
        bg-success: "#052e16";
        text-success: "#86efac";
        border-success: "#166534";
        bg-warning: "#451a03";
        text-warning: "#fbbf24";
        border-warning: "#92400e";
        text-info: "#7dd3fc";
        border-info: "#075985";
        text-pink: "#f9a8d4";
        border-pink: "#9d174d";
        border-accent-light: "#3d3d5a";
        bg-loading: "#1e1b4b";
        border-loading: "#312e81";
        text-loading-title: "#a5b4fc";
        bg-progress: "#3d3d5a";
        bg-console: "#1a1a2e";
        bg-console-header: "#16162a";
        bg-console-filter: "#12122a";
        border-console: "#2d2d4a";
        text-console: "#a1a1aa";
        text-console-title: "#c4b5fd";
        border-console-button: "#52525b";
        text-console-button: "#a1a1aa";
        text-console-log-latest: "#6ee7b7";
        text-console-warn: "#fbbf24";
        text-console-warn-latest: "#fcd34d";
        text-console-error: "#f87171";
        text-console-error-latest: "#fca5a5";
        text-console-empty: "#9ca3af";
        bg-console-badge: "#dc2626";
        bg-console-badge-log: "rgba(110, 231, 183, 0.15)";
        bg-console-badge-warn: "rgba(251, 191, 36, 0.15)";
        bg-console-badge-error: "rgba(248, 113, 113, 0.15)";
        border-console-accent: "#818cf8";
        text-console-filter-active: "#c4b5fd";
        border-console-filter-active: "#a78bfa";
        bg-console-filter-active: "rgba(196, 181, 253, 0.15)";
    }
}

/// Creates and returns the reactive theme state for the application.
///
/// Initializes a theme signal defaulting to "light" and a derived root class
/// signal that combines `c_app_root` with the active theme class. Uses `watch!`
/// to reactively update the root class whenever the theme signal changes.
///
/// # Returns
///
/// - `ThemeState`: The reactive theme state containing the theme signal and root class signal.
pub(crate) fn use_theme() -> ThemeState {
    let theme: Signal<String> = use_signal(|| "light".to_string());
    let root_class: Signal<String> =
        use_signal(|| format!("{} {}", c_app_root().get_name(), theme_class_name("light")));
    watch!(theme, |theme_value| {
        root_class.set(format!(
            "{} {}",
            c_app_root().get_name(),
            theme_class_name(&theme_value)
        ));
    });
    ThemeState { theme, root_class }
}

/// Creates a click event handler that toggles the theme between "light" and "dark".
///
/// # Arguments
///
/// - `Signal<String>`: The theme signal to toggle.
///
/// # Returns
///
/// - `NativeEventHandler`: A click event handler that flips the theme value.
pub(crate) fn toggle_theme(theme_signal: Signal<String>) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        let current: String = theme_signal.get();
        if current == "light" {
            theme_signal.set("dark".to_string());
        } else {
            theme_signal.set("light".to_string());
        }
    })
}

/// Returns the CSS class name for the given theme value.
///
/// # Arguments
///
/// - `&str`: The theme name ("light" or "dark").
///
/// # Returns
///
/// - `&'static str`: The CSS class name for the theme.
pub(crate) fn theme_class_name(theme: &str) -> &'static str {
    if theme == "dark" {
        c_theme_dark().get_name()
    } else {
        c_theme_light().get_name()
    }
}
