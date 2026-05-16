use crate::*;

/// Reads the current hash-based route from the browser URL.
///
/// # Returns
///
/// - `String`: The hash fragment without the leading `#`, or `"/"` if empty.
pub fn current_route() -> String {
    let window: Window = window().expect("no global window exists");
    let hash: String = window.location().hash().unwrap_or_default();
    let route: String = hash.strip_prefix('#').unwrap_or(&hash).to_string();
    if route.is_empty() {
        "/".to_string()
    } else {
        route
    }
}

/// Navigates to a new hash-based route.
///
/// # Arguments
///
/// - `&str`: The target route path.
pub fn navigate(route: &str) {
    let window: Window = window().expect("no global window exists");
    let location: Location = window.location();
    let new_hash: String = format!("#{}", route);
    let _ = location.set_hash(&new_hash);
}

/// Creates a link click handler that navigates to the given route.
///
/// # Arguments
///
/// - `String`: The target route path.
///
/// # Returns
///
/// - `NativeEventHandler`: An event handler for click events.
pub fn link_handler(route: String) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        navigate(&route);
    })
}

/// Subscribes to browser `hashchange` events and updates the given signal.
///
/// Registers a global event listener on `window` that reads the current
/// route on every hash change and writes it into the provided signal.
/// The closure is leaked via `Closure::forget` so it persists for the
/// entire application lifetime.
///
/// # Arguments
///
/// - `Signal<String>`: The reactive signal that holds the current route and will be updated on each hash change.
pub fn use_hash_change(route_signal: Signal<String>) {
    let window: Window = window().expect("no global window exists");
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let new_route: String = current_route();
        route_signal.set(new_route);
    }));
    let _ = window.add_event_listener_with_callback(
        &NativeEventName::HashChange.to_string(),
        closure.as_ref().unchecked_ref(),
    );
    closure.forget();
}
