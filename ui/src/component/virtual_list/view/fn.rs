use crate::*;

/// A high-performance virtual list component for rendering large datasets.
///
/// Only renders the visible items plus an overscan buffer, keeping DOM node count
/// constant regardless of total list size. Supports multiple instances on the same page
/// through unique container ids.
///
/// # Arguments
///
/// - `VirtualNode<EuvVirtualListProps>`: The props node containing configuration, item renderer, and optional callbacks.
///
/// # Returns
///
/// - `VirtualNode`: The virtual list container with windowed rendering.
#[component]
pub fn euv_virtual_list(node: VirtualNode<EuvVirtualListProps>) -> VirtualNode {
    let EuvVirtualListProps {
        config,
        item_renderer,
        on_scroll,
        on_visible_range_change,
    } = node.try_get_props().unwrap_or_default();
    let state: UseVirtualList = UseVirtualList::use_scroll_state();
    let container_id: String = config.id.clone();
    let total_count: usize = config.total_count;
    let item_height: i32 = config.item_height;
    let overscan_count: usize = config.overscan_count;
    state.schedule_measure_by_id(&container_id);
    let scroll_handler: Option<Rc<dyn Fn(Event)>> = {
        let state: UseVirtualList = state;
        let container_id: String = container_id.clone();
        let on_scroll: Option<VirtualListScrollHandler> = on_scroll;
        Some(Rc::new(move |_: Event| {
            if let Some(element) = UseVirtualList::try_get_container_by_id(&container_id) {
                let html_element: HtmlElement = element.unchecked_into();
                let scroll_offset: i32 = html_element.scroll_top();
                state.get_scroll_offset().set(scroll_offset);
                if let Some(ref callback) = on_scroll {
                    callback(scroll_offset);
                }
            }
        }))
    };
    let scroll_offset: i32 = state.get_scroll_offset().get();
    let viewport_height: i32 = state.get_viewport_height().get();
    let (visible_start, visible_end, render_start, render_end): (usize, usize, usize, usize) =
        UseVirtualList::compute_visible_range(
            scroll_offset,
            viewport_height,
            total_count,
            item_height,
            overscan_count,
        );
    if let Some(ref callback) = on_visible_range_change {
        callback((visible_start, visible_end));
    }
    let total_height: i32 = total_count as i32 * item_height;
    let top_padding: i32 = render_start as i32 * item_height;
    let children: Vec<VirtualNode> = (render_start..render_end)
        .map(|index: usize| {
            let item_node: VirtualNode = (item_renderer)(index);
            html! {
                div {
                    key: index.to_string()
                    style: format!("height: {item_height}px; box-sizing: border-box;")
                    item_node
                }
            }
        })
        .collect();
    html! {
        div {
            class: c_virtual_list_container()
            id: container_id
            onscroll: scroll_handler
            div {
                style: format!("position: relative; height: {total_height}px;")
                div {
                    style: format!("position: absolute; top: {top_padding}px; left: 0; right: 0;")
                    children
                }
            }
        }
    }
}
