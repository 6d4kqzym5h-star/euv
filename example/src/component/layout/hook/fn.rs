use crate::*;

/// Registers global event listeners that preserve `env(safe-area-inset-*)`
/// values after exiting any type of fullscreen on Android, and ensures that
/// the system back button exits native fullscreen instead of navigating away.
///
/// On initialisation, reads the current `env(safe-area-inset-*)` pixel values
/// through a sentinel `<div>` and caches them in thread-local storage.
/// When a `fullscreenchange` or `resize` event fires, the cached values are
/// written directly as inline CSS custom properties on the real app root
/// element so that layout never depends on the potentially stale `env()`
/// function result.
///
/// When a native (browser) fullscreen is entered — for example the user taps
/// the fullscreen button on a `<video controls>` element — a browser history
/// entry is added via `overlay_push_state` so that the system back gesture
/// will fire `popstate`. A `popstate` guard registered via
/// [`register_popstate_guard`] then calls `document.exitFullscreen()` to leave
/// fullscreen, consuming the history entry without navigating to the previous
/// route. When the native fullscreen is exited through other means (e.g. the
/// browser's own exit button), the `fullscreenchange` handler consumes the
/// extra history entry via `overlay_back`.
///
/// This hook should be called once during app initialization and covers:
/// - Native video fullscreen → exit via system back button
/// - CSS simulated fullscreen → exit (canvas drawing mode)
/// - Any future fullscreen scenarios
pub(crate) fn use_safe_area_fix() {
    cache_safe_area_insets();
    use_window_event("fullscreenchange", || {
        let is_fullscreen: bool = window()
            .expect("no global window exists")
            .document()
            .expect("should have a document")
            .fullscreen_element()
            .is_some();
        if is_fullscreen {
            NATIVE_FULLSCREEN_ACTIVE.with(|flag: &Cell<bool>| flag.set(true));
            overlay_push_state();
        } else {
            let was_active: bool = NATIVE_FULLSCREEN_ACTIVE.with(|flag: &Cell<bool>| flag.get());
            if was_active {
                NATIVE_FULLSCREEN_ACTIVE.with(|flag: &Cell<bool>| flag.set(false));
                let exit_by_popstate: bool =
                    NATIVE_FULLSCREEN_EXIT_BY_POPSTATE.with(|flag: &Cell<bool>| flag.get());
                if exit_by_popstate {
                    NATIVE_FULLSCREEN_EXIT_BY_POPSTATE.with(|flag: &Cell<bool>| flag.set(false));
                } else {
                    overlay_back(None);
                }
            }
            apply_cached_insets();
        }
    });
    use_window_event("webkitfullscreenchange", || {
        apply_cached_insets();
    });
    use_window_event("resize", || {
        apply_cached_insets();
    });
    register_popstate_guard(Rc::new(|| {
        if !NATIVE_FULLSCREEN_ACTIVE.with(|flag: &Cell<bool>| flag.get()) {
            return false;
        }
        NATIVE_FULLSCREEN_EXIT_BY_POPSTATE.with(|flag: &Cell<bool>| flag.set(true));
        let document_value: Document = window()
            .expect("no global window exists")
            .document()
            .expect("should have a document");
        document_value.exit_fullscreen();
        true
    }));
}

