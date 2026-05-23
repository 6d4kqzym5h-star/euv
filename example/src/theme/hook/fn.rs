use crate::*;

/// Creates and returns the reactive theme state for the application.
///
/// Initializes a theme signal defaulting to "light" and a derived root class
/// signal that combines the appropriate app root class (`c_app_root` or
/// `c_mobile_app_root`) with the active theme class. Uses `watch!` to
/// reactively update the root class whenever the theme or mobile signal changes.
///
/// # Arguments
///
/// - `Signal<bool>` - The reactive signal indicating whether the viewport is mobile-sized.
///
/// # Returns
///
/// - `ThemeState` - The reactive theme state containing the theme signal and root class signal.
pub(crate) fn use_theme(mobile_signal: Signal<bool>) -> ThemeState {
    let theme: Signal<String> = use_signal(|| "light".to_string());
    let initial_mobile: bool = mobile_signal.get();
    let initial_root: &'static str = if initial_mobile {
        c_mobile_app_root().get_name()
    } else {
        c_app_root().get_name()
    };
    let root_class: Signal<String> =
        use_signal(|| format!("{} {}", initial_root, theme_class_name("light")));
    watch!(mobile_signal, theme, |mobile, theme_value| {
        let root: &'static str = if mobile {
            c_mobile_app_root().get_name()
        } else {
            c_app_root().get_name()
        };
        root_class.set(format!("{} {}", root, theme_class_name(&theme_value)));
    });
    ThemeState { theme, root_class }
}

/// Creates a click event handler that toggles the theme between "light" and "dark".
///
/// # Arguments
///
/// - `Signal<String>` - The theme signal to toggle.
///
/// # Returns
///
/// - `NativeEventHandler` - A click event handler that flips the theme value.
pub(crate) fn toggle_theme(theme_signal: Signal<String>) -> NativeEventHandler {
    NativeEventHandler::create(NativeEventName::Click, move |_event: Event| {
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
/// - `&str` - The theme name ("light" or "dark").
///
/// # Returns
///
/// - `&'static str` - The CSS class name for the theme.
pub(crate) fn theme_class_name(theme: &str) -> &'static str {
    if theme == "dark" {
        c_theme_dark().get_name()
    } else {
        c_theme_light().get_name()
    }
}
