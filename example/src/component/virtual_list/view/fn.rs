use crate::*;

/// Creates a scroll event handler that tracks the container scroll position and viewport height.
///
/// Reads `scrollTop` and `clientHeight` from the scroll container element
/// referenced by `VIRTUAL_LIST_CONTAINER_ID` and updates the corresponding signals.
///
/// # Arguments
///
/// - `UseVirtualList` - The virtual list state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - A scroll handler for the virtual list container.
pub(crate) fn virtual_list_on_scroll(state: UseVirtualList) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        if let Some(container) = virtual_list_container() {
            let html_element: HtmlElement = container.unchecked_into();
            state.get_scroll_offset().set(html_element.scroll_top());
            state
                .get_viewport_height()
                .set(html_element.client_height());
        }
    }))
}

/// Reads the container `clientHeight` and writes it to the viewport height signal.
///
/// Should only be called when the DOM is already present (e.g. inside a resize
/// callback or after the first paint). For initial mount use
/// `virtual_list_schedule_measure` instead.
///
/// # Arguments
///
/// - `UseVirtualList` - The virtual list state.
pub(crate) fn virtual_list_update_viewport_height(state: UseVirtualList) {
    if let Some(container) = virtual_list_container() {
        let html_element: HtmlElement = container.unchecked_into();
        state
            .get_viewport_height()
            .set(html_element.client_height());
    }
}

/// Schedules a viewport height measurement on the next animation frame.
///
/// Defers the measurement via `requestAnimationFrame` so the DOM has been
/// fully rendered before reading `clientHeight`. This is necessary on initial
/// mount because the component render function executes before the DOM nodes
/// are inserted. After the signal update, a re-render is automatically
/// triggered to correct the visible range.
///
/// # Arguments
///
/// - `UseVirtualList` - The virtual list state.
pub(crate) fn virtual_list_schedule_measure(state: UseVirtualList) {
    let callback: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        virtual_list_update_viewport_height(state);
    }));
    window()
        .expect("no global window exists")
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .expect("failed to request animation frame");
    callback.forget();
}

/// Returns the virtual list container element by its id.
///
/// # Returns
///
/// - `Option<Element>` - The container element, if found in the document.
fn virtual_list_container() -> Option<Element> {
    window()
        .expect("no global window exists")
        .document()
        .expect("should have a document")
        .get_element_by_id(VIRTUAL_LIST_CONTAINER_ID)
}

/// Computes the range of visible item indices for the virtual list.
///
/// Calculates the start and end indices based on the current scroll offset,
/// viewport height, fixed item height, and total item count. Includes an
/// overscan buffer to reduce blank areas during fast scrolling.
///
/// # Arguments
///
/// - `i32` - The current scroll offset in pixels.
/// - `i32` - The current viewport height in pixels.
/// - `usize` - The total number of items in the list.
/// - `i32` - The fixed height of each item in pixels.
/// - `usize` - The number of overscan items to render beyond the viewport.
///
/// # Returns
///
/// - `(usize, usize, usize, usize)` - A tuple of (visible_start, visible_end, render_start, render_end).
///   visible_start/visible_end represent the actual visible range without overscan.
///   render_start/render_end represent the rendering range including overscan.
pub(crate) fn compute_visible_range(
    scroll_offset: i32,
    viewport_height: i32,
    total_count: usize,
    item_height: i32,
    overscan_count: usize,
) -> (usize, usize, usize, usize) {
    let visible_start: usize = (scroll_offset / item_height).max(0) as usize;
    let visible_count: usize = if viewport_height > 0 {
        let viewport_bottom: i32 = scroll_offset + viewport_height;
        let visible_end: usize =
            ((viewport_bottom + item_height - 1) / item_height).max(0) as usize;
        visible_end - visible_start
    } else {
        VIRTUAL_LIST_DEFAULT_VISIBLE_COUNT
    };
    let visible_end: usize = (visible_start + visible_count).min(total_count);
    let render_start: usize = visible_start.saturating_sub(overscan_count);
    let render_end: usize = (visible_end + overscan_count).min(total_count);
    (visible_start, visible_end, render_start, render_end)
}
