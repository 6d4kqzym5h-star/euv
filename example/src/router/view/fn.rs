use crate::*;

/// Reads the current hash-based route from the browser URL.
///
/// # Returns
///
/// - `String` - The hash fragment without the leading `#`, or `"/"` if empty.
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
/// - `&str` - The target route path.
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
/// - `String` - The target route path.
///
/// # Returns
///
/// - `NativeEventHandler` - An event handler for click events.
pub fn link_handler(route: String) -> NativeEventHandler {
    NativeEventHandler::new(NativeEventName::Click, move |_event: NativeEvent| {
        navigate(&route);
    })
}

/// Checks whether the current viewport width qualifies as a mobile device.
///
/// Uses `MOBILE_BREAKPOINT` (768px) as the threshold.
///
/// # Returns
///
/// - `bool` - `true` if the viewport width is less than the mobile breakpoint.
pub fn is_mobile() -> bool {
    let window: Window = window().expect("no global window exists");
    let width: f64 = window
        .inner_width()
        .ok()
        .map(|v| Number::from(v).value_of())
        .unwrap_or(0.0);
    width < MOBILE_BREAKPOINT as f64
}
