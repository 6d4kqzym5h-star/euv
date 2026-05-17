use crate::*;

/// A lifecycle demo page showing mount and update tracking.
///
/// # Returns
///
/// - `VirtualNode` - The lifecycle demo page virtual DOM tree.
pub fn page_lifecycle() -> VirtualNode {
    let state: UseLifecycle = use_lifecycle();
    watch!(state.get_render_count(), |render_count_value| {
        Console::log(&format!(
            "watch! render count changed: {}",
            render_count_value
        ));
    });
    html! {
        div {
            class: c_page_container()
            { page_header("Lifecycle", "Track component updates and lifecycle events.") }
            my_card {
                title: "Render Counter"
                p {
                    class: c_render_count_text()
                    "This page has been rendered "
                    span {
                        class: c_counter_value()
                        state.get_render_count()
                    }
                    " times."
                }
                primary_button {
                    label: "Trigger Update"
                    onclick: lifecycle_on_trigger(state)
                    "Trigger Update"
                }
            }
            my_card {
                title: "Event Log"
                div {
                    class: c_log_container()
                    for (index, log) in { state.get_logs().get().iter().enumerate() } {
                        div {
                            key: index.to_string()
                            class: c_log_item()
                            style: { color: if { index == 0 } { "#059669".to_string() } else { "#6b7280".to_string() }; font-weight: if { index == 0 } { "600".to_string() } else { "400".to_string() }; }
                            log.clone()
                        }
                    }
                }
            }
        }
    }
}
