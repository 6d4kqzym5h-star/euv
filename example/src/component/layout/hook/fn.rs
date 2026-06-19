use crate::*;

/// Registers global event listeners that force the browser to recalculate
/// `env(safe-area-inset-*)` values after exiting any type of fullscreen.
///
/// Listens for the native Fullscreen API exit events (`fullscreenchange`,
/// `webkitfullscreenchange`) as well as `resize` which fires after orientation
/// or viewport changes that may invalidate inset values.
///
/// On detecting a fullscreen exit, forces a layout recalculation by toggling
/// the root element's `display` property momentarily. This ensures iOS Safari
/// and other WebKit browsers re-evaluate the `env()` CSS functions.
///
/// This hook should be called once during app initialization and covers:
/// - Native video fullscreen → exit
/// - CSS simulated fullscreen → exit (canvas drawing mode)
/// - Any future fullscreen scenarios
pub(crate) fn use_safe_area_fix() {
    use_window_event("fullscreenchange", || {
        resize();
    });
    use_window_event("webkitfullscreenchange", || {
        resize();
    });
    use_window_event("resize", || {
        resize();
    });
}

/// Resizes the window.
pub(crate) fn resize() {
    let deferred_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let win: Window = window().expect("no global window exists");
        let scroll_x: f64 = win.scroll_x().unwrap_or(0.0);
        let scroll_y: f64 = win.scroll_y().unwrap_or(0.0);
        win.scroll_to_with_x_and_y(scroll_x, scroll_y);
    }));
    let _ = window()
        .expect("no global window exists")
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            deferred_closure.as_ref().unchecked_ref::<Function>(),
            360,
        );
    deferred_closure.forget();
}

/// Creates a click event handler that toggles the mobile nav drawer signal
/// with proper browser history management.
///
/// When toggling from open to closed, calls `overlay_back` to remove the
/// extra history entry that was pushed when the drawer opened. When toggling
/// from closed to open, the `use_overlay_history` hook handles the
/// `pushState` call automatically.
///
/// # Arguments
///
/// - `Signal<bool>` - The boolean signal controlling the drawer visibility.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A click event handler that toggles the drawer.
pub(crate) fn use_drawer_toggle(drawer_open: Signal<bool>) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let is_open: bool = drawer_open.get();
        if is_open {
            overlay_back(None);
        }
        drawer_open.set(!is_open);
    }))
}

/// Creates a reactive signal that tracks whether the viewport is in mobile mode
/// and subscribes to browser `resize` events to keep it updated.
///
/// The resize handler is debounced by `RESIZE_DEBOUNCE_MILLIS` (16ms) to avoid
/// excessive recomputation during continuous resize operations.
/// The listener is automatically removed when the hook context is cleared.
///
/// # Returns
///
/// - `Signal<bool>` - A reactive signal that is `true` when the viewport is mobile-sized.
pub(crate) fn use_resize() -> Signal<bool> {
    let mobile_signal: Signal<bool> = use_signal(is_mobile);
    let timer_signal: Signal<Option<i32>> = use_signal(|| None);
    let debounce_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        let mobile: bool = is_mobile();
        mobile_signal.set(mobile);
    }));
    let debounce_callback: Function = debounce_closure
        .as_ref()
        .unchecked_ref::<Function>()
        .clone();
    debounce_closure.forget();
    let timeout_window: Window = window().expect("no global window exists");
    use_window_event("resize", move || {
        let old_timer: Option<i32> = timer_signal.get();
        if let Some(timer_id) = old_timer {
            timeout_window.clear_timeout_with_handle(timer_id);
        }
        let new_timer: i32 = timeout_window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &debounce_callback,
                RESIZE_DEBOUNCE_MILLIS,
            )
            .unwrap_or_default();
        timer_signal.set(Some(new_timer));
    });
    mobile_signal
}
