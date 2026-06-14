use crate::*;

/// A virtual list demo page showcasing high-performance rendering of 10,000 items.
///
/// Uses a windowed rendering strategy that only creates DOM nodes for the
/// currently visible items plus a small overscan buffer, keeping the DOM
/// node count constant regardless of total list size.
///
/// # Returns
///
/// - `VirtualNode` - The virtual list demo page virtual DOM tree.
#[component]
pub(crate) fn page_virtual_list(node: VirtualNode<PageVirtualListProps>) -> VirtualNode {
    let PageVirtualListProps = node.try_get_props().unwrap_or_default();
    let state: UseVirtualList = use_virtual_list();
    virtual_list_schedule_measure(state);
    use_window_event("resize", move || {
        virtual_list_update_viewport_height(state);
    });
    let scroll_offset: i32 = state.get_scroll_offset().get();
    let viewport_height: i32 = state.get_viewport_height().get();
    let (visible_start, visible_end, render_start, render_end): (usize, usize, usize, usize) =
        compute_visible_range(
            scroll_offset,
            viewport_height,
            VIRTUAL_LIST_TOTAL_COUNT,
            VIRTUAL_LIST_ITEM_HEIGHT,
            VIRTUAL_LIST_OVERSCAN_COUNT,
        );
    let total_height: i32 = VIRTUAL_LIST_TOTAL_COUNT as i32 * VIRTUAL_LIST_ITEM_HEIGHT;
    let top_padding: i32 = render_start as i32 * VIRTUAL_LIST_ITEM_HEIGHT;
    let visible_count: usize = visible_end - visible_start;
    html! {
        div {
            class: c_page_container_wide()
            page_header {
                icon: "📊"
                title: "Virtual List"
                subtitle: "High-performance windowed list rendering 10,000 items with minimal DOM nodes."
            }
            div {
                class: c_virtual_list_card()
                h3 {
                    class: c_card_title()
                    "10,000 Items Virtual Scroll"
                }
                div {
                    class: c_virtual_list_status()
                    span {
                        class: c_virtual_list_status_item()
                        "Total: "
                        span {
                            class: c_virtual_list_status_value()
                            VIRTUAL_LIST_TOTAL_COUNT.to_string()
                        }
                    }
                    span {
                        class: c_virtual_list_status_item()
                        "Visible: "
                        span {
                            class: c_virtual_list_status_value()
                            visible_count.to_string()
                        }
                    }
                    span {
                        class: c_virtual_list_status_item()
                        "Range: "
                        span {
                            class: c_virtual_list_status_value()
                            format!("{visible_start}-{}", visible_end.saturating_sub(1))
                        }
                    }
                }
                div {
                    class: c_virtual_list_container()
                    id: VIRTUAL_LIST_CONTAINER_ID
                    onscroll: virtual_list_on_scroll(state)
                    div {
                        style: format!("position: relative; height: {total_height}px;")
                        div {
                            style: format!("position: absolute; top: {top_padding}px; left: 0; right: 0;")
                            for index in { render_start..render_end } {
                                div {
                                    key: index.to_string()
                                    class: if { index % 2 == 0 } { c_virtual_list_row_even() } else { c_virtual_list_row_odd() }
                                    style: format!("height: {}px; box-sizing: border-box;", VIRTUAL_LIST_ITEM_HEIGHT)
                                    span {
                                        class: c_virtual_list_row_index()
                                        index.to_string()
                                    }
                                    span {
                                        class: c_virtual_list_row_label()
                                        format!("Item #{index}")
                                    }
                                    span {
                                        class: c_virtual_list_row_description()
                                        format!("Description for item {index}")
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
