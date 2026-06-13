use crate::*;

/// A lifecycle demo page showing mount and update tracking.
///
/// # Returns
///
/// - `VirtualNode` - The lifecycle demo page virtual DOM tree.
#[component]
pub(crate) fn page_lifecycle(node: VirtualNode<PageLifecycleProps>) -> VirtualNode {
    let PageLifecycleProps = node.try_get_props().unwrap_or_default();
    let state: UseLifecycle = use_lifecycle();
    let render_count: Signal<i32> = state.get_render_count();
    let logs: Signal<Vec<String>> = state.get_logs();
    watch!(render_count, |render_count_value: i32| {
        Console::log(&format!(
            "watch! render count changed: {render_count_value}"
        ));
    });
    html! {
        div {
            class: c_page_container()
            page_header {
                title: "Lifecycle"
                subtitle: "Track component updates and lifecycle events."
            }
            my_card {
                title: "Render Counter"
                p {
                    class: c_render_count_text()
                    "This page has been rendered "
                    span {
                        class: c_counter_value()
                        render_count
                    }
                    " times."
                }
                primary_button {
                    label: "Update"
                    onclick: lifecycle_on_trigger(state)
                    "Update"
                }
            }
            my_card {
                title: "Event Log"
                div {
                    class: c_log_container()
                    for (index, log) in { logs.get().iter().enumerate() } {
                        div {
                            key: index.to_string()
                            class: c_log_item()
                            log.clone()
                        }
                    }
                }
            }
        }
    }
}