/// Reads the current `env(safe-area-inset-*)` pixel values via a temporary
/// sentinel element and persists them in thread-local storage.
///
/// The sentinel `<div>` is created with `padding-top: env(safe-area-inset-top)`
/// (and similarly for the other three sides). After forcing a layout
/// calculation, `getComputedStyle` yields the resolved pixel value, which is
/// then stored in `SAFE_AREA_INSET_*` thread-local cells.
///
/// If the top inset is empty or `0px` (i.e. no safe area on this device or
/// immersive mode not active), the values are not cached and no override is
/// applied.
fn cache_safe_area_insets() {
    let top_cached: String =
        SAFE_AREA_INSET_TOP.with(|cell: &RefCell<String>| cell.borrow().clone());
    if !top_cached.is_empty() {
        return;
    }
    let win: Window = window().expect("no global window exists");
    let document_value: Document = win.document().expect("should have a document");
    let body: HtmlElement = document_value.body().expect("should have a body");
    let sentinel: HtmlElement = document_value
        .create_element("div")
        .expect("should create div")
        .unchecked_into();
    let _ = sentinel.style().set_property("position", "absolute");
    let _ = sentinel.style().set_property("visibility", "hidden");
    let _ = sentinel.style().set_property("pointer-events", "none");
    let _ = sentinel
        .style()
        .set_property("padding-top", "env(safe-area-inset-top, 0px)");
    let _ = sentinel
        .style()
        .set_property("padding-right", "env(safe-area-inset-right, 0px)");
    let _ = sentinel
        .style()
        .set_property("padding-bottom", "env(safe-area-inset-bottom, 0px)");
    let _ = sentinel
        .style()
        .set_property("padding-left", "env(safe-area-inset-left, 0px)");
    let _ = body.append_child(&sentinel);
    let Some(computed) = win.get_computed_style(&sentinel).ok().flatten() else {
        let _ = body.remove_child(&sentinel);
        return;
    };
    let top_value: String = computed
        .get_property_value("padding-top")
        .unwrap_or_default();
    let right_value: String = computed
        .get_property_value("padding-right")
        .unwrap_or_default();
    let bottom_value: String = computed
        .get_property_value("padding-bottom")
        .unwrap_or_default();
    let left_value: String = computed
        .get_property_value("padding-left")
        .unwrap_or_default();
    let _ = body.remove_child(&sentinel);
    if top_value.is_empty() || top_value == "0px" {
        return;
    }
    SAFE_AREA_INSET_TOP.with(|cell: &RefCell<String>| *cell.borrow_mut() = top_value);
    SAFE_AREA_INSET_RIGHT.with(|cell: &RefCell<String>| *cell.borrow_mut() = right_value);
    SAFE_AREA_INSET_BOTTOM.with(|cell: &RefCell<String>| *cell.borrow_mut() = bottom_value);
    SAFE_AREA_INSET_LEFT.with(|cell: &RefCell<String>| *cell.borrow_mut() = left_value);
}

/// Writes the cached safe-area inset values as inline CSS custom properties
/// on the real app root element and any fullscreen overlay containers.
///
/// Class rules such as `c_mobile_app_root`, `c_app_nav`, `c_app_main`,
/// `c_mobile_nav_drawer`, and `c_canvas_container_fullscreen` consume
/// `var(--safe-area-inset-top)` in their `padding` declarations. By overriding
/// these CSS custom properties with inline style (which has higher specificity
/// than the stylesheet rule from `vars!`), all `var()` references resolve to
/// the cached pixel values, bypassing the stale `env()` function after a
/// fullscreen exit.
///
/// The canvas fullscreen container is `position: fixed` and outside the app
/// root subtree, so it does not inherit the inline overrides — it must be
/// patched separately.
pub(crate) fn apply_cached_insets() {
    let top_value: String =
        SAFE_AREA_INSET_TOP.with(|cell: &RefCell<String>| cell.borrow().clone());
    if top_value.is_empty() {
        return;
    }
    let right_value: String =
        SAFE_AREA_INSET_RIGHT.with(|cell: &RefCell<String>| cell.borrow().clone());
    let bottom_value: String =
        SAFE_AREA_INSET_BOTTOM.with(|cell: &RefCell<String>| cell.borrow().clone());
    let left_value: String =
        SAFE_AREA_INSET_LEFT.with(|cell: &RefCell<String>| cell.borrow().clone());
    let document_value: Document = window()
        .expect("no global window exists")
        .document()
        .expect("should have a document");
    let apply_to = |element: &HtmlElement| {
        let _ = element
            .style()
            .set_property("--safe-area-inset-top", &top_value);
        let _ = element
            .style()
            .set_property("--safe-area-inset-right", &right_value);
        let _ = element
            .style()
            .set_property("--safe-area-inset-bottom", &bottom_value);
        let _ = element
            .style()
            .set_property("--safe-area-inset-left", &left_value);
    };
    if let Some(app_root) = document_value
        .query_selector(".c_mobile_app_root")
        .ok()
        .flatten()
        .map(|element: Element| element.unchecked_into::<HtmlElement>())
        .or_else(|| {
            document_value
                .query_selector(".c_app_root")
                .ok()
                .flatten()
                .map(|element: Element| element.unchecked_into::<HtmlElement>())
        })
    {
        apply_to(&app_root);
    }
    if let Some(canvas_fullscreen) = document_value
        .query_selector(".c_canvas_container_fullscreen")
        .ok()
        .flatten()
        .map(|element: Element| element.unchecked_into::<HtmlElement>())
    {
        apply_to(&canvas_fullscreen);
    }
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
            overlay_stack_close();
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
