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
    Some(Rc::new(move |_event: Event| {
        let window_value: Window = window().expect("no global window exists");
        let document_value: Document = window_value.document().expect("should have a document");
        if let Some(container) = document_value.get_element_by_id(VIRTUAL_LIST_CONTAINER_ID) {
            let html_element: HtmlElement = container.unchecked_into();
            state.get_scroll_offset().set(html_element.scroll_top());
            state
                .get_viewport_height()
                .set(html_element.client_height());
        }
    }))
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
/// - `(usize, usize)` - A tuple of (start_index, end_index) for the visible range.
pub(crate) fn compute_visible_range(
    scroll_offset: i32,
    viewport_height: i32,
    total_count: usize,
    item_height: i32,
    overscan_count: usize,
) -> (usize, usize) {
    let start_index: usize = (scroll_offset / item_height).max(0) as usize;
    let visible_count: usize = if viewport_height > 0 {
        (viewport_height / item_height) as usize + 1
    } else {
        20
    };
    let overscanned_start: usize = start_index.saturating_sub(overscan_count);
    let overscanned_end: usize = (start_index + visible_count + overscan_count).min(total_count);
    (overscanned_start, overscanned_end)
}
