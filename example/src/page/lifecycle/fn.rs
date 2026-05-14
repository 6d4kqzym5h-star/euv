use crate::*;

/// Builds the log list virtual nodes from the reactive log signal.
fn build_log_nodes(logs_read: Signal<Vec<String>>) -> VirtualNode {
    let log_list: Vec<String> = logs_read.get();
    let mut children: Vec<VirtualNode> = Vec::new();
    for (index, log) in log_list.iter().enumerate() {
        let log_clone: String = log.clone();
        let index_clone: usize = index;
        let color: String = if index_clone == 0 {
            "#059669".to_string()
        } else {
            "#6b7280".to_string()
        };
        let weight: String = if index_clone == 0 {
            "600".to_string()
        } else {
            "400".to_string()
        };
        children.push(rsx! {
            div {
                key: index_clone.to_string()
                class: c_log_item()
                style: {color: {color}; font_weight: {weight};}
                log_clone
            }
        });
    }
    VirtualNode::Fragment(children)
}

/// A lifecycle demo page showing mount and update tracking.
///
/// # Returns
///
/// - `VirtualNode`: The lifecycle demo page virtual DOM tree.
pub fn page_lifecycle() -> VirtualNode {
    let render_count: Signal<i32> = use_signal(|| 1);
    let render_count_updater: Signal<i32> = render_count;
    let render_count_read: Signal<i32> = render_count;
    let logs: Signal<Vec<String>> = use_signal(|| vec!["Component mounted".to_string()]);
    let logs_updater: Signal<Vec<String>> = logs;
    let logs_read: Signal<Vec<String>> = logs;
    let increment_and_log = move || {
        let current: i32 = render_count_updater.get();
        render_count_updater.set(current + 1);
        let mut current_logs: Vec<String> = logs_updater.get();
        current_logs.push(format!("Updated: render count = {}", current + 1));
        logs_updater.set(current_logs);
    };
    rsx! {
        div {
            class: c_page_container()
            div {
                class: c_page_header()
                h1 {
                    class: c_page_title()
                    "Lifecycle"
                }
                p {
                    class: c_page_subtitle()
                    "Track component updates and lifecycle events."
                }
            }
            my_card {
                title: "Render Counter"
                p {
                    class: c_render_count_text()
                    "This page has been rendered "
                    span {
                        class: c_counter_value()
                        render_count_read
                    }
                    " times."
                }
                primary_button {
                    label: "Trigger Update"
                    onclick: move |_event: NativeEvent| {
                        increment_and_log();
                    }
                    "Trigger Update"
                }
            }
            my_card {
                title: "Event Log"
                div {
                    class: c_log_container()
                    build_log_nodes(logs_read)
                }
            }
        }
    }
}
