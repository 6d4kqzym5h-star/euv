use crate::*;

/// Subscribes to browser `hashchange` events and updates the given signal.
///
/// Registers a global event listener on `window` that reads the current
/// route on every hash change and writes it into the provided signal.
/// The closure is leaked via `Closure::forget` so it persists for the
/// entire application lifetime.
///
/// # Arguments
///
/// - `Signal<String>` - The reactive signal that holds the current route and will be updated on each hash change.
pub(crate) fn use_hash_change(route_signal: Signal<String>) {
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

/// Creates a reactive signal that tracks whether the viewport is in mobile mode
/// and subscribes to browser `resize` events to keep it updated.
///
/// The resize handler is debounced by `RESIZE_DEBOUNCE_MILLIS` (150ms) to avoid
/// excessive recomputation during continuous resize operations.
/// The closures are leaked via `Closure::forget` so they persist for the
/// entire application lifetime.
///
/// # Returns
///
/// - `Signal<bool>` - A reactive signal that is `true` when the viewport is mobile-sized.
pub(crate) fn use_resize() -> Signal<bool> {
    let mobile_signal: Signal<bool> = use_signal(is_mobile);
    let timer_signal: Signal<Option<i32>> = use_signal(|| None);
    let debounce_window: Window = window().expect("no global window exists");
    let timeout_window: Window = debounce_window.clone();
    let resize_window: Window = debounce_window.clone();
    let debounce_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let mobile: bool = is_mobile();
        mobile_signal.set(mobile);
    }));
    let debounce_callback: Function = debounce_closure
        .as_ref()
        .unchecked_ref::<Function>()
        .clone();
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let old_timer: Option<i32> = timer_signal.get();
        if let Some(timer_id) = old_timer {
            debounce_window.clear_timeout_with_handle(timer_id);
        }
        let new_timer: i32 = timeout_window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &debounce_callback,
                RESIZE_DEBOUNCE_MILLIS,
            )
            .unwrap_or(0);
        timer_signal.set(Some(new_timer));
    }));
    let _ = resize_window.add_event_listener_with_callback(
        &NativeEventName::Resize.to_string(),
        closure.as_ref().unchecked_ref(),
    );
    closure.forget();
    debounce_closure.forget();
    mobile_signal
}
