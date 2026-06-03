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
    let theme: Signal<String> = use_signal(|| THEME_LIGHT.to_string());
    let initial_mobile: bool = mobile_signal.get();
    let initial_root: &'static str = if initial_mobile {
        c_mobile_app_root().get_name()
    } else {
        c_app_root().get_name()
    };
    let root_class: Signal<String> = use_signal(|| {
        format!(
            "{initial_root} {theme_class}",
            theme_class = theme_class_name(THEME_LIGHT)
        )
    });
    watch!(mobile_signal, theme, |mobile: bool, theme_value: String| {
        let root: &'static str = if mobile {
            c_mobile_app_root().get_name()
        } else {
            c_app_root().get_name()
        };
        root_class.set(format!(
            "{root} {theme_class}",
            theme_class = theme_class_name(&theme_value)
        ));
    });
    ThemeState { theme, root_class }
}

/// The CSS class added to the root element while a theme switch is in flight.
///
/// While present, the global rule injected by `inject_app_global_css`
/// (`.euv-theme-switching * { transition: none !important; ... }`) disables
/// every transition and animation on the page, so swapping the theme's CSS
/// variables repaints instantly instead of triggering hundreds of
/// simultaneous colour transitions (which caused jank on the list page).
const THEME_SWITCHING_CLASS: &str = "euv-theme-switching";

/// Creates a click event handler that toggles the theme between "light" and "dark".
///
/// To keep the switch smooth even on pages with many elements (e.g. the list
/// page), this temporarily disables all CSS transitions/animations during the
/// variable swap and re-enables them on the next animation frame. This avoids
/// every list row, button, card, etc. animating its colour change at the same
/// time, which previously caused dropped frames.
///
/// # Arguments
///
/// - `Signal<String>` - The theme signal to toggle.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click event handler that flips the theme value.
pub(crate) fn toggle_theme(theme_signal: Signal<String>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_event: Event| {
        suppress_theme_transitions();
        let current: String = theme_signal.get();
        if current == THEME_LIGHT {
            theme_signal.set(THEME_DARK.to_string());
        } else {
            theme_signal.set(THEME_LIGHT.to_string());
        }
        restore_theme_transitions();
    }))
}

/// Returns the document's root element (`<html>`), if available.
///
/// # Returns
///
/// - `Option<Element>` - The root element, or `None` if the DOM is unavailable.
fn document_root_element() -> Option<Element> {
    window()?.document()?.document_element()
}

/// Adds the `euv-theme-switching` class to the root element, disabling all
/// transitions and animations for the duration of the theme swap.
fn suppress_theme_transitions() {
    if let Some(root) = document_root_element() {
        let _ = root.class_list().add_1(THEME_SWITCHING_CLASS);
    }
}

/// Removes the `euv-theme-switching` class on a later animation frame, once the
/// new theme variables have been applied and painted.
///
/// Two nested `requestAnimationFrame` calls are used so the class is only
/// removed after the browser has had a chance to apply the swapped CSS
/// variables without any transition, guaranteeing the colour change is
/// instantaneous before normal transitions are restored.
fn restore_theme_transitions() {
    let Some(outer_window) = window() else {
        return;
    };
    let outer_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let Some(inner_window) = window() else {
            return;
        };
        let inner_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
            if let Some(root) = document_root_element() {
                let _ = root.class_list().remove_1(THEME_SWITCHING_CLASS);
            }
        }) as Box<dyn FnMut()>);
        let _ = inner_window.request_animation_frame(inner_closure.as_ref().unchecked_ref());
        inner_closure.forget();
    }) as Box<dyn FnMut()>);
    let _ = outer_window.request_animation_frame(outer_closure.as_ref().unchecked_ref());
    outer_closure.forget();
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
    if theme == THEME_DARK {
        c_theme_dark().get_name()
    } else {
        c_theme_light().get_name()
    }
}
